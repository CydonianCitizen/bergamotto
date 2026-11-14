import { listen } from '@tauri-apps/api/event';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { ChevronLeft, ChevronRight, FileStack, MousePointer2, Upload } from 'lucide-react';
import { useEffect, useRef, useState } from 'react';
import { errorText, ipc } from '../ipc';
import type { ActionDescriptor, WheelPresentation } from '../types/actions';
import { ActionIcon } from './ActionIcon';
import { center, hitWedge, wedge } from './wheelGeometry';

const pageSize = 8;
// Wry sends GTK widget coordinates and AppKit points, but Win32 screen pixels.
const physicalDragCoordinates = navigator.userAgent.includes('Windows');
const empty: WheelPresentation = { files: [], actions: [] };
type Point = { x: number; y: number };

export default function RadialWheel() {
  const [selection, setSelection] = useState(empty);
  const [drag, setDrag] = useState<WheelPresentation | null>(null);
  const [point, setPoint] = useState<Point | null>(null);
  const [page, setPage] = useState(0);
  const [notice, setNotice] = useState('');
  const [busy, setBusy] = useState(false);
  const model = useRef({ selection: empty, drag: null as WheelPresentation | null, page: 0 });
  const invoking = useRef(false);
  const active = drag ?? selection;
  const pages = Math.max(1, Math.ceil(active.actions.length / pageSize));
  const visiblePage = Math.min(page, pages - 1);
  const actions = active.actions.slice(visiblePage * pageSize, (visiblePage + 1) * pageSize);
  const hovered = point ? hitWedge(point.x, point.y, actions.length) : null;
  const extensions = new Set(active.files.map(file => {
    const name = file.split(/[/\\]/).at(-1) ?? '';
    const dot = name.lastIndexOf('.');
    return dot > 0 ? name.slice(dot + 1).toUpperCase() : '';
  }));

  async function execute(action: ActionDescriptor, files: string[]) {
    if (invoking.current) return;
    invoking.current = true;
    setBusy(true);
    setNotice('');
    try {
      await ipc.invokeAction(action.id, files);
      await getCurrentWindow().hide();
    } catch (error) {
      setNotice(errorText(error));
    } finally {
      invoking.current = false;
      setBusy(false);
    }
  }

  useEffect(() => {
    let alive = true;
    let revision = 0;
    let dragRevision = 0;
    let presentationEvent = false;
    const latest = model.current;

    function clearDrag() {
      dragRevision += 1;
      latest.drag = null;
      setDrag(null);
      setPoint(null);
    }

    function present(payload: WheelPresentation) {
      revision += 1;
      clearDrag();
      latest.selection = payload;
      latest.page = 0;
      setSelection(payload);
      setPage(0);
      setNotice('');
    }

    function fail(error: unknown) {
      if (alive) setNotice(errorText(error));
    }

    function logical(position: Point) {
      const scale = physicalDragCoordinates ? window.devicePixelRatio : 1;
      return { x: position.x / scale, y: position.y / scale };
    }

    const subscriptions = [
      listen<WheelPresentation>('wheel:present', ({ payload }) => {
        presentationEvent = true;
        present(payload);
      }).then(async unlisten => {
        if (alive) {
          void ipc.wheelState().then(payload => {
            if (alive && !presentationEvent) present(payload);
          }).catch(fail);
        }
        return unlisten;
      }),
      listen('actions:updated', () => {
        const files = latest.selection.files;
        if (!files.length) return;
        const request = ++revision;
        void ipc.matchActions(files).then(actions => {
          if (alive && request === revision) {
            latest.selection = { files, actions };
            setSelection(latest.selection);
          }
        }).catch(fail);
      }),
      getCurrentWindow().onDragDropEvent(({ payload }) => {
        switch (payload.type) {
          case 'enter': {
            const request = ++dragRevision;
            latest.drag = { files: payload.paths, actions: [] };
            setDrag(latest.drag);
            setPoint(logical(payload.position));
            setNotice('');
            void ipc.matchActions(payload.paths).then(actions => {
              if (alive && request === dragRevision) {
                latest.drag = { files: payload.paths, actions };
                setDrag(latest.drag);
              }
            }).catch(fail);
            break;
          }
          case 'over':
            setPoint(logical(payload.position));
            break;
          case 'leave':
            clearDrag();
            break;
          case 'drop': {
            const payloadPosition = logical(payload.position);
            const matched = latest.drag?.actions ?? [];
            const lastPage = Math.max(0, Math.ceil(matched.length / pageSize) - 1);
            const shownPage = Math.min(latest.page, lastPage);
            const shown = matched.slice(shownPage * pageSize, (shownPage + 1) * pageSize);
            const index = hitWedge(payloadPosition.x, payloadPosition.y, shown.length);
            const target = index === null ? undefined : shown[index];
            present({ files: payload.paths, actions: matched });
            if (target) {
              void execute(target, payload.paths);
            } else {
              const request = revision;
              void ipc.matchActions(payload.paths).then(actions => {
                if (alive && revision === request) {
                  latest.selection = { files: payload.paths, actions };
                  setSelection(latest.selection);
                }
              }).catch(fail);
            }
            break;
          }
        }
      }),
    ];

    function escape(event: KeyboardEvent) {
      if (event.key === 'Escape') void getCurrentWindow().hide().catch(fail);
    }
    window.addEventListener('keydown', escape);
    for (const subscription of subscriptions) void subscription.catch(fail);
    return () => {
      alive = false;
      window.removeEventListener('keydown', escape);
      for (const subscription of subscriptions) void subscription.then(unlisten => unlisten()).catch(() => {});
    };
  }, []);

  function turnPage(next: number) {
    model.current.page = next;
    setPage(next);
    setPoint(null);
  }

  return (
    <main className="wheel-view" aria-label="Radial wheel">
      <div className="wheel-halo" />
      <svg viewBox="0 0 420 420" className="wheel-svg" aria-label="Matching actions"
        onPointerMove={event => {
          const bounds = event.currentTarget.getBoundingClientRect();
          setPoint({ x: event.clientX - bounds.left, y: event.clientY - bounds.top });
        }}
        onPointerLeave={() => setPoint(null)}
      >
        <defs>
          <linearGradient id="glass" x2="0.85" y2="1">
            <stop stopColor="#2b3937" stopOpacity=".99" />
            <stop offset="1" stopColor="#121a1d" stopOpacity=".98" />
          </linearGradient>
          <linearGradient id="glass-active" x2="0.85" y2="1">
            <stop stopColor="#d8f39b" />
            <stop offset="1" stopColor="#8cbd7d" />
          </linearGradient>
        </defs>
        <circle cx={center} cy={center} r={174} className="wheel-outline" />
        {actions.map((action, index) => {
          const { path, label: [x, y] } = wedge(index, actions.length);
          const shortLabel = action.label.length > 18 ? `${action.label.slice(0, 16)}…` : action.label;
          return (
            <g key={action.id} className={`wheel-wedge ${index === hovered ? 'active' : ''}`}
              role="button" tabIndex={busy ? -1 : 0} aria-label={action.label} aria-disabled={busy}
              onClick={() => void execute(action, active.files)}
              onKeyDown={event => {
                if (event.key === 'Enter' || event.key === ' ') {
                  event.preventDefault();
                  void execute(action, active.files);
                }
              }}>
              <title>{action.label}</title>
              <path d={path} />
              <foreignObject x={x - 11} y={y - 19} width={22} height={22} pointerEvents="none">
                <ActionIcon icon={action.icon} type={action.executor.type} size={22} />
              </foreignObject>
              <text x={x} y={y + 21} textAnchor="middle" pointerEvents="none">{shortLabel}</text>
            </g>
          );
        })}
      </svg>
      <button type="button" className={`wheel-hub ${drag ? 'dragging' : ''}`} aria-label="Hide radial wheel"
        onClick={() => { void getCurrentWindow().hide().catch(error => setNotice(errorText(error))); }}>
        {active.files.length ? <FileStack size={23} /> : <Upload size={23} />}
        <strong>{active.files.length ? `${active.files.length} ${active.files.length === 1 ? 'file' : 'files'}` : 'Drop files'}</strong>
        <span>{active.files.length ? (extensions.size > 1 ? 'Mixed types' : [...extensions][0] || 'Selected') : 'Esc to close'}</span>
      </button>
      <div className="wheel-caption" aria-live="polite">
        {notice ? <span className="wheel-notice">{notice}</span> : pages > 1 ? (
          <div className="wheel-pages">
            <button aria-label="Previous actions" disabled={visiblePage === 0} onClick={() => turnPage(visiblePage - 1)}><ChevronLeft size={17} /></button>
            <span>{visiblePage + 1} / {pages}</span>
            <button aria-label="Next actions" disabled={visiblePage === pages - 1} onClick={() => turnPage(visiblePage + 1)}><ChevronRight size={17} /></button>
          </div>
        ) : <span><MousePointer2 size={12} />{actions.length ? 'Choose an action' : active.files.length ? 'No matching actions' : 'Drop files anywhere on the wheel'}</span>}
      </div>
    </main>
  );
}
