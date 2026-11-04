Fix the rounding of invoice totals.

Totals now round half up to two decimals, as the tax rules require.

Tested: I added unit tests for totals that end in 5, and ran
the full test suite, which passes.
