import { Arguments, Extensions, Field, Section, StringMap } from './Fields';
import { executors, platformLabels, platforms } from './manifest';
import type { ActionDescriptor, Cardinality, ExecutorConfig, OutputMode } from '../../types/actions';
import WebhookSecret from './WebhookSecret';

const optional = (value: string) => value || undefined;

export default function VisualEditor({ action, isNew, update, onFieldError }: {
  action: ActionDescriptor;
  isNew: boolean;
  update: (next: ActionDescriptor) => void;
  onFieldError: (message: string) => void;
}) {
  const executor = action.executor;
  const output = action.output;
  function patch(fields: Partial<ActionDescriptor>) {
    update({ ...action, ...fields });
  }
  function changeExecutor(executor: ExecutorConfig) {
    patch({ executor });
  }
  return <div className="visual-editor">
    <Section title="Details" detail="Name and describe the action as it appears on the wheel.">
      <div className="field-grid">
        <Field label="Action label"><input value={action.label} placeholder="Name this action" onChange={event => patch({ label: event.target.value })} /></Field>
        <Field label="Icon"><input value={action.icon ?? ''} placeholder="e.g. terminal" onChange={event => patch({ icon: optional(event.target.value) })} /></Field>
      </div>
      <Field label="Description"><textarea rows={2} value={action.description ?? ''} placeholder="What does this action do?" onChange={event => patch({ description: optional(event.target.value) })} /></Field>
      <Field label="Action ID" hint={isNew ? 'Use lowercase letters, digits, dots, dashes, or underscores.' : undefined}>
        <input className="mono" readOnly={!isNew} value={action.id} onChange={event => patch({ id: event.target.value })} />
      </Field>
    </Section>
    <Section title="Matching" detail="Choose which files and platforms can use this action.">
      <Extensions values={action.matching.extensions} update={extensions => patch({ matching: { ...action.matching, extensions } })} />
      <Field label="Selection mode">
        <select value={action.matching.cardinality} onChange={event => patch({ matching: { ...action.matching, cardinality: event.target.value as Cardinality } })}>
          <option value="single">One file</option>
          <option value="each">Each file separately</option>
          <option value="batch">All files in one run</option>
        </select>
      </Field>
      <fieldset className="field">
        <legend>Platforms</legend>
        <div className="platforms">
          {platforms.map(platform => <label key={platform}>
            <input type="checkbox" checked={action.platforms.includes(platform)} onChange={event => patch({
              platforms: event.target.checked ? [...action.platforms, platform] : action.platforms.filter(value => value !== platform),
            })} />{platformLabels[platform]}
          </label>)}
        </div>
      </fieldset>
    </Section>
    <Section title="Executor" detail="Configure the command, script, or HTTP request to run.">
      <div className="field-grid">
        <Field label="Executor type">
          <select value={executor.type} onChange={event => {
            onFieldError('');
            changeExecutor(executors[event.target.value as ExecutorConfig['type']]());
          }}>
            <option value="process">Process</option><option value="script">Script</option><option value="webhook">Webhook</option>
          </select>
        </Field>
        <Field label="Timeout (seconds)">
          <input type="number" min={1} max={4294967295} value={executor.timeoutSeconds ?? ''} placeholder="60"
            onChange={event => changeExecutor({ ...executor, timeoutSeconds: event.target.value ? Number(event.target.value) : undefined })} />
        </Field>
      </div>
      {executor.type === 'process' && <>
        <Field label="Program"><input className="mono" value={executor.program} onChange={event => changeExecutor({ ...executor, program: event.target.value })} /></Field>
        <Arguments values={executor.args ?? []} update={args => changeExecutor({ ...executor, args })} />
        <Field label="Working directory"><input className="mono" value={executor.workingDirectory ?? ''} onChange={event => changeExecutor({ ...executor, workingDirectory: optional(event.target.value) })} /></Field>
        <StringMap label="Environment variables" value={executor.env} onError={onFieldError} onChange={env => changeExecutor({ ...executor, env })} />
      </>}
      {executor.type === 'script' && <>
        <div className="field-grid">
          <Field label="Interpreter"><input className="mono" value={executor.interpreter} onChange={event => changeExecutor({ ...executor, interpreter: event.target.value })} /></Field>
          <Field label="Script source">
            <select value={executor.scriptPath === undefined ? 'inline' : 'path'} onChange={event => changeExecutor({
              ...executor, scriptBody: event.target.value === 'inline' ? '' : undefined, scriptPath: event.target.value === 'path' ? '' : undefined,
            })}>
              <option value="inline">Inline script</option><option value="path">Script file</option>
            </select>
          </Field>
        </div>
        {executor.scriptPath === undefined
          ? <Field label="Script body"><textarea className="mono" rows={6} spellCheck={false} value={executor.scriptBody ?? ''} onChange={event => changeExecutor({ ...executor, scriptBody: event.target.value })} /></Field>
          : <Field label="Script path"><input className="mono" value={executor.scriptPath} onChange={event => changeExecutor({ ...executor, scriptPath: event.target.value })} /></Field>}
      </>}
      {executor.type === 'webhook' && <>
        <div className="field-grid webhook-url">
          <Field label="Method"><input className="mono" value={executor.method} onChange={event => changeExecutor({ ...executor, method: event.target.value.toUpperCase() })} /></Field>
          <Field label="URL"><input type="url" value={executor.url} placeholder="https://" onChange={event => changeExecutor({ ...executor, url: event.target.value })} /></Field>
        </div>
        <div className="field-grid">
          <Field label="File field name"><input value={executor.fileFieldName ?? ''} placeholder="file" onChange={event => changeExecutor({ ...executor, fileFieldName: optional(event.target.value) })} /></Field>
          <Field label="Credential key"><input value={executor.credentialKey ?? ''} onChange={event => changeExecutor({ ...executor, credentialKey: optional(event.target.value) })} /></Field>
        </div>
        <StringMap label="Headers" value={executor.headers} onError={onFieldError} onChange={headers => changeExecutor({ ...executor, headers })} />
        {executor.credentialKey?.trim() && <WebhookSecret key={executor.credentialKey} credentialKey={executor.credentialKey} />}
      </>}
    </Section>
    <Section title="Output" detail="Choose where the result should go.">
      <Field label="Output mode">
        <select value={output.mode} onChange={event => patch({ output: { ...output, mode: event.target.value as OutputMode } })}>
          <option value="none">None</option><option value="file">File</option><option value="clipboard">Clipboard</option><option value="stdout">Standard output</option>
        </select>
      </Field>
      <div className="field-grid">
        <Field label="Output extension"><input value={output.extension ?? ''} placeholder="Same as input" onChange={event => patch({ output: { ...output, extension: optional(event.target.value) } })} /></Field>
        <Field label="Filename suffix"><input value={output.suffix ?? ''} onChange={event => patch({ output: { ...output, suffix: optional(event.target.value) } })} /></Field>
      </div>
      <label className="checkbox"><input type="checkbox" checked={output.overwriteOriginal ?? false}
        onChange={event => patch({ output: { ...output, overwriteOriginal: event.target.checked } })} /> Overwrite the original file</label>
    </Section>
  </div>;
}
