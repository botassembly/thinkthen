# Small leftovers from the security ticket

Found by the independent review of ticket 0019 on 2026-09-19. None is a hole. `sdlc/records/0019-safe-before-public.md` holds the full reasoning. The release pass picks these up.

1. **A refused port is reported as a scheme problem.** `http://host:/v1` is rightly refused, and the message names the wrong rule. The fix is a variant of the address error with its own sentence.
2. **`--field` and `--options` echo a pointer as it was typed.** A label with a control character is refused since ticket 0013, and a pointer gets no such check. The value is the operator's own argument, so the risk is small. The fix is the check ticket 0013 already wrote.
3. **A transport failure that can never succeed is still retried.** A header the HTTP library refuses is attempted three times. It is cosmetic.

Three limits stand by decision, and Ian can overturn each: an empty port is refused although the URL standard allows it, the host keeps the case it was typed in so that no recording's digest changes, and a 16 MiB record can cost about 100 MB of memory while it is encoded.
