// Loopback-only transport for docs capture. Save bodies, never headers or keys.
import http from 'node:http';
import fs from 'node:fs';
import { createHash } from 'node:crypto';

class CaptureError extends Error {}

export async function captureBackend({ file, key, upstream = 'https://api.typesafe.ai/v1', maxCalls = 100, maxBytes = 300_000, replayOnly = false }) {
  if (typeof file !== 'string' || file.trim().length === 0) throw new CaptureError('capture requires an explicit cache file');
  for (const limit of [maxCalls, maxBytes]) if (!Number.isSafeInteger(limit) || limit <= 0) throw new CaptureError('capture limits must be positive integers');
  const canonical = (value) => Array.isArray(value) ? value.map(canonical) : value && typeof value === 'object' ? Object.fromEntries(Object.keys(value).sort().map((name) => [name, canonical(value[name])])) : value;
  const entries = fs.existsSync(file) ? JSON.parse(fs.readFileSync(file, 'utf8')) : {};
  let calls = 0;
  let bytes = 0;
  let served = 0;
  let pending = Promise.resolve();
  let failure = null;
  const server = http.createServer((request, response) => {
    const handle = async () => {
      try {
        if (failure) throw new CaptureError(failure);
        if (request.method !== 'POST') throw new CaptureError('capture accepts POST only');
        const chunks = [];
        let size = 0;
        for await (const chunk of request) {
          size += chunk.length;
          if (size > maxBytes) throw new CaptureError('capture request exceeds byte cap');
          chunks.push(chunk);
        }
        const body = JSON.parse(Buffer.concat(chunks).toString());
        const answers = {};
        for (const [name, question] of Object.entries(body.questions)) {
          const single = { state: body.state, model: body.model, questions: { q1: question } };
          const encoded = JSON.stringify(canonical(single));
          const digest = createHash('sha256').update(encoded).digest('hex');
          if (!entries[digest]) {
            if (replayOnly) {
              const miss = `${file}.miss.json`;
              const descriptor = fs.openSync(miss, 'w', 0o600);
              try {
                fs.fchmodSync(descriptor, 0o600);
                fs.writeFileSync(descriptor, JSON.stringify({ digest, request: JSON.parse(encoded) }, null, 2) + '\n');
              } finally { fs.closeSync(descriptor); }
              throw new CaptureError('saved response is missing');
            }
            const requestBytes = Buffer.byteLength(encoded);
            if (calls >= maxCalls || bytes + requestBytes >= maxBytes) throw new CaptureError('capture budget exhausted');
            calls++;
            bytes += requestBytes;
            const reply = await fetch(`${upstream.replace(/\/$/, '')}${request.url}`, { method: 'POST', headers: { 'content-type': 'application/json', authorization: `Bearer ${key}` }, body: encoded, signal: AbortSignal.timeout(60_000) });
            const parts = [];
            for await (const part of reply.body) {
              bytes += part.length;
              if (bytes > maxBytes) throw new CaptureError('capture byte cap exhausted');
              parts.push(part);
            }
            if (!reply.ok) throw new CaptureError(`capture upstream returned HTTP ${reply.status}`);
            const result = JSON.parse(Buffer.concat(parts).toString());
            if (!result.answers?.q1) throw new CaptureError('capture upstream returned no answer');
            entries[digest] = { request: single, response: result };
            fs.writeFileSync(file, JSON.stringify(entries, null, 2) + '\n', { mode: 0o600 });
          }
          answers[name] = entries[digest].response.answers.q1;
          served++;
        }
        response.writeHead(200, { 'content-type': 'application/json' });
        response.end(JSON.stringify({ answers }));
      } catch (error) {
        // JSON, fetch and filesystem errors may include body text or paths.
        failure ??= error instanceof CaptureError ? error.message : 'docs capture failed';
        response.writeHead(502, { 'content-type': 'application/json' });
        response.end(JSON.stringify({ error: 'docs capture failed' }));
        console.error(failure);
      }
    };
    pending = pending.then(handle, handle);
  });
  await new Promise((resolve, reject) => { server.once('error', reject); server.listen(0, '127.0.0.1', resolve); });
  return { url: `http://127.0.0.1:${server.address().port}`, close: () => new Promise((resolve) => server.close(resolve)), usage: () => ({ calls, bytes }), served: () => served, failure: () => failure };
}
