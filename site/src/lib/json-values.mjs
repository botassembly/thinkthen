// Every top-level JSON object or array in a text, in order: a reply printed whole, pretty
// or compact, one per line, or mixed with plain lines. Text that is not JSON is skipped.
export function jsonValues(text) {
  const out = [];
  let i = 0;
  while (i < text.length) {
    const c = text[i];
    if (c !== '{' && c !== '[') { i++; continue; }
    const end = balancedEnd(text, i);
    if (end < 0) { i++; continue; }
    try {
      out.push(JSON.parse(text.slice(i, end + 1)));
      i = end + 1;
    } catch {
      i = end + 1;
    }
  }
  return out;
}

// The index of the bracket that closes the one at `start`, or -1.
function balancedEnd(text, start) {
  let depth = 0;
  let inString = false;
  for (let i = start; i < text.length; i++) {
    const c = text[i];
    if (inString) {
      if (c === '\\') i++;
      else if (c === '"') inString = false;
    } else if (c === '"') inString = true;
    else if (c === '{' || c === '[') depth++;
    else if (c === '}' || c === ']') {
      depth--;
      if (depth === 0) return i;
    }
  }
  return -1;
}
