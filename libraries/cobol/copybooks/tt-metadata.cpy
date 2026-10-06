      *> Unpublished 0430 counted native carriers, Linux x86_64.
      *> BASED groups borrow native storage or caller ALLOCATE storage.
      *> Overlay nested named fields with their matching typed BASED group.
       01 tt-usage-v1 based.
          02 v-input-tokens usage binary-double unsigned.
          02 v-output-tokens usage binary-double unsigned.
       01 tt-optional-usage-v1 based.
          02 v-present usage binary-long signed.
          02 v-padding-4 pic x(4).
          02 v-value pic x(16).
       01 tt-question-source-v1 based.
          02 v-origin usage binary-long unsigned.
          02 v-padding-4 pic x(4).
          02 v-answered-by pic x(16).
       01 tt-question-sources-v1 based.
          02 v-data usage pointer.
          02 v-len usage binary-double unsigned.
       01 tt-observation-identity-v1 based.
          02 v-kind usage binary-long unsigned.
          02 v-padding-4 pic x(4).
          02 v-data pic x(16).
          02 v-observation-id redefines v-data
             pic x(16).
          02 v-failure-id redefines v-data
             pic x(16).
       01 tt-observation-identities-v1 based.
          02 v-data usage pointer.
          02 v-len usage binary-double unsigned.
       01 tt-profile-warning-v1 based.
          02 v-tuned-for pic x(16).
          02 v-running pic x(16).
       01 tt-optional-profile-warning-v1 based.
          02 v-present usage binary-long signed.
          02 v-padding-4 pic x(4).
          02 v-value pic x(32).
       01 tt-batch-v1 based.
          02 v-kind usage binary-long unsigned.
          02 v-padding-4 pic x(4).
          02 v-records usage binary-double unsigned.
       01 tt-optional-batch-v1 based.
          02 v-present usage binary-long signed.
          02 v-padding-4 pic x(4).
          02 v-value pic x(16).
       01 tt-batch-warning-v1 based.
          02 v-tuned-for pic x(16).
          02 v-running pic x(16).
       01 tt-optional-batch-warning-v1 based.
          02 v-present usage binary-long signed.
          02 v-padding-4 pic x(4).
          02 v-value pic x(32).
       01 tt-attempt-v1 based.
          02 v-ordinal usage binary-double unsigned.
          02 v-request-sha256 pic x(16).
          02 v-wall-ms usage binary-double unsigned.
          02 v-outcome usage binary-long unsigned.
          02 v-padding-36 pic x(4).
          02 v-sdk-request-id pic x(16).
          02 v-status pic x(8).
          02 v-server-ms pic x(16).
          02 v-request-id pic x(24).
       01 tt-attempts-v1 based.
          02 v-data usage pointer.
          02 v-len usage binary-double unsigned.
       01 tt-optional-attempts-v1 based.
          02 v-present usage binary-long signed.
          02 v-padding-4 pic x(4).
          02 v-value pic x(16).
       01 tt-meta-v1 based.
          02 v-tool pic x(16).
          02 v-question-sha256 pic x(24).
          02 v-questions-sha256 pic x(24).
          02 v-url pic x(16).
          02 v-model pic x(16).
          02 v-usage pic x(24).
          02 v-requests-sent usage binary-double unsigned.
          02 v-cached usage binary-long signed.
          02 v-padding-132 pic x(4).
          02 v-requests pic x(16).
          02 v-failed-questions usage binary-double unsigned.
          02 v-profile-warning pic x(40).
          02 v-batch-setting pic x(24).
          02 v-batch-warning pic x(40).
          02 v-context-sha256 pic x(24).
          02 v-attempts pic x(24).
          02 v-origin pic x(8).
          02 v-question-sources pic x(16).
          02 v-observations pic x(16).
          02 v-answered-by pic x(24).
       01 tt-optional-meta-v1 based.
          02 v-present usage binary-long signed.
          02 v-padding-4 pic x(4).
          02 v-value pic x(376).
       01 tt-facts-v1 based.
          02 v-call-id pic x(16).
          02 v-cache-answers usage binary-double unsigned.
          02 v-estimated-cost-usd pic x(24).
          02 v-input-tokens pic x(16).
          02 v-model pic x(24).
          02 v-output-tokens pic x(16).
          02 v-records usage binary-double unsigned.
          02 v-requests-sent usage binary-double unsigned.
          02 v-seconds usage float-long.
          02 v-command-ms pic x(16).
       01 tt-optional-facts-v1 based.
          02 v-present usage binary-long signed.
          02 v-padding-4 pic x(4).
          02 v-value pic x(144).
       01 tt-stopped-v1 based.
          02 v-at pic x(16).
          02 v-cause usage binary-long unsigned.
          02 v-status pic x(8).
          02 v-retryable usage binary-long signed.
       01 tt-optional-stopped-v1 based.
          02 v-present usage binary-long signed.
          02 v-padding-4 pic x(4).
          02 v-value pic x(32).
       01 tt-error-v1 based.
          02 v-code usage binary-long signed.
          02 v-padding-4 pic x(4).
          02 v-message pic x(16).
          02 v-retryable usage binary-long signed.
          02 v-padding-28 pic x(4).
          02 v-stopped pic x(40).
       01 tt-optional-error-v1 based.
          02 v-present usage binary-long signed.
          02 v-padding-4 pic x(4).
          02 v-value pic x(72).
