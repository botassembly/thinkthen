// A path or an address in a table may break after a slash and nowhere else
// inside a name. `pathHtml` escapes the text and puts a <wbr> after each
// slash, but not inside the "://" of an address. Use it inside <code class="path">. site.css keeps it whole while a table has
// columns, and lets it wrap on a phone or a tablet.
const escape = (s) => s.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;');

export function pathHtml(text) {
  return escape(text).replace(/(?<!:\/?)\/(?=.)/g, '/<wbr>');
}
