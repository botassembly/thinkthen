# The strict probability total refused two live replies in about thirty

Status: Resolved by ticket 0038

An authorized live evaluation on 2026-09-20 died twice on the same refusal. It used `annotate` with a question set of one `choose` question, 17 options, and a description on each. The first attempt was refused at request 19. The resume replayed those and was refused at request 30. Twenty-nine replies were accepted and two were refused.

The refusal, word for word:

    thinkthen: the reply was refused: the answer to question `q1` has
    probabilities whose total differs from one by more than member count ×
    f64::EPSILON

The check is `crates/thinkthen-core/src/answer.rs:37`. The prospective plan says to keep the strict rule and "collect rounding evidence during an authorized product probe". This is the first evidence.

## Further live evidence

Later authorized checks brought the request-level total to 58: 33 requests with 17-option answers and 25 requests with 5-option answers. Thirty of the 17-option requests answered and three were refused. Twenty-three of the 5-option requests answered and two were refused. The five refusals are about one in twelve requests.

The later three refusals ran through a temporary experiment binary that compared the binary distance directly with `0.01`. That did not prove a decimal miss greater than one hundredth: `abs(0.99 - 1.0)` is `0.010000000000000009` in binary floating point and fails that comparison, while `0.9900000000000001` passes. The temporary change did not enter this repository.

The largest recorded input usage for the same one-question, 17-option request shape is 1,330 tokens. A 50-attempt capture job therefore needs 66,500 input tokens before headroom; a reservation of 84,000 supplies slightly more than 25 percent. One such job has about a one-percent chance of seeing no refusal at the observed rate.

Ticket 0038 then ran that one job. It made 50 independent calls with retries disabled. Forty answered and ten were refused. Every refused distribution had 17 members and the same computed total, `0.9900000000000001`; the active tolerance was `3.774758283725532e-15`. The ledger moved from 18,440,118 to 18,524,118 charged tokens, exactly the 84,000-token reservation. No second job ran.

## What is known

- The refusal moves. Request 19 was refused once and accepted on the next try. The same request bytes got a reply that passed.
- The rule has been in the code since 2026-09-19. The accuracy round on 2026-09-20 sent 770 `choose` requests with 77 options and 1,500 yes-or-no requests, and its notes say "no job was refused, no request failed".
- The tolerance grows with the member count. At 17 members it is about 3.8e-15. At 77 members it is about 1.7e-14. A shorter list gets a tighter bound, and a sum of 17 floats can miss one by a few units in the last place depending on the order of addition.

## What is not known

The five earlier refused totals remain unknown because those replies were not recorded and their diagnostic omitted the total. The capture result and the floating-point edge explain those observations without evidence of a miss greater than one decimal hundredth.

## Why it matters

A user with five or seventeen options lost five of 58 live requests to exit code 4, and a stream stops at the first one. Every surface inherits it. The guard charges a job's whole cap when the job dies.

## Resolution

Ticket 0038 added the safe measurement, ran the one capped capture, and found the same computed total of `0.9900000000000001` in all ten refusals. System One now receives the evidence-backed tolerance `0.01 + member count × f64::EPSILON`. The generic rule remains strict. Exact boundary tests keep `0.99` and `1.01` while refusing `0.98`, `1.02`, `0.85`, `1.15`, zero, and three ones. The correction pass is complete; cache locking follows.

## What Ian can overturn

The System One exception and its place before cache locking. The generic strict rule remains the default.
