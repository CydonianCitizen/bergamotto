export const center = 210;
const innerRadius = 68;
const outerRadius = 165;
const gap = 4;
const rotation = -90;

function point(radius: number, angle: number) {
  const radians = angle * Math.PI / 180;
  return [center + Math.cos(radians) * radius, center + Math.sin(radians) * radius];
}

export function wedge(index: number, count: number) {
  const sweep = 360 / count;
  const middle = rotation + index * sweep;
  const start = middle - (sweep - gap) / 2;
  const end = middle + (sweep - gap) / 2;
  const largeArc = sweep - gap > 180 ? 1 : 0;
  return {
    path: [
      'M', ...point(innerRadius, start), 'L', ...point(outerRadius, start),
      'A', outerRadius, outerRadius, 0, largeArc, 1, ...point(outerRadius, end),
      'L', ...point(innerRadius, end),
      'A', innerRadius, innerRadius, 0, largeArc, 0, ...point(innerRadius, start), 'Z',
    ].join(' '),
    label: point(118, middle),
  };
}

export function hitWedge(x: number, y: number, count: number): number | null {
  const dx = x - center;
  const dy = y - center;
  const radius = Math.hypot(dx, dy);
  if (radius < innerRadius || radius > outerRadius || count === 0) return null;

  const sweep = 360 / count;
  const angle = Math.atan2(dy, dx) * 180 / Math.PI - rotation;
  const shifted = ((angle + sweep / 2 + 360) % 360);
  const localAngle = shifted % sweep;
  return localAngle >= gap / 2 && localAngle <= sweep - gap / 2 ? Math.floor(shifted / sweep) : null;
}
