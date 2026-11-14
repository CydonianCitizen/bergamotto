import { Plus, X } from 'lucide-react';
import { useId, useState } from 'react';
import type { ChangeEvent, ReactNode } from 'react';

export function Field({ label, children, hint }: { label: string; children: ReactNode; hint?: string }) {
  return <label className="field"><span>{label}</span>{children}{hint && <small>{hint}</small>}</label>;
}

export function Section({ title, detail, children }: { title: string; detail: string; children: ReactNode }) {
  return <section className="editor-section">
    <div className="section-title"><h3>{title}</h3><p>{detail}</p></div>
    <div className="section-fields">{children}</div>
  </section>;
}

export function Extensions({ values, update }: { values: string[]; update: (next: string[]) => void }) {
  const id = useId();
  const [text, setText] = useState('');
  function add() {
    const additions = text.split(/[\s,]+/).map(value => value.replace(/^\./, '').toLowerCase()).filter(Boolean);
    if (!additions.length) return;
    if (additions.includes('*')) update(['*']);
    else update([...new Set([...values.filter(value => value !== '*'), ...additions])]);
    setText('');
  }
  return <div className="field">
    <label htmlFor={id}>File extensions</label>
    <div className="chip-input">
      {values.map(value => <button type="button" className="chip" key={value} aria-label={`Remove ${value === '*' ? 'any extension' : value}`}
        onClick={() => update(values.filter(extension => extension !== value))}>
        {value === '*' ? 'Any extension' : value}<X size={12} />
      </button>)}
      <input id={id} value={text} spellCheck={false} placeholder="Add extension" onChange={event => setText(event.target.value)}
        onBlur={add} onKeyDown={event => {
          if (event.key === 'Enter' || event.key === ',') {
            event.preventDefault();
            add();
          }
        }} />
    </div>
    <small>Press Enter to add an extension. Use * for any file type.</small>
  </div>;
}

export function Arguments({ values, update }: { values: string[]; update: (next: string[]) => void }) {
  const id = useId();
  return <fieldset className="field argument-list">
    <legend>Arguments</legend>
    {values.map((value, index) => <div key={index}>
      <input className="mono" aria-label={`Argument ${index + 1}`} value={value} onChange={event => {
        update(values.map((argument, position) => position === index ? event.target.value : argument));
      }} />
      <button className="icon-button" type="button" aria-label={`Remove argument ${index + 1}`}
        onClick={() => update(values.filter((_, position) => position !== index))}><X size={16} /></button>
    </div>)}
    <button type="button" className="button secondary" aria-describedby={id} onClick={() => update([...values, ''])}>
      <Plus size={15} /> Add argument
    </button>
    <small id={id}>Enter each argument separately. Use ${'{input}'} and ${'{output}'} as file placeholders.</small>
  </fieldset>;
}

export function StringMap({ label, value, onChange, onError }: {
  label: string;
  value?: Record<string, string>;
  onChange: (value: Record<string, string>) => void;
  onError: (message: string) => void;
}) {
  const [text, setText] = useState(() => JSON.stringify(value ?? {}, null, 2));
  function change(event: ChangeEvent<HTMLTextAreaElement>) {
    const raw = event.target.value;
    setText(raw);
    try {
      const next: unknown = JSON.parse(raw || '{}');
      if (typeof next !== 'object' || next === null || Array.isArray(next)
        || Object.values(next).some(value => typeof value !== 'string')) {
        throw new Error();
      }
      onChange(next as Record<string, string>);
      onError('');
    } catch {
      onError(`${label} must be a JSON object with string values.`);
    }
  }
  return <Field label={label}><textarea rows={3} className="mono" spellCheck={false} value={text} onChange={change} /></Field>;
}
