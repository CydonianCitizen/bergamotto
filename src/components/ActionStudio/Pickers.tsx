import { Check, Palette } from 'lucide-react';
import { useState } from 'react';
import { AVAILABLE_ICONS, ActionIcon } from '../ActionIcon';

export const ACCENT_PALETTE = [
  { name: 'Bergamot Lime', hex: '#84cc16' },
  { name: 'Bright Lime', hex: '#a3e635' },
  { name: 'Emerald', hex: '#10b981' },
  { name: 'Teal', hex: '#14b8a6' },
  { name: 'Cyan', hex: '#06b6d4' },
  { name: 'Sky Blue', hex: '#0ea5e9' },
  { name: 'Royal Blue', hex: '#3b82f6' },
  { name: 'Indigo', hex: '#6366f1' },
  { name: 'Purple', hex: '#8b5cf6' },
  { name: 'Pink', hex: '#ec4899' },
  { name: 'Rose', hex: '#f43f5e' },
  { name: 'Orange', hex: '#f97316' },
  { name: 'Amber', hex: '#f59e0b' },
];

export function IconPicker({
  value,
  onChange,
}: {
  value?: string;
  onChange: (icon?: string) => void;
}) {
  const [open, setOpen] = useState(false);
  const current = value || 'terminal';

  return (
    <div className="icon-picker-field">
      <div className="icon-picker-trigger">
        <button
          type="button"
          className="button secondary icon-preview-btn"
          onClick={() => setOpen(!open)}
          aria-expanded={open}
          aria-label="Choose icon"
        >
          <span className="picker-icon-box">
            <ActionIcon icon={value} type="process" size={20} />
          </span>
          <span className="picker-icon-name">{value || 'Choose icon…'}</span>
        </button>
        {value && (
          <button
            type="button"
            className="icon-button"
            title="Clear icon"
            onClick={() => onChange(undefined)}
          >
            ✕
          </button>
        )}
      </div>

      {open && (
        <div className="icon-picker-popover">
          <div className="icon-picker-grid">
            {AVAILABLE_ICONS.map(item => {
              const selected = current === item.id;
              const IconComp = item.icon;
              return (
                <button
                  key={item.id}
                  type="button"
                  className={`icon-grid-item ${selected ? 'selected' : ''}`}
                  title={item.label}
                  onClick={() => {
                    onChange(item.id);
                    setOpen(false);
                  }}
                >
                  <IconComp size={20} strokeWidth={selected ? 2.2 : 1.7} />
                  <span className="icon-grid-label">{item.id}</span>
                </button>
              );
            })}
          </div>
          <div className="icon-picker-custom">
            <span>Custom icon:</span>
            <input
              type="text"
              placeholder="e.g. terminal"
              value={value ?? ''}
              onChange={e => onChange(e.target.value.trim() || undefined)}
            />
          </div>
        </div>
      )}
    </div>
  );
}

export function ColorPicker({
  value,
  onChange,
}: {
  value?: string;
  onChange: (color?: string) => void;
}) {
  return (
    <div className="color-picker-field">
      <div className="color-swatch-list">
        <button
          type="button"
          className={`color-swatch none ${!value ? 'selected' : ''}`}
          title="Default Theme Accent"
          onClick={() => onChange(undefined)}
        >
          {!value && <Check size={13} />}
        </button>
        {ACCENT_PALETTE.map(color => {
          const selected = value?.toLowerCase() === color.hex.toLowerCase();
          return (
            <button
              key={color.hex}
              type="button"
              className={`color-swatch ${selected ? 'selected' : ''}`}
              style={{ backgroundColor: color.hex }}
              title={color.name}
              onClick={() => onChange(color.hex)}
            >
              {selected && <Check size={14} color="#000" strokeWidth={2.5} />}
            </button>
          );
        })}
      </div>
      <div className="color-custom-input">
        <Palette size={15} className="palette-icon" />
        <input
          type="color"
          className="color-native-input"
          value={value ?? '#84cc16'}
          onChange={e => onChange(e.target.value)}
        />
        <input
          type="text"
          className="color-hex-text mono"
          placeholder="Hex e.g. #84cc16"
          value={value ?? ''}
          onChange={e => {
            const v = e.target.value.trim();
            onChange(v || undefined);
          }}
        />
      </div>
    </div>
  );
}
