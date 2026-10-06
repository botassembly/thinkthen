      *> Unpublished 0430 counted native carriers, Linux x86_64.
      *> BASED groups borrow native storage or caller ALLOCATE storage.
      *> Overlay nested named fields with their matching typed BASED group.
       01 tt-decide-value-v1 based.
          02 v-kind usage binary-long unsigned.
          02 v-padding-4 pic x(4).
          02 v-data pic x(24).
          02 v-boolean redefines v-data
             usage binary-long signed.
          02 v-authored redefines v-data
             pic x(24).
       01 tt-probability-v1 based.
          02 v-name pic x(16).
          02 v-probability usage float-long.
       01 tt-probabilities-v1 based.
          02 v-data usage pointer.
          02 v-len usage binary-double unsigned.
       01 tt-optional-probabilities-v1 based.
          02 v-present usage binary-long signed.
          02 v-padding-4 pic x(4).
          02 v-value pic x(16).
       01 tt-named-answer-v1 based.
          02 v-pick pic x(16).
          02 v-probabilities pic x(16).
          02 v-confidence pic x(16).
       01 tt-score-answer-v1 based.
          02 v-level pic x(16).
          02 v-probabilities pic x(16).
          02 v-confidence pic x(16).
       01 tt-answer-v1 based.
          02 v-kind usage binary-long unsigned.
          02 v-padding-4 pic x(4).
          02 v-data pic x(48).
          02 v-probability redefines v-data
             usage float-long.
          02 v-choice redefines v-data
             pic x(48).
          02 v-tag redefines v-data
             pic x(16).
          02 v-score redefines v-data
             pic x(48).
          02 v-find redefines v-data
             pic x(48).
       01 tt-optional-answer-v1 based.
          02 v-present usage binary-long signed.
          02 v-padding-4 pic x(4).
          02 v-value pic x(56).
       01 tt-location-v1 based.
          02 v-file pic x(24).
          02 v-first-line pic x(16).
          02 v-last-line pic x(16).
       01 tt-optional-location-v1 based.
          02 v-present usage binary-long signed.
          02 v-padding-4 pic x(4).
          02 v-value pic x(56).
       01 tt-member-value-v1 based.
          02 v-kind usage binary-long unsigned.
          02 v-padding-4 pic x(4).
          02 v-data pic x(32).
          02 v-decide redefines v-data
             pic x(32).
          02 v-choose redefines v-data
             pic x(24).
          02 v-tag redefines v-data
             pic x(16).
          02 v-score redefines v-data
             usage float-long.
       01 tt-member-failure-v1 based.
          02 v-failure-id pic x(16).
          02 v-cause usage binary-long unsigned.
          02 v-padding-20 pic x(4).
       01 tt-member-success-v1 based.
          02 v-answer-id pic x(16).
          02 v-value pic x(40).
          02 v-answer pic x(56).
          02 v-threshold pic x(24).
       01 tt-member-v1 based.
          02 v-name pic x(16).
          02 v-request pic x(16).
          02 v-question pic x(344).
          02 v-state usage binary-long unsigned.
          02 v-padding-380 pic x(4).
          02 v-data pic x(136).
          02 v-success redefines v-data
             pic x(136).
          02 v-failure redefines v-data
             pic x(24).
       01 tt-members-v1 based.
          02 v-data usage pointer.
          02 v-len usage binary-double unsigned.
