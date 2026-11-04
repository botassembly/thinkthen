// Every documentation route has a Markdown twin at the same address without its final slash.
export const markdownTwin = (route) => route === '/' ? '/index.md' : `${route.replace(/\/$/, '')}.md`;
