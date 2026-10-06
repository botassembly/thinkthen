# 0451: Add production Guides and a reusable lesson template

Guides reuse the existing Astro layout, Markdown and draft handling. The section links working Learn and Recipe pages. Published lessons require prerequisites, limits and actual video/article links; incomplete drafts stay out of public listings, search, sitemap and Markdown exports. No lesson or media link was invented for publication.

Fresh whole-change review accepted 855f13e7d. Rendered complete/draft fixtures, navigation, links, exports and 160 shell examples passed, along with keyboard, viewport and theme checks. The stale command-help fixture now lists runs. Image flags are documented with their three-function restriction; future proxy wording preserves the existing boundary. The final affected site build, full tests and lint run on the landing commit before main is pushed.

## What the build taught us

The section can be useful before marketing finishes its lesson catalog. Draft exclusion and required publication fields reuse the site's existing behavior instead of adding a publication workflow.
