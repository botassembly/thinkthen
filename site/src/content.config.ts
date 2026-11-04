import { defineCollection } from 'astro:content';
import { z } from 'astro/zod';
import { docsLoader, i18nLoader } from '@astrojs/starlight/loaders';
import site from './data/site.json';
import { docsSchema, i18nSchema } from '@astrojs/starlight/schema';

// Every page carries its brief. scripts/check.mjs and the build both rely on these fields.
export const collections = {
  i18n: defineCollection({ loader: i18nLoader(), schema: i18nSchema() }),
  docs: defineCollection({
    loader: docsLoader(),
    schema: (context) => docsSchema({
      extend: z.object({
        description: z.string().min(20),
        goal: z.string().min(10),
        reader: z.string().min(5),
        kind: z.enum(['tutorial', 'how-to', 'reference', 'explanation', 'answer', 'blog', 'article', 'index']),
        verified: z.coerce.date(),
        date: z.coerce.date().optional(),
        author: z.string().optional(),
      }),
    })(context).transform((data) => data.hero ? { ...data, hero: { ...data.hero, ...site.hero } } : data),
  }),
};
