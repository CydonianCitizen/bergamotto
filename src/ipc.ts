import { invoke } from '@tauri-apps/api/core';
import type { ActionDescriptor, AppSettings, JobSnapshot, PackCollision, PackPreview, ToolStatus, WheelPresentation } from './types/actions';

export const ipc = {
  listActions: () => invoke<ActionDescriptor[]>('list_actions'),
  setSecret: (key: string, value: string) => invoke<void>('set_secret', { key, value }),
  hasSecret: (key: string) => invoke<boolean>('has_secret', { key }),
  deleteSecret: (key: string) => invoke<void>('delete_secret', { key }),
  saveAction: (action: ActionDescriptor) => invoke<void>('save_action', { action }),
  createAction: (action: ActionDescriptor) => invoke<void>('create_action', { action }),
  deleteAction: (id: string) => invoke<void>('delete_action', { id }),
  duplicateAction: (id: string) => invoke<ActionDescriptor>('duplicate_action', { id }),
  matchActions: (files: string[]) => invoke<ActionDescriptor[]>('match_actions', { files }),
  invokeAction: (actionId: string, files: string[], dragSession: number | null) =>
    invoke<string>('invoke_action', { actionId, files, dragSession }),
  cancelJob: (jobId: string) => invoke<void>('cancel_job', { jobId }),
  dismissJob: (jobId: string) => invoke<void>('dismiss_job', { jobId }),
  jobSnapshot: () => invoke<JobSnapshot>('job_snapshot'),
  openActionsFolder: () => invoke<void>('open_actions_folder'),
  presentWheel: (files: string[]) => invoke<void>('present_wheel', { files }),
  wheelState: () => invoke<WheelPresentation>('wheel_state'),
  getSettings: () => invoke<AppSettings>('get_settings'),
  saveSettings: ({ modifierKey, customPaths, toolOverrides }: AppSettings) =>
    invoke<void>('save_settings', { modifierKey, customPaths, toolOverrides }),
  checkActionDependencies: () => invoke<Record<string, ToolStatus>>('check_action_dependencies'),
  readActionPack: (clipboard: boolean) => invoke<string | null>('read_action_pack', { clipboard }),
  previewActionPack: (document: string) => invoke<PackPreview>('preview_action_pack', { document }),
  importActionPack: (document: string, collision: PackCollision) => invoke<string[]>('import_action_pack', { document, collision }),
  exportActionPack: (id: string | null, clipboard: boolean) => invoke<boolean>('export_action_pack', { id, clipboard }),
  toggleGlobalDrag: () => invoke<void>('toggle_global_drag'),
  closeDragWheel: (session: number) => invoke<void>('close_drag_wheel', { session }),
};

export function errorText(error: unknown) {
  return error instanceof Error ? error.message : String(error);
}
