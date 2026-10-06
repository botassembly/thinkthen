      *> Unpublished 0430 counted native carriers, Linux x86_64.
      *> BASED groups borrow native storage or caller ALLOCATE storage.
      *> Overlay nested named fields with their matching typed BASED group.
       01 tt-observed-probabilities-v1 based.
          02 v-kind usage binary-long unsigned.
          02 v-padding-4 pic x(4).
          02 v-data pic x(16).
          02 v-yes redefines v-data
             usage float-long.
          02 v-named redefines v-data
             pic x(16).
       01 tt-observation-success-v1 based.
          02 v-answer-id pic x(16).
          02 v-observation-id pic x(16).
          02 v-value pic x(40).
          02 v-probabilities pic x(24).
          02 v-confidence pic x(16).
       01 tt-question-observation-v1 based.
          02 v-index usage binary-double unsigned.
          02 v-member pic x(24).
          02 v-stage pic x(8).
          02 v-position usage binary-double unsigned.
          02 v-question-sha256 pic x(16).
          02 v-model pic x(16).
          02 v-url pic x(16).
          02 v-requests pic x(16).
          02 v-requests-sent usage binary-double unsigned.
          02 v-cached usage binary-long signed.
          02 v-padding-124 pic x(4).
          02 v-failed-questions usage binary-double unsigned.
          02 v-usage pic x(24).
          02 v-question-sources pic x(16).
          02 v-state usage binary-long unsigned.
          02 v-padding-180 pic x(4).
          02 v-data pic x(112).
          02 v-success redefines v-data
             pic x(112).
          02 v-failure redefines v-data
             pic x(24).
       01 tt-row-observation-v1 based.
          02 v-index usage binary-double unsigned.
          02 v-function usage binary-long unsigned.
          02 v-padding-12 pic x(4).
          02 v-data pic x(1072).
          02 v-decide redefines v-data
             pic x(1016).
          02 v-choose redefines v-data
             pic x(1008).
          02 v-tag redefines v-data
             pic x(1000).
          02 v-score redefines v-data
             pic x(992).
          02 v-filter redefines v-data
             pic x(992).
          02 v-rank redefines v-data
             pic x(1024).
          02 v-find redefines v-data
             pic x(1032).
          02 v-annotate redefines v-data
             pic x(1000).
          02 v-recognize redefines v-data
             pic x(1072).
          02 v-relate redefines v-data
             pic x(1016).
       01 tt-observation-v1 based.
          02 v-kind usage binary-long unsigned.
          02 v-padding-4 pic x(4).
          02 v-data pic x(1088).
          02 v-question redefines v-data
             pic x(296).
          02 v-row redefines v-data
             pic x(1088).
       01 tt-summary-v1 based.
          02 v-state usage binary-long unsigned.
          02 v-padding-4 pic x(4).
          02 v-schema pic x(16).
          02 v-answer-id pic x(24).
          02 v-function pic x(8).
          02 v-count usage binary-double unsigned.
          02 v-observation-count usage binary-double unsigned.
          02 v-meta pic x(384).
          02 v-facts pic x(152).
          02 v-attempts pic x(24).
          02 v-error pic x(80).
