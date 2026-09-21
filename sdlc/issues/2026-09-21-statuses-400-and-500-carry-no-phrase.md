# Statuses 400 and 500 carry no phrase

Status: Open

`specification/backends.md` fixes a sentence for six statuses: 401 "the key was refused", 402 "the account has no credit", 403 "the key may not use this model or address", 404 "nothing answers at this address", 422 "the backend refused the request as malformed or too large", and 429 "the backend's rate limit was reached". Every other status prints the bare code. The status a real vendor sends for "the model does not exist" or "context length exceeded" is 400, and the user sees a number with no sentence.

## Reproduction

    $ echo '{"mode":"status","status":400,"body":"the model jev-latest does not exist; check your spelling"}' > ctl/mode.json
    $ thinkthen decide 'Is it?' --max-retries 0 --url http://127.0.0.1:8903 < ev.txt
    thinkthen: the backend answered with status 400
    (exit 4)

A 500 prints the same bare shape once its retries are spent.

## Expected

One phrase for 400 in the same table, so the statuses a vendor commonly sends each carry a sentence. 500 already belongs to the retried family (`backends.md:47` lists it), so the wait-and-retry story frames it before its code prints. The no-body rule stands: `backends.md` forbids printing the response body, because a backend can quote the evidence back in an error. This page asks for a fixed phrase, never for the body.

## How bad it is for a user

Minor. The code and the exit code are honest, and a user with a misspelled model name cannot tell why the request failed.

Found by experiment 218, wave 1.5, the blind seat.
