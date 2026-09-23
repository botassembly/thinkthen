# A delivered request is sent again after a transport failure

Status: Open

`specification/backends.md:47` says a retry happens after a transport failure. A transport failure after the request's bytes left can mean the backend received it, and the vendor bills each delivery. The fifth review measured this on the stand-in, which follows the page: a server that read the whole request and then reset the connection received three full POSTs from one call, and a server that read it and never answered received three as well, one per timeout (`--max-retries 2`, the default).

The stand-in stopped doing this on 2026-09-23 (`standin/tests/no_resend.rs`). It sends no transport failure again. A refusal, a failed connect, and a name that did not resolve already failed at once, and every other failure may have delivered the request. A 429 or 5xx reply keeps its retries, because the backend answered and said it did not take the request. The error still says whether the caller's own second try could help.

The command (`crates/thinkthen`) and the page still retry a transport failure. What would close this: amend the backends page to the stand-in's rule and change the command's retry loop to match, or rule that the command keeps the page's rule and record why the double bill is acceptable there. The stand-in is the engine the surfaces bind until the merge, so the surfaces and the command now differ on this point until one of those lands.
