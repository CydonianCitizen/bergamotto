import { useEffect, useRef, useState } from 'react';
import { errorText, ipc } from '../../ipc';
import { Field } from './Fields';

export default function WebhookSecret({ credentialKey }: { credentialKey: string }) {
  const [configured, setConfigured] = useState<boolean | null>(null);
  const [value, setValue] = useState('');
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState('');
  const revision = useRef(0);

  useEffect(() => {
    const request = ++revision.current;
    void ipc.hasSecret(credentialKey).then(result => {
      if (request === revision.current) setConfigured(result);
    }).catch(reason => {
      if (request === revision.current) setError(errorText(reason));
    });
    return () => { revision.current += 1; };
  }, [credentialKey]);

  async function update(next: boolean) {
    revision.current += 1;
    setBusy(true);
    setError('');
    setValue('');
    try {
      if (next) await ipc.setSecret(credentialKey, value);
      else await ipc.deleteSecret(credentialKey);
      setConfigured(next);
    } catch (reason) {
      setError(errorText(reason));
    } finally {
      setBusy(false);
    }
  }

  return <div className="webhook-secret">
    <div className="credential-heading">
      <strong>OS credential store</strong>
      {configured !== null && <span className={`credential-status ${configured ? 'configured' : 'missing'}`} role="status">
        {configured ? 'Configured' : 'Missing'}
      </span>}
    </div>
    <Field label={configured ? 'Replacement token' : 'Token'}>
      <input type="password" autoComplete="new-password" spellCheck={false} value={value} disabled={busy} onChange={event => setValue(event.target.value)} />
    </Field>
    {error && <p className="save-status invalid" role="alert">{error}</p>}
    <div className="footer-buttons">
      <button type="button" className="button secondary" disabled={busy || !value} onClick={() => void update(true)}>{configured ? 'Update token' : 'Store token'}</button>
      <button type="button" className="button destructive" disabled={busy || configured !== true} onClick={() => void update(false)}>Delete token</button>
    </div>
  </div>;
}
