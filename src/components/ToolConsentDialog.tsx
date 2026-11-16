import { listen } from '@tauri-apps/api/event';
import { AlertCircle, Check, Copy, Download, ExternalLink, HardDrive, ShieldAlert, X } from 'lucide-react';
import { useEffect, useRef, useState } from 'react';
import { errorText, ipc } from '../ipc';
import type { ManagedToolInfo, ToolDownloadProgress } from '../types/actions';

interface ToolConsentDialogProps {
  toolId: string;
  onInstalled?: () => void;
  onUseMyOwn?: () => void;
  onClose: () => void;
}

export default function ToolConsentDialog({
  toolId,
  onInstalled,
  onUseMyOwn,
  onClose,
}: ToolConsentDialogProps) {
  const dialog = useRef<HTMLDialogElement>(null);
  const [toolInfo, setToolInfo] = useState<ManagedToolInfo | null>(null);
  const [loading, setLoading] = useState(true);
  const [downloading, setDownloading] = useState(false);
  const [progress, setProgress] = useState<ToolDownloadProgress | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [copiedManual, setCopiedManual] = useState(false);

  useEffect(() => {
    dialog.current?.showModal();
    let alive = true;

    void ipc
      .getManagedToolsStatus()
      .then(tools => {
        if (!alive) return;
        const found = tools.find(t => t.id === toolId);
        setToolInfo(found ?? null);
        setLoading(false);
      })
      .catch(err => {
        if (!alive) return;
        setError(errorText(err));
        setLoading(false);
      });

    const unlisten = listen<ToolDownloadProgress>('managed-tool:progress', event => {
      if (!alive) return;
      if (event.payload.tool === toolId) {
        setProgress(event.payload);
        if (event.payload.stage === 'completed') {
          setDownloading(false);
          onInstalled?.();
        } else if (event.payload.stage === 'failed') {
          setDownloading(false);
        }
      }
    });

    return () => {
      alive = false;
      void unlisten.then(fn => fn());
    };
  }, [toolId, onInstalled]);

  async function handleInstall() {
    setDownloading(true);
    setError(null);
    setProgress({
      tool: toolId,
      downloadedBytes: 0,
      totalBytes: toolInfo?.estimatedSizeBytes ?? null,
      stage: 'downloading',
    });

    try {
      await ipc.installManagedTool(toolId);
    } catch (err) {
      setError(errorText(err));
      setDownloading(false);
    }
  }

  async function handleCancelDownload() {
    try {
      await ipc.cancelManagedToolDownload(toolId);
    } catch {
      // ignore
    }
    setDownloading(false);
    setProgress(null);
  }

  async function copyManualInstruction() {
    if (toolInfo?.manualInstructions) {
      try {
        await navigator.clipboard.writeText(toolInfo.manualInstructions);
        setCopiedManual(true);
        setTimeout(() => setCopiedManual(false), 2000);
      } catch {
        // ignore
      }
    }
  }

  function formatBytes(bytes: number): string {
    if (bytes <= 0) return '0 B';
    const mb = bytes / (1024 * 1024);
    if (mb >= 1) return `${mb.toFixed(1)} MB`;
    return `${(bytes / 1024).toFixed(0)} KB`;
  }

  const percent =
    progress?.totalBytes && progress.totalBytes > 0
      ? Math.min(100, Math.round((progress.downloadedBytes / progress.totalBytes) * 100))
      : null;

  return (
    <dialog
      ref={dialog}
      className="studio-dialog consent-dialog"
      aria-labelledby="consent-title"
      onCancel={event => {
        event.preventDefault();
        if (!downloading) onClose();
      }}
    >
      <div className="consent-content">
        <div className="consent-header">
          <div className="consent-icon-badge">
            <Download size={24} />
          </div>
          <div>
            <h2 id="consent-title">Tool Required: {toolInfo?.name ?? toolId}</h2>
            <p className="consent-subtitle">
              {toolInfo?.description ?? 'This action requires an external tool to run.'}
            </p>
          </div>
          {!downloading && (
            <button
              type="button"
              className="button-icon-close"
              onClick={onClose}
              aria-label="Close"
            >
              <X size={18} />
            </button>
          )}
        </div>

        {loading ? (
          <div className="consent-loading">Loading tool details…</div>
        ) : !toolInfo ? (
          <div className="consent-error">
            <AlertCircle size={18} />
            <span>Unknown tool "{toolId}".</span>
          </div>
        ) : (
          <>
            <div className="consent-meta-grid">
              <div className="meta-card">
                <span className="meta-label">Version</span>
                <span className="meta-value">{toolInfo.version ?? 'Latest stable'}</span>
              </div>
              <div className="meta-card">
                <span className="meta-label">Download Size</span>
                <span className="meta-value">
                  {toolInfo.estimatedSizeBytes > 0
                    ? `~${formatBytes(toolInfo.estimatedSizeBytes)}`
                    : 'Unknown'}
                </span>
              </div>
              <div className="meta-card">
                <span className="meta-label">License</span>
                <span className="meta-value">{toolInfo.licenseName}</span>
              </div>
            </div>

            {/* License notice banner */}
            <div className={`consent-license-banner ${toolInfo.isCopyleft ? 'copyleft' : ''}`}>
              <div className="banner-icon">
                <ShieldAlert size={20} />
              </div>
              <div className="banner-text">
                <strong>Open-Source License Notice:</strong>
                <p>
                  <strong>{toolInfo.name}</strong> is licensed under{' '}
                  <a
                    href={toolInfo.licenseUrl}
                    target="_blank"
                    rel="noreferrer"
                    className="license-link inline-link"
                  >
                    {toolInfo.licenseName} <ExternalLink size={12} />
                  </a>
                  {toolInfo.isCopyleft && ' (copyleft license)'}. By downloading, you agree to its license terms.
                </p>
                <div className="banner-links">
                  <a
                    href={toolInfo.sourceUrl}
                    target="_blank"
                    rel="noreferrer"
                    className="inline-link"
                  >
                    View Source Code <ExternalLink size={12} />
                  </a>
                </div>
              </div>
            </div>

            {!toolInfo.supportedOnCurrentPlatform && toolInfo.manualInstructions && (
              <div className="consent-manual-box">
                <p>
                  Automatic download is not available for this platform. Please install it using your system package manager:
                </p>
                <div className="cli-snippet">
                  <code>{toolInfo.manualInstructions}</code>
                  <button
                    type="button"
                    className="button secondary sm icon-text"
                    onClick={copyManualInstruction}
                  >
                    {copiedManual ? <Check size={14} /> : <Copy size={14} />}
                    {copiedManual ? 'Copied' : 'Copy'}
                  </button>
                </div>
              </div>
            )}

            {downloading && (
              <div className="consent-progress-section">
                <div className="progress-status-row">
                  <span className="stage-label">
                    {progress?.stage === 'extracting'
                      ? 'Extracting archive…'
                      : progress?.stage === 'verifying'
                      ? 'Verifying installation…'
                      : 'Downloading…'}
                  </span>
                  <span className="size-label">
                    {progress ? formatBytes(progress.downloadedBytes) : '0 B'}
                    {progress?.totalBytes ? ` / ${formatBytes(progress.totalBytes)}` : ''}
                    {percent !== null ? ` (${percent}%)` : ''}
                  </span>
                </div>
                <div className="progress-bar-track">
                  <div
                    className="progress-bar-fill"
                    style={{ width: `${percent ?? (progress?.stage === 'downloading' ? 30 : 80)}%` }}
                  />
                </div>
              </div>
            )}

            {error && (
              <div className="consent-error-box">
                <AlertCircle size={16} />
                <span>{error}</span>
              </div>
            )}

            <div className="consent-footer">
              {downloading ? (
                <button
                  type="button"
                  className="button secondary"
                  onClick={handleCancelDownload}
                >
                  Cancel Download
                </button>
              ) : (
                <>
                  <button
                    type="button"
                    className="button secondary"
                    onClick={onClose}
                  >
                    Cancel
                  </button>
                  {onUseMyOwn && (
                    <button
                      type="button"
                      className="button secondary"
                      onClick={() => {
                        onClose();
                        onUseMyOwn();
                      }}
                      title="Open Settings to specify your existing binary path"
                    >
                      <HardDrive size={15} /> Use My Own
                    </button>
                  )}
                  {toolInfo.supportedOnCurrentPlatform && (
                    <button
                      type="button"
                      className="button accent"
                      onClick={handleInstall}
                    >
                      <Download size={15} /> Download &amp; Install
                    </button>
                  )}
                </>
              )}
            </div>
          </>
        )}
      </div>
    </dialog>
  );
}
