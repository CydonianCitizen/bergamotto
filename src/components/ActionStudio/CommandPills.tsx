import { Terminal } from 'lucide-react';
import type { ActionDescriptor } from '../../types/actions';

export const VARIABLE_PILLS = [
  { token: '${input}', label: 'Input File', desc: 'Path to single input file' },
  { token: '${inputs}', label: 'All Input Files', desc: 'All selected input paths' },
  { token: '${output}', label: 'Output File', desc: 'Staged output target path' },
  { token: '${input_dir}', label: 'File Folder', desc: 'Directory containing input file' },
  { token: '${input_stem}', label: 'File Name (No Ext)', desc: 'Filename without extension' },
  { token: '${input_ext}', label: 'File Extension', desc: 'File extension (e.g. mp4)' },
  { token: '${work_dir}', label: 'Temp Work Dir', desc: 'Isolated scratch folder' },
] as const;

export function VariablePills({
  onInsert,
}: {
  onInsert: (token: string) => void;
}) {
  return (
    <div className="variable-pills-bar">
      <span className="pills-title">Insert Variable:</span>
      <div className="pills-container">
        {VARIABLE_PILLS.map(pill => (
          <button
            key={pill.token}
            type="button"
            className="pill-button"
            title={`${pill.token} — ${pill.desc}`}
            onClick={() => onInsert(pill.token)}
          >
            <span className="pill-plus">+</span>
            <span className="pill-label">{pill.label}</span>
            <code className="pill-code">{pill.token}</code>
          </button>
        ))}
      </div>
    </div>
  );
}

export function CommandPreview({
  action,
  samplePath = '/Users/alex/Photos/vacation.mov',
}: {
  action: ActionDescriptor;
  samplePath?: string;
}) {
  const parts = samplePath.split(/[/\\]/);
  const fileName = parts.at(-1) ?? 'file.ext';
  const dotIndex = fileName.lastIndexOf('.');
  const inputStem = dotIndex > 0 ? fileName.slice(0, dotIndex) : fileName;
  const inputExt = dotIndex > 0 ? fileName.slice(dotIndex + 1) : '';
  const inputDir = parts.slice(0, -1).join('/') || '/Users/alex/Photos';

  const outputExt = action.output.extension || inputExt || 'mp4';
  const suffix = action.output.suffix ?? '';
  const outputPath =
    action.output.mode === 'file'
      ? `${inputDir}/${inputStem}${suffix}.${outputExt}`
      : undefined;

  function interpolate(arg: string): string {
    return arg
      .replaceAll('${input}', samplePath)
      .replaceAll('${inputs}', samplePath)
      .replaceAll('${input_dir}', inputDir)
      .replaceAll('${input_stem}', inputStem)
      .replaceAll('${input_ext}', inputExt)
      .replaceAll('${output}', outputPath ?? `${inputDir}/${inputStem}-out.${outputExt}`)
      .replaceAll('${work_dir}', '/tmp/.bergamot-scratch');
  }

  let previewText = '';

  if (action.executor.type === 'process') {
    const program = action.executor.program || 'program';
    const args = (action.executor.args ?? []).map(interpolate);
    const escaped = args.map(a => (a.includes(' ') || a.includes('"') ? `"${a}"` : a));
    previewText = `${program} ${escaped.join(' ')}`.trim();
  } else if (action.executor.type === 'script') {
    const interpreter = action.executor.interpreter || 'bash';
    if (action.executor.scriptPath) {
      previewText = `${interpreter} ${action.executor.scriptPath} "${samplePath}"`;
    } else {
      const body = action.executor.scriptBody ?? '';
      previewText = `# Runs via ${interpreter} with $1="${samplePath}"\n${body.slice(0, 160)}${body.length > 160 ? '\n…' : ''}`;
    }
  } else if (action.executor.type === 'webhook') {
    const method = action.executor.method || 'POST';
    const url = action.executor.url || 'https://api.example.com/upload';
    const field = action.executor.fileFieldName ?? 'file';
    previewText = `curl -X ${method} "${url}" -F "${field}=@${samplePath}"`;
  }

  return (
    <div className="command-preview-card">
      <div className="preview-header">
        <span className="preview-label">
          <Terminal size={14} /> Live Command Preview
        </span>
        <span className="preview-sample">
          Sample: <code>{fileName}</code>
        </span>
      </div>
      <pre className="preview-code mono">{previewText || '(Incomplete configuration)'}</pre>
      {outputPath && action.output.mode === 'file' && (
        <div className="preview-output-target">
          <span>Target output:</span> <code>{outputPath}</code>
        </div>
      )}
    </div>
  );
}
