import { Check, Copy, Keyboard } from 'lucide-react';
import { useEffect, useRef, useState } from 'react';
import { errorText, ipc } from '../../ipc';
import type { AppSettings, ModifierKey } from '../../types/actions';
import { Field } from './Fields';

export default function SettingsPanel({ settings, close }: { settings: AppSettings; close: () => void }) {
  const dialog = useRef<HTMLDialogElement>(null);
  const [modifierKey, setModifierKey] = useState<ModifierKey>(settings.modifierKey);
  const [globalShortcut, setGlobalShortcut] = useState(settings.globalShortcut || 'Alt+Shift+Space');
  const [isRecording, setIsRecording] = useState(false);
  const [shortcutStatus, setShortcutStatus] = useState<'idle' | 'valid' | 'conflict' | 'checking'>('idle');
  const [shortcutMessage, setShortcutMessage] = useState('');
  const [copiedHint, setCopiedHint] = useState(false);

  const [paths, setPaths] = useState(settings.customPaths.join('\n'));
  const [overrides, setOverrides] = useState(Object.entries(settings.toolOverrides).map(([tool, path]) => ({ tool, path })));
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState('');

  useEffect(() => { dialog.current?.showModal(); }, []);

  async function checkShortcut(candidate: string) {
    if (!candidate.trim()) {
      setShortcutStatus('idle');
      setShortcutMessage('');
      return;
    }
    setShortcutStatus('checking');
    setShortcutMessage('Checking with operating system for conflicts...');
    try {
      await ipc.testGlobalShortcut(candidate);
      setShortcutStatus('valid');
      setShortcutMessage(`✓ Shortcut active and registered with OS`);
    } catch (reason) {
      setShortcutStatus('conflict');
      setShortcutMessage(errorText(reason));
    }
  }

  function handleKeyDown(event: React.KeyboardEvent) {
    if (!isRecording) return;
    event.preventDefault();
    event.stopPropagation();

    if (event.key === 'Escape') {
      setIsRecording(false);
      return;
    }

    const modifiers: string[] = [];
    if (event.ctrlKey) modifiers.push('Ctrl');
    if (event.altKey) modifiers.push('Alt');
    if (event.shiftKey) modifiers.push('Shift');
    if (event.metaKey) modifiers.push('Super');

    if (['Control', 'Alt', 'Shift', 'Meta'].includes(event.key)) {
      return;
    }

    let mainKey = event.key;
    if (event.code === 'Space' || event.key === ' ') {
      mainKey = 'Space';
    } else if (event.code.startsWith('Key')) {
      mainKey = event.code.slice(3).toUpperCase();
    } else if (event.code.startsWith('Digit')) {
      mainKey = event.code.slice(5);
    } else if (event.key.length === 1) {
      mainKey = event.key.toUpperCase();
    }

    if (modifiers.length === 0) {
      modifiers.push('Alt');
    }

    const combination = [...modifiers, mainKey].join('+');
    setGlobalShortcut(combination);
    setIsRecording(false);
    void checkShortcut(combination);
  }

  async function copyCliHint() {
    try {
      await navigator.clipboard.writeText('bergamot --radial-clipboard');
      setCopiedHint(true);
      setTimeout(() => setCopiedHint(false), 2000);
    } catch {
      // ignore
    }
  }

  async function save() {
    setBusy(true);
    setError('');
    try {
      const entries = overrides.map(({ tool, path }) => [tool.trim(), path.trim()] as const);
      const toolOverrides = Object.fromEntries(entries);
      if (Object.keys(toolOverrides).length !== entries.length) throw new Error('Use one override per tool.');
      await ipc.saveSettings({
        ...settings,
        modifierKey,
        globalShortcut,
        customPaths: paths.split('\n').map(path => path.trim()).filter(Boolean),
        toolOverrides,
      });
      close();
    } catch (reason) {
      setError(errorText(reason));
    } finally {
      setBusy(false);
    }
  }

  return (
    <dialog
      ref={dialog}
      className="studio-dialog"
      aria-labelledby="settings-title"
      onCancel={event => {
        event.preventDefault();
        if (!busy) close();
      }}
    >
      <form
        onSubmit={event => {
          event.preventDefault();
          void save();
        }}
      >
        <h2 id="settings-title">Settings &amp; Hotkeys</h2>
        <fieldset disabled={busy}>
          {/* Wayland & Global Shortcut Section */}
          <div className="settings-section">
            <h3 className="settings-subtitle">
              <Keyboard size={18} /> Global Activation Shortcut (Wayland &amp; Desktop)
            </h3>
            <p className="field-hint">
              Press this keyboard combination from anywhere to open the radial wheel with your current selection or clipboard files.
            </p>

            <div
              className={`shortcut-recorder-box ${isRecording ? 'recording' : ''}`}
              tabIndex={0}
              onKeyDown={handleKeyDown}
              role="button"
              aria-label="Click to record new shortcut"
            >
              <div className="shortcut-keys-display">
                {isRecording ? (
                  <span className="recording-prompt">
                    <span className="recording-dot" /> Press combination on your keyboard (e.g. Alt + Shift + Space)...
                  </span>
                ) : (
                  globalShortcut.split('+').map((key, i) => (
                    <span key={i} className="kbd-wrap">
                      {i > 0 && <span className="kbd-plus">+</span>}
                      <kbd className="key-badge">{key}</kbd>
                    </span>
                  ))
                )}
              </div>

              <div className="recorder-actions">
                {isRecording ? (
                  <button
                    type="button"
                    className="button secondary sm"
                    onClick={e => {
                      e.stopPropagation();
                      setIsRecording(false);
                    }}
                  >
                    Cancel (Esc)
                  </button>
                ) : (
                  <button
                    type="button"
                    className="button accent sm"
                    onClick={e => {
                      e.stopPropagation();
                      setIsRecording(true);
                      setShortcutStatus('idle');
                      setShortcutMessage('');
                    }}
                  >
                    Change Shortcut
                  </button>
                )}
              </div>
            </div>

            {/* Conflict & Status Notice */}
            {shortcutStatus !== 'idle' && (
              <div className={`shortcut-status-banner ${shortcutStatus}`}>
                {shortcutMessage}
              </div>
            )}

            {/* Wayland Tip & Compositor Command */}
            <div className="wayland-tip-box">
              <div className="tip-header">
                <strong>Wayland Integration:</strong>
                <span>Copy any file in your file manager (<kbd>Ctrl+C</kbd>), then press your shortcut to open the wheel.</span>
              </div>
              <div className="tip-cli">
                <code>bergamot --radial-clipboard</code>
                <button
                  type="button"
                  className="button secondary sm icon-text"
                  onClick={copyCliHint}
                  title="Copy command for KDE/Hyprland shortcut binding"
                >
                  {copiedHint ? <Check size={14} /> : <Copy size={14} />}
                  {copiedHint ? 'Copied!' : 'Copy CLI Command'}
                </button>
              </div>
            </div>
          </div>

          <Field
            label="X11 / Windows / macOS Drag Modifier"
            hint="Modifier to hold while dragging files in OS file manager (X11, macOS, Windows)."
          >
            <select value={modifierKey} onChange={event => setModifierKey(event.target.value as ModifierKey)}>
              <option value="Shift">Shift</option>
              <option value="Ctrl">Ctrl</option>
              <option value="Alt">Alt</option>
              <option value="Alt+Shift">Alt + Shift</option>
              <option value="Ctrl+Shift">Ctrl + Shift</option>
              <option value="Ctrl+Alt">Ctrl + Alt</option>
            </select>
          </Field>

          <Field label="Custom executable directories" hint="One absolute directory per line, searched in order before system PATH.">
            <textarea className="mono" rows={3} spellCheck={false} value={paths} onChange={event => setPaths(event.target.value)} />
          </Field>

          <fieldset className="field">
            <legend>Tool overrides</legend>
            {overrides.map(({ tool, path }, index) => (
              <div className="tool-override" key={index}>
                <input
                  className="mono"
                  placeholder="Tool name"
                  aria-label="Tool name"
                  required
                  value={tool}
                  onChange={event =>
                    setOverrides(
                      overrides.map((entry, row) => (row === index ? { ...entry, tool: event.target.value } : entry)),
                    )
                  }
                />
                <input
                  className="mono"
                  placeholder="Absolute executable path"
                  aria-label="Absolute executable path"
                  required
                  value={path}
                  onChange={event =>
                    setOverrides(
                      overrides.map((entry, row) => (row === index ? { ...entry, path: event.target.value } : entry)),
                    )
                  }
                />
                <button type="button" className="button secondary" onClick={() => setOverrides(overrides.filter((_, row) => row !== index))}>
                  Remove
                </button>
              </div>
            ))}
            <button className="button secondary" type="button" onClick={() => setOverrides([...overrides, { tool: '', path: '' }])}>
              Add override
            </button>
          </fieldset>
        </fieldset>

        {error && (
          <p className="save-status invalid" role="alert">
            {error}
          </p>
        )}
        <div className="footer-buttons">
          <button className="button secondary" type="button" onClick={close} disabled={busy}>
            Cancel
          </button>
          <button className="button accent" type="submit" disabled={busy || isRecording}>
            Save settings
          </button>
        </div>
      </form>
    </dialog>
  );
}
