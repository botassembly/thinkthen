// Who makes ThinkThen, and the public links for the author and his company.
// The footer, the About page and the structured data read them from here.
// Each link returned 200 when ticket 0054 was written, on 2026-10-01.

import { REPO } from './repo.mjs';

export const SITE_ORIGIN = 'https://thinkthen.dev';

export const AUTHOR = {
  name: 'Ian Maurer',
  role: 'CTO of GenomOncology',
  blog: 'https://www.imaurer.com',
  links: [
    { label: 'Blog', href: 'https://www.imaurer.com', icon: 'blog' },
    { label: 'LinkedIn', href: 'https://www.linkedin.com/in/ianmaurer/', icon: 'linkedin' },
    { label: 'X', href: 'https://x.com/imaurer', icon: 'x' },
    { label: 'GitHub', href: 'https://github.com/imaurer', icon: 'github' },
  ],
};

export const COMPANY = {
  name: 'GenomOncology',
  url: 'https://www.genomoncology.com/',
  logoLight: '/brand/genomoncology-logo-light.png',
  logoDark: '/brand/genomoncology-logo-dark.png',
  logoWidth: 262,
  logoHeight: 48,
};

export const LICENSE = { name: 'MIT License', href: `${REPO}/blob/main/LICENSE` };
export const ISSUES = `${REPO}/issues`;
export const SECURITY = `${REPO}/security`;

// Schema.org nodes. The home page and the About page share them by @id.
export const ORGANIZATION_LD = {
  '@type': 'Organization',
  '@id': `${COMPANY.url}#organization`,
  name: COMPANY.name,
  url: COMPANY.url,
  logo: `${SITE_ORIGIN}${COMPANY.logoLight}`,
};

export const PERSON_LD = {
  '@type': 'Person',
  '@id': `${SITE_ORIGIN}/about/#ian-maurer`,
  name: AUTHOR.name,
  jobTitle: 'CTO',
  url: AUTHOR.blog,
  worksFor: { '@id': ORGANIZATION_LD['@id'] },
  sameAs: AUTHOR.links.filter((l) => l.href !== AUTHOR.blog).map((l) => l.href),
};
