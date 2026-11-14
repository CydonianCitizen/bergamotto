import { listen } from '@tauri-apps/api/event';
import { AlertCircle, Check, LoaderCircle, Minus, X } from 'lucide-react';
import { useEffect, useState } from 'react';
import { errorText, ipc } from '../ipc';
import type { JobProgress, JobSnapshot } from '../types/actions';

function JobPill({ job }: { job: JobProgress }) {
  const [error, setError] = useState('');
  const Icon = { running: LoaderCircle, cancelling: LoaderCircle, succeeded: Check, failed: AlertCircle, cancelled: Minus }[job.status];
  const active = job.status === 'running' || job.status === 'cancelling';

  return (
    <article className={`hud-pill ${job.status}`}>
      <div className="hud-icon"><Icon size={19} className={active ? 'spinning' : ''} /></div>
      <div className="hud-copy">
        <strong>{job.label}</strong>
        <p title={error || job.message}>{error || job.message}</p>
        <progress value={job.fraction} max={1} aria-label={`${job.label} progress`} />
      </div>
      <button className="hud-close" aria-label={active ? 'Cancel job' : 'Dismiss job'} disabled={job.status === 'cancelling'}
        onClick={() => void (active ? ipc.cancelJob(job.jobId) : ipc.dismissJob(job.jobId)).catch(error => setError(errorText(error)))}>
        <X size={17} />
      </button>
    </article>
  );
}

export default function ProgressHud() {
  const [snapshot, setSnapshot] = useState<JobSnapshot>({ revision: -1, jobs: [] });
  const [error, setError] = useState('');
  useEffect(() => {
    let alive = true;
    function update(next: JobSnapshot) {
      if (alive) setSnapshot(current => next.revision >= current.revision ? next : current);
    }
    function fail(error: unknown) {
      if (alive) setError(errorText(error));
    }
    const subscription = listen<JobSnapshot>('jobs:updated', ({ payload }) => update(payload));
    void subscription.then(() => ipc.jobSnapshot()).then(update).catch(fail);
    return () => {
      alive = false;
      void subscription.then(unlisten => unlisten()).catch(() => {});
    };
  }, []);

  return <main className="hud-view" aria-label="Job progress">
    {error && <p className="hud-pill" role="alert">{error}</p>}
    {snapshot.jobs.map(job => <JobPill key={job.jobId} job={job} />)}
  </main>;
}
