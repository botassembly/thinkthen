# A reply over 1 MiB is thrown away and called unreachable

Status: Open

Filed on 2026-09-25 by experiment 218, wave 2, rows C2 and B3. Checked at main `20e9b8d4` against the repository's loopback backend with a fake key. No request left the machine. Seat: a person typing, and a script in a loop.

## What happens

The engine reads at most 1 MiB of a reply (`MAX_RESPONSE_BYTES` in `crates/thinkthen/src/engine/http.rs:45`). A longer reply fails the read, and `http.rs:269` turns that into a transport failure. The command then prints the fixed guidance for an unknown transport failure. The request was already sent and answered, so the backend billed it. The tool drops the answer and tells the user the backend could not be reached.

`relate` and `recognize` put many questions in one request, so their replies pass 1 MiB at sizes a user meets:

- `relate` over 150 line names answers. Over 180 names it fails. The loopback backend counts one request each time.
- `recognize` over 32,000 bytes of text answers. Over 64,000 bytes it fails. `recognize --dry-run` at the built-in address plans one request for that text, so the hosted path takes the same route.

An earlier pass, `closed/2026-09-19-hands-on-test-pass-one.md`, saw this case name the 1,048,576 limit. `closed/2026-09-21-transport-failure-messages-paste-the-http-clients-own-words.md` then replaced client text with fixed guidance, and the size cause went with it.

## Smallest reproduction

```sh
cargo build --release -p thinkthen -p conformance-backend
target/release/conformance-backend        # prints PORT, keep it open
python3 -c "print('\n'.join('Name%d' % i for i in range(180)))" > names.txt
THINKTHEN_API_KEY=fake target/release/thinkthen relate r --lines \
  --url http://127.0.0.1:PORT/generic/v1 --no-cache < names.txt
echo $?
```

Expected: 32,220 edges on standard output and exit 0, or a refusal before the send that names the reply size.

Got: exit 4 and `thinkthen: the backend could not be reached; check --url and the network`. Type `count` into the backend: it read 1 request. 150 names give 22,350 edges and exit 0.

The same text through `recognize`:

```sh
python3 -c "print(('word ' * 12800)[:63999] + '.', end='')" > long.txt
THINKTHEN_API_KEY=fake target/release/thinkthen recognize --kind 'P=a' --kind 'O=b' \
  --url http://127.0.0.1:PORT/generic/v1 --no-cache < long.txt
```

Got: the same sentence at exit 4, after one request.

## Harm to a user

Money first. The user pays for a request whose answer the tool discards. The message sends the user to check the address and the network, and both are fine. A retry by hand pays again and fails again. A script that retries on exit 4 pays in a loop.

## What would fix it

Pick one, or both:

1. Raise the reply limit to fit the largest request the planner builds, or derive it from the plan. The planner knows the question count before it sends, so it can bound the reply size.
2. Keep a limit, and plan requests so each reply fits under it. Name the limit in the refusal when a reply still passes it: the backend answered with more than N bytes, and the answer was not kept.

Either way, a reply that passed the limit is its own failure with its own sentence. The generic transport sentence does not fit, because the connection worked.

## Tests to add when fixed

- An outside-in command test against the loopback backend: `relate` at 180 line names answers, or refuses before any send with the size sentence.
- The same for `recognize` at 64,000 bytes of text.
