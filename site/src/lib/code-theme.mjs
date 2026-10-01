// The site's one code theme. Every colour is a CSS variable, and
// src/styles/site.css sets each variable for the dark and the light page.
// So one highlight serves both themes, and the page ships no colour script.
//
// The palette follows the deck's code panes: keys, keywords and shell
// command names green, quoted strings amber, numbers pink, true, false and
// null blue, comments muted. Everything else stays in ink. Code panes are
// the one exception to the rule that values stay in ink (Ian, 2026-09-30).
// Output panes never pass through this theme.
//
// The scopes are narrow on purpose. Bash scopes a bare argument such as
// --band as an unquoted string, so only quoted strings take amber.
// Operators such as =, << and <- stay in ink.

const v = (name) => `var(--code-${name})`;

export const CODE_THEME = {
  name: 'thinkthen',
  type: 'dark',
  colors: { 'editor.foreground': v('ink'), 'editor.background': v('card') },
  tokenColors: [
    { settings: { foreground: v('ink') } },
    { scope: ['comment', 'punctuation.definition.comment'], settings: { foreground: v('comment'), fontStyle: 'italic' } },
    { scope: ['string.quoted', 'punctuation.definition.string'], settings: { foreground: v('string') } },
    { scope: ['constant.numeric'], settings: { foreground: v('number') } },
    { scope: ['constant.language'], settings: { foreground: v('literal') } },
    {
      scope: ['keyword', 'storage', 'support.type.property-name', 'meta.object-literal.key', 'variable.other.jq.key'],
      settings: { foreground: v('key') },
    },
    { scope: ['keyword.operator', 'punctuation.separator', 'keyword.operator.assignment'], settings: { foreground: v('ink') } },
    // A shell command name takes the key colour, as the deck's thinkthen
    // command and its cat, echo and printf do.
    { scope: ['entity.name.command'], settings: { foreground: v('key') } },
    // A diff: headers and ranges muted. Added and removed lines stay in
    // ink, as heredoc input does, because green and red mean yes and no.
    { scope: ['meta.diff.header', 'meta.diff.range'], settings: { foreground: v('comment') } },
  ],
};

// Shiki carries no jq grammar. This small one covers what the site's jq
// file uses: comments, strings, numbers, true, false and null, keywords,
// variables, and object keys. It is written for this site under the
// repository's MIT licence.
export const JQ_GRAMMAR = {
  name: 'jq',
  scopeName: 'source.jq',
  patterns: [
    { match: '#.*$', name: 'comment.line.number-sign.jq' },
    {
      begin: '"', end: '"', name: 'string.quoted.double.jq',
      patterns: [{ match: '\\\\.', name: 'constant.character.escape.jq' }],
    },
    { match: '\\b-?\\d+(?:\\.\\d+)?\\b', name: 'constant.numeric.jq' },
    { match: '\\b(?:true|false|null)\\b', name: 'constant.language.jq' },
    {
      match: '\\b(?:reduce|foreach|as|if|then|elif|else|end|and|or|not|def|try|catch|label|import|include)\\b',
      name: 'keyword.control.jq',
    },
    { match: '\\$\\w+', name: 'variable.other.jq' },
    { match: '\\b[A-Za-z_]\\w*(?=\\s*:)', name: 'variable.other.jq.key' },
    { match: '\\|=|\\+=|-=|//|==|!=|<=|>=|[|+\\-*/<>=]', name: 'keyword.operator.jq' },
  ],
};
