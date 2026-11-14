import {
  Archive,
  Braces,
  FileCode2,
  FileImage,
  FileText,
  Film,
  Folder,
  GitMerge,
  Globe,
  Hash,
  Image,
  Lock,
  Music2,
  ScanLine,
  Scissors,
  Share2,
  Sparkles,
  Terminal,
  UploadCloud,
  Wand2,
  Zap,
  type LucideIcon,
} from 'lucide-react';
import type { ExecutorConfig } from '../types/actions';

export interface IconOption {
  id: string;
  label: string;
  icon: LucideIcon;
}

export const AVAILABLE_ICONS: IconOption[] = [
  { id: 'video', label: 'Video', icon: Film },
  { id: 'audio', label: 'Audio', icon: Music2 },
  { id: 'image', label: 'Image', icon: Image },
  { id: 'document', label: 'Document', icon: FileText },
  { id: 'archive', label: 'Archive', icon: Archive },
  { id: 'terminal', label: 'Terminal', icon: Terminal },
  { id: 'code', label: 'Code', icon: FileCode2 },
  { id: 'wand', label: 'Magic Wand', icon: Wand2 },
  { id: 'cloud-upload', label: 'Cloud Upload', icon: UploadCloud },
  { id: 'lock', label: 'Security / Lock', icon: Lock },
  { id: 'hash', label: 'Hash / Checksum', icon: Hash },
  { id: 'scissors', label: 'Cut / Trim', icon: Scissors },
  { id: 'merge', label: 'Merge', icon: GitMerge },
  { id: 'sparkles', label: 'AI / Enhance', icon: Sparkles },
  { id: 'folder', label: 'Folder', icon: Folder },
  { id: 'share', label: 'Share', icon: Share2 },
  { id: 'zap', label: 'Fast Action', icon: Zap },
  { id: 'globe', label: 'Webhook / Web', icon: Globe },
];

const icons = new Map<string, LucideIcon>(Object.entries({
  archive: Archive,
  braces: Braces,
  code: FileCode2,
  film: Film,
  video: Film,
  image: Image,
  'file-image': FileImage,
  music: Music2,
  audio: Music2,
  document: FileText,
  'file-text': FileText,
  globe: Globe,
  scan: ScanLine,
  terminal: Terminal,
  'upload-cloud': UploadCloud,
  'cloud-upload': UploadCloud,
  lock: Lock,
  hash: Hash,
  scissors: Scissors,
  merge: GitMerge,
  sparkles: Sparkles,
  folder: Folder,
  share: Share2,
  wand: Wand2,
  zap: Zap,
}));

const fallback: Record<ExecutorConfig['type'], LucideIcon> = {
  process: Terminal,
  script: Braces,
  webhook: Globe,
};

export function ActionIcon({
  icon,
  type,
  size = 18,
}: {
  icon?: string;
  type: ExecutorConfig['type'];
  size?: number;
}) {
  const Icon = icons.get(icon ?? '') ?? fallback[type];
  return <Icon aria-hidden="true" size={size} strokeWidth={1.7} />;
}
