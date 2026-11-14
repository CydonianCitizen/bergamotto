import {
  Edit3,
  FileText,
  Film,
  Globe,
  Image,
  Layers,
  Music2,
  Plus,
  RotateCcw,
  RotateCw,
} from 'lucide-react';
import { useMemo, useState } from 'react';
import type { ActionDescriptor } from '../../types/actions';
import { ActionIcon } from '../ActionIcon';
import { center, hitWedge, wedge } from '../wheelGeometry';

export interface FilterPreset {
  id: string;
  label: string;
  icon: typeof Globe;
  testExtension?: string;
  defaultExtensions: string[];
}

export const PREVIEW_FILTERS: FilterPreset[] = [
  { id: 'all', label: 'All Actions', icon: Layers, defaultExtensions: ['*'] },
  { id: 'images', label: 'Images', icon: Image, testExtension: 'png', defaultExtensions: ['png', 'jpg', 'webp'] },
  { id: 'videos', label: 'Video', icon: Film, testExtension: 'mp4', defaultExtensions: ['mp4', 'mov', 'mkv'] },
  { id: 'audio', label: 'Audio', icon: Music2, testExtension: 'mp3', defaultExtensions: ['mp3', 'wav'] },
  { id: 'docs', label: 'Documents', icon: FileText, testExtension: 'pdf', defaultExtensions: ['pdf', 'txt'] },
];

export function WheelCustomizer({
  actions,
  selectedAction,
  onSelectAction,
  onEditAction,
  onAddSlice,
  onReorder,
}: {
  actions: ActionDescriptor[];
  selectedAction?: ActionDescriptor | null;
  onSelectAction: (action: ActionDescriptor) => void;
  onEditAction: (action: ActionDescriptor) => void;
  onAddSlice: (presetExtensions?: string[]) => void;
  onReorder: (orderedIds: string[]) => Promise<void>;
}) {
  const [filter, setFilter] = useState<string>('all');
  const [customExt, setCustomExt] = useState('');
  const [hoveredIndex, setHoveredIndex] = useState<number | null>(null);
  const [busy, setBusy] = useState(false);

  // Filter actions according to active preview filter
  const visibleActions = useMemo(() => {
    let testExt: string | null = null;
    if (filter === 'custom' && customExt.trim()) {
      testExt = customExt.trim().replace(/^\./, '').toLowerCase();
    } else {
      const preset = PREVIEW_FILTERS.find(p => p.id === filter);
      testExt = preset?.testExtension ?? null;
    }

    const enabled = actions.filter(a => a.enabled);

    let filtered = enabled;
    if (testExt) {
      const ext = testExt;
      filtered = enabled.filter(action => {
        const extensions = action.matching.extensions.map(e => e.toLowerCase());
        return extensions.includes('*') || extensions.includes(ext);
      });
    }

    // Sort by sortOrder, label, id
    return [...filtered].sort((a, b) => {
      const orderA = a.sortOrder ?? 0;
      const orderB = b.sortOrder ?? 0;
      if (orderA !== orderB) return orderA - orderB;
      const labelCmp = a.label.localeCompare(b.label);
      if (labelCmp !== 0) return labelCmp;
      return a.id.localeCompare(b.id);
    });
  }, [actions, filter, customExt]);

  async function moveSlice(index: number, direction: 'cw' | 'ccw') {
    if (busy || visibleActions.length < 2) return;
    const targetIndex =
      direction === 'cw'
        ? (index + 1) % visibleActions.length
        : (index - 1 + visibleActions.length) % visibleActions.length;

    const reorderedVisible = [...visibleActions];
    const [moved] = reorderedVisible.splice(index, 1);
    reorderedVisible.splice(targetIndex, 0, moved);

    // Build the complete ordered IDs list across all actions
    const visibleIdSet = new Set(reorderedVisible.map(a => a.id));
    const otherActions = actions.filter(a => !visibleIdSet.has(a.id));
    const allOrderedIds = [...reorderedVisible.map(a => a.id), ...otherActions.map(a => a.id)];

    setBusy(true);
    try {
      await onReorder(allOrderedIds);
    } finally {
      setBusy(false);
    }
  }

  function handleAddWithCurrentFilter() {
    const preset = PREVIEW_FILTERS.find(p => p.id === filter);
    if (filter === 'custom' && customExt.trim()) {
      onAddSlice([customExt.trim().replace(/^\./, '').toLowerCase()]);
    } else if (preset) {
      onAddSlice(preset.defaultExtensions);
    } else {
      onAddSlice(['*']);
    }
  }

  return (
    <div className="wheel-customizer-pane">
      <div className="customizer-toolbar">
        <div className="filter-button-group">
          <span className="filter-label">Preview filter:</span>
          {PREVIEW_FILTERS.map(preset => {
            const IconComp = preset.icon;
            const active = filter === preset.id;
            return (
              <button
                key={preset.id}
                type="button"
                className={`filter-chip ${active ? 'active' : ''}`}
                onClick={() => setFilter(preset.id)}
              >
                <IconComp size={15} />
                <span>{preset.label}</span>
              </button>
            );
          })}
          <div className={`custom-ext-box ${filter === 'custom' ? 'active' : ''}`}>
            <input
              type="text"
              placeholder=".ext"
              value={customExt}
              onChange={e => {
                setCustomExt(e.target.value);
                setFilter('custom');
              }}
              onFocus={() => setFilter('custom')}
            />
          </div>
        </div>

        <button
          type="button"
          className="button accent add-slice-header-btn"
          onClick={handleAddWithCurrentFilter}
        >
          <Plus size={16} /> Add Slice
        </button>
      </div>

      <div className="customizer-main-stage">
        <div className="wheel-preview-canvas">
          <div className="wheel-canvas-container">
            <svg
              viewBox="0 0 420 420"
              className="wheel-svg"
              onPointerMove={event => {
                const bounds = event.currentTarget.getBoundingClientRect();
                setHoveredIndex(
                  hitWedge(event.clientX - bounds.left, event.clientY - bounds.top, visibleActions.length)
                );
              }}
              onPointerLeave={() => setHoveredIndex(null)}
            >
              <defs>
                <linearGradient id="studio-glass" x2="0.85" y2="1">
                  <stop stopColor="#2b3937" stopOpacity=".99" />
                  <stop offset="1" stopColor="#121a1d" stopOpacity=".98" />
                </linearGradient>
                <linearGradient id="studio-glass-active" x2="0.85" y2="1">
                  <stop stopColor="#d8f39b" />
                  <stop offset="1" stopColor="#8cbd7d" />
                </linearGradient>
              </defs>

              <circle cx={center} cy={center} r={174} className="wheel-outline" />

              {visibleActions.map((action, index) => {
                const { path, label: [x, y] } = wedge(index, visibleActions.length);
                const isHovered = index === hoveredIndex;
                const isSelected = action.id === selectedAction?.id;
                const active = isHovered || isSelected;

                const pathStyle = active && action.accentColor
                  ? { fill: action.accentColor }
                  : action.accentColor
                  ? { stroke: action.accentColor, strokeWidth: 1.8 }
                  : undefined;

                const shortLabel =
                  action.label.length > 16 ? `${action.label.slice(0, 14)}…` : action.label || 'Action';

                return (
                  <g
                    key={action.id}
                    className={`wheel-wedge ${active ? 'active' : ''} ${isSelected ? 'selected-slice' : ''}`}
                    role="button"
                    tabIndex={0}
                    aria-label={action.label}
                    onClick={() => onSelectAction(action)}
                  >
                    <title>{action.label}</title>
                    <path d={path} style={pathStyle} />
                    <foreignObject x={x - 11} y={y - 19} width={22} height={22} pointerEvents="none">
                      <ActionIcon icon={action.icon} type={action.executor.type} size={22} />
                    </foreignObject>
                    <text x={x} y={y + 21} textAnchor="middle" pointerEvents="none">
                      {shortLabel}
                    </text>
                  </g>
                );
              })}
            </svg>

            <button
              type="button"
              className="wheel-hub customizer-hub"
              onClick={handleAddWithCurrentFilter}
              title="Add a new slice to this wheel"
            >
              <Plus size={24} />
              <strong>Add Slice</strong>
              <span>{visibleActions.length} slices</span>
            </button>
          </div>
        </div>

        <div className="wheel-slices-sidebar">
          <div className="slices-sidebar-header">
            <h4>Radial Wheel Slices ({visibleActions.length})</h4>
            <p>Order is clockwise starting from the top.</p>
          </div>

          <div className="slices-list-scroll">
            {visibleActions.map((action, index) => {
              const isSelected = action.id === selectedAction?.id;
              return (
                <div
                  key={action.id}
                  className={`slice-order-row ${isSelected ? 'selected' : ''}`}
                  onClick={() => onSelectAction(action)}
                >
                  <span className="slice-index-badge">#{index + 1}</span>
                  <span
                    className="slice-color-indicator"
                    style={{ backgroundColor: action.accentColor ?? '#84cc16' }}
                  />
                  <span className="slice-icon-wrap">
                    <ActionIcon icon={action.icon} type={action.executor.type} size={16} />
                  </span>
                  <div className="slice-name-group">
                    <strong>{action.label || 'Untitled'}</strong>
                    <small>{action.executor.type}</small>
                  </div>

                  <div className="slice-action-buttons">
                    <button
                      type="button"
                      className="icon-button"
                      disabled={busy}
                      title="Move Counter-Clockwise (Up)"
                      onClick={e => {
                        e.stopPropagation();
                        void moveSlice(index, 'ccw');
                      }}
                    >
                      <RotateCcw size={14} />
                    </button>
                    <button
                      type="button"
                      className="icon-button"
                      disabled={busy}
                      title="Move Clockwise (Down)"
                      onClick={e => {
                        e.stopPropagation();
                        void moveSlice(index, 'cw');
                      }}
                    >
                      <RotateCw size={14} />
                    </button>
                    <button
                      type="button"
                      className="icon-button"
                      title="Edit Action"
                      onClick={e => {
                        e.stopPropagation();
                        onEditAction(action);
                      }}
                    >
                      <Edit3 size={14} />
                    </button>
                  </div>
                </div>
              );
            })}

            {!visibleActions.length && (
              <div className="empty-slices-state">
                <p>No enabled actions match the &ldquo;{filter}&rdquo; filter.</p>
                <button
                  type="button"
                  className="button accent"
                  onClick={handleAddWithCurrentFilter}
                >
                  <Plus size={15} /> Create matching action
                </button>
              </div>
            )}
          </div>
        </div>
      </div>
    </div>
  );
}
