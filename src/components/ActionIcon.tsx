import { Braces, FileCode2, FileImage, Film, Globe, Image, ScanLine, Terminal, UploadCloud, Zap } from 'lucide-react';
import type { ExecutorConfig } from '../types/actions';

const icons = new Map(Object.entries({
  braces: Braces, code: FileCode2, film: Film, image: Image, 'file-image': FileImage,
  globe: Globe, scan: ScanLine, terminal: Terminal, 'upload-cloud': UploadCloud, zap: Zap,
}));
const fallback = { process: Terminal, script: Braces, webhook: Globe };

export function ActionIcon({ icon, type, size = 18 }: { icon?: string; type: ExecutorConfig['type']; size?: number }) {
  const Icon = icons.get(icon ?? '') ?? fallback[type];
  return <Icon aria-hidden="true" size={size} strokeWidth={1.7} />;
}
