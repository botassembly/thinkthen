import matrix from '../data/function-surfaces.json' with { type: 'json' };
export const functions = matrix.functions;
export const surfaces = matrix.surfaces;
export const cellId = (fn, surface) => `functions/${fn}/${surface}`;
export const cellRoute = (fn, surface = 'command') => `/${cellId(fn, surface)}/`;
export const surfaceRoute = (surface) => {
  const entry = surfaces.find(({ id }) => id === surface) ?? surfaces[0];
  return entry.id === 'command' ? '/install/' : `/${entry.section}/${entry.id}/`;
};
export function pickerRoute(route, surface) {
  const fn = route.match(/^\/functions\/([^/]+)(?:\/[^/]+)?\/?$/)?.[1];
  return functions.includes(fn) ? cellRoute(fn, surface) : surfaceRoute(surface);
}
