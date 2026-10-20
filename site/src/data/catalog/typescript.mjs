import { NATIVE_PLATFORMS } from './platforms.mjs';

export default {
    slug: 'typescript', name: 'TypeScript', deckHeading: 'TypeScript',
    versions: { runtime: 'Node.js 22 or later', platforms: NATIVE_PLATFORMS },
    backends: 'code',
    lang: 'ts', tab: 'TypeScript',
    blurb: "Await one of the ten `Client` methods. Read each answer from `call.value`.",
    unsureWord: '`null`',
    facts: "`call.facts` retains final native observations.",
    errors: "`ClientError` retains native kind, retryability, failure facts and completed results.",
    settings: "`new Client({...})` reads environment settings with explicit overrides. Use native snake_case names, including `base_url`.",
    install: [['npm install thinkthen', null], ['pnpm add thinkthen', null], ['bun add thinkthen', null]],
    particular: ["`client.start` returns an asynchronous packet iterator with cancellation and cleanup.", "Close the client in `finally`; generated results remain owned after cleanup."],
  };
