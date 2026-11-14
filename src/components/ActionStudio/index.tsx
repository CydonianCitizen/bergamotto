import { listen } from '@tauri-apps/api/event';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { open } from '@tauri-apps/plugin-dialog';
import { Aperture, Braces, Check, CircleAlert, Copy, FolderOpen, Layers3, Play, Plus, Save, SlidersHorizontal, Trash2, Upload } from 'lucide-react';
import { useEffect, useRef, useState } from 'react';
import { errorText, ipc } from '../../ipc';
import type { ActionDescriptor } from '../../types/actions';
import { ActionIcon } from '../ActionIcon';
import { formatManifest, inspectManifest, newAction, validateManifest } from './manifest';
import VisualEditor from './VisualEditor';

type Tab = 'visual' | 'json';

export default function ActionStudio() {
  const [actions, setActions] = useState<ActionDescriptor[]>([]);
  const [saved, setSaved] = useState<ActionDescriptor | null>(null);
  const [draft, setDraft] = useState<ActionDescriptor | null>(null);
  const [raw, setRaw] = useState('');
  const [tab, setTab] = useState<Tab>('visual');
  const [editorVersion, setEditorVersion] = useState(0);
  const [fieldError, setFieldError] = useState('');
  const [busy, setBusy] = useState(false);
  const [loading, setLoading] = useState(true);
  const [notice, setNotice] = useState<{ message: string; error?: boolean } | null>(null);
  const [dragging, setDragging] = useState(false);
  const [globalDrag, setGlobalDrag] = useState<boolean | null>(null);
  const [confirmDelete, setConfirmDelete] = useState(false);
  const listRevision = useRef(0);
  const scrollArea = useRef<HTMLDivElement>(null);
  const savedId = saved?.id;
  const current = actions.find(action => action.id === savedId);
  const json = tab === 'json' ? inspectManifest(raw) : null;
  const candidate = json ? json.action : draft;
  const issues = json ? json.issues : draft ? validateManifest(JSON.parse(formatManifest(draft))) : [];
  const idChanged = Boolean(candidate && savedId && candidate.id !== savedId);
  const dirty = !!draft && (!saved || (tab === 'json' ? raw !== formatManifest(saved) : formatManifest(draft) !== formatManifest(saved)));
  const conflict = Boolean(saved && (!current || formatManifest(current) !== formatManifest(saved)));
  const canShowVisual = !!json?.action || (!!draft && raw === formatManifest(draft));

  function report(error: unknown) {
    setNotice({ message: errorText(error), error: true });
  }

  async function refresh() {
    const request = ++listRevision.current;
    const next = await ipc.listActions();
    if (request === listRevision.current) setActions(next);
    return next;
  }

  function select(action: ActionDescriptor, existing = true) {
    setSaved(existing ? action : null);
    setDraft(action);
    setRaw(formatManifest(action));
    setTab('visual');
    setFieldError('');
    setEditorVersion(version => version + 1);
    setNotice(null);
    setConfirmDelete(false);
    scrollArea.current?.scrollTo(0, 0);
  }

  useEffect(() => {
    let alive = true;
    let changed = false;
    const hook = listen<boolean>('global-drag:changed', ({ payload }) => {
      changed = true;
      if (alive) setGlobalDrag(payload);
    });
    void hook.then(ipc.globalDragEnabled).then(enabled => {
      if (alive && !changed) setGlobalDrag(enabled);
    }).catch(report);
    const registry = listen('actions:updated', () => void refresh().catch(report));
    void registry.then(refresh).catch(report).finally(() => setLoading(false));
    const drops = getCurrentWindow().onDragDropEvent(({ payload }) => {
      setDragging(payload.type === 'enter' || payload.type === 'over');
      if (payload.type === 'drop') void ipc.presentWheel(payload.paths).catch(report);
    });
    void drops.catch(report);
    return () => {
      alive = false;
      for (const task of [registry, drops, hook]) void task.then(unlisten => unlisten()).catch(() => {});
    };
  }, []);

  useEffect(() => {
    if (!draft && actions[0]) select(actions[0]);
    else if (conflict && !dirty) {
      if (current) select(current);
      else {
        setSaved(null);
        setDraft(null);
      }
    }
  }, [actions, conflict, current, dirty, draft]);

  function changeTab(next: Tab) {
    if (next === tab || !draft) return;
    if (next === 'json') setRaw(formatManifest(draft));
    else if (json?.action) setDraft(json.action);
    setTab(next);
  }

  async function task(operation: () => Promise<void>) {
    setBusy(true);
    setNotice(null);
    try {
      await operation();
    } catch (error) {
      report(error);
    } finally {
      setBusy(false);
    }
  }

  async function save() {
    if (!candidate || issues.length || idChanged || conflict || fieldError) return;
    await (saved ? ipc.saveAction(candidate) : ipc.createAction(candidate));
    const updated = (await refresh()).find(action => action.id === candidate.id) ?? candidate;
    setSaved(updated);
    setDraft(updated);
    setRaw(formatManifest(updated));
    setNotice({ message: 'Action saved' });
  }

  async function toggle() {
    if (!draft) return;
    if (!saved || dirty) {
      const next = { ...(candidate ?? draft), enabled: !(candidate ?? draft).enabled };
      if (tab === 'json') setRaw(formatManifest(next));
      else setDraft(next);
      return;
    }
    const next = { ...saved, enabled: !saved.enabled };
    await ipc.saveAction(next);
    const updated = (await refresh()).find(action => action.id === next.id) ?? next;
    setSaved(updated);
    setDraft(updated);
    setRaw(formatManifest(updated));
    setNotice({ message: updated.enabled ? 'Action enabled' : 'Action disabled' });
  }

  async function duplicate() {
    if (!savedId) return;
    const copy = await ipc.duplicateAction(savedId);
    await refresh();
    select(copy);
    setNotice({ message: 'Action duplicated' });
  }

  async function remove() {
    if (!savedId) return;
    await ipc.deleteAction(savedId);
    const remaining = await refresh();
    if (remaining[0]) select(remaining[0]);
    else {
      setDraft(null);
      setSaved(null);
    }
    setConfirmDelete(false);
    setNotice({ message: 'Action deleted' });
  }

  async function testOnWheel() {
    const paths = await open({ multiple: true, title: 'Choose files for the radial wheel' });
    if (paths?.length) await ipc.presentWheel(paths);
  }

  const enabled = actions.filter(action => action.enabled).length;
  const blocked = issues.length > 0 || Boolean(fieldError) || idChanged || conflict;
  const visibleAction = candidate ?? draft;

  return (
    <main className="studio">
      <header className="studio-header">
        <div className="brand"><span className="brand-mark"><Aperture size={25} strokeWidth={1.7} /></span><span>Bergamotto</span></div>
        <div className="top-actions">
          <button className="button secondary" type="button" role="switch" aria-label="Global Shift+Drag Hook"
            aria-checked={globalDrag ?? false} disabled={globalDrag === null} onClick={() => void ipc.toggleGlobalDrag().catch(report)}>
            Global Shift+Drag Hook{globalDrag === null ? '' : globalDrag ? ': Enabled' : ': Disabled'}
          </button>
          <button className="button secondary" onClick={() => void task(ipc.openActionsFolder)} disabled={busy}><FolderOpen size={16} /> Actions folder</button>
          <button className="button accent" onClick={() => void task(testOnWheel)} disabled={busy}><Play size={15} /> Test on Wheel</button>
        </div>
      </header>
      <div className="studio-layout">
        <aside className="sidebar" aria-label="Actions">
          <div className="sidebar-heading"><div><h2>Actions</h2><p>{actions.length} total · {enabled} enabled</p></div>
            <button type="button" aria-label="New action" title="New action" className="icon-button add-action" disabled={busy} onClick={() => select(newAction(), false)}><Plus size={20} /></button>
          </div>
          <nav className="action-list" aria-label="Action library">
            {!savedId && draft && <div className="action-row selected"><span className="list-icon"><Plus size={17} /></span><span className="row-copy"><strong>New action</strong><small>Unsaved</small></span></div>}
            {actions.map(action => <button key={action.id} type="button" className={`action-row ${savedId === action.id ? 'selected' : ''}`}
              aria-current={savedId === action.id ? 'true' : undefined} disabled={busy} onClick={() => select(action)}>
              <span className="list-icon"><ActionIcon icon={action.icon} type={action.executor.type} /></span>
              <span className="row-copy"><strong>{action.label}</strong><small>{action.executor.type}</small></span>
              <span className={`enabled-dot ${action.enabled ? 'on' : ''}`} aria-label={action.enabled ? 'Enabled' : 'Disabled'} />
            </button>)}
            {!actions.length && !draft && <p className="sidebar-empty">{loading ? 'Loading actions…' : 'No saved actions'}</p>}
          </nav>
          <div className={`drop-target ${dragging ? 'dragging' : ''}`}><Upload size={17} />
            <div><strong>Drop sample files</strong><span>Open the radial wheel with your selection.</span></div>
          </div>
        </aside>
        <section className="editor" aria-label="Action editor">
          <div className="editor-header">
            <div className="editor-heading">
              <div><h1>Action Studio</h1><p>Configure the actions on your radial wheel.</p></div>
              {visibleAction && <button className="enabled-switch" type="button" role="switch" aria-checked={visibleAction.enabled}
                disabled={busy || conflict || (tab === 'json' && !candidate)} aria-label="Action enabled" onClick={() => void task(toggle)}>
                <span className={visibleAction.enabled ? 'on' : ''} />{visibleAction.enabled ? 'Enabled' : 'Disabled'}
              </button>}
            </div>
            {draft && <div className="tab-bar">
              <div className="tabs" aria-label="Editor mode">
                <button type="button" className={tab === 'visual' ? 'selected' : ''} aria-pressed={tab === 'visual'}
                  disabled={busy || (tab === 'json' && !canShowVisual)} onClick={() => changeTab('visual')}><SlidersHorizontal size={16} /> Visual editor</button>
                <button type="button" className={tab === 'json' ? 'selected' : ''} aria-pressed={tab === 'json'} disabled={busy || Boolean(fieldError)} onClick={() => changeTab('json')}><Braces size={16} /> JSON Inspector</button>
              </div>
              <div className="editor-tools">
                <button type="button" className="icon-button" aria-label="Duplicate saved action" title="Duplicate saved action" disabled={!saved || busy || dirty} onClick={() => void task(duplicate)}><Copy size={17} /></button>
                <button type="button" className="icon-button danger" aria-label="Delete action" title="Delete action" disabled={!saved || busy || conflict} onClick={() => setConfirmDelete(true)}><Trash2 size={17} /></button>
              </div>
            </div>}
          </div>
          <div className="editor-content" ref={scrollArea}>
            {conflict && <div className="editor-alert" role="alert"><CircleAlert size={17} /><span>The saved manifest changed on disk.</span>
              <button className="button secondary" onClick={() => current ? select(current) : select(draft!, false)}>Reload</button>
            </div>}
            {draft ? (
              tab === 'visual' ? <fieldset className="editor-form" disabled={busy}>
                <VisualEditor key={editorVersion} action={draft} isNew={!saved} onFieldError={setFieldError} update={setDraft} />
              </fieldset> : <div className="json-editor">
                <label className="field"><span>Action manifest</span>
                  <textarea className="mono" spellCheck={false} disabled={busy} value={raw} onChange={event => setRaw(event.target.value)} />
                </label>
                <div aria-live="polite" className={`schema-status ${blocked ? 'invalid' : ''}`}>
                  {blocked ? <CircleAlert size={16} /> : <Check size={16} />}
                  <div>{idChanged && <p>Keep the saved action ID to update this manifest.</p>}
                    {issues.length ? issues.map(issue => <p key={issue}>{issue}</p>) : !blocked ? <p>Manifest is valid</p> : null}
                  </div>
                </div>
              </div>
            ) : <div className="studio-empty"><Layers3 size={30} /><h2>{loading ? 'Loading actions…' : 'Create your first action'}</h2>
              {!loading && <button className="button accent" onClick={() => select(newAction(), false)}><Plus size={17} /> New action</button>}
            </div>}
          </div>
          <footer className="editor-footer">
            <div className={`save-status ${notice?.error || (dirty && blocked) ? 'invalid' : ''}`} aria-live="polite">
              {notice ? notice.message : fieldError || (dirty && tab === 'visual' && issues.length ? issues[0] : dirty ? 'Unsaved changes' : draft ? 'All changes saved' : '')}
            </div>
            {draft && <div className="footer-buttons">
              {dirty && <button type="button" className="button secondary" disabled={busy} onClick={() => saved ? select(current ?? saved) : setDraft(null)}>Discard changes</button>}
              <button type="button" className="button accent" disabled={!dirty || busy || blocked} onClick={() => void task(save)}><Save size={16} /> Save action</button>
            </div>}
          </footer>
        </section>
      </div>
      {confirmDelete && <div className="dialog-backdrop" onKeyDown={event => { if (event.key === 'Escape') setConfirmDelete(false); }}>
        <section className="confirm-dialog" role="alertdialog" aria-modal="true" aria-labelledby="delete-title">
          <h2 id="delete-title">Delete action?</h2>
          <p>The manifest for {saved?.label} will be removed from your actions folder.</p>
          <div><button autoFocus className="button secondary" onClick={() => setConfirmDelete(false)}>Keep action</button>
            <button className="button destructive" disabled={busy} onClick={() => void task(remove)}><Trash2 size={16} /> Delete action</button></div>
        </section>
      </div>}
    </main>
  );
}
