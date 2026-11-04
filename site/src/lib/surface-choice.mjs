import { surfaces } from './function-surfaces.mjs';
const key = 'thinkthen-language';
export function readSurface() {
  try {
    const value = localStorage.getItem(key);
    return surfaces.some(({id}) => id === value) ? value : 'command';
  } catch { return 'command'; }
}
export function writeSurface(surface) { try { localStorage.setItem(key, surface); } catch {} }
export function applySurfaceGroups(surface) {
  for (const group of document.querySelectorAll('[data-language-group]')) {
    const variants = [...group.querySelectorAll(':scope > [data-language-variant]')];
    const selected = variants.find((variant) => variant.dataset.languageVariant === surface);
    const shown = selected ?? variants.find((variant) => variant.dataset.languageVariant === 'command');
    for (const variant of variants) variant.hidden = variant !== shown;
    const fallback = group.querySelector(':scope > [data-language-fallback]');
    if (fallback) fallback.hidden = Boolean(selected) || surface === 'command';
    // One installer stays in the actual reading/tab order: Command needs it
    // first; other surfaces show their question and package setup before it.
    const installer = [...(group.parentElement?.children ?? [])].find((node) => node.dataset.commandInstall === group.dataset.languageGroup);
    if (installer) {
      const before = shown?.dataset.languageVariant === 'command';
      const inPlace = before ? installer.nextElementSibling === group : group.nextElementSibling === installer;
      if (!inPlace) {
        const focused = installer.contains(document.activeElement) ? document.activeElement : null;
        if (before) group.before(installer); else group.after(installer);
        focused?.focus({ preventScroll: true });
      }
    }
  }
}
