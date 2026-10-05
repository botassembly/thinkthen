import { Engine } from 'thinkthen';
new Engine({backend: 'liquid'});
// @ts-expect-error a backend name is text
new Engine({backend: 1});
