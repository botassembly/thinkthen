// Derive the available first-call variants from the same cells as the static routes.
import fs from 'node:fs';
import path from 'node:path';
import { functions, surfaces, cellId } from './function-surfaces.mjs';
export function missingFunctionVariants(examples, existing) {
  const used = [...new Set(examples.map((id) => id.match(/^functions\/([^/]+)\/[^/]+$/)?.[1]).filter(Boolean))];
  if (used.length !== 1 || !functions.includes(used[0])) return [];
  return surfaces.filter(({ id }) => !existing.includes(id)).flatMap(({ id }) => {
    const example = cellId(used[0], id);
    const dir = path.resolve('examples', example);
    return fs.existsSync(dir) && fs.readdirSync(dir).some((file) => file.startsWith('show.')) ? [{ surface: id, example }] : [];
  });
}
export function expandFunctionGroups(source) {
  return source.replace(/<LanguageGroup\b[^>]*>([\s\S]*?)<\/LanguageGroup>/g, (group, body) => {
    const extra = missingFunctionVariants([...body.matchAll(/<Example\s+id="([^"]+)"/g)].map((m) => m[1]), [...body.matchAll(/<LanguageVariant\s+surface="([^"]+)"/g)].map((m) => m[1]));
    return group.replace('</LanguageGroup>', extra.map(({surface, example}) => `<LanguageVariant surface="${surface}">\n<Example id="${example}" wrap />\n</LanguageVariant>\n`).join('') + '</LanguageGroup>');
  });
}
