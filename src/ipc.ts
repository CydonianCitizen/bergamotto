import { invoke } from '@tauri-apps/api/core';
import type { ActionDescriptor, JobSnapshot, WheelPresentation } from './types/actions';

export const ipc = {
  listActions: () => invoke<ActionDescriptor[]>('list_actions'),
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
  globalDragEnabled: () => invoke<boolean>('global_drag_enabled'),
  toggleGlobalDrag: () => invoke<void>('toggle_global_drag'),
  closeDragWheel: (session: number) => invoke<void>('close_drag_wheel', { session }),
};

export function errorText(error: unknown) {
  return error instanceof Error ? error.message : String(error);
}
