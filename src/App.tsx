import { lazy, Suspense } from 'react';

const RadialWheel = lazy(() => import('./components/RadialWheel'));
const ProgressHud = lazy(() => import('./components/ProgressHud'));
const ActionStudio = lazy(() => import('./components/ActionStudio'));

export default function App() {
  const route = location.hash.slice(1);
  return (
    <Suspense fallback={null}>
      {route === '/wheel' ? <RadialWheel /> : route === '/hud' ? <ProgressHud /> : <ActionStudio />}
    </Suspense>
  );
}
