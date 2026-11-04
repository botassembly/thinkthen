// The docs pages, their routes and front matter, read from src/content/docs.
import fs from 'node:fs';
import path from 'node:path';
import YAML from 'yaml';
import site from '../src/data/site.json' with { type: 'json' };
import { exampleMarkdown, fileMarkdown } from '../src/lib/examples.mjs';
import productComponents from '../src/lib/markdown-components.mjs';
import { fact } from '../src/lib/facts.mjs';
import { expandFunctionGroups } from '../src/lib/function-variants.mjs';
import { markdownTwin } from '../src/lib/routes.mjs';

export const DOCS = path.resolve('src/content/docs');

export function pages({ include404 = false } = {}) {
  const out = [];
  const walk = (dir) => {
    for (const f of fs.readdirSync(dir, { withFileTypes: true })) {
      const p = path.join(dir, f.name);
      if (f.isDirectory()) walk(p);
      else if (/\.mdx?$/.test(f.name) && (include404 || f.name !== '404.mdx')) out.push(page(p));
    }
  };
  walk(DOCS);
  return out.sort((a, b) => a.route.localeCompare(b.route));
}

function page(file) {
  const raw = fs.readFileSync(file, 'utf8');
  const m = raw.match(/^---\n([\s\S]*?)\n---\n?/);
  const data = m ? YAML.parse(m[1]) : {};
  if (data.hero) data.hero = { ...data.hero, ...site.hero };
  const body = m ? raw.slice(m[0].length) : raw;
  let slug = data.slug ?? path.relative(DOCS, file).replace(/\.mdx?$/, '').toLowerCase();
  slug = slug === 'index' ? '' : slug.replace(/\/index$/, '');
  return { file, slug, route: slug ? `/${slug}/` : '/', twin: markdownTwin(slug ? `/${slug}/` : '/'), data, body, frontmatter: data.hero ? YAML.stringify(data).trimEnd() : (m ? m[1] : '') };
}

// The page body as plain Markdown: each component becomes its text. Code blocks and inline code
// pass through unchanged. A component with no handler fails, so the Markdown twins and
// llms-full.txt never carry a raw tag; give a new component a handler in
// src/lib/markdown-components.mjs (product components) or here (shared components).
const wrapper = (title) => (a) => (a[title] ? `**${a[title]}**\n\n` : '');
const SHARED = {
  Example: (a) => exampleMarkdown(a.id, a.lines ? Number(a.lines) : undefined),
  ExampleFile: (a) => fileMarkdown(a.id, a.file, a.lang),
  Fact: (a) => fact(a.k),
  // Starlight's layout components keep their titles and drop their frames.
  Aside: wrapper('title'), Tabs: wrapper(), TabItem: wrapper('label'), Steps: wrapper(), CardGrid: wrapper(), Card: wrapper('title'), Hero: wrapper('title'),
  LinkCard: (a) => `[${a.title}](${a.href})${a.description ? `: ${a.description}` : ''}\n\n`,
  Badge: (a) => a.text ?? '',
  NotFoundSearch: () => 'Use the site search to find a page, or [start at the home page](/).\n\n',
};
const HANDLERS = { ...SHARED, ...productComponents };

// The attributes of one tag: name="text", name='text', name={expression} or a bare name.
function attributes(text) {
  const out = {};
  for (const m of text.matchAll(/([A-Za-z_][\w-]*)(?:=(?:"([^"]*)"|'([^']*)'|\{([^}]*)\}))?/g)) {
    let v = m[2] ?? m[3] ?? m[4] ?? true;
    if (m[4] !== undefined) v = v.trim().replace(/^(['"`])([\s\S]*)\1$/, '$2');
    out[m[1]] = v;
  }
  return out;
}

const TAG = /<(\/?)([A-Z][A-Za-z0-9]*)((?:\s+[A-Za-z_][\w-]*(?:=(?:"[^"]*"|'[^']*'|\{[^}]*\}))?)*)\s*(\/?)>/g;

function renderComponents(text) {
  return text.replace(TAG, (tag, close, name, attrs, _selfClose, offset) => {
    const h = HANDLERS[name];
    if (!h) throw new Error(`no Markdown form for <${name}>: add a handler in src/lib/markdown-components.mjs`);
    if (close) return '';
    // A tag indented inside a list item keeps its rendered lines inside that item.
    const indent = text.slice(text.lastIndexOf('\n', offset - 1) + 1, offset);
    const out = h(attributes(attrs));
    return /^[ \t]+$/.test(indent) ? out.replace(/\n(?=.)/g, `\n${indent}`) : out;
  });
}

// Splits Markdown into code (fenced blocks and inline code) and the rest.
export function splitCode(text) {
  return text.split(/(^[ \t]*(?:```|~~~)[^\n]*\n[\s\S]*?^[ \t]*(?:```|~~~)[ \t]*$|`[^`\n]+`)/m);
}

export function bodyMarkdown(body) {
  const parts = splitCode(expandFunctionGroups(body));
  return parts
    .map((part, i) => (i % 2 ? part : renderComponents(part.replace(/^import .*$\n?/gm, '').replace(/\{\/\*[\s\S]*?\*\/\}/g, ''))))
    .join('')
    .trim();
}

// Render the same hero words in Markdown and HTML from the shared site facts.
export function pageMarkdown(page) {
  const hero = page.data.hero;
  return `# ${hero?.title ?? page.data.title}\n\n${hero?.tagline ? `${hero.tagline}\n\n` : ''}${bodyMarkdown(page.body)}`;
}
