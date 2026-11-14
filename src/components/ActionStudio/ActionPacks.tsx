import { useEffect, useRef, useState } from 'react';
import { errorText, ipc } from '../../ipc';
import type { ActionDescriptor, PackCollision, PackPreview } from '../../types/actions';
import { Field } from './Fields';

export default function ActionPacks({ selected, hasEnabled, onImport, close }: {
  selected: ActionDescriptor | undefined;
  hasEnabled: boolean;
  onImport: (ids: string[]) => Promise<void>;
  close: () => void;
}) {
  const dialog = useRef<HTMLDialogElement>(null);
  const [scope, setScope] = useState(selected ? 'selected' : 'enabled');
  const [incoming, setIncoming] = useState<{ document: string; preview: PackPreview } | null>(null);
  const [busy, setBusy] = useState(false);
  const [notice, setNotice] = useState<{ text: string; error?: boolean } | null>(null);
  const canExport = scope === 'selected' ? !!selected : hasEnabled;

  useEffect(() => { dialog.current?.showModal(); }, []);

  async function run(action: () => Promise<void>) {
    setBusy(true);
    setNotice(null);
    try { await action(); }
    catch (reason) { setNotice({ text: errorText(reason), error: true }); }
    finally { setBusy(false); }
  }

  function read(clipboard: boolean) {
    setIncoming(null);
    void run(async () => {
      const document = await ipc.readActionPack(clipboard);
      if (document !== null) setIncoming({ document, preview: await ipc.previewActionPack(document) });
    });
  }

  function importPack(collision: PackCollision) {
    if (!incoming) return;
    void run(async () => {
      const ids = await ipc.importActionPack(incoming.document, collision);
      setIncoming(null);
      await onImport(ids);
      setNotice({ text: `Imported ${ids.length} ${ids.length === 1 ? 'action' : 'actions'}.` });
    });
  }

  function exportPack(clipboard: boolean) {
    void run(async () => {
      const exported = await ipc.exportActionPack(scope === 'selected' ? selected?.id ?? null : null, clipboard);
      if (exported) setNotice({ text: clipboard ? 'Action pack copied to clipboard.' : 'Action pack exported.' });
    });
  }

  return <dialog ref={dialog} className="studio-dialog" aria-labelledby="packs-title" onCancel={event => { event.preventDefault(); if (!busy) close(); }}>
    <div className="packs-content">
      <h2 id="packs-title">Action Packs</h2>
      <fieldset disabled={busy || !!incoming}>
        <legend>Export</legend>
        <Field label="Actions to export">
          <select value={scope} onChange={event => setScope(event.target.value)}>
            {selected && <option value="selected">{selected.label}</option>}
            <option value="enabled">All enabled actions</option>
          </select>
        </Field>
        <div className="footer-buttons">
          <button type="button" className="button secondary" disabled={!canExport} onClick={() => exportPack(true)}>Copy JSON</button>
          <button type="button" className="button secondary" disabled={!canExport} onClick={() => exportPack(false)}>Save JSON file</button>
        </div>
      </fieldset>
      <fieldset disabled={busy}>
        <legend>Import</legend>
        {incoming ? <>
          <p>{incoming.preview.count} {incoming.preview.count === 1 ? 'action' : 'actions'} ready to import. Review their commands and URLs in Studio before running them.</p>
          {incoming.preview.collisions.length > 0 && <p className="pack-collisions">Existing IDs that match<br />{incoming.preview.collisions.join(', ')}</p>}
          <div className="footer-buttons">
            <button type="button" className="button secondary" onClick={() => { setIncoming(null); setNotice(null); }}>Cancel import</button>
            {incoming.preview.collisions.length > 0 ? <>
              <button type="button" className="button secondary" onClick={() => importPack('overwrite')}>Overwrite matching IDs</button>
              <button type="button" className="button accent" onClick={() => importPack('duplicate')}>Keep both</button>
            </> : <button type="button" className="button accent" onClick={() => importPack('reject')}>Import actions</button>}
          </div>
        </> : <div className="footer-buttons">
          <button type="button" className="button secondary" onClick={() => read(true)}>Paste JSON</button>
          <button type="button" className="button secondary" onClick={() => read(false)}>Open JSON file</button>
        </div>}
      </fieldset>
      {notice && <p className={`save-status ${notice.error ? 'invalid' : ''}`} role={notice.error ? 'alert' : 'status'}>{notice.text}</p>}
      <div className="footer-buttons"><button type="button" className="button secondary" onClick={close} disabled={busy}>Done</button></div>
    </div>
  </dialog>;
}
