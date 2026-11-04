// Reads the cell values of an .xlsx workbook, so a page can show a spreadsheet input as tables.
// It reads strings and numbers as stored; it does not evaluate formulas.
import fs from 'node:fs';
import zlib from 'node:zlib';

// The files inside a zip archive, read from its central directory.
function unzip(buf) {
  let eocd = buf.length - 22;
  while (eocd >= 0 && buf.readUInt32LE(eocd) !== 0x06054b50) eocd--;
  if (eocd < 0) throw new Error('not a zip archive');
  const count = buf.readUInt16LE(eocd + 10);
  let p = buf.readUInt32LE(eocd + 16);
  const files = new Map();
  for (let i = 0; i < count; i++) {
    const method = buf.readUInt16LE(p + 10);
    const size = buf.readUInt32LE(p + 20);
    const nameLen = buf.readUInt16LE(p + 28);
    const extraLen = buf.readUInt16LE(p + 30);
    const commentLen = buf.readUInt16LE(p + 32);
    const local = buf.readUInt32LE(p + 42);
    const name = buf.toString('utf8', p + 46, p + 46 + nameLen);
    const start = local + 30 + buf.readUInt16LE(local + 26) + buf.readUInt16LE(local + 28);
    const data = buf.subarray(start, start + size);
    files.set(name, method === 8 ? zlib.inflateRawSync(data) : data);
    p += 46 + nameLen + extraLen + commentLen;
  }
  return files;
}

const unescape = (s) => s.replace(/&lt;/g, '<').replace(/&gt;/g, '>').replace(/&quot;/g, '"').replace(/&apos;/g, "'").replace(/&amp;/g, '&');
const text = (xml) => unescape([...xml.matchAll(/<t(?:\s[^>]*)?>([\s\S]*?)<\/t>/g)].map((m) => m[1]).join(''));
const column = (ref) => [...ref.replace(/\d+$/, '')].reduce((n, c) => n * 26 + c.charCodeAt(0) - 64, 0) - 1;

// [{ name, rows: [[cell, ...], ...] }, ...] in workbook order.
export function readXlsx(file) {
  const files = unzip(fs.readFileSync(file));
  const read = (n) => files.get(n)?.toString('utf8') ?? '';
  const shared = [...read('xl/sharedStrings.xml').matchAll(/<si>([\s\S]*?)<\/si>/g)].map((m) => text(m[1]));
  const rels = new Map([...read('xl/_rels/workbook.xml.rels').matchAll(/<Relationship\s[^>]*>/g)].map((m) => [m[0].match(/Id="([^"]+)"/)[1], m[0].match(/Target="([^"]+)"/)[1]]));
  return [...read('xl/workbook.xml').matchAll(/<sheet\s[^>]*>/g)].map((m) => {
    const name = unescape(m[0].match(/name="([^"]*)"/)[1]);
    const target = rels.get(m[0].match(/r:id="([^"]+)"/)[1]).replace(/^\/?(xl\/)?/, 'xl/');
    const rows = [...read(target).matchAll(/<row\b[^>]*>([\s\S]*?)<\/row>/g)].map((r) => {
      const cells = [];
      for (const c of r[1].matchAll(/<c\b([^>]*?)(?:\/>|>([\s\S]*?)<\/c>)/g)) {
        const ref = c[1].match(/r="([A-Z]+\d+)"/)?.[1];
        const type = c[1].match(/t="([^"]+)"/)?.[1];
        const body = c[2] ?? '';
        const v = body.match(/<v>([\s\S]*?)<\/v>/)?.[1];
        const value = type === 'inlineStr' ? text(body) : type === 's' ? shared[Number(v)] : v === undefined ? '' : unescape(v);
        cells[ref ? column(ref) : cells.length] = value;
      }
      return Array.from(cells, (c) => c ?? '');
    });
    return { name, rows };
  });
}

// The workbook as Markdown: one heading and one table per worksheet.
export function xlsxMarkdown(file, title) {
  const cell = (s) => String(s).replace(/\|/g, '\\|');
  return readXlsx(file)
    .map(({ name, rows }) => {
      const width = Math.max(1, ...rows.map((r) => r.length));
      const pad = (r) => Array.from({ length: width }, (_, i) => cell(r[i] ?? ''));
      const [head = [], ...body] = rows;
      const table = [`| ${pad(head).join(' | ')} |`, `| ${pad([]).map(() => '---').join(' | ')} |`, ...body.map((r) => `| ${pad(r).join(' | ')} |`)].join('\n');
      return `${title}, worksheet ${name}:\n\n${table}`;
    })
    .join('\n\n');
}
