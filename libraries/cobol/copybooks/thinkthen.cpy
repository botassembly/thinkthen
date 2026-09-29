      *> Public data ABI for TT-DECIDE, TT-VALIDATE and TT-PARSE.
      *> Include in WORKING-STORAGE; do not change the native group's padding.
       01 tt-answer.
          02 tt-outcome usage binary-long signed.
             88 outcome-no value 0.
             88 outcome-yes value 1.
             88 outcome-not-sure value 2.
          02 tt-alignment-pad usage binary-long unsigned.
          02 tt-probability usage float-long.
       01 tt-failure.
          02 tt-failure-code usage binary-long signed.
             88 failure-usage value 1.
             88 failure-backend value 2.
             88 failure-deadline value 3.
             88 failure-local value 4.
             88 failure-cancelled value 5.
             88 failure-defect value 6.
          02 tt-failure-retryable usage binary-long signed.
          02 tt-failure-message pic x(512).
          02 tt-failure-facts-length usage binary-double unsigned.
          02 tt-failure-facts-json pic x(8192).
       01 tt-facts.
          02 tt-facts-length usage binary-double unsigned.
          02 tt-facts-json pic x(8192).
       01 tt-field-kind usage binary-long signed.
          88 field-not-sure value 1.
          88 field-failed value 2.
          88 field-resolved value 3.
