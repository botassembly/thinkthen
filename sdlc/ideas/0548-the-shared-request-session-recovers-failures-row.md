# the shared request session recovers failures row by row: a failed row returns a failure value beside answered rows, so DuckDB packed try-details can leave its direct engine call

Status: open.

Kind: idea
When: after 0.2.0 ships

## Problem

the shared request session recovers failures row by row: a failed row returns a failure value beside answered rows, so DuckDB packed try-details can leave its direct engine call. one execution path for every DuckDB route; see the 0538 amendment.
