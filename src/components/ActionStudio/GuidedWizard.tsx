import {
  Archive,
  ChevronLeft,
  ChevronRight,
  FileText,
  Film,
  Globe,
  Image,
  Music2,
  Sparkles,
} from 'lucide-react';
import { useState } from 'react';
import type { ActionDescriptor, Cardinality, OutputMode } from '../../types/actions';
import { ColorPicker, IconPicker } from './Pickers';
import { CommandPreview, VariablePills } from './CommandPills';

export const FILE_CATEGORIES = [
  { id: 'images', label: 'Images', icon: Image, extensions: ['png', 'jpg', 'jpeg', 'webp', 'gif', 'svg'] },
  { id: 'videos', label: 'Videos', icon: Film, extensions: ['mp4', 'mov', 'mkv', 'avi', 'webm'] },
  { id: 'audio', label: 'Audio', icon: Music2, extensions: ['mp3', 'wav', 'flac', 'aac', 'ogg', 'm4a'] },
  { id: 'docs', label: 'PDF & Docs', icon: FileText, extensions: ['pdf', 'txt', 'md', 'docx', 'csv'] },
  { id: 'archives', label: 'Archives', icon: Archive, extensions: ['zip', 'tar', 'gz', '7z', 'bz2'] },
  { id: 'any', label: 'Any File (*)', icon: Globe, extensions: ['*'] },
];

export const RECIPE_TEMPLATES: {
  id: string;
  title: string;
  description: string;
  icon: string;
  accentColor: string;
  create: () => Partial<ActionDescriptor>;
}[] = [
  {
    id: 'ffmpeg-convert',
    title: 'Convert Media Format (FFmpeg)',
    description: 'Convert video or audio files to standard MP4 / MP3 formats.',
    icon: 'film',
    accentColor: '#84cc16',
    create: () => ({
      label: 'Convert with FFmpeg',
      icon: 'film',
      accentColor: '#84cc16',
      matching: { extensions: ['mov', 'mkv', 'avi', 'webm'], cardinality: 'each' },
      executor: {
        type: 'process',
        program: 'ffmpeg',
        args: ['-i', '${input}', '-c:v', 'libx264', '-crf', '23', '${output}'],
        timeoutSeconds: 600,
      },
      output: { mode: 'file', extension: 'mp4', suffix: '-converted', overwriteOriginal: false },
    }),
  },
  {
    id: 'ffmpeg-compress',
    title: 'Compress Video',
    description: 'Reduce video file size using high-efficiency H.264 compression.',
    icon: 'video',
    accentColor: '#0ea5e9',
    create: () => ({
      label: 'Compress Video',
      icon: 'video',
      accentColor: '#0ea5e9',
      matching: { extensions: ['mp4', 'mov', 'mkv'], cardinality: 'each' },
      executor: {
        type: 'process',
        program: 'ffmpeg',
        args: ['-i', '${input}', '-vcodec', 'libx264', '-crf', '28', '${output}'],
        timeoutSeconds: 600,
      },
      output: { mode: 'file', extension: 'mp4', suffix: '-compressed', overwriteOriginal: false },
    }),
  },
  {
    id: 'script-runner',
    title: 'Run Script on File (Python / Bash)',
    description: 'Process files through a custom inline Python or Bash script.',
    icon: 'code',
    accentColor: '#8b5cf6',
    create: () => ({
      label: 'Process with Script',
      icon: 'code',
      accentColor: '#8b5cf6',
      matching: { extensions: ['*'], cardinality: 'batch' },
      executor: {
        type: 'script',
        interpreter: 'python3',
        scriptBody: `import sys\n\nfor path in sys.argv[1:]:\n    print(f"Working on: {path}")\n`,
        timeoutSeconds: 120,
      },
      output: { mode: 'stdout', overwriteOriginal: false },
    }),
  },
  {
    id: 'webhook-upload',
    title: 'Send File to Webhook / API',
    description: 'Upload files via multipart HTTP POST request to a webhook.',
    icon: 'cloud-upload',
    accentColor: '#ec4899',
    create: () => ({
      label: 'Upload to Webhook',
      icon: 'cloud-upload',
      accentColor: '#ec4899',
      matching: { extensions: ['*'], cardinality: 'batch' },
      executor: {
        type: 'webhook',
        method: 'POST',
        url: 'https://httpbin.org/post',
        fileFieldName: 'file',
        timeoutSeconds: 60,
      },
      output: { mode: 'none', overwriteOriginal: false },
    }),
  },
  {
    id: 'custom-cli',
    title: 'Custom CLI Command',
    description: 'Run any terminal command or CLI binary with your files.',
    icon: 'terminal',
    accentColor: '#f97316',
    create: () => ({
      label: 'Run CLI Command',
      icon: 'terminal',
      accentColor: '#f97316',
      matching: { extensions: ['*'], cardinality: 'single' },
      executor: {
        type: 'process',
        program: 'echo',
        args: ['Selected file is:', '${input}'],
        timeoutSeconds: 30,
      },
      output: { mode: 'stdout', overwriteOriginal: false },
    }),
  },
];

export function GuidedWizard({
  action,
  update,
}: {
  action: ActionDescriptor;
  update: (next: ActionDescriptor) => void;
}) {
  const [step, setStep] = useState<1 | 2 | 3>(1);

  function applyRecipe(recipeId: string) {
    const recipe = RECIPE_TEMPLATES.find(r => r.id === recipeId);
    if (!recipe) return;
    const partial = recipe.create();
    update({
      ...action,
      ...partial,
      id: action.id,
      executor: partial.executor ?? action.executor,
      matching: partial.matching ?? action.matching,
      output: partial.output ?? action.output,
    } as ActionDescriptor);
  }

  function toggleCategory(catExtensions: string[]) {
    const isAny = catExtensions.includes('*');
    if (isAny) {
      update({
        ...action,
        matching: { ...action.matching, extensions: ['*'] },
      });
      return;
    }

    const current = action.matching.extensions.filter(e => e !== '*');
    const allPresent = catExtensions.every(ext => current.includes(ext));

    let next: string[];
    if (allPresent) {
      next = current.filter(ext => !catExtensions.includes(ext));
      if (!next.length) next = ['*'];
    } else {
      next = [...new Set([...current, ...catExtensions])];
    }

    update({
      ...action,
      matching: { ...action.matching, extensions: next },
    });
  }

  function insertVariable(token: string) {
    if (action.executor.type === 'process') {
      const currentArgs = action.executor.args ?? [];
      update({
        ...action,
        executor: {
          ...action.executor,
          args: [...currentArgs, token],
        },
      });
    } else if (action.executor.type === 'script') {
      const currentBody = action.executor.scriptBody ?? '';
      update({
        ...action,
        executor: {
          ...action.executor,
          scriptBody: `${currentBody} ${token}`.trim(),
        },
      });
    }
  }

  return (
    <div className="guided-wizard-container">
      <div className="wizard-stepper">
        <button
          type="button"
          className={`wizard-step-tab ${step === 1 ? 'active' : ''}`}
          onClick={() => setStep(1)}
        >
          <span className="step-num">1</span>
          <span className="step-text">Recipe & Style</span>
        </button>
        <span className="step-divider" />
        <button
          type="button"
          className={`wizard-step-tab ${step === 2 ? 'active' : ''}`}
          onClick={() => setStep(2)}
        >
          <span className="step-num">2</span>
          <span className="step-text">Trigger Files</span>
        </button>
        <span className="step-divider" />
        <button
          type="button"
          className={`wizard-step-tab ${step === 3 ? 'active' : ''}`}
          onClick={() => setStep(3)}
        >
          <span className="step-num">3</span>
          <span className="step-text">Command & Output</span>
        </button>
      </div>

      <div className="wizard-content">
        {step === 1 && (
          <div className="wizard-step-pane">
            <div className="wizard-pane-header">
              <h3>Choose a Recipe Template</h3>
              <p>Start with a battle-tested action template or customize name and icon.</p>
            </div>

            <div className="recipe-grid">
              {RECIPE_TEMPLATES.map(recipe => (
                <button
                  key={recipe.id}
                  type="button"
                  className="recipe-card"
                  onClick={() => applyRecipe(recipe.id)}
                >
                  <div className="recipe-header">
                    <span className="recipe-icon" style={{ color: recipe.accentColor }}>
                      <Sparkles size={18} />
                    </span>
                    <strong>{recipe.title}</strong>
                  </div>
                  <p>{recipe.description}</p>
                </button>
              ))}
            </div>

            <div className="wizard-form-box">
              <div className="field">
                <label>Action Label:</label>
                <input
                  type="text"
                  placeholder="e.g. Convert to MP4"
                  value={action.label}
                  onChange={e => update({ ...action, label: e.target.value })}
                />
              </div>

              <div className="field-grid">
                <div className="field">
                  <label>Wedge Icon:</label>
                  <IconPicker
                    value={action.icon}
                    onChange={icon => update({ ...action, icon })}
                  />
                </div>
                <div className="field">
                  <label>Accent Color:</label>
                  <ColorPicker
                    value={action.accentColor}
                    onChange={accentColor => update({ ...action, accentColor })}
                  />
                </div>
              </div>
            </div>
          </div>
        )}

        {step === 2 && (
          <div className="wizard-step-pane">
            <div className="wizard-pane-header">
              <h3>Which files should trigger this action?</h3>
              <p>Toggle common file formats or type custom extensions below.</p>
            </div>

            <div className="category-pill-grid">
              {FILE_CATEGORIES.map(cat => {
                const IconComp = cat.icon;
                const isAny = cat.extensions.includes('*');
                const selected = isAny
                  ? action.matching.extensions.includes('*')
                  : cat.extensions.every(ext => action.matching.extensions.includes(ext));

                return (
                  <button
                    key={cat.id}
                    type="button"
                    className={`category-pill ${selected ? 'selected' : ''}`}
                    onClick={() => toggleCategory(cat.extensions)}
                  >
                    <IconComp size={18} />
                    <strong>{cat.label}</strong>
                    <small>
                      {isAny ? 'Matches everything' : cat.extensions.slice(0, 3).map(e => `.${e}`).join(', ')}
                    </small>
                  </button>
                );
              })}
            </div>

            <div className="wizard-form-box">
              <div className="field">
                <label>Active Extensions:</label>
                <div className="active-extensions-tags">
                  {action.matching.extensions.map(ext => (
                    <span key={ext} className="ext-badge">
                      {ext === '*' ? 'All files (*)' : `.${ext}`}
                    </span>
                  ))}
                </div>
              </div>

              <div className="field">
                <label>Drop Cardinality:</label>
                <select
                  value={action.matching.cardinality}
                  onChange={e =>
                    update({
                      ...action,
                      matching: {
                        ...action.matching,
                        cardinality: e.target.value as Cardinality,
                      },
                    })
                  }
                >
                  <option value="single">Single File Only (requires 1 file)</option>
                  <option value="each">Each File Separately (runs once per dropped file)</option>
                  <option value="batch">All Files in Batch (passes all files at once)</option>
                </select>
              </div>
            </div>
          </div>
        )}

        {step === 3 && (
          <div className="wizard-step-pane">
            <div className="wizard-pane-header">
              <h3>Command & Output Configuration</h3>
              <p>Build the command line and see the real-time execution preview below.</p>
            </div>

            {action.executor.type === 'process' && (() => {
              const proc = action.executor;
              return (
                <div className="wizard-form-box">
                  <div className="field">
                    <label>Command / Program Executable:</label>
                    <input
                      type="text"
                      className="mono"
                      placeholder="e.g. ffmpeg, imagemagick, pandoc"
                      value={proc.program}
                      onChange={e =>
                        update({
                          ...action,
                          executor: { ...proc, program: e.target.value },
                        })
                      }
                    />
                  </div>

                  <div className="field">
                    <label>Arguments List:</label>
                    <VariablePills onInsert={insertVariable} />
                    <div className="args-tag-list">
                      {(proc.args ?? []).map((arg, idx) => (
                        <div key={idx} className="arg-row">
                          <input
                            type="text"
                            className="mono"
                            value={arg}
                            onChange={e => {
                              const nextArgs = [...(proc.args ?? [])];
                              nextArgs[idx] = e.target.value;
                              update({
                                ...action,
                                executor: { ...proc, args: nextArgs },
                              });
                            }}
                          />
                          <button
                            type="button"
                            className="icon-button"
                            title="Remove argument"
                            onClick={() => {
                              const nextArgs = (proc.args ?? []).filter((_: string, i: number) => i !== idx);
                              update({
                                ...action,
                                executor: { ...proc, args: nextArgs },
                              });
                            }}
                          >
                            ✕
                          </button>
                        </div>
                      ))}
                      <button
                        type="button"
                        className="button secondary add-arg-btn"
                        onClick={() =>
                          update({
                            ...action,
                            executor: {
                              ...proc,
                              args: [...(proc.args ?? []), ''],
                            },
                          })
                        }
                      >
                        + Add Argument
                      </button>
                    </div>
                  </div>
                </div>
              );
            })()}

            {action.executor.type === 'script' && (
              <div className="wizard-form-box">
                <div className="field">
                  <label>Interpreter:</label>
                  <input
                    type="text"
                    className="mono"
                    value={action.executor.interpreter}
                    onChange={e =>
                      update({
                        ...action,
                        executor: { ...action.executor, interpreter: e.target.value } as typeof action.executor,
                      })
                    }
                  />
                </div>
                <VariablePills onInsert={insertVariable} />
                <div className="field">
                  <label>Script Body:</label>
                  <textarea
                    rows={6}
                    className="mono"
                    spellCheck={false}
                    value={action.executor.scriptBody ?? ''}
                    onChange={e =>
                      update({
                        ...action,
                        executor: { ...action.executor, scriptBody: e.target.value } as typeof action.executor,
                      })
                    }
                  />
                </div>
              </div>
            )}

            {action.executor.type === 'webhook' && (
              <div className="wizard-form-box">
                <div className="field-grid">
                  <div className="field">
                    <label>Method:</label>
                    <input
                      type="text"
                      className="mono"
                      value={action.executor.method}
                      onChange={e =>
                        update({
                          ...action,
                          executor: { ...action.executor, method: e.target.value.toUpperCase() } as typeof action.executor,
                        })
                      }
                    />
                  </div>
                  <div className="field">
                    <label>Webhook URL:</label>
                    <input
                      type="url"
                      value={action.executor.url}
                      onChange={e =>
                        update({
                          ...action,
                          executor: { ...action.executor, url: e.target.value } as typeof action.executor,
                        })
                      }
                    />
                  </div>
                </div>
              </div>
            )}

            <div className="wizard-form-box">
              <div className="field">
                <label>Output Destination:</label>
                <select
                  value={action.output.mode}
                  onChange={e =>
                    update({
                      ...action,
                      output: { ...action.output, mode: e.target.value as OutputMode },
                    })
                  }
                >
                  <option value="none">None (just execute)</option>
                  <option value="file">File (save output beside input)</option>
                  <option value="clipboard">Clipboard (copy result text)</option>
                  <option value="stdout">Standard Output (logs only)</option>
                </select>
              </div>

              {action.output.mode === 'file' && (
                <div className="field-grid">
                  <div className="field">
                    <label>Output Extension:</label>
                    <input
                      type="text"
                      placeholder="e.g. mp4"
                      value={action.output.extension ?? ''}
                      onChange={e =>
                        update({
                          ...action,
                          output: { ...action.output, extension: e.target.value.trim() || undefined },
                        })
                      }
                    />
                  </div>
                  <div className="field">
                    <label>Filename Suffix:</label>
                    <input
                      type="text"
                      placeholder="e.g. -converted"
                      value={action.output.suffix ?? ''}
                      onChange={e =>
                        update({
                          ...action,
                          output: { ...action.output, suffix: e.target.value || undefined },
                        })
                      }
                    />
                  </div>
                </div>
              )}
            </div>

            <CommandPreview action={action} />
          </div>
        )}
      </div>

      <div className="wizard-footer-nav">
        <button
          type="button"
          className="button secondary"
          disabled={step === 1}
          onClick={() => setStep(s => (s > 1 ? ((s - 1) as 1 | 2 | 3) : s))}
        >
          <ChevronLeft size={16} /> Back
        </button>
        <span className="step-indicator">Step {step} of 3</span>
        <button
          type="button"
          className="button accent"
          disabled={step === 3}
          onClick={() => setStep(s => (s < 3 ? ((s + 1) as 1 | 2 | 3) : s))}
        >
          Next <ChevronRight size={16} />
        </button>
      </div>
    </div>
  );
}
