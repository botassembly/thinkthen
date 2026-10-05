// Bounded tree reader for generated site HTML; not a browser parser.
const VOID = new Set(['br', 'img', 'meta', 'link', 'input', 'hr', 'source', 'area', 'col']);

export function parseHtml(html) {
  html = html.replace(/<!--[\s\S]*?-->/g, '');
  const root = { tag: '#root', attrs: {}, kids: [] };
  const stack = [root];
  const tagRe = /<(\/?)([a-zA-Z][a-zA-Z0-9-]*)((?:"[^"]*"|'[^']*'|[^>"'])*)>/g;
  let at = 0;
  let m;
  while ((m = tagRe.exec(html))) {
    if (m.index > at) text(stack, html.slice(at, m.index));
    at = tagRe.lastIndex;
    const [, closing, tag, rawAttrs] = m;
    const name = tag.toLowerCase();
    if (closing) {
      for (let i = stack.length - 1; i > 0; i -= 1) {
        if (stack[i].tag === name) { stack.length = i; break; }
      }
      continue;
    }
    const node = { tag: name, attrs: attrsOf(rawAttrs), kids: [] };
    stack[stack.length - 1].kids.push(node);
    if (!VOID.has(name) && !/\/\s*$/.test(rawAttrs)) stack.push(node);
  }
  if (at < html.length) text(stack, html.slice(at));
  return root;
}

function text(stack, raw) {
  if (!raw) return;
  stack[stack.length - 1].kids.push({ tag: '#text', text: decode(raw) });
}

function attrsOf(raw) {
  const out = {};
  const re = /([a-zA-Z_:][-a-zA-Z0-9_:.]*)(?:\s*=\s*("([^"]*)"|'([^']*)'|([^\s"'>]+)))?/g;
  let m;
  while ((m = re.exec(raw))) out[m[1].toLowerCase()] = decode(m[3] ?? m[4] ?? m[5] ?? '');
  return out;
}

function decode(s) {
  return s
    .replace(/&lt;/g, '<').replace(/&gt;/g, '>').replace(/&quot;/g, '"')
    .replace(/&#x([0-9a-f]+);/gi, (_, h) => String.fromCodePoint(parseInt(h, 16)))
    .replace(/&#([0-9]+);/g, (_, d) => String.fromCodePoint(Number(d))).replace(/&nbsp;/g, ' ')
    .replace(/&apos;/g, "'").replace(/&middot;/g, '·').replace(/&amp;/g, '&');
}

export function findElement(node, test) {
  if (test(node)) return node;
  for (const kid of node.kids || []) {
    const hit = findElement(kid, test);
    if (hit) return hit;
  }
  return null;
}

export const hasClass = (node, name) => (node.attrs?.class || '').split(/\s+/).includes(name);

export function textContent(node) {
  if (node.tag === '#text') return node.text;
  if (['script', 'style', 'button', 'pre'].includes(node.tag)) return '';
  if (node.tag === 'br') return ' ';
  return (node.kids || []).map(textContent).join('');
}
