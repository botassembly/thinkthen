// Parse the built command help once for generated reference and function tables.
export function sections(text) {
  const out = { about: '', usage: '', commands: [], args: [], options: [] };
  let current = 'about';
  for (const line of text.split('\n')) {
    if (/^Usage: /.test(line)) { out.usage = line.replace(/^Usage: /, ''); current = 'none'; continue; }
    if (line === 'Commands:') { current = 'commands'; continue; }
    if (line === 'Arguments:') { current = 'args'; continue; }
    if (line === 'Options:') { current = 'options'; continue; }
    if (current === 'about' && line.trim()) out.about += (out.about ? ' ' : '') + line.trim();
    if (current === 'commands') {
      const m = line.match(/^ {2}(\S+)\s*(.*)$/);
      if (m) out.commands.push({ name: m[1], summary: m[2].trim() });
    }
    if (current === 'args') {
      const m = line.match(/^ {2}(\S+)\s*(.*)$/);
      if (m) out.args.push({ name: m[1], ...brackets(m[2].trim()) });
    }
    if (current === 'options') {
      const m = line.match(/^\s+(?:(-\w), )?(--[\w-]+)((?:\[=<[^>]+>\])|(?:(?: <[^>]+>)+))?\s*(.*)$/);
      if (m) out.options.push({ short: m[1] ?? '', flag: m[2], value: (m[3] ?? '').trim(), ...brackets(m[4].trim()) });
    }
  }
  return out;
}

// "Backend name [THINKTHEN_BACKEND; default: systemone] [possible values: a, b]"
function brackets(text) {
  const out = { description: text, env: '', default: '', values: '' };
  const groups = [...text.matchAll(/\[([^\]]*)\]/g)];
  for (const [whole, inner] of groups) {
    out.description = out.description.replace(whole, '').trim();
    if (inner.startsWith('possible values: ')) { out.values = inner.slice('possible values: '.length); continue; }
    for (const part of inner.split('; ')) {
      if (part.startsWith('default: ')) out.default = part.slice('default: '.length);
      else out.env = out.env ? `${out.env}, ${part}` : part;
    }
  }
  return out;
}

