export type Platform = 'macos' | 'windows' | 'linux';
export type ModifierKey = 'Shift' | 'Ctrl' | 'Alt';

export interface AppSettings {
  globalHookEnabled: boolean;
  modifierKey: ModifierKey;
  customPaths: string[];
  toolOverrides: Record<string, string>;
}

export interface ToolStatus {
  tool: string;
  path: string | null;
  message: string | null;
}

export type PackCollision = 'reject' | 'overwrite' | 'duplicate';
export interface PackPreview {
  count: number;
  collisions: string[];
}
export type Cardinality = 'single' | 'each' | 'batch';
export type OutputMode = 'none' | 'file' | 'clipboard' | 'stdout';

export interface MatchingRule {
  extensions: string[];
  cardinality: Cardinality;
}

export type ExecutorConfig = {
  type: 'process';
  program: string;
  args?: string[];
  workingDirectory?: string;
  env?: Record<string, string>;
  timeoutSeconds?: number;
} | {
  type: 'script';
  interpreter: string;
  scriptBody?: string;
  scriptPath?: string;
  timeoutSeconds?: number;
} | {
  type: 'webhook';
  method: string;
  url: string;
  headers?: Record<string, string>;
  fileFieldName?: string;
  credentialKey?: string;
  timeoutSeconds?: number;
};

export interface OutputConfig {
  mode: OutputMode;
  extension?: string;
  suffix?: string;
  overwriteOriginal?: boolean;
}

export interface ActionDescriptor {
  schemaVersion: number;
  id: string;
  label: string;
  description?: string;
  icon?: string;
  enabled: boolean;
  platforms: Platform[];
  matching: MatchingRule;
  executor: ExecutorConfig;
  output: OutputConfig;
}

export interface DragContext {
  platform: Platform;
  files: string[];
}

export interface ActionInvocation {
  actionId: string;
  context: DragContext;
}

export interface ActionProgress {
  actionId: string;
  completed: number;
  total: number;
}

export interface ActionResult {
  actionId: string;
  completed: number;
  outputFiles: string[];
  stdout: number[];
  stderr: number[];
  clipboard: string | null;
}

export interface WheelPresentation {
  files: string[];
  actions: ActionDescriptor[];
  dragSession: number | null;
  modifierKey: ModifierKey;
}

export interface JobProgress extends ActionProgress {
  jobId: string;
  label: string;
  fraction: number;
  status: 'running' | 'cancelling' | 'succeeded' | 'cancelled' | 'failed';
  message: string;
}

export interface JobSnapshot {
  revision: number;
  jobs: JobProgress[];
}
