import { useEffect, useRef, useState } from 'react';
import { errorText, ipc } from '../../ipc';
import type { AppSettings, ModifierKey } from '../../types/actions';
import { Field } from './Fields';

export default function SettingsPanel({ settings, close }: { settings: AppSettings; close: () => void }) {
  const dialog = useRef<HTMLDialogElement>(null);
  const [modifierKey, setModifierKey] = useState<ModifierKey>(settings.modifierKey);
  const [paths, setPaths] = useState(settings.customPaths.join('\n'));
  const [overrides, setOverrides] = useState(Object.entries(settings.toolOverrides).map(([tool, path]) => ({ tool, path })));
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState('');

  useEffect(() => { dialog.current?.showModal(); }, []);

  async function save() {
    setBusy(true);
    setError('');
    try {
      const entries = overrides.map(({ tool, path }) => [tool.trim(), path.trim()] as const);
      const toolOverrides = Object.fromEntries(entries);
      if (Object.keys(toolOverrides).length !== entries.length) throw new Error('Use one override per tool.');
      await ipc.saveSettings({ ...settings, modifierKey, customPaths: paths.split('\n').map(path => path.trim()).filter(Boolean), toolOverrides });
      close();
    } catch (reason) {
      setError(errorText(reason));
    } finally {
      setBusy(false);
    }
  }

  return <dialog ref={dialog} className="studio-dialog" aria-labelledby="settings-title" onCancel={event => { event.preventDefault(); if (!busy) close(); }}>
    <form onSubmit={event => { event.preventDefault(); void save(); }}>
      <h2 id="settings-title">Settings &amp; Paths</h2>
      <fieldset disabled={busy}>
        <Field label="Drag modifier">
          <select value={modifierKey} onChange={event => setModifierKey(event.target.value as ModifierKey)}>
            <option>Shift</option><option>Ctrl</option><option>Alt</option>
          </select>
        </Field>
        <Field label="Custom executable directories" hint="One absolute directory per line, searched in order before system PATH.">
          <textarea className="mono" rows={4} spellCheck={false} value={paths} onChange={event => setPaths(event.target.value)} />
        </Field>
        <fieldset className="field">
          <legend>Tool overrides</legend>
          {overrides.map(({ tool, path }, index) => <div className="tool-override" key={index}>
            <input className="mono" placeholder="Tool name" aria-label="Tool name" required value={tool}
              onChange={event => setOverrides(overrides.map((entry, row) => row === index ? { ...entry, tool: event.target.value } : entry))} />
            <input className="mono" placeholder="Absolute executable path" aria-label="Absolute executable path" required value={path}
              onChange={event => setOverrides(overrides.map((entry, row) => row === index ? { ...entry, path: event.target.value } : entry))} />
            <button type="button" className="button secondary" onClick={() => setOverrides(overrides.filter((_, row) => row !== index))}>Remove</button>
          </div>)}
          <button className="button secondary" type="button" onClick={() => setOverrides([...overrides, { tool: '', path: '' }])}>Add override</button>
        </fieldset>
      </fieldset>
      {error && <p className="save-status invalid" role="alert">{error}</p>}
      <div className="footer-buttons">
        <button className="button secondary" type="button" onClick={close} disabled={busy}>Cancel</button>
        <button className="button accent" type="submit" disabled={busy}>Save settings</button>
      </div>
    </form>
  </dialog>;
}
