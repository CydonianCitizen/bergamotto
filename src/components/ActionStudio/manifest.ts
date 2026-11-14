import checkManifest from 'virtual:action-validator';
import type { ActionDescriptor, ExecutorConfig, Platform } from '../../types/actions';

export function validateManifest(value: unknown) {
  if (checkManifest(value)) return [];
  const type = (value as Partial<ActionDescriptor> | null)?.executor?.type;
  const errors = checkManifest.errors ?? [];
  const relevant = errors.filter(error => {
    if (error.schemaPath === '#/properties/executor/oneOf' && type && type in executors) return false;
    const definition = error.schemaPath.match(/^#\/definitions\/(process|script|webhook)\//)?.[1];
    return !definition || !type || definition === type;
  });
  return [...new Set((relevant.length ? relevant : errors).map(error => {
    const field = error.instancePath.replaceAll('/', '.').slice(1) || 'Manifest';
    const detail = error.keyword === 'additionalProperties'
      ? `Unexpected field "${String(error.params.additionalProperty)}"`
      : error.message;
    return `${field}: ${detail}`;
  }))];
}

export function formatManifest(action: ActionDescriptor) {
  return JSON.stringify(action, null, 2);
}

export function inspectManifest(text: string): { action?: ActionDescriptor; issues: string[] } {
  try {
    const action: unknown = JSON.parse(text);
    const issues = validateManifest(action);
    return issues.length ? { issues } : { action: action as ActionDescriptor, issues };
  } catch (error) {
    return { issues: [error instanceof Error ? error.message : String(error)] };
  }
}

export const platforms: Platform[] = ['macos', 'windows', 'linux'];
export const platformLabels: Record<Platform, string> = { macos: 'macOS', windows: 'Windows', linux: 'Linux' };

export const executors: Record<ExecutorConfig['type'], () => ExecutorConfig> = {
  process: () => ({ type: 'process', program: '', args: [], timeoutSeconds: 60 }),
  script: () => ({ type: 'script', interpreter: '', scriptBody: '', timeoutSeconds: 60 }),
  webhook: () => ({ type: 'webhook', method: 'POST', url: '', timeoutSeconds: 60 }),
};

export function newAction(presetExtensions?: string[]): ActionDescriptor {
  return {
    schemaVersion: 1,
    id: `user.action-${crypto.randomUUID().slice(0, 8)}`,
    label: '',
    enabled: true,
    platforms: [...platforms],
    matching: { cardinality: 'single', extensions: presetExtensions && presetExtensions.length ? presetExtensions : ['*'] },
    executor: executors.process(),
    output: { mode: 'none', overwriteOriginal: false },
  };
}
