import { listen } from '@tauri-apps/api/event';
import { open } from '@tauri-apps/plugin-dialog';
import { CheckCircle2, Clock, Download, FileCode, FolderOpen, LoaderCircle, Play, XCircle } from 'lucide-react';
import { useEffect, useRef, useState } from 'react';
import { errorText, ipc } from '../../ipc';
import type { ActionDescriptor, JobSnapshot, ToolStatus } from '../../types/actions';
import ToolConsentDialog from '../ToolConsentDialog';

export function TestSandbox({
  action,
  toolStatus,
}: {
  action: ActionDescriptor;
  toolStatus?: ToolStatus;
}) {
  const [samplePath, setSamplePath] = useState('/tmp/bergamot-test-file.txt');
  const [running, setRunning] = useState(false);
  const [duration, setDuration] = useState<number | null>(null);
  const [showConsent, setShowConsent] = useState(false);
  const [result, setResult] = useState<{
    status: 'succeeded' | 'failed' | 'cancelled';
    message: string;
  } | null>(null);
  const activeJobId = useRef<string | null>(null);
  const startTime = useRef<number>(0);

  const toolFound = Boolean(toolStatus?.path);
  const toolName = toolStatus?.tool ?? (action.executor.type === 'process' ? action.executor.program : undefined);

  async function chooseFile() {
    try {
      const selected = await open({
        multiple: false,
        directory: false,
        title: 'Choose original file for action test',
      });
      if (typeof selected === 'string') {
        setSamplePath(selected);
      }
    } catch (e) {
      console.error(e);
    }
  }

  async function runTest() {
    if (running || !action.id) return;
    if (!toolFound) {
      setShowConsent(true);
      return;
    }
    setRunning(true);
    setResult(null);
    setDuration(null);
    startTime.current = performance.now();

    try {
      const jobId = await ipc.invokeAction(action.id, [samplePath], null);
      activeJobId.current = jobId;
    } catch (error) {
      setRunning(false);
      setDuration(Math.round(performance.now() - startTime.current));
      setResult({
        status: 'failed',
        message: errorText(error),
      });
    }
  }

  useEffect(() => {
    let alive = true;
    const subscription = listen<JobSnapshot>('jobs:updated', ({ payload }) => {
      if (!alive || !activeJobId.current) return;
      const job = payload.jobs.find(j => j.jobId === activeJobId.current);
      if (job && (job.status === 'succeeded' || job.status === 'failed' || job.status === 'cancelled')) {
        setRunning(false);
        setDuration(Math.round(performance.now() - startTime.current));
        setResult({
          status: job.status,
          message: job.message || `Action ${job.status}`,
        });
        activeJobId.current = null;
      }
    });

    return () => {
      alive = false;
      void subscription.then(unlisten => unlisten()).catch(() => {});
    };
  }, []);

  return (
    <div className="test-sandbox-card">
      <div className="sandbox-header">
        <h4>Action test</h4>
        <p>
          Runs the saved action on the selected original file. The action can modify that file or
          perform its normal side effects; this run is not isolated.
        </p>
      </div>

      {toolName && (
        <div className={`tool-resolution-banner ${toolFound ? 'found' : 'missing'}`}>
          {toolFound ? (
            <>
              <CheckCircle2 size={16} />
              <span>
                <strong>Tool available ({toolStatus?.source}):</strong> <code>{toolStatus?.path}</code>
              </span>
            </>
          ) : (
            <>
              <XCircle size={16} />
              <div className="tool-missing-row">
                <span>
                  <strong>Tool missing:</strong> <code>{toolName}</code> could not be found.
                </span>
                <button
                  type="button"
                  className="button accent sm icon-text"
                  onClick={() => setShowConsent(true)}
                >
                  <Download size={13} /> Install Tool
                </button>
              </div>
            </>
          )}
        </div>
      )}

      <div className="sandbox-controls">
        <div className="sample-file-field">
          <label>Original file to use:</label>
          <div className="file-input-row">
            <input
              type="text"
              className="mono"
              value={samplePath}
              onChange={e => setSamplePath(e.target.value)}
              placeholder="/path/to/sample/file.ext"
            />
            <button
              type="button"
              className="button secondary"
              onClick={() => void chooseFile()}
              title="Browse files"
            >
              <FolderOpen size={15} /> Browse…
            </button>
          </div>
        </div>

        <button
          type="button"
          className="button accent run-test-btn"
          disabled={running || !samplePath.trim()}
          onClick={() => void runTest()}
        >
          {running ? <LoaderCircle size={15} className="spinning" /> : <Play size={15} />}
          {running ? 'Executing action…' : 'Run Test Now'}
        </button>
      </div>

      {result && (
        <div className={`sandbox-result-box ${result.status}`}>
          <div className="result-header">
            <span className="result-status-tag">
              {result.status === 'succeeded' ? <CheckCircle2 size={14} /> : <XCircle size={14} />}
              {result.status.toUpperCase()}
            </span>
            {duration !== null && (
              <span className="result-duration">
                <Clock size={13} /> {duration} ms
              </span>
            )}
          </div>
          <div className="result-output-log mono">
            <FileCode size={13} />
            <span>{result.message}</span>
          </div>
        </div>
      )}

      {showConsent && toolName && (
        <ToolConsentDialog
          toolId={toolName}
          onClose={() => setShowConsent(false)}
          onInstalled={() => {
            setShowConsent(false);
          }}
        />
      )}
    </div>
  );
}
