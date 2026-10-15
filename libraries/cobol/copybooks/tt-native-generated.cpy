      *> Generated from measured canonical C declarations; do not edit.
01 tt-n-complete-usage-persistence-v1 based.
 02 v-kind usage binary-long unsigned.
01 tt-n-complete-utf8-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-json-array-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-json-entry-v1 based.
 02 v-name.
 03 v-name-data usage pointer.
 03 v-name-len usage binary-double unsigned.
 02 v-value usage pointer.
01 tt-n-complete-json-object-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-json-v1 based.
 02 v-kind usage binary-long unsigned.
 02 filler pic x(4).
 02 v-data.
 03 v-data-union-storage pic x(16).
 03 v-data-boolean redefines v-data-union-storage usage binary-long unsigned.
 03 v-data-number redefines v-data-union-storage.
 04 v-data-number-data usage pointer.
 04 v-data-number-len usage binary-double unsigned.
 03 v-data-string redefines v-data-union-storage.
 04 v-data-string-data usage pointer.
 04 v-data-string-len usage binary-double unsigned.
 03 v-data-array redefines v-data-union-storage.
 04 v-data-array-data usage pointer.
 04 v-data-array-len usage binary-double unsigned.
 03 v-data-object redefines v-data-union-storage.
 04 v-data-object-data usage pointer.
 04 v-data-object-len usage binary-double unsigned.
01 tt-n-complete-attempt-outcome-v1 based.
 02 v-kind usage binary-long unsigned.
01 tt-n-complete-utf8-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-value-data usage pointer.
 03 v-value-len usage binary-double unsigned.
01 tt-n-complete-sdk-request-id-v1 based.
 02 v-value.
 03 v-value-data usage pointer.
 03 v-value-len usage binary-double unsigned.
01 tt-n-complete-u64-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage binary-double unsigned.
01 tt-n-complete-extension-v1 based.
 02 v-name.
 03 v-name-data usage pointer.
 03 v-name-len usage binary-double unsigned.
 02 v-json.
 03 v-json-data usage pointer.
 03 v-json-len usage binary-double unsigned.
 02 v-value usage pointer.
01 tt-n-complete-extensions-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-attempt-v1 based.
 02 v-ordinal usage binary-double unsigned.
 02 v-outcome usage pointer.
 02 v-request-id.
 03 v-request-id-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-request-id-value.
 04 v-request-id-value-data usage pointer.
 04 v-request-id-value-len usage binary-double unsigned.
 02 v-request-sha256.
 03 v-request-sha256-data usage pointer.
 03 v-request-sha256-len usage binary-double unsigned.
 02 v-sdk-request-id usage pointer.
 02 v-server-ms.
 03 v-server-ms-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-server-ms-value usage binary-double unsigned.
 02 v-status.
 03 v-status-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-status-value usage binary-double unsigned.
 02 v-wall-ms usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-attempt-list-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-attempt-list-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-value-data usage pointer.
 03 v-value-len usage binary-double unsigned.
01 tt-n-complete-call-id-v1 based.
 02 v-value.
 03 v-value-data usage pointer.
 03 v-value-len usage binary-double unsigned.
01 tt-n-complete-call-id-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-u32-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 v-value usage binary-long unsigned.
01 tt-n-complete-persistence-observation-v1 based.
 02 v-advice.
 03 v-advice-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-advice-value.
 04 v-advice-value-data usage pointer.
 04 v-advice-value-len usage binary-double unsigned.
 02 v-observed-at.
 03 v-observed-at-data usage pointer.
 03 v-observed-at-len usage binary-double unsigned.
 02 v-state usage pointer.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-persistence-observation-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-immediate-facts-v1 based.
 02 v-attempts.
 03 v-attempts-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-attempts-value.
 04 v-attempts-value-data usage pointer.
 04 v-attempts-value-len usage binary-double unsigned.
 02 v-cache-answers usage binary-double unsigned.
 02 v-call-id.
 03 v-call-id-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-call-id-value usage pointer.
 02 v-estimated-cost-usd.
 03 v-estimated-cost-usd-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-estimated-cost-usd-value.
 04 v-estimated-cost-usd-value-data usage pointer.
 04 v-estimated-cost-usd-value-len usage binary-double unsigned.
 02 v-held-model-mismatch.
 03 v-held-model-mismatch-presence usage binary-long unsigned.
 03 v-held-model-mismatch-value usage binary-long unsigned.
 02 v-input-tokens.
 03 v-input-tokens-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-input-tokens-value usage binary-double unsigned.
 02 v-largest-request-bytes usage binary-double unsigned.
 02 v-largest-request-estimated-input-tokens.
 03 v-largest-request-estimated-input-tokens-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-largest-request-estimated-input-tokens-value usage binary-double unsigned.
 02 v-model.
 03 v-model-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-model-value.
 04 v-model-value-data usage pointer.
 04 v-model-value-len usage binary-double unsigned.
 02 v-output-tokens.
 03 v-output-tokens-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-output-tokens-value usage binary-double unsigned.
 02 v-records usage binary-double unsigned.
 02 v-requests-sent usage binary-double unsigned.
 02 v-seconds usage float-long.
 02 v-token-estimate-method.
 03 v-token-estimate-method-data usage pointer.
 03 v-token-estimate-method-len usage binary-double unsigned.
 02 v-usage-persistence.
 03 v-usage-persistence-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-usage-persistence-value usage pointer.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-answer-yes-no-v1 based.
 02 v-kind.
 03 v-kind-data usage pointer.
 03 v-kind-len usage binary-double unsigned.
 02 v-probability usage float-long.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-f64-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage float-long.
01 tt-n-complete-number-map-entry-v1 based.
 02 v-name.
 03 v-name-data usage pointer.
 03 v-name-len usage binary-double unsigned.
 02 v-value usage float-long.
01 tt-n-complete-number-map-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-answer-choice-v1 based.
 02 v-confidence.
 03 v-confidence-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-confidence-value usage float-long.
 02 v-kind.
 03 v-kind-data usage pointer.
 03 v-kind-len usage binary-double unsigned.
 02 v-pick.
 03 v-pick-data usage pointer.
 03 v-pick-len usage binary-double unsigned.
 02 v-probabilities.
 03 v-probabilities-data usage pointer.
 03 v-probabilities-len usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-answer-tag-v1 based.
 02 v-kind.
 03 v-kind-data usage pointer.
 03 v-kind-len usage binary-double unsigned.
 02 v-probabilities.
 03 v-probabilities-data usage pointer.
 03 v-probabilities-len usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-answer-score-v1 based.
 02 v-confidence.
 03 v-confidence-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-confidence-value usage float-long.
 02 v-kind.
 03 v-kind-data usage pointer.
 03 v-kind-len usage binary-double unsigned.
 02 v-level.
 03 v-level-data usage pointer.
 03 v-level-len usage binary-double unsigned.
 02 v-probabilities.
 03 v-probabilities-data usage pointer.
 03 v-probabilities-len usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-answer-v1 based.
 02 v-kind usage binary-long unsigned.
 02 filler pic x(4).
 02 v-data.
 03 v-data-union-storage pic x(8).
 03 v-data-yes-no redefines v-data-union-storage usage pointer.
 03 v-data-choice redefines v-data-union-storage usage pointer.
 03 v-data-tag redefines v-data-union-storage usage pointer.
 03 v-data-score redefines v-data-union-storage usage pointer.
01 tt-n-complete-answer-id-v1 based.
 02 v-value.
 03 v-value-data usage pointer.
 03 v-value-len usage binary-double unsigned.
01 tt-n-complete-image-media-v1 based.
 02 v-kind usage binary-long unsigned.
01 tt-n-complete-image-v1 based.
 02 v-base64.
 03 v-base64-data usage pointer.
 03 v-base64-len usage binary-double unsigned.
 02 v-height usage binary-double unsigned.
 02 v-media usage pointer.
 02 v-width usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-image-list-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-image-list-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-value-data usage pointer.
 03 v-value-len usage binary-double unsigned.
01 tt-n-complete-json-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-batch-setting-v1 based.
 02 v-kind usage binary-long unsigned.
 02 filler pic x(4).
 02 v-data.
 03 v-data-union-storage pic x(16).
 03 v-data-integer redefines v-data-union-storage usage binary-double unsigned.
 03 v-data-string redefines v-data-union-storage.
 04 v-data-string-data usage pointer.
 04 v-data-string-len usage binary-double unsigned.
01 tt-n-complete-batch-setting-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-batch-warning-v1 based.
 02 v-running usage pointer.
 02 v-tuned-for usage pointer.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-batch-warning-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-observation-id-v1 based.
 02 v-value.
 03 v-value-data usage pointer.
 03 v-value-len usage binary-double unsigned.
01 tt-n-complete-observation-observation-id-v1 based.
 02 v-observation-id usage pointer.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-failure-id-v1 based.
 02 v-value.
 03 v-value-data usage pointer.
 03 v-value-len usage binary-double unsigned.
01 tt-n-complete-observation-failure-id-v1 based.
 02 v-failure-id usage pointer.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-observation-v1 based.
 02 v-kind usage binary-long unsigned.
 02 filler pic x(4).
 02 v-data.
 03 v-data-union-storage pic x(8).
 03 v-data-observation-id redefines v-data-union-storage usage pointer.
 03 v-data-failure-id redefines v-data-union-storage usage pointer.
01 tt-n-complete-observation-list-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-origin-v1 based.
 02 v-kind usage binary-long unsigned.
01 tt-n-complete-origin-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-profile-warning-v1 based.
 02 v-running.
 03 v-running-data usage pointer.
 03 v-running-len usage binary-double unsigned.
 02 v-tuned-for.
 03 v-tuned-for-data usage pointer.
 03 v-tuned-for-len usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-profile-warning-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-question-source-v1 based.
 02 v-answered-by.
 03 v-answered-by-data usage pointer.
 03 v-answered-by-len usage binary-double unsigned.
 02 v-batch-size.
 03 v-batch-size-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-batch-size-value usage binary-double unsigned.
 02 v-origin usage pointer.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-question-source-list-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-string-list-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-usage-v1 based.
 02 v-input-tokens.
 03 v-input-tokens-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-input-tokens-value usage binary-double unsigned.
 02 v-output-tokens.
 03 v-output-tokens-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-output-tokens-value usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-usage-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-meta-v1 based.
 02 v-answered-by.
 03 v-answered-by-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-answered-by-value.
 04 v-answered-by-value-data usage pointer.
 04 v-answered-by-value-len usage binary-double unsigned.
 02 v-attempts.
 03 v-attempts-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-attempts-value.
 04 v-attempts-value-data usage pointer.
 04 v-attempts-value-len usage binary-double unsigned.
 02 v-batch-setting.
 03 v-batch-setting-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-batch-setting-value usage pointer.
 02 v-batch-warning.
 03 v-batch-warning-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-batch-warning-value usage pointer.
 02 v-cached usage binary-long unsigned.
 02 filler pic x(4).
 02 v-context-sha256.
 03 v-context-sha256-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-context-sha256-value.
 04 v-context-sha256-value-data usage pointer.
 04 v-context-sha256-value-len usage binary-double unsigned.
 02 v-failed-questions usage binary-double unsigned.
 02 v-model.
 03 v-model-data usage pointer.
 03 v-model-len usage binary-double unsigned.
 02 v-observations.
 03 v-observations-data usage pointer.
 03 v-observations-len usage binary-double unsigned.
 02 v-origin.
 03 v-origin-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-origin-value usage pointer.
 02 v-profile-warning.
 03 v-profile-warning-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-profile-warning-value usage pointer.
 02 v-question-sha256.
 03 v-question-sha256-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-question-sha256-value.
 04 v-question-sha256-value-data usage pointer.
 04 v-question-sha256-value-len usage binary-double unsigned.
 02 v-question-sources.
 03 v-question-sources-data usage pointer.
 03 v-question-sources-len usage binary-double unsigned.
 02 v-questions-sha256.
 03 v-questions-sha256-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-questions-sha256-value.
 04 v-questions-sha256-value-data usage pointer.
 04 v-questions-sha256-value-len usage binary-double unsigned.
 02 v-requests.
 03 v-requests-data usage pointer.
 03 v-requests-len usage binary-double unsigned.
 02 v-requests-sent usage binary-double unsigned.
 02 v-tool.
 03 v-tool-data usage pointer.
 03 v-tool-len usage binary-double unsigned.
 02 v-url.
 03 v-url-data usage pointer.
 03 v-url-len usage binary-double unsigned.
 02 v-usage.
 03 v-usage-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-usage-value usage pointer.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-batch-string-v1 based.
 02 v-kind usage binary-long unsigned.
01 tt-n-complete-batch-v1 based.
 02 v-kind usage binary-long unsigned.
 02 filler pic x(4).
 02 v-data.
 03 v-data-union-storage pic x(8).
 03 v-data-integer redefines v-data-union-storage usage binary-double unsigned.
 03 v-data-string redefines v-data-union-storage usage pointer.
01 tt-n-complete-batch-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-string-type-v1 based.
 02 v-kind usage binary-long unsigned.
01 tt-n-complete-input-declaration-string-root-v1 based.
 02 v-type usage pointer.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-input-property-type-string-v1 based.
 02 v-type.
 03 v-type-data usage pointer.
 03 v-type-len usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-input-property-type-number-v1 based.
 02 v-type.
 03 v-type-data usage pointer.
 03 v-type-len usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-input-property-type-boolean-v1 based.
 02 v-type.
 03 v-type-data usage pointer.
 03 v-type-len usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-string-root-v1 based.
 02 v-type usage pointer.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-input-property-type-array-v1 based.
 02 v-items usage pointer.
 02 v-type.
 03 v-type-data usage pointer.
 03 v-type-len usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-input-property-type-v1 based.
 02 v-kind usage binary-long unsigned.
 02 filler pic x(4).
 02 v-data.
 03 v-data-union-storage pic x(8).
 03 v-data-string redefines v-data-union-storage usage pointer.
 03 v-data-number redefines v-data-union-storage usage pointer.
 03 v-data-boolean redefines v-data-union-storage usage pointer.
 03 v-data-array redefines v-data-union-storage usage pointer.
01 tt-n-complete-input-property-type-map-entry-v1 based.
 02 v-name.
 03 v-name-data usage pointer.
 03 v-name-len usage binary-double unsigned.
 02 v-value usage pointer.
01 tt-n-complete-input-property-type-map-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-string-list-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-value-data usage pointer.
 03 v-value-len usage binary-double unsigned.
01 tt-n-complete-object-type-v1 based.
 02 v-kind usage binary-long unsigned.
01 tt-n-complete-input-declaration-object-root-v1 based.
 02 v-properties.
 03 v-properties-data usage pointer.
 03 v-properties-len usage binary-double unsigned.
 02 v-required.
 03 v-required-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-required-value.
 04 v-required-value-data usage pointer.
 04 v-required-value-len usage binary-double unsigned.
 02 v-type usage pointer.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-input-declaration-v1 based.
 02 v-kind usage binary-long unsigned.
 02 filler pic x(4).
 02 v-data.
 03 v-data-union-storage pic x(8).
 03 v-data-string-root redefines v-data-union-storage usage pointer.
 03 v-data-object-root redefines v-data-union-storage usage pointer.
01 tt-n-complete-input-declaration-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-label-v1 based.
 02 v-description.
 03 v-description-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-description-value usage pointer.
 02 v-name.
 03 v-name-data usage pointer.
 03 v-name-len usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-label-list-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-label-list-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-value-data usage pointer.
 03 v-value-len usage binary-double unsigned.
01 tt-n-complete-question-name-v1 based.
 02 v-value.
 03 v-value-data usage pointer.
 03 v-value-len usage binary-double unsigned.
01 tt-n-complete-question-name-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-wording-version-v1 based.
 02 v-value usage binary-double unsigned.
01 tt-n-complete-wording-version-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-question-document-decide-v1 based.
 02 v-batch.
 03 v-batch-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-batch-value usage pointer.
 02 v-context-schema.
 03 v-context-schema-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-context-schema-value usage pointer.
 02 v-item-schema.
 03 v-item-schema-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-item-schema-value usage pointer.
 02 v-label-details.
 03 v-label-details-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-label-details-value.
 04 v-label-details-value-data usage pointer.
 04 v-label-details-value-len usage binary-double unsigned.
 02 v-model.
 03 v-model-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-model-value.
 04 v-model-value-data usage pointer.
 04 v-model-value-len usage binary-double unsigned.
 02 v-name.
 03 v-name-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-name-value usage pointer.
 02 v-on.
 03 v-on-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-on-value.
 04 v-on-value-data usage pointer.
 04 v-on-value-len usage binary-double unsigned.
 02 v-profile.
 03 v-profile-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-profile-value.
 04 v-profile-value-data usage pointer.
 04 v-profile-value-len usage binary-double unsigned.
 02 v-wording-version.
 03 v-wording-version-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-wording-version-value usage pointer.
 02 v-false.
 03 v-false--presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-false--value usage pointer.
 02 v-text.
 03 v-text-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-text-value usage pointer.
 02 v-true.
 03 v-true--presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-true--value usage pointer.
 02 v-verb.
 03 v-verb-data usage pointer.
 03 v-verb-len usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-question-document-choose-v1 based.
 02 v-batch.
 03 v-batch-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-batch-value usage pointer.
 02 v-context-schema.
 03 v-context-schema-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-context-schema-value usage pointer.
 02 v-item-schema.
 03 v-item-schema-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-item-schema-value usage pointer.
 02 v-label-details.
 03 v-label-details-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-label-details-value.
 04 v-label-details-value-data usage pointer.
 04 v-label-details-value-len usage binary-double unsigned.
 02 v-model.
 03 v-model-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-model-value.
 04 v-model-value-data usage pointer.
 04 v-model-value-len usage binary-double unsigned.
 02 v-name.
 03 v-name-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-name-value usage pointer.
 02 v-on.
 03 v-on-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-on-value.
 04 v-on-value-data usage pointer.
 04 v-on-value-len usage binary-double unsigned.
 02 v-profile.
 03 v-profile-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-profile-value.
 04 v-profile-value-data usage pointer.
 04 v-profile-value-len usage binary-double unsigned.
 02 v-wording-version.
 03 v-wording-version-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-wording-version-value usage pointer.
 02 v-options.
 03 v-options-data usage pointer.
 03 v-options-len usage binary-double unsigned.
 02 v-text.
 03 v-text-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-text-value usage pointer.
 02 v-verb.
 03 v-verb-data usage pointer.
 03 v-verb-len usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-question-document-tag-v1 based.
 02 v-batch.
 03 v-batch-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-batch-value usage pointer.
 02 v-context-schema.
 03 v-context-schema-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-context-schema-value usage pointer.
 02 v-item-schema.
 03 v-item-schema-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-item-schema-value usage pointer.
 02 v-label-details.
 03 v-label-details-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-label-details-value.
 04 v-label-details-value-data usage pointer.
 04 v-label-details-value-len usage binary-double unsigned.
 02 v-model.
 03 v-model-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-model-value.
 04 v-model-value-data usage pointer.
 04 v-model-value-len usage binary-double unsigned.
 02 v-name.
 03 v-name-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-name-value usage pointer.
 02 v-on.
 03 v-on-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-on-value.
 04 v-on-value-data usage pointer.
 04 v-on-value-len usage binary-double unsigned.
 02 v-profile.
 03 v-profile-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-profile-value.
 04 v-profile-value-data usage pointer.
 04 v-profile-value-len usage binary-double unsigned.
 02 v-wording-version.
 03 v-wording-version-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-wording-version-value usage pointer.
 02 v-labels.
 03 v-labels-data usage pointer.
 03 v-labels-len usage binary-double unsigned.
 02 v-text.
 03 v-text-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-text-value usage pointer.
 02 v-verb.
 03 v-verb-data usage pointer.
 03 v-verb-len usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-question-document-score-v1 based.
 02 v-batch.
 03 v-batch-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-batch-value usage pointer.
 02 v-context-schema.
 03 v-context-schema-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-context-schema-value usage pointer.
 02 v-item-schema.
 03 v-item-schema-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-item-schema-value usage pointer.
 02 v-label-details.
 03 v-label-details-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-label-details-value.
 04 v-label-details-value-data usage pointer.
 04 v-label-details-value-len usage binary-double unsigned.
 02 v-model.
 03 v-model-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-model-value.
 04 v-model-value-data usage pointer.
 04 v-model-value-len usage binary-double unsigned.
 02 v-name.
 03 v-name-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-name-value usage pointer.
 02 v-on.
 03 v-on-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-on-value.
 04 v-on-value-data usage pointer.
 04 v-on-value-len usage binary-double unsigned.
 02 v-profile.
 03 v-profile-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-profile-value.
 04 v-profile-value-data usage pointer.
 04 v-profile-value-len usage binary-double unsigned.
 02 v-wording-version.
 03 v-wording-version-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-wording-version-value usage pointer.
 02 v-levels.
 03 v-levels-data usage pointer.
 03 v-levels-len usage binary-double unsigned.
 02 v-text.
 03 v-text-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-text-value usage pointer.
 02 v-verb.
 03 v-verb-data usage pointer.
 03 v-verb-len usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-question-document-v1 based.
 02 v-kind usage binary-long unsigned.
 02 filler pic x(4).
 02 v-data.
 03 v-data-union-storage pic x(8).
 03 v-data-decide redefines v-data-union-storage usage pointer.
 03 v-data-choose redefines v-data-union-storage usage pointer.
 03 v-data-tag redefines v-data-union-storage usage pointer.
 03 v-data-score redefines v-data-union-storage usage pointer.
01 tt-n-complete-version-v1 based.
 02 v-kind usage binary-long unsigned.
01 tt-n-complete-physical-source-v1 based.
 02 v-file.
 03 v-file-data usage pointer.
 03 v-file-len usage binary-double unsigned.
 02 v-first-line.
 03 v-first-line-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-first-line-value usage binary-double unsigned.
 02 v-last-line.
 03 v-last-line-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-last-line-value usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-physical-source-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-rank-member-result-v1 based.
 02 v-answer usage pointer.
 02 v-answer-id usage pointer.
 02 v-images.
 03 v-images-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-images-value.
 04 v-images-value-data usage pointer.
 04 v-images-value-len usage binary-double unsigned.
 02 v-meta usage pointer.
 02 v-question usage pointer.
 02 v-schema usage pointer.
 02 v-source.
 03 v-source-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-source-value usage pointer.
 02 v-threshold.
 03 v-threshold-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-threshold-value usage pointer.
 02 v-value usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-rank-member-v1 based.
 02 v-name.
 03 v-name-data usage pointer.
 03 v-name-len usage binary-double unsigned.
 02 v-result usage pointer.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-rank-member-list-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-rank-member-list-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-value-data usage pointer.
 03 v-value-len usage binary-double unsigned.
01 tt-n-complete-threshold-v1 based.
 02 v-kind usage binary-long unsigned.
 02 filler pic x(4).
 02 v-data.
 03 v-data-union-storage pic x(16).
 03 v-data-number redefines v-data-union-storage usage float-long.
 03 v-data-string redefines v-data-union-storage.
 04 v-data-string-data usage pointer.
 04 v-data-string-len usage binary-double unsigned.
01 tt-n-complete-threshold-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-decide-value-v1 based.
 02 v-value usage pointer.
01 tt-n-complete-decide-value-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-atomic-decide-value-v1 based.
 02 v-answer usage pointer.
 02 v-answer-id usage pointer.
 02 v-images.
 03 v-images-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-images-value.
 04 v-images-value-data usage pointer.
 04 v-images-value-len usage binary-double unsigned.
 02 v-index.
 03 v-index-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-index-value usage binary-double unsigned.
 02 v-input.
 03 v-input-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-input-value usage pointer.
 02 v-members.
 03 v-members-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-members-value.
 04 v-members-value-data usage pointer.
 04 v-members-value-len usage binary-double unsigned.
 02 v-meta usage pointer.
 02 v-question usage pointer.
 02 v-question-name.
 03 v-question-name-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-question-name-value.
 04 v-question-name-value-data usage pointer.
 04 v-question-name-value-len usage binary-double unsigned.
 02 v-schema usage pointer.
 02 v-source.
 03 v-source-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-source-value usage pointer.
 02 v-threshold.
 03 v-threshold-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-threshold-value usage pointer.
 02 v-value.
 03 v-value-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value-value usage pointer.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-session-packet-decide-row-v1 based.
 02 v-function.
 03 v-function-data usage pointer.
 03 v-function-len usage binary-double unsigned.
 02 v-kind.
 03 v-kind-data usage pointer.
 03 v-kind-len usage binary-double unsigned.
 02 v-value usage pointer.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-atomic-nullable-string-v1 based.
 02 v-answer usage pointer.
 02 v-answer-id usage pointer.
 02 v-images.
 03 v-images-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-images-value.
 04 v-images-value-data usage pointer.
 04 v-images-value-len usage binary-double unsigned.
 02 v-index.
 03 v-index-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-index-value usage binary-double unsigned.
 02 v-input.
 03 v-input-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-input-value usage pointer.
 02 v-members.
 03 v-members-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-members-value.
 04 v-members-value-data usage pointer.
 04 v-members-value-len usage binary-double unsigned.
 02 v-meta usage pointer.
 02 v-question usage pointer.
 02 v-question-name.
 03 v-question-name-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-question-name-value.
 04 v-question-name-value-data usage pointer.
 04 v-question-name-value-len usage binary-double unsigned.
 02 v-schema usage pointer.
 02 v-source.
 03 v-source-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-source-value usage pointer.
 02 v-threshold.
 03 v-threshold-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-threshold-value usage pointer.
 02 v-value.
 03 v-value-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value-value.
 04 v-value-value-data usage pointer.
 04 v-value-value-len usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-session-packet-choose-row-v1 based.
 02 v-function.
 03 v-function-data usage pointer.
 03 v-function-len usage binary-double unsigned.
 02 v-kind.
 03 v-kind-data usage pointer.
 03 v-kind-len usage binary-double unsigned.
 02 v-value usage pointer.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-atomic-array-of-string-v1 based.
 02 v-answer usage pointer.
 02 v-answer-id usage pointer.
 02 v-images.
 03 v-images-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-images-value.
 04 v-images-value-data usage pointer.
 04 v-images-value-len usage binary-double unsigned.
 02 v-index.
 03 v-index-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-index-value usage binary-double unsigned.
 02 v-input.
 03 v-input-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-input-value usage pointer.
 02 v-members.
 03 v-members-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-members-value.
 04 v-members-value-data usage pointer.
 04 v-members-value-len usage binary-double unsigned.
 02 v-meta usage pointer.
 02 v-question usage pointer.
 02 v-question-name.
 03 v-question-name-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-question-name-value.
 04 v-question-name-value-data usage pointer.
 04 v-question-name-value-len usage binary-double unsigned.
 02 v-schema usage pointer.
 02 v-source.
 03 v-source-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-source-value usage pointer.
 02 v-threshold.
 03 v-threshold-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-threshold-value usage pointer.
 02 v-value.
 03 v-value-data usage pointer.
 03 v-value-len usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-session-packet-tag-row-v1 based.
 02 v-function.
 03 v-function-data usage pointer.
 03 v-function-len usage binary-double unsigned.
 02 v-kind.
 03 v-kind-data usage pointer.
 03 v-kind-len usage binary-double unsigned.
 02 v-value usage pointer.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-atomic-double-v1 based.
 02 v-answer usage pointer.
 02 v-answer-id usage pointer.
 02 v-images.
 03 v-images-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-images-value.
 04 v-images-value-data usage pointer.
 04 v-images-value-len usage binary-double unsigned.
 02 v-index.
 03 v-index-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-index-value usage binary-double unsigned.
 02 v-input.
 03 v-input-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-input-value usage pointer.
 02 v-members.
 03 v-members-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-members-value.
 04 v-members-value-data usage pointer.
 04 v-members-value-len usage binary-double unsigned.
 02 v-meta usage pointer.
 02 v-question usage pointer.
 02 v-question-name.
 03 v-question-name-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-question-name-value.
 04 v-question-name-value-data usage pointer.
 04 v-question-name-value-len usage binary-double unsigned.
 02 v-schema usage pointer.
 02 v-source.
 03 v-source-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-source-value usage pointer.
 02 v-threshold.
 03 v-threshold-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-threshold-value usage pointer.
 02 v-value usage float-long.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-session-packet-score-row-v1 based.
 02 v-function.
 03 v-function-data usage pointer.
 03 v-function-len usage binary-double unsigned.
 02 v-kind.
 03 v-kind-data usage pointer.
 03 v-kind-len usage binary-double unsigned.
 02 v-value usage pointer.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-atomic-boolean-v1 based.
 02 v-answer usage pointer.
 02 v-answer-id usage pointer.
 02 v-images.
 03 v-images-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-images-value.
 04 v-images-value-data usage pointer.
 04 v-images-value-len usage binary-double unsigned.
 02 v-index.
 03 v-index-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-index-value usage binary-double unsigned.
 02 v-input.
 03 v-input-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-input-value usage pointer.
 02 v-members.
 03 v-members-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-members-value.
 04 v-members-value-data usage pointer.
 04 v-members-value-len usage binary-double unsigned.
 02 v-meta usage pointer.
 02 v-question usage pointer.
 02 v-question-name.
 03 v-question-name-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-question-name-value.
 04 v-question-name-value-data usage pointer.
 04 v-question-name-value-len usage binary-double unsigned.
 02 v-schema usage pointer.
 02 v-source.
 03 v-source-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-source-value usage pointer.
 02 v-threshold.
 03 v-threshold-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-threshold-value usage pointer.
 02 v-value usage binary-long unsigned.
 02 filler pic x(4).
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-session-packet-filter-row-v1 based.
 02 v-function.
 03 v-function-data usage pointer.
 03 v-function-len usage binary-double unsigned.
 02 v-kind.
 03 v-kind-data usage pointer.
 03 v-kind-len usage binary-double unsigned.
 02 v-value usage pointer.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-value-v1 based.
 02 v-kind usage binary-long unsigned.
 02 filler pic x(4).
 02 v-data.
 03 v-data-union-storage pic x(16).
 03 v-data-boolean redefines v-data-union-storage usage binary-long unsigned.
 03 v-data-null redefines v-data-union-storage usage binary-long unsigned.
 03 v-data-string redefines v-data-union-storage.
 04 v-data-string-data usage pointer.
 04 v-data-string-len usage binary-double unsigned.
 03 v-data-array redefines v-data-union-storage.
 04 v-data-array-data usage pointer.
 04 v-data-array-len usage binary-double unsigned.
 03 v-data-number redefines v-data-union-storage usage float-long.
01 tt-n-complete-value-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-annotation-member-answer-id-v1 based.
 02 v-answer usage pointer.
 02 v-answer-id usage pointer.
 02 v-observations.
 03 v-observations-data usage pointer.
 03 v-observations-len usage binary-double unsigned.
 02 v-question usage pointer.
 02 v-question-sources.
 03 v-question-sources-data usage pointer.
 03 v-question-sources-len usage binary-double unsigned.
 02 v-request.
 03 v-request-data usage pointer.
 03 v-request-len usage binary-double unsigned.
 02 v-threshold.
 03 v-threshold-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-threshold-value usage pointer.
 02 v-usage.
 03 v-usage-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-usage-value usage pointer.
 02 v-value.
 03 v-value-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value-value usage pointer.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-failure-cause-v1 based.
 02 v-kind usage binary-long unsigned.
01 tt-n-complete-failure-field-kind-v1 based.
 02 v-kind usage binary-long unsigned.
01 tt-n-complete-failure-v1 based.
 02 v-cause usage pointer.
 02 v-kind usage pointer.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-annotation-member-failure-id-v1 based.
 02 v-failure usage pointer.
 02 v-failure-id usage pointer.
 02 v-observations.
 03 v-observations-data usage pointer.
 03 v-observations-len usage binary-double unsigned.
 02 v-question usage pointer.
 02 v-question-sources.
 03 v-question-sources-data usage pointer.
 03 v-question-sources-len usage binary-double unsigned.
 02 v-request.
 03 v-request-data usage pointer.
 03 v-request-len usage binary-double unsigned.
 02 v-threshold.
 03 v-threshold-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-threshold-value usage pointer.
 02 v-usage.
 03 v-usage-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-usage-value usage pointer.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-annotation-member-v1 based.
 02 v-kind usage binary-long unsigned.
 02 filler pic x(4).
 02 v-data.
 03 v-data-union-storage pic x(8).
 03 v-data-answer-id redefines v-data-union-storage usage pointer.
 03 v-data-failure-id redefines v-data-union-storage usage pointer.
01 tt-n-complete-annotation-member-map-entry-v1 based.
 02 v-name.
 03 v-name-data usage pointer.
 03 v-name-len usage binary-double unsigned.
 02 v-value usage pointer.
01 tt-n-complete-annotation-member-map-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-position-v1 based.
 02 v-file.
 03 v-file-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-file-value.
 04 v-file-value-data usage pointer.
 04 v-file-value-len usage binary-double unsigned.
 02 v-first.
 03 v-first-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-first-value usage binary-double unsigned.
 02 v-images.
 03 v-images-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-images-value.
 04 v-images-value-data usage pointer.
 04 v-images-value-len usage binary-double unsigned.
 02 v-last.
 03 v-last-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-last-value usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-position-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-failed-v1 based.
 02 v-failed usage pointer.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-annotated-field-v1 based.
 02 v-kind usage binary-long unsigned.
 02 filler pic x(4).
 02 v-data.
 03 v-data-union-storage pic x(16).
 03 v-data-boolean redefines v-data-union-storage usage binary-long unsigned.
 03 v-data-null redefines v-data-union-storage usage binary-long unsigned.
 03 v-data-string redefines v-data-union-storage.
 04 v-data-string-data usage pointer.
 04 v-data-string-len usage binary-double unsigned.
 03 v-data-array redefines v-data-union-storage.
 04 v-data-array-data usage pointer.
 04 v-data-array-len usage binary-double unsigned.
 03 v-data-number redefines v-data-union-storage usage float-long.
 03 v-data-object redefines v-data-union-storage usage pointer.
01 tt-n-complete-annotated-field-map-entry-v1 based.
 02 v-name.
 03 v-name-data usage pointer.
 03 v-name-len usage binary-double unsigned.
 02 v-value usage pointer.
01 tt-n-complete-annotated-field-map-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-annotated-row-v1 based.
 02 v-value.
 03 v-value-data usage pointer.
 03 v-value-len usage binary-double unsigned.
01 tt-n-complete-annotation-v1 based.
 02 v-answer-id usage pointer.
 02 v-answers.
 03 v-answers-data usage pointer.
 03 v-answers-len usage binary-double unsigned.
 02 v-file.
 03 v-file-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-file-value.
 04 v-file-value-data usage pointer.
 04 v-file-value-len usage binary-double unsigned.
 02 v-first-line.
 03 v-first-line-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-first-line-value usage binary-double unsigned.
 02 v-index.
 03 v-index-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-index-value usage binary-double unsigned.
 02 v-input.
 03 v-input-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-input-value usage pointer.
 02 v-last-line.
 03 v-last-line-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-last-line-value usage binary-double unsigned.
 02 v-meta usage pointer.
 02 v-position.
 03 v-position-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-position-value usage pointer.
 02 v-schema usage pointer.
 02 v-source.
 03 v-source-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-source-value usage pointer.
 02 v-value usage pointer.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-session-packet-annotate-row-v1 based.
 02 v-function.
 03 v-function-data usage pointer.
 03 v-function-len usage binary-double unsigned.
 02 v-kind.
 03 v-kind-data usage pointer.
 03 v-kind-len usage binary-double unsigned.
 02 v-value usage pointer.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-atomic-decide-value-list-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-session-packet-decide-aggregate-v1 based.
 02 v-function.
 03 v-function-data usage pointer.
 03 v-function-len usage binary-double unsigned.
 02 v-kind.
 03 v-kind-data usage pointer.
 03 v-kind-len usage binary-double unsigned.
 02 v-value.
 03 v-value-data usage pointer.
 03 v-value-len usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-atomic-nullable-string-list-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-session-packet-choose-aggregate-v1 based.
 02 v-function.
 03 v-function-data usage pointer.
 03 v-function-len usage binary-double unsigned.
 02 v-kind.
 03 v-kind-data usage pointer.
 03 v-kind-len usage binary-double unsigned.
 02 v-value.
 03 v-value-data usage pointer.
 03 v-value-len usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-atomic-array-of-string-list-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-session-packet-tag-aggregate-v1 based.
 02 v-function.
 03 v-function-data usage pointer.
 03 v-function-len usage binary-double unsigned.
 02 v-kind.
 03 v-kind-data usage pointer.
 03 v-kind-len usage binary-double unsigned.
 02 v-value.
 03 v-value-data usage pointer.
 03 v-value-len usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-atomic-double-list-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-session-packet-score-aggregate-v1 based.
 02 v-function.
 03 v-function-data usage pointer.
 03 v-function-len usage binary-double unsigned.
 02 v-kind.
 03 v-kind-data usage pointer.
 03 v-kind-len usage binary-double unsigned.
 02 v-value.
 03 v-value-data usage pointer.
 03 v-value-len usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-atomic-boolean-list-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-session-packet-filter-aggregate-v1 based.
 02 v-function.
 03 v-function-data usage pointer.
 03 v-function-len usage binary-double unsigned.
 02 v-kind.
 03 v-kind-data usage pointer.
 03 v-kind-len usage binary-double unsigned.
 02 v-value.
 03 v-value-data usage pointer.
 03 v-value-len usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-atomic-non-zero-usize-v1 based.
 02 v-answer usage pointer.
 02 v-answer-id usage pointer.
 02 v-images.
 03 v-images-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-images-value.
 04 v-images-value-data usage pointer.
 04 v-images-value-len usage binary-double unsigned.
 02 v-index.
 03 v-index-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-index-value usage binary-double unsigned.
 02 v-input.
 03 v-input-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-input-value usage pointer.
 02 v-members.
 03 v-members-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-members-value.
 04 v-members-value-data usage pointer.
 04 v-members-value-len usage binary-double unsigned.
 02 v-meta usage pointer.
 02 v-question usage pointer.
 02 v-question-name.
 03 v-question-name-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-question-name-value.
 04 v-question-name-value-data usage pointer.
 04 v-question-name-value-len usage binary-double unsigned.
 02 v-schema usage pointer.
 02 v-source.
 03 v-source-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-source-value usage pointer.
 02 v-threshold.
 03 v-threshold-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-threshold-value usage pointer.
 02 v-value usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-atomic-non-zero-usize-list-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-session-packet-rank-aggregate-v1 based.
 02 v-function.
 03 v-function-data usage pointer.
 03 v-function-len usage binary-double unsigned.
 02 v-kind.
 03 v-kind-data usage pointer.
 03 v-kind-len usage binary-double unsigned.
 02 v-value.
 03 v-value-data usage pointer.
 03 v-value-len usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-find-answer-v1 based.
 02 v-confidence.
 03 v-confidence-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-confidence-value usage float-long.
 02 v-kind.
 03 v-kind-data usage pointer.
 03 v-kind-len usage binary-double unsigned.
 02 v-pick.
 03 v-pick-data usage pointer.
 03 v-pick-len usage binary-double unsigned.
 02 v-probabilities.
 03 v-probabilities-data usage pointer.
 03 v-probabilities-len usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-find-candidate-v1 based.
 02 v-index.
 03 v-index-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-index-value usage binary-double unsigned.
 02 v-input.
 03 v-input-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-input-value usage pointer.
 02 v-probability usage float-long.
 02 v-source.
 03 v-source-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-source-value usage pointer.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-find-candidate-list-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-find-candidate-list-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-value-data usage pointer.
 03 v-value-len usage binary-double unsigned.
01 tt-n-complete-find-question-document-v1 based.
 02 v-batch.
 03 v-batch-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-batch-value usage pointer.
 02 v-context-schema.
 03 v-context-schema-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-context-schema-value usage pointer.
 02 v-item-schema.
 03 v-item-schema-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-item-schema-value usage pointer.
 02 v-label-details.
 03 v-label-details-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-label-details-value.
 04 v-label-details-value-data usage pointer.
 04 v-label-details-value-len usage binary-double unsigned.
 02 v-model.
 03 v-model-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-model-value.
 04 v-model-value-data usage pointer.
 04 v-model-value-len usage binary-double unsigned.
 02 v-name.
 03 v-name-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-name-value usage pointer.
 02 v-none usage binary-long unsigned.
 02 filler pic x(4).
 02 v-on.
 03 v-on-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-on-value.
 04 v-on-value-data usage pointer.
 04 v-on-value-len usage binary-double unsigned.
 02 v-profile.
 03 v-profile-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-profile-value.
 04 v-profile-value-data usage pointer.
 04 v-profile-value-len usage binary-double unsigned.
 02 v-text.
 03 v-text-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-text-value usage pointer.
 02 v-verb.
 03 v-verb-data usage pointer.
 03 v-verb-len usage binary-double unsigned.
 02 v-wording-version.
 03 v-wording-version-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-wording-version-value usage pointer.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-find-v1 based.
 02 v-answer usage pointer.
 02 v-answer-id usage pointer.
 02 v-candidates.
 03 v-candidates-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-candidates-value.
 04 v-candidates-value-data usage pointer.
 04 v-candidates-value-len usage binary-double unsigned.
 02 v-file.
 03 v-file-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-file-value.
 04 v-file-value-data usage pointer.
 04 v-file-value-len usage binary-double unsigned.
 02 v-first-line.
 03 v-first-line-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-first-line-value usage binary-double unsigned.
 02 v-index.
 03 v-index-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-index-value usage binary-double unsigned.
 02 v-last-line.
 03 v-last-line-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-last-line-value usage binary-double unsigned.
 02 v-meta usage pointer.
 02 v-position.
 03 v-position-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-position-value usage pointer.
 02 v-question usage pointer.
 02 v-schema usage pointer.
 02 v-threshold.
 03 v-threshold-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-threshold-value usage pointer.
 02 v-value.
 03 v-value-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value-value usage pointer.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-session-packet-find-aggregate-v1 based.
 02 v-function.
 03 v-function-data usage pointer.
 03 v-function-len usage binary-double unsigned.
 02 v-kind.
 03 v-kind-data usage pointer.
 03 v-kind-len usage binary-double unsigned.
 02 v-value usage pointer.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-annotation-list-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-session-packet-annotate-aggregate-v1 based.
 02 v-function.
 03 v-function-data usage pointer.
 03 v-function-len usage binary-double unsigned.
 02 v-kind.
 03 v-kind-data usage pointer.
 03 v-kind-len usage binary-double unsigned.
 02 v-value.
 03 v-value-data usage pointer.
 03 v-value-len usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-number-map-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-value-data usage pointer.
 03 v-value-len usage binary-double unsigned.
01 tt-n-complete-name-odds-v1 based.
 02 v-edges.
 03 v-edges-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-edges-value.
 04 v-edges-value-data usage pointer.
 04 v-edges-value-len usage binary-double unsigned.
 02 v-end usage binary-double unsigned.
 02 v-kinds.
 03 v-kinds-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-kinds-value.
 04 v-kinds-value-data usage pointer.
 04 v-kinds-value-len usage binary-double unsigned.
 02 v-start usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-name-odds-list-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-place-v1 based.
 02 v-end usage binary-double unsigned.
 02 v-start usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-pair-odds-v1 based.
 02 v-probability usage float-long.
 02 v-relation.
 03 v-relation-data usage pointer.
 03 v-relation-len usage binary-double unsigned.
 02 v-source usage pointer.
 02 v-target usage pointer.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-pair-odds-list-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-piece-odds-v1 based.
 02 v-end usage binary-double unsigned.
 02 v-start usage binary-double unsigned.
 02 v-tags.
 03 v-tags-data usage pointer.
 03 v-tags-len usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-piece-odds-list-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-place-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-recognition-proposal-v1 based.
 02 v-end usage binary-double unsigned.
 02 v-kept usage binary-long unsigned.
 02 filler pic x(4).
 02 v-kind.
 03 v-kind-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-kind-value.
 04 v-kind-value-data usage pointer.
 04 v-kind-value-len usage binary-double unsigned.
 02 v-selected.
 03 v-selected-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-selected-value usage pointer.
 02 v-span-probability usage float-long.
 02 v-start usage binary-double unsigned.
 02 v-strength.
 03 v-strength-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-strength-value usage float-long.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-recognition-proposal-list-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-recognition-odds-recognize-answer-v1 based.
 02 v-names.
 03 v-names-data usage pointer.
 03 v-names-len usage binary-double unsigned.
 02 v-pairs.
 03 v-pairs-data usage pointer.
 03 v-pairs-len usage binary-double unsigned.
 02 v-pieces.
 03 v-pieces-data usage pointer.
 03 v-pieces-len usage binary-double unsigned.
 02 v-proposals.
 03 v-proposals-data usage pointer.
 03 v-proposals-len usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-boundary-proposal-v1 based.
 02 v-end usage binary-double unsigned.
 02 v-length usage binary-double unsigned.
 02 v-probability usage float-long.
 02 v-start usage binary-double unsigned.
 02 v-text.
 03 v-text-data usage pointer.
 03 v-text-len usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-boundary-proposal-list-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-recognition-odds-boundary-odds-v1 based.
 02 v-pieces.
 03 v-pieces-data usage pointer.
 03 v-pieces-len usage binary-double unsigned.
 02 v-proposals.
 03 v-proposals-data usage pointer.
 03 v-proposals-len usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-recognition-odds-v1 based.
 02 v-kind usage binary-long unsigned.
 02 filler pic x(4).
 02 v-data.
 03 v-data-union-storage pic x(8).
 03 v-data-recognize-answer redefines v-data-union-storage usage pointer.
 03 v-data-boundary-odds redefines v-data-union-storage usage pointer.
01 tt-n-complete-json-map-entry-v1 based.
 02 v-name.
 03 v-name-data usage pointer.
 03 v-name-len usage binary-double unsigned.
 02 v-value usage pointer.
01 tt-n-complete-json-map-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-recognition-mode-v1 based.
 02 v-kind usage binary-long unsigned.
01 tt-n-complete-recognition-mode-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-relation-rule-v1 based.
 02 v-either usage binary-long unsigned.
 02 filler pic x(4).
 02 v-name.
 03 v-name-data usage pointer.
 03 v-name-len usage binary-double unsigned.
 02 v-reads.
 03 v-reads-data usage pointer.
 03 v-reads-len usage binary-double unsigned.
 02 v-single.
 03 v-single-presence usage binary-long unsigned.
 03 v-single-value usage binary-long unsigned.
 02 v-source.
 03 v-source-data usage pointer.
 03 v-source-len usage binary-double unsigned.
 02 v-target.
 03 v-target-data usage pointer.
 03 v-target-len usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-relation-rule-list-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-relation-rule-list-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-value-data usage pointer.
 03 v-value-len usage binary-double unsigned.
01 tt-n-complete-recognition-stage-context-v1 based.
 02 v-boundary.
 03 v-boundary-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-boundary-value.
 04 v-boundary-value-data usage pointer.
 04 v-boundary-value-len usage binary-double unsigned.
 02 v-kind-edge.
 03 v-kind-edge-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-kind-edge-value.
 04 v-kind-edge-value-data usage pointer.
 04 v-kind-edge-value-len usage binary-double unsigned.
 02 v-relation.
 03 v-relation-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-relation-value.
 04 v-relation-value-data usage pointer.
 04 v-relation-value-len usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-recognition-stage-context-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-verb-v1 based.
 02 v-kind usage binary-long unsigned.
01 tt-n-complete-recognize-question-document-v1 based.
 02 v-batch.
 03 v-batch-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-batch-value usage pointer.
 02 v-context-schema.
 03 v-context-schema-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-context-schema-value usage pointer.
 02 v-entity-definition.
 03 v-entity-definition-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-entity-definition-value usage pointer.
 02 v-instructions.
 03 v-instructions-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-instructions-value usage pointer.
 02 v-item-schema.
 03 v-item-schema-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-item-schema-value usage pointer.
 02 v-kinds.
 03 v-kinds-data usage pointer.
 03 v-kinds-len usage binary-double unsigned.
 02 v-label-details.
 03 v-label-details-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-label-details-value.
 04 v-label-details-value-data usage pointer.
 04 v-label-details-value-len usage binary-double unsigned.
 02 v-mode.
 03 v-mode-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-mode-value usage pointer.
 02 v-model.
 03 v-model-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-model-value.
 04 v-model-value-data usage pointer.
 04 v-model-value-len usage binary-double unsigned.
 02 v-name.
 03 v-name-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-name-value usage pointer.
 02 v-on.
 03 v-on-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-on-value.
 04 v-on-value-data usage pointer.
 04 v-on-value-len usage binary-double unsigned.
 02 v-profile.
 03 v-profile-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-profile-value.
 04 v-profile-value-data usage pointer.
 04 v-profile-value-len usage binary-double unsigned.
 02 v-relation-threshold.
 03 v-relation-threshold-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-relation-threshold-value usage pointer.
 02 v-relations.
 03 v-relations-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-relations-value.
 04 v-relations-value-data usage pointer.
 04 v-relations-value-len usage binary-double unsigned.
 02 v-snippet-pieces.
 03 v-snippet-pieces-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-snippet-pieces-value usage binary-double unsigned.
 02 v-stage-context.
 03 v-stage-context-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-stage-context-value usage pointer.
 02 v-threshold usage pointer.
 02 v-verb usage pointer.
 02 v-wording-version.
 03 v-wording-version-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-wording-version-value usage pointer.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-entity-v1 based.
 02 v-end usage binary-double unsigned.
 02 v-file.
 03 v-file-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-file-value.
 04 v-file-value-data usage pointer.
 04 v-file-value-len usage binary-double unsigned.
 02 v-first-line.
 03 v-first-line-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-first-line-value usage binary-double unsigned.
 02 v-kind.
 03 v-kind-data usage pointer.
 03 v-kind-len usage binary-double unsigned.
 02 v-last-line.
 03 v-last-line-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-last-line-value usage binary-double unsigned.
 02 v-length usage binary-double unsigned.
 02 v-start usage binary-double unsigned.
 02 v-strength usage float-long.
 02 v-text.
 03 v-text-data usage pointer.
 03 v-text-len usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-entity-list-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-entity-edge-v1 based.
 02 v-either.
 03 v-either-presence usage binary-long unsigned.
 03 v-either-value usage binary-long unsigned.
 02 v-probability usage float-long.
 02 v-relation.
 03 v-relation-data usage pointer.
 03 v-relation-len usage binary-double unsigned.
 02 v-source usage pointer.
 02 v-target usage pointer.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-entity-edge-list-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-entity-edge-list-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-value-data usage pointer.
 03 v-value-len usage binary-double unsigned.
01 tt-n-complete-recognize-entities-v1 based.
 02 v-entities.
 03 v-entities-data usage pointer.
 03 v-entities-len usage binary-double unsigned.
 02 v-relations.
 03 v-relations-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-relations-value.
 04 v-relations-value-data usage pointer.
 04 v-relations-value-len usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-boundary-mode-v1 based.
 02 v-kind usage binary-long unsigned.
01 tt-n-complete-recognize-mode-proposals-v1 based.
 02 v-mode usage pointer.
 02 v-proposals.
 03 v-proposals-data usage pointer.
 03 v-proposals-len usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-recognize-v1 based.
 02 v-kind usage binary-long unsigned.
 02 filler pic x(4).
 02 v-data.
 03 v-data-union-storage pic x(8).
 03 v-data-entities redefines v-data-union-storage usage pointer.
 03 v-data-mode-proposals redefines v-data-union-storage usage pointer.
01 tt-n-complete-recognition-v1 based.
 02 v-answer usage pointer.
 02 v-answer-id usage pointer.
 02 v-file.
 03 v-file-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-file-value.
 04 v-file-value-data usage pointer.
 04 v-file-value-len usage binary-double unsigned.
 02 v-first-line.
 03 v-first-line-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-first-line-value usage binary-double unsigned.
 02 v-index.
 03 v-index-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-index-value usage binary-double unsigned.
 02 v-input.
 03 v-input-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-input-value usage pointer.
 02 v-last-line.
 03 v-last-line-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-last-line-value usage binary-double unsigned.
 02 v-meta usage pointer.
 02 v-position.
 03 v-position-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-position-value usage pointer.
 02 v-question usage pointer.
 02 v-schema usage pointer.
 02 v-source.
 03 v-source-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-source-value usage pointer.
 02 v-value usage pointer.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-recognition-list-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-session-packet-recognize-aggregate-v1 based.
 02 v-function.
 03 v-function-data usage pointer.
 03 v-function-len usage binary-double unsigned.
 02 v-kind.
 03 v-kind-data usage pointer.
 03 v-kind-len usage binary-double unsigned.
 02 v-value.
 03 v-value-data usage pointer.
 03 v-value-len usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-relation-direction-v1 based.
 02 v-kind usage binary-long unsigned.
01 tt-n-complete-relation-method-v1 based.
 02 v-kind usage binary-long unsigned.
01 tt-n-complete-related-entity-v1 based.
 02 v-kind.
 03 v-kind-data usage pointer.
 03 v-kind-len usage binary-double unsigned.
 02 v-name.
 03 v-name-data usage pointer.
 03 v-name-len usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-related-entity-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-relation-member-answer-id-v1 based.
 02 v-direction usage pointer.
 02 v-method usage pointer.
 02 v-observations.
 03 v-observations-data usage pointer.
 03 v-observations-len usage binary-double unsigned.
 02 v-question usage pointer.
 02 v-question-sources.
 03 v-question-sources-data usage pointer.
 03 v-question-sources-len usage binary-double unsigned.
 02 v-reads.
 03 v-reads-data usage pointer.
 03 v-reads-len usage binary-double unsigned.
 02 v-relation.
 03 v-relation-data usage pointer.
 03 v-relation-len usage binary-double unsigned.
 02 v-request.
 03 v-request-data usage pointer.
 03 v-request-len usage binary-double unsigned.
 02 v-source usage pointer.
 02 v-target.
 03 v-target-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-target-value usage pointer.
 02 v-threshold usage pointer.
 02 v-usage.
 03 v-usage-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-usage-value usage pointer.
 02 v-accepted usage binary-long unsigned.
 02 filler pic x(4).
 02 v-answer usage pointer.
 02 v-answer-id usage pointer.
 02 v-probability usage float-long.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-relation-member-failure-id-v1 based.
 02 v-direction usage pointer.
 02 v-method usage pointer.
 02 v-observations.
 03 v-observations-data usage pointer.
 03 v-observations-len usage binary-double unsigned.
 02 v-question usage pointer.
 02 v-question-sources.
 03 v-question-sources-data usage pointer.
 03 v-question-sources-len usage binary-double unsigned.
 02 v-reads.
 03 v-reads-data usage pointer.
 03 v-reads-len usage binary-double unsigned.
 02 v-relation.
 03 v-relation-data usage pointer.
 03 v-relation-len usage binary-double unsigned.
 02 v-request.
 03 v-request-data usage pointer.
 03 v-request-len usage binary-double unsigned.
 02 v-source usage pointer.
 02 v-target.
 03 v-target-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-target-value usage pointer.
 02 v-threshold usage pointer.
 02 v-usage.
 03 v-usage-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-usage-value usage pointer.
 02 v-failure usage pointer.
 02 v-failure-id usage pointer.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-relation-member-v1 based.
 02 v-kind usage binary-long unsigned.
 02 filler pic x(4).
 02 v-data.
 03 v-data-union-storage pic x(8).
 03 v-data-answer-id redefines v-data-union-storage usage pointer.
 03 v-data-failure-id redefines v-data-union-storage usage pointer.
01 tt-n-complete-relation-member-list-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-answers-v1 based.
 02 v-questions.
 03 v-questions-data usage pointer.
 03 v-questions-len usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-session-input-source-v1 based.
 02 v-index usage binary-double unsigned.
 02 v-source usage pointer.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-session-input-source-list-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-session-input-source-list-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-value-data usage pointer.
 03 v-value-len usage binary-double unsigned.
01 tt-n-complete-relate-fields-v1 based.
 02 v-kind.
 03 v-kind-data usage pointer.
 03 v-kind-len usage binary-double unsigned.
 02 v-name.
 03 v-name-data usage pointer.
 03 v-name-len usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-relate-fields-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-relate-question-document-v1 based.
 02 v-batch.
 03 v-batch-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-batch-value usage pointer.
 02 v-context-schema.
 03 v-context-schema-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-context-schema-value usage pointer.
 02 v-fields.
 03 v-fields-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-fields-value usage pointer.
 02 v-item-schema.
 03 v-item-schema-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-item-schema-value usage pointer.
 02 v-label-details.
 03 v-label-details-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-label-details-value.
 04 v-label-details-value-data usage pointer.
 04 v-label-details-value-len usage binary-double unsigned.
 02 v-model.
 03 v-model-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-model-value.
 04 v-model-value-data usage pointer.
 04 v-model-value-len usage binary-double unsigned.
 02 v-name.
 03 v-name-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-name-value usage pointer.
 02 v-on.
 03 v-on-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-on-value.
 04 v-on-value-data usage pointer.
 04 v-on-value-len usage binary-double unsigned.
 02 v-profile.
 03 v-profile-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-profile-value.
 04 v-profile-value-data usage pointer.
 04 v-profile-value-len usage binary-double unsigned.
 02 v-relations.
 03 v-relations-data usage pointer.
 03 v-relations-len usage binary-double unsigned.
 02 v-threshold usage pointer.
 02 v-verb.
 03 v-verb-data usage pointer.
 03 v-verb-len usage binary-double unsigned.
 02 v-wording-version.
 03 v-wording-version-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-wording-version-value usage pointer.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-relation-endpoint-related-entity-v1 based.
 02 v-kind.
 03 v-kind-data usage pointer.
 03 v-kind-len usage binary-double unsigned.
 02 v-name.
 03 v-name-data usage pointer.
 03 v-name-len usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-relation-endpoint-source-v1 based.
 02 v-file.
 03 v-file-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-file-value.
 04 v-file-value-data usage pointer.
 04 v-file-value-len usage binary-double unsigned.
 02 v-first-line.
 03 v-first-line-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-first-line-value usage binary-double unsigned.
 02 v-kind.
 03 v-kind-data usage pointer.
 03 v-kind-len usage binary-double unsigned.
 02 v-last-line.
 03 v-last-line-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-last-line-value usage binary-double unsigned.
 02 v-name.
 03 v-name-data usage pointer.
 03 v-name-len usage binary-double unsigned.
 02 v-ordinal usage binary-double unsigned.
 02 v-record.
 03 v-record-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-record-value usage pointer.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-relation-endpoint-v1 based.
 02 v-kind usage binary-long unsigned.
 02 filler pic x(4).
 02 v-data.
 03 v-data-union-storage pic x(8).
 03 v-data-related-entity redefines v-data-union-storage usage pointer.
 03 v-data-source redefines v-data-union-storage usage pointer.
01 tt-n-complete-related-entity-edge-v1 based.
 02 v-either.
 03 v-either-presence usage binary-long unsigned.
 03 v-either-value usage binary-long unsigned.
 02 v-probability usage float-long.
 02 v-relation.
 03 v-relation-data usage pointer.
 03 v-relation-len usage binary-double unsigned.
 02 v-source usage pointer.
 02 v-target usage pointer.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-related-entity-edge-list-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-relation-v1 based.
 02 v-answer usage pointer.
 02 v-answer-id usage pointer.
 02 v-file.
 03 v-file-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-file-value.
 04 v-file-value-data usage pointer.
 04 v-file-value-len usage binary-double unsigned.
 02 v-first-line.
 03 v-first-line-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-first-line-value usage binary-double unsigned.
 02 v-index.
 03 v-index-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-index-value usage binary-double unsigned.
 02 v-input.
 03 v-input-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-input-value usage pointer.
 02 v-input-sources.
 03 v-input-sources-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-input-sources-value.
 04 v-input-sources-value-data usage pointer.
 04 v-input-sources-value-len usage binary-double unsigned.
 02 v-last-line.
 03 v-last-line-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-last-line-value usage binary-double unsigned.
 02 v-meta usage pointer.
 02 v-position.
 03 v-position-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-position-value usage pointer.
 02 v-question usage pointer.
 02 v-schema usage pointer.
 02 v-value.
 03 v-value-data usage pointer.
 03 v-value-len usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-session-packet-relate-aggregate-v1 based.
 02 v-function.
 03 v-function-data usage pointer.
 03 v-function-len usage binary-double unsigned.
 02 v-kind.
 03 v-kind-data usage pointer.
 03 v-kind-len usage binary-double unsigned.
 02 v-value usage pointer.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-request-function-v1 based.
 02 v-kind usage binary-long unsigned.
01 tt-n-complete-answer-id-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-failure-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-failure-id-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-json-list-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-session-probabilities-yes-no-v1 based.
 02 v-kind.
 03 v-kind-data usage pointer.
 03 v-kind-len usage binary-double unsigned.
 02 v-value usage float-long.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-session-named-probability-v1 based.
 02 v-name.
 03 v-name-data usage pointer.
 03 v-name-len usage binary-double unsigned.
 02 v-probability usage float-long.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-session-named-probability-list-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-session-probabilities-named-v1 based.
 02 v-kind.
 03 v-kind-data usage pointer.
 03 v-kind-len usage binary-double unsigned.
 02 v-value.
 03 v-value-data usage pointer.
 03 v-value-len usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-session-probabilities-v1 based.
 02 v-kind usage binary-long unsigned.
 02 filler pic x(4).
 02 v-data.
 03 v-data-union-storage pic x(8).
 03 v-data-yes-no redefines v-data-union-storage usage pointer.
 03 v-data-named redefines v-data-union-storage usage pointer.
01 tt-n-complete-session-probabilities-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-token-usage-v1 based.
 02 v-input-tokens usage binary-double unsigned.
 02 v-output-tokens usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-token-usage-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-session-question-detail-v1 based.
 02 v-answer-id.
 03 v-answer-id-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-answer-id-value usage pointer.
 02 v-cached usage binary-long unsigned.
 02 filler pic x(4).
 02 v-confidence.
 03 v-confidence-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-confidence-value usage float-long.
 02 v-failed-questions usage binary-double unsigned.
 02 v-failure.
 03 v-failure-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-failure-value usage pointer.
 02 v-failure-id.
 03 v-failure-id-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-failure-id-value usage pointer.
 02 v-input.
 03 v-input-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-input-value usage pointer.
 02 v-input-source.
 03 v-input-source-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-input-source-value usage pointer.
 02 v-input-sources.
 03 v-input-sources-data usage pointer.
 03 v-input-sources-len usage binary-double unsigned.
 02 v-inputs.
 03 v-inputs-data usage pointer.
 03 v-inputs-len usage binary-double unsigned.
 02 v-model.
 03 v-model-data usage pointer.
 03 v-model-len usage binary-double unsigned.
 02 v-observations.
 03 v-observations-data usage pointer.
 03 v-observations-len usage binary-double unsigned.
 02 v-probabilities.
 03 v-probabilities-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-probabilities-value usage pointer.
 02 v-question usage pointer.
 02 v-question-sha256.
 03 v-question-sha256-data usage pointer.
 03 v-question-sha256-len usage binary-double unsigned.
 02 v-question-sources.
 03 v-question-sources-data usage pointer.
 03 v-question-sources-len usage binary-double unsigned.
 02 v-raw-pick.
 03 v-raw-pick-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-raw-pick-value.
 04 v-raw-pick-value-data usage pointer.
 04 v-raw-pick-value-len usage binary-double unsigned.
 02 v-reported-usage.
 03 v-reported-usage-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-reported-usage-value usage pointer.
 02 v-requests.
 03 v-requests-data usage pointer.
 03 v-requests-len usage binary-double unsigned.
 02 v-requests-sent usage binary-double unsigned.
 02 v-threshold.
 03 v-threshold-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-threshold-value usage pointer.
 02 v-url.
 03 v-url-data usage pointer.
 03 v-url-len usage binary-double unsigned.
 02 v-usage.
 03 v-usage-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-usage-value usage pointer.
 02 v-value.
 03 v-value-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value-value usage pointer.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-session-observation-question-v1 based.
 02 v-detail usage pointer.
 02 v-index usage binary-double unsigned.
 02 v-kind.
 03 v-kind-data usage pointer.
 03 v-kind-len usage binary-double unsigned.
 02 v-member.
 03 v-member-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-member-value.
 04 v-member-value-data usage pointer.
 04 v-member-value-len usage binary-double unsigned.
 02 v-position usage binary-double unsigned.
 02 v-stage.
 03 v-stage-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-stage-value.
 04 v-stage-value-data usage pointer.
 04 v-stage-value-len usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-session-judgment-decision-v1 based.
 02 v-kind.
 03 v-kind-data usage pointer.
 03 v-kind-len usage binary-double unsigned.
 02 v-value.
 03 v-value-presence usage binary-long unsigned.
 03 v-value-value usage binary-long unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-session-judgment-choice-v1 based.
 02 v-kind.
 03 v-kind-data usage pointer.
 03 v-kind-len usage binary-double unsigned.
 02 v-value.
 03 v-value-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value-value.
 04 v-value-value-data usage pointer.
 04 v-value-value-len usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-session-judgment-score-v1 based.
 02 v-kind.
 03 v-kind-data usage pointer.
 03 v-kind-len usage binary-double unsigned.
 02 v-value usage float-long.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-session-judgment-tags-v1 based.
 02 v-kind.
 03 v-kind-data usage pointer.
 03 v-kind-len usage binary-double unsigned.
 02 v-value.
 03 v-value-data usage pointer.
 03 v-value-len usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-session-judgment-v1 based.
 02 v-kind usage binary-long unsigned.
 02 filler pic x(4).
 02 v-data.
 03 v-data-union-storage pic x(8).
 03 v-data-decision redefines v-data-union-storage usage pointer.
 03 v-data-choice redefines v-data-union-storage usage pointer.
 03 v-data-score redefines v-data-union-storage usage pointer.
 03 v-data-tags redefines v-data-union-storage usage pointer.
01 tt-n-complete-session-observed-row-judgment-v1 based.
 02 v-kind.
 03 v-kind-data usage pointer.
 03 v-kind-len usage binary-double unsigned.
 02 v-value usage pointer.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-annotation-value-decision-v1 based.
 02 v-kind.
 03 v-kind-data usage pointer.
 03 v-kind-len usage binary-double unsigned.
 02 v-value.
 03 v-value-presence usage binary-long unsigned.
 03 v-value-value usage binary-long unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-annotation-value-choice-v1 based.
 02 v-kind.
 03 v-kind-data usage pointer.
 03 v-kind-len usage binary-double unsigned.
 02 v-value.
 03 v-value-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value-value.
 04 v-value-value-data usage pointer.
 04 v-value-value-len usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-annotation-value-score-v1 based.
 02 v-kind.
 03 v-kind-data usage pointer.
 03 v-kind-len usage binary-double unsigned.
 02 v-value usage float-long.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-annotation-value-tags-v1 based.
 02 v-kind.
 03 v-kind-data usage pointer.
 03 v-kind-len usage binary-double unsigned.
 02 v-value.
 03 v-value-data usage pointer.
 03 v-value-len usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-annotation-value-failed-v1 based.
 02 v-kind.
 03 v-kind-data usage pointer.
 03 v-kind-len usage binary-double unsigned.
 02 v-value usage pointer.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-annotation-value-v1 based.
 02 v-kind usage binary-long unsigned.
 02 filler pic x(4).
 02 v-data.
 03 v-data-union-storage pic x(8).
 03 v-data-decision redefines v-data-union-storage usage pointer.
 03 v-data-choice redefines v-data-union-storage usage pointer.
 03 v-data-score redefines v-data-union-storage usage pointer.
 03 v-data-tags redefines v-data-union-storage usage pointer.
 03 v-data-failed redefines v-data-union-storage usage pointer.
01 tt-n-complete-session-annotation-v1 based.
 02 v-name.
 03 v-name-data usage pointer.
 03 v-name-len usage binary-double unsigned.
 02 v-value usage pointer.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-session-annotation-list-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-session-observed-row-annotated-v1 based.
 02 v-kind.
 03 v-kind-data usage pointer.
 03 v-kind-len usage binary-double unsigned.
 02 v-value.
 03 v-value-data usage pointer.
 03 v-value-len usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-boundary-proposal-list-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-value-data usage pointer.
 03 v-value-len usage binary-double unsigned.
01 tt-n-complete-recognition-edge-v1 based.
 02 v-either usage binary-long unsigned.
 02 filler pic x(4).
 02 v-probability usage float-long.
 02 v-relation.
 03 v-relation-data usage pointer.
 03 v-relation-len usage binary-double unsigned.
 02 v-source usage pointer.
 02 v-target usage pointer.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-recognition-edge-list-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-recognition-edge-list-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-value-data usage pointer.
 03 v-value-len usage binary-double unsigned.
01 tt-n-complete-session-recognition-v1 based.
 02 v-entities.
 03 v-entities-data usage pointer.
 03 v-entities-len usage binary-double unsigned.
 02 v-mode usage pointer.
 02 v-proposals.
 03 v-proposals-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-proposals-value.
 04 v-proposals-value-data usage pointer.
 04 v-proposals-value-len usage binary-double unsigned.
 02 v-relations.
 03 v-relations-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-relations-value.
 04 v-relations-value-data usage pointer.
 04 v-relations-value-len usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-session-observed-row-recognized-v1 based.
 02 v-kind.
 03 v-kind-data usage pointer.
 03 v-kind-len usage binary-double unsigned.
 02 v-value usage pointer.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-session-observed-row-find-v1 based.
 02 v-kind.
 03 v-kind-data usage pointer.
 03 v-kind-len usage binary-double unsigned.
 02 v-value.
 03 v-value-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value-value usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-entity-document-v1 based.
 02 v-kind.
 03 v-kind-data usage pointer.
 03 v-kind-len usage binary-double unsigned.
 02 v-name.
 03 v-name-data usage pointer.
 03 v-name-len usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-session-relation-edge-v1 based.
 02 v-either usage binary-long unsigned.
 02 filler pic x(4).
 02 v-probability usage float-long.
 02 v-relation.
 03 v-relation-data usage pointer.
 03 v-relation-len usage binary-double unsigned.
 02 v-source usage pointer.
 02 v-target usage pointer.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-session-relation-edge-list-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-session-observed-row-relations-v1 based.
 02 v-kind.
 03 v-kind-data usage pointer.
 03 v-kind-len usage binary-double unsigned.
 02 v-value.
 03 v-value-data usage pointer.
 03 v-value-len usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-session-observed-row-v1 based.
 02 v-kind usage binary-long unsigned.
 02 filler pic x(4).
 02 v-data.
 03 v-data-union-storage pic x(8).
 03 v-data-judgment redefines v-data-union-storage usage pointer.
 03 v-data-annotated redefines v-data-union-storage usage pointer.
 03 v-data-recognized redefines v-data-union-storage usage pointer.
 03 v-data-find redefines v-data-union-storage usage pointer.
 03 v-data-relations redefines v-data-union-storage usage pointer.
01 tt-n-complete-session-observation-row-v1 based.
 02 v-index usage binary-double unsigned.
 02 v-kind.
 03 v-kind-data usage pointer.
 03 v-kind-len usage binary-double unsigned.
 02 v-value usage pointer.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-session-observation-v1 based.
 02 v-kind usage binary-long unsigned.
 02 filler pic x(4).
 02 v-data.
 03 v-data-union-storage pic x(8).
 03 v-data-question redefines v-data-union-storage usage pointer.
 03 v-data-row redefines v-data-union-storage usage pointer.
01 tt-n-complete-session-packet-observation-v1 based.
 02 v-function usage pointer.
 02 v-kind.
 03 v-kind-data usage pointer.
 03 v-kind-len usage binary-double unsigned.
 02 v-value usage pointer.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-facts-v1 based.
 02 v-attempts.
 03 v-attempts-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-attempts-value.
 04 v-attempts-value-data usage pointer.
 04 v-attempts-value-len usage binary-double unsigned.
 02 v-cache-answers usage binary-double unsigned.
 02 v-call-id usage pointer.
 02 v-estimated-cost-usd.
 03 v-estimated-cost-usd-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-estimated-cost-usd-value.
 04 v-estimated-cost-usd-value-data usage pointer.
 04 v-estimated-cost-usd-value-len usage binary-double unsigned.
 02 v-held-model-mismatch.
 03 v-held-model-mismatch-presence usage binary-long unsigned.
 03 v-held-model-mismatch-value usage binary-long unsigned.
 02 v-input-tokens.
 03 v-input-tokens-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-input-tokens-value usage binary-double unsigned.
 02 v-largest-request-bytes usage binary-double unsigned.
 02 v-largest-request-estimated-input-tokens.
 03 v-largest-request-estimated-input-tokens-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-largest-request-estimated-input-tokens-value usage binary-double unsigned.
 02 v-model.
 03 v-model-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-model-value.
 04 v-model-value-data usage pointer.
 04 v-model-value-len usage binary-double unsigned.
 02 v-output-tokens.
 03 v-output-tokens-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-output-tokens-value usage binary-double unsigned.
 02 v-records usage binary-double unsigned.
 02 v-requests-sent usage binary-double unsigned.
 02 v-seconds usage float-long.
 02 v-token-estimate-method.
 03 v-token-estimate-method-data usage pointer.
 03 v-token-estimate-method-len usage binary-double unsigned.
 02 v-usage-persistence.
 03 v-usage-persistence-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-usage-persistence-value usage pointer.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-facts-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-input-denial-initial-request-v1 based.
 02 v-kind.
 03 v-kind-data usage pointer.
 03 v-kind-len usage binary-double unsigned.
 02 v-limit usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-input-denial-additional-request-v1 based.
 02 v-kind.
 03 v-kind-data usage pointer.
 03 v-kind-len usage binary-double unsigned.
 02 v-limit usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-input-denial-retry-v1 based.
 02 v-kind.
 03 v-kind-data usage pointer.
 03 v-kind-len usage binary-double unsigned.
 02 v-last-status usage binary-double unsigned.
 02 v-limit usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-input-denial-v1 based.
 02 v-kind usage binary-long unsigned.
 02 filler pic x(4).
 02 v-data.
 03 v-data-union-storage pic x(8).
 03 v-data-initial-request redefines v-data-union-storage usage pointer.
 03 v-data-additional-request redefines v-data-union-storage usage pointer.
 03 v-data-retry redefines v-data-union-storage usage pointer.
01 tt-n-complete-input-denial-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-failure-kind-v1 based.
 02 v-kind usage binary-long unsigned.
01 tt-n-complete-send-denial-before-first-send-v1 based.
 02 v-kind.
 03 v-kind-data usage pointer.
 03 v-kind-len usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-send-denial-before-additional-send-v1 based.
 02 v-kind.
 03 v-kind-data usage pointer.
 03 v-kind-len usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-send-denial-before-retry-v1 based.
 02 v-kind.
 03 v-kind-data usage pointer.
 03 v-kind-len usage binary-double unsigned.
 02 v-last-status usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-send-denial-v1 based.
 02 v-kind usage binary-long unsigned.
 02 filler pic x(4).
 02 v-data.
 03 v-data-union-storage pic x(8).
 03 v-data-before-first-send redefines v-data-union-storage usage pointer.
 03 v-data-before-additional-send redefines v-data-union-storage usage pointer.
 03 v-data-before-retry redefines v-data-union-storage usage pointer.
01 tt-n-complete-send-denial-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-stop-cause-v1 based.
 02 v-kind usage binary-long unsigned.
01 tt-n-complete-stopped-v1 based.
 02 v-at.
 03 v-at-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-at-value usage binary-double unsigned.
 02 v-cause usage pointer.
 02 v-retryable usage binary-long unsigned.
 02 filler pic x(4).
 02 v-status.
 03 v-status-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-status-value usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-error-v1 based.
 02 v-estimated-input-denial.
 03 v-estimated-input-denial-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-estimated-input-denial-value usage pointer.
 02 v-kind usage pointer.
 02 v-message.
 03 v-message-data usage pointer.
 03 v-message-len usage binary-double unsigned.
 02 v-retryable usage binary-long unsigned.
 02 filler pic x(4).
 02 v-send-budget-denial.
 03 v-send-budget-denial-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-send-budget-denial-value usage pointer.
 02 v-stopped usage pointer.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-call-error-v1 based.
 02 v-error usage pointer.
 02 v-facts.
 03 v-facts-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-facts-value usage pointer.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-call-error-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-session-packet-terminal-v1 based.
 02 v-facts.
 03 v-facts-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-facts-value usage pointer.
 02 v-failure.
 03 v-failure-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-failure-value usage pointer.
 02 v-kind.
 03 v-kind-data usage pointer.
 03 v-kind-len usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-complete-session-packet-v1 based.
 02 v-kind usage binary-long unsigned.
 02 filler pic x(4).
 02 v-data.
 03 v-data-union-storage pic x(8).
 03 v-data-decide-row redefines v-data-union-storage usage pointer.
 03 v-data-choose-row redefines v-data-union-storage usage pointer.
 03 v-data-tag-row redefines v-data-union-storage usage pointer.
 03 v-data-score-row redefines v-data-union-storage usage pointer.
 03 v-data-filter-row redefines v-data-union-storage usage pointer.
 03 v-data-annotate-row redefines v-data-union-storage usage pointer.
 03 v-data-decide-aggregate redefines v-data-union-storage usage pointer.
 03 v-data-choose-aggregate redefines v-data-union-storage usage pointer.
 03 v-data-tag-aggregate redefines v-data-union-storage usage pointer.
 03 v-data-score-aggregate redefines v-data-union-storage usage pointer.
 03 v-data-filter-aggregate redefines v-data-union-storage usage pointer.
 03 v-data-rank-aggregate redefines v-data-union-storage usage pointer.
 03 v-data-find-aggregate redefines v-data-union-storage usage pointer.
 03 v-data-annotate-aggregate redefines v-data-union-storage usage pointer.
 03 v-data-recognize-aggregate redefines v-data-union-storage usage pointer.
 03 v-data-relate-aggregate redefines v-data-union-storage usage pointer.
 03 v-data-observation redefines v-data-union-storage usage pointer.
 03 v-data-terminal redefines v-data-union-storage usage pointer.
01 tt-n-cobol-text based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-AnnotateDefinition based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-batch usage pointer.
 02 v-m-profile usage pointer.
 02 v-m-questions usage pointer.
 02 v-m-threshold usage pointer.
 02 v-m-version usage pointer.
01 tt-n-cobol-AnnotateQuestion based.
 02 v-kind usage binary-double unsigned.
 02 v-value usage pointer.
01 tt-n-cobol-AnnotateQuestionChoose based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-choose usage pointer.
 02 v-m-context-schema usage pointer.
 02 v-m-item-schema usage pointer.
 02 v-m-name usage pointer.
 02 v-m-on usage pointer.
 02 v-m-options usage pointer.
 02 v-m-threshold usage pointer.
 02 v-m-wording-version usage pointer.
01 tt-n-cobol-AnnotateQuestionDecide based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-context-schema usage pointer.
 02 v-m-decide usage pointer.
 02 v-m-false usage pointer.
 02 v-m-item-schema usage pointer.
 02 v-m-name usage pointer.
 02 v-m-on usage pointer.
 02 v-m-threshold usage pointer.
 02 v-m-true usage pointer.
 02 v-m-wording-version usage pointer.
01 tt-n-cobol-AnnotateQuestionScore based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-context-schema usage pointer.
 02 v-m-item-schema usage pointer.
 02 v-m-levels usage pointer.
 02 v-m-name usage pointer.
 02 v-m-on usage pointer.
 02 v-m-score usage pointer.
 02 v-m-wording-version usage pointer.
01 tt-n-cobol-AnnotateQuestionTag based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-context-schema usage pointer.
 02 v-m-item-schema usage pointer.
 02 v-m-labels usage pointer.
 02 v-m-name usage pointer.
 02 v-m-on usage pointer.
 02 v-m-tag usage pointer.
 02 v-m-threshold usage pointer.
 02 v-m-wording-version usage pointer.
01 tt-n-cobol-Choose based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-batch usage pointer.
 02 v-m-choose usage pointer.
 02 v-m-context-schema usage pointer.
 02 v-m-item-schema usage pointer.
 02 v-m-model usage pointer.
 02 v-m-name usage pointer.
 02 v-m-on usage pointer.
 02 v-m-options usage pointer.
 02 v-m-profile usage pointer.
 02 v-m-threshold usage pointer.
 02 v-m-wording-version usage pointer.
01 tt-n-cobol-Criterion based.
 02 v-kind usage binary-double unsigned.
 02 v-value usage pointer.
01 tt-n-cobol-Cut based.
 02 v-kind usage binary-double unsigned.
 02 v-value usage pointer.
01 tt-n-cobol-Decide based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-batch usage pointer.
 02 v-m-context-schema usage pointer.
 02 v-m-decide usage pointer.
 02 v-m-false usage pointer.
 02 v-m-item-schema usage pointer.
 02 v-m-model usage pointer.
 02 v-m-name usage pointer.
 02 v-m-on usage pointer.
 02 v-m-profile usage pointer.
 02 v-m-threshold usage pointer.
 02 v-m-true usage pointer.
 02 v-m-wording-version usage pointer.
01 tt-n-cobol-Description based.
 02 v-kind usage binary-double unsigned.
 02 v-value usage pointer.
01 tt-n-cobol-Find based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-context-schema usage pointer.
 02 v-m-find usage pointer.
 02 v-m-item-schema usage pointer.
 02 v-m-model usage pointer.
 02 v-m-name usage pointer.
 02 v-m-on usage pointer.
 02 v-m-profile usage pointer.
 02 v-m-wording-version usage pointer.
01 tt-n-cobol-InputDeclaration based.
 02 v-kind usage binary-double unsigned.
 02 v-value usage pointer.
01 tt-n-cobol-InputDeclarationObject based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-properties usage pointer.
 02 v-m-required usage pointer.
 02 v-m-type usage pointer.
01 tt-n-cobol-InputDeclarationString based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-type usage pointer.
01 tt-n-cobol-InputProperty based.
 02 v-kind usage binary-double unsigned.
 02 v-value usage pointer.
01 tt-n-cobol-InputPropertyArray based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-items usage pointer.
 02 v-m-type usage pointer.
01 tt-n-cobol-InputPropertyBoolean based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-type usage pointer.
01 tt-n-cobol-InputPropertyNumber based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-type usage pointer.
01 tt-n-cobol-InputPropertyString based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-type usage pointer.
01 tt-n-cobol-Labels based.
 02 v-kind usage binary-double unsigned.
 02 v-value usage pointer.
01 tt-n-cobol-Levels based.
 02 v-kind usage binary-double unsigned.
 02 v-value usage pointer.
01 tt-n-cobol-Name based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-Options based.
 02 v-kind usage binary-double unsigned.
 02 v-value usage pointer.
01 tt-n-cobol-Pointers based.
 02 v-kind usage binary-double unsigned.
 02 v-value usage pointer.
01 tt-n-cobol-Profile based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-QuestionBatch based.
 02 v-kind usage binary-double unsigned.
 02 v-value usage pointer.
01 tt-n-cobol-QuestionText based.
 02 v-kind usage binary-double unsigned.
 02 v-value usage pointer.
01 tt-n-cobol-Relate based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-context-schema usage pointer.
 02 v-m-item-schema usage pointer.
 02 v-m-model usage pointer.
 02 v-m-name usage pointer.
 02 v-m-profile usage pointer.
 02 v-m-relate usage pointer.
 02 v-m-threshold usage pointer.
 02 v-m-version usage pointer.
 02 v-m-wording-version usage pointer.
01 tt-n-cobol-Relation based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-either usage pointer.
 02 v-m-name usage pointer.
 02 v-m-reads usage pointer.
 02 v-m-single usage pointer.
 02 v-m-source usage pointer.
 02 v-m-target usage pointer.
01 tt-n-cobol-Score based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-batch usage pointer.
 02 v-m-context-schema usage pointer.
 02 v-m-item-schema usage pointer.
 02 v-m-levels usage pointer.
 02 v-m-model usage pointer.
 02 v-m-name usage pointer.
 02 v-m-on usage pointer.
 02 v-m-profile usage pointer.
 02 v-m-score usage pointer.
 02 v-m-wording-version usage pointer.
01 tt-n-cobol-Tag based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-batch usage pointer.
 02 v-m-context-schema usage pointer.
 02 v-m-item-schema usage pointer.
 02 v-m-labels usage pointer.
 02 v-m-model usage pointer.
 02 v-m-name usage pointer.
 02 v-m-on usage pointer.
 02 v-m-profile usage pointer.
 02 v-m-tag usage pointer.
 02 v-m-threshold usage pointer.
 02 v-m-wording-version usage pointer.
01 tt-n-cobol-Threshold based.
 02 v-kind usage binary-double unsigned.
 02 v-value usage pointer.
01 tt-n-cobol-CacheDocument based.
 02 v-kind usage binary-double unsigned.
 02 v-value usage pointer.
01 tt-n-cobol-ContextSchema based.
 02 v-kind usage binary-double unsigned.
 02 v-value usage pointer.
01 tt-n-cobol-DisabledCache based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-Settings based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-backend usage pointer.
 02 v-m-base-url usage pointer.
 02 v-m-batch usage pointer.
 02 v-m-cache usage pointer.
 02 v-m-max-estimated-input-tokens-total usage pointer.
 02 v-m-max-request-bytes usage pointer.
 02 v-m-max-requests usage pointer.
 02 v-m-max-requests-total usage pointer.
 02 v-m-max-retries usage pointer.
 02 v-m-model usage pointer.
 02 v-m-profile usage pointer.
 02 v-m-proxy usage pointer.
 02 v-m-record usage pointer.
 02 v-m-refresh-cache usage pointer.
 02 v-m-replay usage pointer.
 02 v-m-throttle usage pointer.
 02 v-m-timeout usage pointer.
 02 v-m-usd-per-million-input usage pointer.
 02 v-m-usd-per-million-output usage pointer.
01 tt-n-cobol-ImageMedia based.
 02 v-kind usage binary-double unsigned.
 02 v-value usage pointer.
01 tt-n-cobol-OptionSchema based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-description usage pointer.
 02 v-m-name usage pointer.
01 tt-n-cobol-ReaderMedia based.
 02 v-kind usage binary-double unsigned.
 02 v-value usage pointer.
01 tt-n-cobol-RecognitionExample based.
 02 v-kind usage binary-double unsigned.
 02 v-value usage pointer.
01 tt-n-cobol-RecognitionExampleEntity based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-end usage pointer.
 02 v-m-kind usage pointer.
 02 v-m-start usage pointer.
01 tt-n-cobol-RecognitionExampleText based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-entities usage pointer.
 02 v-m-kinds usage pointer.
 02 v-m-text usage pointer.
01 tt-n-cobol-RecognitionMode based.
 02 v-kind usage binary-double unsigned.
 02 v-value usage pointer.
01 tt-n-cobol-RecognitionSeedSpan based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-end usage pointer.
 02 v-m-kind usage pointer.
 02 v-m-start usage pointer.
01 tt-n-cobol-RecognitionStageContext based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-boundary usage pointer.
 02 v-m-kind-edge usage pointer.
 02 v-m-relation usage pointer.
01 tt-n-cobol-RecognizeDefinition based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-context-schema usage pointer.
 02 v-m-item-schema usage pointer.
 02 v-m-model usage pointer.
 02 v-m-name usage pointer.
 02 v-m-on usage pointer.
 02 v-m-profile usage pointer.
 02 v-m-recognize usage pointer.
 02 v-m-relation-threshold usage pointer.
 02 v-m-threshold usage pointer.
 02 v-m-version usage pointer.
 02 v-m-wording-version usage pointer.
01 tt-n-cobol-RecognizeQuestion based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-entity-definition usage pointer.
 02 v-m-instructions usage pointer.
 02 v-m-kinds usage pointer.
 02 v-m-mode usage pointer.
 02 v-m-relations usage pointer.
 02 v-m-snippet-pieces usage pointer.
 02 v-m-stage-context usage pointer.
01 tt-n-cobol-RelateFields based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-kind usage pointer.
 02 v-m-name usage pointer.
01 tt-n-cobol-RelateQuestion based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-fields usage pointer.
 02 v-m-relations usage pointer.
01 tt-n-cobol-Request based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-call usage pointer.
 02 v-m-schema usage pointer.
01 tt-n-cobol-RequestBatch based.
 02 v-kind usage binary-double unsigned.
 02 v-value usage pointer.
01 tt-n-cobol-RequestCall based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-function usage pointer.
 02 v-m-input usage pointer.
 02 v-m-options usage pointer.
 02 v-m-question usage pointer.
01 tt-n-cobol-RequestDefinition based.
 02 v-kind usage binary-double unsigned.
 02 v-value usage pointer.
01 tt-n-cobol-DefinitionAnnotateDefinition based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-batch usage pointer.
 02 v-m-profile usage pointer.
 02 v-m-questions usage pointer.
 02 v-m-threshold usage pointer.
 02 v-m-version usage pointer.
01 tt-n-cobol-DefinitionChoose based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-batch usage pointer.
 02 v-m-choose usage pointer.
 02 v-m-context-schema usage pointer.
 02 v-m-item-schema usage pointer.
 02 v-m-model usage pointer.
 02 v-m-name usage pointer.
 02 v-m-on usage pointer.
 02 v-m-options usage pointer.
 02 v-m-profile usage pointer.
 02 v-m-threshold usage pointer.
 02 v-m-wording-version usage pointer.
01 tt-n-cobol-DefinitionDecide based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-batch usage pointer.
 02 v-m-context-schema usage pointer.
 02 v-m-decide usage pointer.
 02 v-m-false usage pointer.
 02 v-m-item-schema usage pointer.
 02 v-m-model usage pointer.
 02 v-m-name usage pointer.
 02 v-m-on usage pointer.
 02 v-m-profile usage pointer.
 02 v-m-threshold usage pointer.
 02 v-m-true usage pointer.
 02 v-m-wording-version usage pointer.
01 tt-n-cobol-DefinitionFind based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-context-schema usage pointer.
 02 v-m-find usage pointer.
 02 v-m-item-schema usage pointer.
 02 v-m-model usage pointer.
 02 v-m-name usage pointer.
 02 v-m-on usage pointer.
 02 v-m-profile usage pointer.
 02 v-m-wording-version usage pointer.
01 tt-n-cobol-DefinitionRelate based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-context-schema usage pointer.
 02 v-m-item-schema usage pointer.
 02 v-m-model usage pointer.
 02 v-m-name usage pointer.
 02 v-m-profile usage pointer.
 02 v-m-relate usage pointer.
 02 v-m-threshold usage pointer.
 02 v-m-version usage pointer.
 02 v-m-wording-version usage pointer.
01 tt-n-cobol-DefinitionScore based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-batch usage pointer.
 02 v-m-context-schema usage pointer.
 02 v-m-item-schema usage pointer.
 02 v-m-levels usage pointer.
 02 v-m-model usage pointer.
 02 v-m-name usage pointer.
 02 v-m-on usage pointer.
 02 v-m-profile usage pointer.
 02 v-m-score usage pointer.
 02 v-m-wording-version usage pointer.
01 tt-n-cobol-DefinitionTag based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-batch usage pointer.
 02 v-m-context-schema usage pointer.
 02 v-m-item-schema usage pointer.
 02 v-m-labels usage pointer.
 02 v-m-model usage pointer.
 02 v-m-name usage pointer.
 02 v-m-on usage pointer.
 02 v-m-profile usage pointer.
 02 v-m-tag usage pointer.
 02 v-m-threshold usage pointer.
 02 v-m-wording-version usage pointer.
01 tt-n-cobol-DefinitionRecognizeDefinition based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-context-schema usage pointer.
 02 v-m-item-schema usage pointer.
 02 v-m-model usage pointer.
 02 v-m-name usage pointer.
 02 v-m-on usage pointer.
 02 v-m-profile usage pointer.
 02 v-m-recognize usage pointer.
 02 v-m-relation-threshold usage pointer.
 02 v-m-threshold usage pointer.
 02 v-m-version usage pointer.
 02 v-m-wording-version usage pointer.
01 tt-n-cobol-RequestFraming based.
 02 v-kind usage binary-double unsigned.
 02 v-value usage pointer.
01 tt-n-cobol-RequestFunction based.
 02 v-kind usage binary-double unsigned.
 02 v-value usage pointer.
01 tt-n-cobol-RequestImage based.
 02 v-kind usage binary-double unsigned.
 02 v-value usage pointer.
01 tt-n-cobol-RequestImageBytes based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-bytes usage pointer.
 02 v-m-kind usage pointer.
 02 v-m-media usage pointer.
01 tt-n-cobol-RequestImageFile based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-kind usage pointer.
 02 v-m-media usage pointer.
 02 v-m-path usage pointer.
01 tt-n-cobol-RequestInput based.
 02 v-kind usage binary-double unsigned.
 02 v-value usage pointer.
01 tt-n-cobol-RequestInputEntities based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-items usage pointer.
 02 v-m-kind usage pointer.
01 tt-n-cobol-RequestInputFeed based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-framing usage pointer.
 02 v-m-images usage pointer.
 02 v-m-kind usage pointer.
 02 v-m-name usage pointer.
 02 v-m-reading usage pointer.
01 tt-n-cobol-RequestInputJson based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-images usage pointer.
 02 v-m-kind usage pointer.
 02 v-m-value usage pointer.
01 tt-n-cobol-RequestInputRecords based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-items usage pointer.
 02 v-m-kind usage pointer.
01 tt-n-cobol-RequestInputSource based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-kind usage pointer.
 02 v-m-source usage pointer.
01 tt-n-cobol-RequestInputText based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-images usage pointer.
 02 v-m-kind usage pointer.
 02 v-m-text usage pointer.
01 tt-n-cobol-RequestInputUnits based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-items usage pointer.
 02 v-m-kind usage pointer.
01 tt-n-cobol-RequestItem based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-context usage pointer.
 02 v-m-examples usage pointer.
 02 v-m-images usage pointer.
 02 v-m-options usage pointer.
 02 v-m-original usage pointer.
 02 v-m-seed-spans usage pointer.
01 tt-n-cobol-RequestOptions based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-attempts usage pointer.
 02 v-m-batch usage pointer.
 02 v-m-context usage pointer.
 02 v-m-context-field usage pointer.
 02 v-m-deadline-ms usage pointer.
 02 v-m-examples usage pointer.
 02 v-m-examples-field usage pointer.
 02 v-m-field usage pointer.
 02 v-m-files-only usage pointer.
 02 v-m-max-requests-total usage pointer.
 02 v-m-mode usage pointer.
 02 v-m-model usage pointer.
 02 v-m-none usage pointer.
 02 v-m-options-field usage pointer.
 02 v-m-relation-threshold usage pointer.
 02 v-m-seed-spans usage pointer.
 02 v-m-seed-spans-field usage pointer.
 02 v-m-snippet-pieces usage pointer.
 02 v-m-stage-context usage pointer.
 02 v-m-threshold usage pointer.
 02 v-m-top usage pointer.
01 tt-n-cobol-RequestOriginal based.
 02 v-kind usage binary-double unsigned.
 02 v-value usage pointer.
01 tt-n-cobol-RequestOriginalJson based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-kind usage pointer.
 02 v-m-value usage pointer.
01 tt-n-cobol-RequestOriginalText based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-kind usage pointer.
 02 v-m-text usage pointer.
01 tt-n-cobol-RequestQuestion based.
 02 v-kind usage binary-double unsigned.
 02 v-value usage pointer.
01 tt-n-cobol-RequestQuestionDefinition based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-kind usage pointer.
 02 v-m-value usage pointer.
01 tt-n-cobol-RequestQuestionFile based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-kind usage pointer.
 02 v-m-path usage pointer.
01 tt-n-cobol-RequestQuestionName based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-kind usage pointer.
 02 v-m-name usage pointer.
01 tt-n-cobol-RequestQuestionReference based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-kind usage pointer.
 02 v-m-reference usage pointer.
01 tt-n-cobol-RequestQuestionText based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-kind usage pointer.
 02 v-m-text usage pointer.
01 tt-n-cobol-RequestReader based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-unit usage pointer.
 02 v-m-window usage pointer.
01 tt-n-cobol-RequestReaderFailure based.
 02 v-kind usage binary-double unsigned.
 02 v-value usage pointer.
01 tt-n-cobol-RequestReaderFailureInvalidInput based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-kind usage pointer.
 02 v-m-location usage pointer.
01 tt-n-cobol-RequestReaderFailureIo based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-kind usage pointer.
 02 v-m-location usage pointer.
01 tt-n-cobol-RequestReaderFailureProducer based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-kind usage pointer.
01 tt-n-cobol-RequestReaderFailureUtf8 based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-kind usage pointer.
 02 v-m-location usage pointer.
01 tt-n-cobol-RequestSessionDescriptor based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-item usage pointer.
 02 v-m-location usage pointer.
01 tt-n-cobol-RequestSource based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-framing usage pointer.
 02 v-m-media usage pointer.
 02 v-m-paths usage pointer.
 02 v-m-reading usage pointer.
01 tt-n-cobol-RequestThreshold based.
 02 v-kind usage binary-double unsigned.
 02 v-value usage pointer.
01 tt-n-cobol-RequestVersion based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-SessionSourceLocation based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-file usage pointer.
 02 v-m-first-line usage pointer.
 02 v-m-last-line usage pointer.
01 tt-n-cobol-SourceUnit based.
 02 v-kind usage binary-double unsigned.
 02 v-value usage pointer.
01 tt-n-cobol-AnnotateDefinitionBatch based.
 02 v-value usage pointer.
01 tt-n-cobol-AnnotateDefinitionQuestions based.
 02 v-keys usage pointer.
 02 v-values usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-AnnotateDefinitionVersion based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-AnnotateQuestionChooseName based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-AnnotateQuestionChooseWordingVersion based.
 02 v-value usage binary-double signed.
01 tt-n-cobol-AnnotateQuestionDecideName based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-AnnotateQuestionDecideWordingVersion based.
 02 v-value usage binary-double signed.
01 tt-n-cobol-AnnotateQuestionScoreName based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-AnnotateQuestionScoreWordingVersion based.
 02 v-value usage binary-double signed.
01 tt-n-cobol-AnnotateQuestionTagName based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-AnnotateQuestionTagWordingVersion based.
 02 v-value usage binary-double signed.
01 tt-n-cobol-ChooseName based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-ChooseWordingVersion based.
 02 v-value usage binary-double signed.
01 tt-n-cobol-CriterionTextValue based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-CriterionObjectValue based.
 02 v-keys usage pointer.
 02 v-values usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-CriterionArrayValue based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-CriterionNullValue based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-CutNumberValue based.
 02 v-value usage float-long.
01 tt-n-cobol-CutTextValue based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-DecideName based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-DecideWordingVersion based.
 02 v-value usage binary-double signed.
01 tt-n-cobol-DescriptionTextValue based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-DescriptionObjectValue based.
 02 v-keys usage pointer.
 02 v-values usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-DescriptionArrayValue based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-DescriptionNullValue based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-FindName based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-FindWordingVersion based.
 02 v-value usage binary-double signed.
01 tt-n-cobol-InputDeclarationObjectProperties based.
 02 v-keys usage pointer.
 02 v-values usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-InputDeclarationObjectRequired based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-InputDeclarationObjectType based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-InputDeclarationStringType based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-InputPropertyArrayItems based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-type usage pointer.
01 tt-n-cobol-InputPropertyArrayType based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-InputPropertyBooleanType based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-InputPropertyNumberType based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-InputPropertyStringType based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-LabelsArrayValue based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-LabelsObjectValue based.
 02 v-keys usage pointer.
 02 v-values usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-LevelsArrayValue based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-LevelsObjectValue based.
 02 v-keys usage pointer.
 02 v-values usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-OptionsArrayValue based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-OptionsObjectValue based.
 02 v-keys usage pointer.
 02 v-values usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-PointersTextValue based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-PointersArrayValue based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-QuestionBatchMaxValue based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-QuestionBatchIntegerValue based.
 02 v-value usage binary-double signed.
01 tt-n-cobol-QuestionTextTextValue based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-QuestionTextObjectValue based.
 02 v-keys usage pointer.
 02 v-values usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-QuestionTextArrayValue based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RelateName based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RelateVersion based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RelateWordingVersion based.
 02 v-value usage binary-double signed.
01 tt-n-cobol-RelationEither based.
 02 v-value usage binary-double unsigned.
01 tt-n-cobol-RelationSingle based.
 02 v-value usage binary-double unsigned.
01 tt-n-cobol-ScoreName based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-ScoreWordingVersion based.
 02 v-value usage binary-double signed.
01 tt-n-cobol-TagName based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-TagWordingVersion based.
 02 v-value usage binary-double signed.
01 tt-n-cobol-ThresholdNumberValue based.
 02 v-value usage float-long.
01 tt-n-cobol-ThresholdTextValue based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-CacheDocumentTextValue based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-ContextSchemaTextValue based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-ContextSchemaObjectValue based.
 02 v-keys usage pointer.
 02 v-values usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-SettingsBackend based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-SettingsBaseUrl based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-SettingsInputTokenBudget based.
 02 v-kind usage binary-double unsigned.
 02 v-value usage pointer.
01 tt-n-cobol-SettingsMaxRequestBytes based.
 02 v-value usage binary-double unsigned.
01 tt-n-cobol-SettingsMaxRequests based.
 02 v-kind usage binary-double unsigned.
 02 v-value usage pointer.
01 tt-n-cobol-SettingsMaxRequestsTotal based.
 02 v-kind usage binary-double unsigned.
 02 v-value usage pointer.
01 tt-n-cobol-SettingsMaxRetries based.
 02 v-value usage binary-double unsigned.
01 tt-n-cobol-SettingsModel based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-SettingsProfile based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-SettingsProxy based.
 02 v-value usage pointer.
01 tt-n-cobol-SettingsRecord based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-SettingsRefreshCache based.
 02 v-value usage binary-double unsigned.
01 tt-n-cobol-SettingsReplay based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-SettingsThrottle based.
 02 v-value usage binary-double unsigned.
01 tt-n-cobol-SettingsTimeout based.
 02 v-value usage binary-double unsigned.
01 tt-n-cobol-SettingsUsdPerMillionInput based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-SettingsUsdPerMillionOutput based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-ImageMediaImageJpegValue based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-ImageMediaImagePngValue based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-OptionSchemaDescription based.
 02 v-value usage pointer.
01 tt-n-cobol-OptionSchemaName based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-ReaderMediaTextValue based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-ReaderMediaImageValue based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RecognitionExampleTextValue based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RecognitionExampleEntityEnd based.
 02 v-value usage binary-double unsigned.
01 tt-n-cobol-RecognitionExampleEntityKind based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RecognitionExampleEntityStart based.
 02 v-value usage binary-double unsigned.
01 tt-n-cobol-RecognitionExampleTextEntities based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RecognitionExampleTextKinds based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RecognitionExampleTextText based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RecognitionModeWholeValue based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RecognitionModeBoundaryOnlyValue based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RecognitionSeedSpanEnd based.
 02 v-value usage binary-double unsigned.
01 tt-n-cobol-RecognitionSeedSpanKind based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RecognitionSeedSpanStart based.
 02 v-value usage binary-double unsigned.
01 tt-n-cobol-RecognitionStageContextBoundary based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RecognitionStageContextKindEdge based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RecognitionStageContextRelation based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RecognizeDefinitionName based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RecognizeDefinitionVersion based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RecognizeDefinitionWordingVersion based.
 02 v-value usage binary-double signed.
01 tt-n-cobol-RecognizeQuestionKinds based.
 02 v-keys usage pointer.
 02 v-values usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RecognizeQuestionRelations based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RecognizeQuestionSnippetPieces based.
 02 v-value usage binary-double unsigned.
01 tt-n-cobol-RelateFieldsKind based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RelateFieldsName based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RelateQuestionRelations based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestBatchIntegerValue based.
 02 v-value usage binary-double unsigned.
01 tt-n-cobol-RequestBatchTextValue based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-DefinitionAnnotateDefinitionBatch based.
 02 v-value usage pointer.
01 tt-n-cobol-DefinitionAnnotateDefinitionQuestions based.
 02 v-keys usage pointer.
 02 v-values usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-DefinitionAnnotateDefinitionVersion based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-DefinitionChooseName based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-DefinitionChooseWordingVersion based.
 02 v-value usage binary-double signed.
01 tt-n-cobol-DefinitionDecideName based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-DefinitionDecideWordingVersion based.
 02 v-value usage binary-double signed.
01 tt-n-cobol-DefinitionFindName based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-DefinitionFindWordingVersion based.
 02 v-value usage binary-double signed.
01 tt-n-cobol-DefinitionRelateName based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-DefinitionRelateVersion based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-DefinitionRelateWordingVersion based.
 02 v-value usage binary-double signed.
01 tt-n-cobol-DefinitionScoreName based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-DefinitionScoreWordingVersion based.
 02 v-value usage binary-double signed.
01 tt-n-cobol-DefinitionTagName based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-DefinitionTagWordingVersion based.
 02 v-value usage binary-double signed.
01 tt-n-cobol-DefinitionRecognizeDefinitionName based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-DefinitionRecognizeDefinitionVersion based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-DefinitionRecognizeDefinitionWordingVersion based.
 02 v-value usage binary-double signed.
01 tt-n-cobol-RequestFramingDocumentValue based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestFramingLinesValue based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestFramingJsonlValue based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestFramingCsvValue based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestFramingTsvValue based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestFunctionDecideValue based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestFunctionChooseValue based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestFunctionTagValue based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestFunctionScoreValue based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestFunctionFilterValue based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestFunctionRankValue based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestFunctionFindValue based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestFunctionAnnotateValue based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestFunctionRecognizeValue based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestFunctionRelateValue based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestImageBytesBytes based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestImageBytesKind based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestImageFileKind based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestImageFilePath based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestInputEntitiesItems based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestInputEntitiesKind based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestInputFeedImages based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestInputFeedKind based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestInputFeedName based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestInputJsonImages based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestInputJsonKind based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestInputJsonValue based.
 02 v-value usage pointer.
01 tt-n-cobol-RequestInputRecordsItems based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestInputRecordsKind based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestInputSourceKind based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestInputTextImages based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestInputTextKind based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestInputTextText based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestInputUnitsItems based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestInputUnitsKind based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestItemExamples based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestItemImages based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestItemOptions based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestItemSeedSpans based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestOptionsAttempts based.
 02 v-value usage binary-double unsigned.
01 tt-n-cobol-RequestOptionsContext based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestOptionsContextField based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestOptionsDeadlineMs based.
 02 v-value usage binary-double signed.
01 tt-n-cobol-RequestOptionsExamples based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestOptionsExamplesField based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestOptionsField based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestOptionsFilesOnly based.
 02 v-value usage binary-double unsigned.
01 tt-n-cobol-RequestOptionsMaxRequestsTotal based.
 02 v-value usage binary-double unsigned.
01 tt-n-cobol-RequestOptionsModel based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestOptionsNone based.
 02 v-value usage binary-double unsigned.
01 tt-n-cobol-RequestOptionsOptionsField based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestOptionsSeedSpans based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestOptionsSeedSpansField based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestOptionsSnippetPieces based.
 02 v-value usage binary-double unsigned.
01 tt-n-cobol-RequestOptionsTop based.
 02 v-value usage binary-double unsigned.
01 tt-n-cobol-RequestOriginalJsonKind based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestOriginalJsonValue based.
 02 v-value usage pointer.
01 tt-n-cobol-RequestOriginalTextKind based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestOriginalTextText based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestQuestionDefinitionKind based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestQuestionFileKind based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestQuestionFilePath based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestQuestionNameKind based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestQuestionNameName based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestQuestionReferenceKind based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestQuestionReferenceReference based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestQuestionTextKind based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestQuestionTextText based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestReaderWindow based.
 02 v-value usage binary-double unsigned.
01 tt-n-cobol-RequestReaderFailureInvalidInputKind based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestReaderFailureIoKind based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestReaderFailureProducerKind based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestReaderFailureUtf8Kind based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestSourcePaths based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestThresholdNumberValue based.
 02 v-value usage float-long.
01 tt-n-cobol-RequestThresholdTextValue based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-SessionSourceLocationFile based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-SessionSourceLocationFirstLine based.
 02 v-value usage binary-double unsigned.
01 tt-n-cobol-SessionSourceLocationLastLine based.
 02 v-value usage binary-double unsigned.
01 tt-n-cobol-SourceUnitLineValue based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-SourceUnitWindowValue based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-SourceUnitFileValue based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-CriterionObjectValueEntry based.
 02 v-value usage pointer.
01 tt-n-cobol-CriterionArrayValueItem based.
 02 v-value usage pointer.
01 tt-n-cobol-DescriptionObjectValueEntry based.
 02 v-value usage pointer.
01 tt-n-cobol-DescriptionArrayValueItem based.
 02 v-value usage pointer.
01 tt-n-cobol-InputDeclarationObjectRequiredItem based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-InputPropertyArrayItemsType based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-PointersArrayValueItem based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-QuestionTextObjectValueEntry based.
 02 v-value usage pointer.
01 tt-n-cobol-QuestionTextArrayValueItem based.
 02 v-value usage pointer.
01 tt-n-cobol-ContextSchemaObjectValueEntry based.
 02 v-value usage pointer.
01 tt-n-cobol-SettingsInputTokenBudgetIntegerValue based.
 02 v-value usage binary-double unsigned.
01 tt-n-cobol-SettingsInputTokenBudgetNullValue based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-SettingsMaxRequestsIntegerValue based.
 02 v-value usage binary-double unsigned.
01 tt-n-cobol-SettingsMaxRequestsNullValue based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-SettingsMaxRequestsTotalIntegerValue based.
 02 v-value usage binary-double unsigned.
01 tt-n-cobol-SettingsMaxRequestsTotalNullValue based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RecognitionExampleTextKindsItem based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestOptionsFieldItem based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestSourcePathsItem based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-AnswerText based.
 02 v-length usage binary-double unsigned.
 02 v-text pic x(8192).
01 tt-n-cobol-AnswerDecision based.
 02 v-present usage binary-double unsigned.
 02 v-value usage pointer.
01 tt-n-cobol-AnswerChoice based.
 02 v-present usage binary-double unsigned.
 02 v-value.
 03 v-value-length usage binary-double unsigned.
 03 v-value-text pic x(8192).
01 tt-n-cobol-AnswerScore based.
 02 v-value usage float-long.
01 tt-n-cobol-AnswerRankIndex based.
 02 v-value usage binary-double unsigned.
01 tt-n-cobol-AnswerFilterOriginal based.
 02 v-original usage pointer.
01 tt-n-cobol-AnswerFindSelection based.
 02 v-present usage binary-double unsigned.
 02 v-index usage binary-double unsigned.
 02 v-original usage pointer.
01 tt-n-cobol-AnswerAnnotationField based.
 02 v-record usage binary-double unsigned.
 02 v-name.
 03 v-name-length usage binary-double unsigned.
 03 v-name-text pic x(8192).
 02 v-kind usage binary-double unsigned.
 02 v-boolean usage binary-double unsigned.
 02 v-number usage float-long.
 02 v-text.
 03 v-text-length usage binary-double unsigned.
 03 v-text-text pic x(8192).
 02 v-tags usage pointer.
 02 v-tag-count usage binary-double unsigned.
 02 v-failure-kind usage binary-double unsigned.
 02 v-failure-cause usage binary-double unsigned.
01 tt-n-cobol-AnswerRecognition based.
 02 v-record usage binary-double unsigned.
 02 v-proposal usage binary-double unsigned.
 02 v-kind.
 03 v-kind-length usage binary-double unsigned.
 03 v-kind-text pic x(8192).
 02 v-text.
 03 v-text-length usage binary-double unsigned.
 03 v-text-text pic x(8192).
 02 v-start usage binary-double unsigned.
 02 v-end usage binary-double unsigned.
 02 v-length usage binary-double unsigned.
 02 v-strength usage float-long.
 02 v-mode usage binary-double unsigned.
 02 v-file-present usage binary-double unsigned.
 02 v-file.
 03 v-file-length usage binary-double unsigned.
 03 v-file-text pic x(8192).
 02 v-first-line-present usage binary-double unsigned.
 02 v-first-line usage binary-double unsigned.
 02 v-last-line-present usage binary-double unsigned.
 02 v-last-line usage binary-double unsigned.
 02 v-extensions usage pointer.
01 tt-n-cobol-AnswerRecognitionEdge based.
 02 v-record usage binary-double unsigned.
 02 v-either-present usage binary-double unsigned.
 02 v-either usage binary-double unsigned.
 02 v-probability usage float-long.
 02 v-relation.
 03 v-relation-length usage binary-double unsigned.
 03 v-relation-text pic x(8192).
 02 v-source.
 03 v-source-record usage binary-double unsigned.
 03 v-source-proposal usage binary-double unsigned.
 03 v-source-kind.
 04 v-source-kind-length usage binary-double unsigned.
 04 v-source-kind-text pic x(8192).
 03 v-source-text.
 04 v-source-text-length usage binary-double unsigned.
 04 v-source-text-text pic x(8192).
 03 v-source-start usage binary-double unsigned.
 03 v-source-end usage binary-double unsigned.
 03 v-source-length usage binary-double unsigned.
 03 v-source-strength usage float-long.
 03 v-source-mode usage binary-double unsigned.
 03 v-source-file-present usage binary-double unsigned.
 03 v-source-file.
 04 v-source-file-length usage binary-double unsigned.
 04 v-source-file-text pic x(8192).
 03 v-source-first-line-present usage binary-double unsigned.
 03 v-source-first-line usage binary-double unsigned.
 03 v-source-last-line-present usage binary-double unsigned.
 03 v-source-last-line usage binary-double unsigned.
 03 v-source-extensions usage pointer.
 02 v-target.
 03 v-target-record usage binary-double unsigned.
 03 v-target-proposal usage binary-double unsigned.
 03 v-target-kind.
 04 v-target-kind-length usage binary-double unsigned.
 04 v-target-kind-text pic x(8192).
 03 v-target-text.
 04 v-target-text-length usage binary-double unsigned.
 04 v-target-text-text pic x(8192).
 03 v-target-start usage binary-double unsigned.
 03 v-target-end usage binary-double unsigned.
 03 v-target-length usage binary-double unsigned.
 03 v-target-strength usage float-long.
 03 v-target-mode usage binary-double unsigned.
 03 v-target-file-present usage binary-double unsigned.
 03 v-target-file.
 04 v-target-file-length usage binary-double unsigned.
 04 v-target-file-text pic x(8192).
 03 v-target-first-line-present usage binary-double unsigned.
 03 v-target-first-line usage binary-double unsigned.
 03 v-target-last-line-present usage binary-double unsigned.
 03 v-target-last-line usage binary-double unsigned.
 03 v-target-extensions usage pointer.
 02 v-extensions usage pointer.
01 tt-n-cobol-AnswerRelation based.
 02 v-record usage binary-double unsigned.
 02 v-relation.
 03 v-relation-length usage binary-double unsigned.
 03 v-relation-text pic x(8192).
 02 v-source.
 03 v-source-length usage binary-double unsigned.
 03 v-source-text pic x(8192).
 02 v-source-kind.
 03 v-source-kind-length usage binary-double unsigned.
 03 v-source-kind-text pic x(8192).
 02 v-target.
 03 v-target-length usage binary-double unsigned.
 03 v-target-text pic x(8192).
 02 v-target-kind.
 03 v-target-kind-length usage binary-double unsigned.
 03 v-target-kind-text pic x(8192).
 02 v-probability usage float-long.
 02 v-either-present usage binary-double unsigned.
 02 v-either usage binary-double unsigned.
01 tt-n-cobol-AnswerError based.
 02 v-estimated-input-denial.
 03 v-estimated-input-denial-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-estimated-input-denial-value usage pointer.
 02 v-kind usage pointer.
 02 v-message.
 03 v-message-length usage binary-double unsigned.
 03 v-message-text pic x(8192).
 02 v-retryable usage binary-long unsigned.
 02 filler pic x(4).
 02 v-send-budget-denial.
 03 v-send-budget-denial-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-send-budget-denial-value usage pointer.
 02 v-stopped usage pointer.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-cobol-AnswerExtensions based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-AnswerExtension based.
 02 v-name.
 03 v-name-length usage binary-double unsigned.
 03 v-name-text pic x(8192).
 02 v-value usage pointer.
01 tt-n-cobol-AnswerStopped based.
 02 v-at.
 03 v-at-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-at-value usage binary-double unsigned.
 02 v-cause usage pointer.
 02 v-retryable usage binary-long unsigned.
 02 filler pic x(4).
 02 v-status.
 03 v-status-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-status-value usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-cobol-AnswerU64Presence based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage binary-double unsigned.
01 tt-n-cobol-AnswerStopCause based.
 02 v-kind usage binary-long unsigned.
01 tt-n-cobol-AnswerSendDenialPresence based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-cobol-AnswerSendDenial based.
 02 v-kind usage binary-long unsigned.
 02 filler pic x(4).
 02 v-data.
 03 v-data-before-first-send usage pointer.
 03 v-data-before-additional-send usage pointer.
 03 v-data-before-retry usage pointer.
01 tt-n-cobol-AnswerSendDenialData based.
 02 v-before-first-send usage pointer.
 02 v-before-additional-send usage pointer.
 02 v-before-retry usage pointer.
01 tt-n-cobol-AnswerSendDenialBeforeRetry based.
 02 v-kind.
 03 v-kind-length usage binary-double unsigned.
 03 v-kind-text pic x(8192).
 02 v-last-status usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-cobol-AnswerSendDenialBeforeAdditionalSend based.
 02 v-kind.
 03 v-kind-length usage binary-double unsigned.
 03 v-kind-text pic x(8192).
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-cobol-AnswerSendDenialBeforeFirstSend based.
 02 v-kind.
 03 v-kind-length usage binary-double unsigned.
 03 v-kind-text pic x(8192).
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-cobol-AnswerFailureKind based.
 02 v-kind usage binary-long unsigned.
01 tt-n-cobol-AnswerInputDenialPresence based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-cobol-AnswerInputDenial based.
 02 v-kind usage binary-long unsigned.
 02 filler pic x(4).
 02 v-data.
 03 v-data-initial-request usage pointer.
 03 v-data-additional-request usage pointer.
 03 v-data-retry usage pointer.
01 tt-n-cobol-AnswerInputDenialData based.
 02 v-initial-request usage pointer.
 02 v-additional-request usage pointer.
 02 v-retry usage pointer.
01 tt-n-cobol-AnswerInputDenialRetry based.
 02 v-kind.
 03 v-kind-length usage binary-double unsigned.
 03 v-kind-text pic x(8192).
 02 v-last-status usage binary-double unsigned.
 02 v-limit usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-cobol-AnswerInputDenialAdditionalRequest based.
 02 v-kind.
 03 v-kind-length usage binary-double unsigned.
 03 v-kind-text pic x(8192).
 02 v-limit usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-cobol-AnswerInputDenialInitialRequest based.
 02 v-kind.
 03 v-kind-length usage binary-double unsigned.
 03 v-kind-text pic x(8192).
 02 v-limit usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-cobol-AnswerImmediateFacts based.
 02 v-attempts.
 03 v-attempts-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-attempts-value.
 04 v-attempts-value-data usage pointer.
 04 v-attempts-value-len usage binary-double unsigned.
 02 v-cache-answers usage binary-double unsigned.
 02 v-call-id.
 03 v-call-id-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-call-id-value usage pointer.
 02 v-estimated-cost-usd.
 03 v-estimated-cost-usd-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-estimated-cost-usd-value.
 04 v-estimated-cost-usd-value-length usage binary-double unsigned.
 04 v-estimated-cost-usd-value-text pic x(8192).
 02 v-held-model-mismatch.
 03 v-held-model-mismatch-presence usage binary-long unsigned.
 03 v-held-model-mismatch-value usage binary-long unsigned.
 02 v-input-tokens.
 03 v-input-tokens-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-input-tokens-value usage binary-double unsigned.
 02 v-largest-request-bytes usage binary-double unsigned.
 02 v-largest-request-estimated-input-tokens.
 03 v-largest-request-estimated-input-tokens-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-largest-request-estimated-input-tokens-value usage binary-double unsigned.
 02 v-model.
 03 v-model-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-model-value.
 04 v-model-value-length usage binary-double unsigned.
 04 v-model-value-text pic x(8192).
 02 v-output-tokens.
 03 v-output-tokens-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-output-tokens-value usage binary-double unsigned.
 02 v-records usage binary-double unsigned.
 02 v-requests-sent usage binary-double unsigned.
 02 v-seconds usage float-long.
 02 v-token-estimate-method.
 03 v-token-estimate-method-length usage binary-double unsigned.
 03 v-token-estimate-method-text pic x(8192).
 02 v-usage-persistence.
 03 v-usage-persistence-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-usage-persistence-value usage pointer.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-cobol-AnswerPersistenceObservationPresence based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-cobol-AnswerPersistenceObservation based.
 02 v-advice.
 03 v-advice-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-advice-value.
 04 v-advice-value-length usage binary-double unsigned.
 04 v-advice-value-text pic x(8192).
 02 v-observed-at.
 03 v-observed-at-length usage binary-double unsigned.
 03 v-observed-at-text pic x(8192).
 02 v-state usage pointer.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-cobol-AnswerUsagePersistence based.
 02 v-kind usage binary-long unsigned.
01 tt-n-cobol-AnswerUtf8Presence based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-value-length usage binary-double unsigned.
 03 v-value-text pic x(8192).
01 tt-n-cobol-AnswerU32Presence based.
 02 v-presence usage binary-long unsigned.
 02 v-value usage binary-long unsigned.
01 tt-n-cobol-AnswerCallIdPresence based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-cobol-AnswerCallId based.
 02 v-value.
 03 v-value-length usage binary-double unsigned.
 03 v-value-text pic x(8192).
01 tt-n-cobol-AnswerAttemptListPresence based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-value-data usage pointer.
 03 v-value-len usage binary-double unsigned.
01 tt-n-cobol-AnswerAttemptList based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-AnswerAttempt based.
 02 v-ordinal usage binary-double unsigned.
 02 v-outcome usage pointer.
 02 v-request-id.
 03 v-request-id-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-request-id-value.
 04 v-request-id-value-length usage binary-double unsigned.
 04 v-request-id-value-text pic x(8192).
 02 v-request-sha256.
 03 v-request-sha256-length usage binary-double unsigned.
 03 v-request-sha256-text pic x(8192).
 02 v-sdk-request-id usage pointer.
 02 v-server-ms.
 03 v-server-ms-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-server-ms-value usage binary-double unsigned.
 02 v-status.
 03 v-status-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-status-value usage binary-double unsigned.
 02 v-wall-ms usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-cobol-AnswerSdkRequestId based.
 02 v-value.
 03 v-value-length usage binary-double unsigned.
 03 v-value-text pic x(8192).
01 tt-n-cobol-AnswerAttemptOutcome based.
 02 v-kind usage binary-long unsigned.
01 tt-n-cobol-AnswerFacts based.
 02 v-attempts.
 03 v-attempts-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-attempts-value.
 04 v-attempts-value-data usage pointer.
 04 v-attempts-value-len usage binary-double unsigned.
 02 v-cache-answers usage binary-double unsigned.
 02 v-call-id usage pointer.
 02 v-estimated-cost-usd.
 03 v-estimated-cost-usd-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-estimated-cost-usd-value.
 04 v-estimated-cost-usd-value-length usage binary-double unsigned.
 04 v-estimated-cost-usd-value-text pic x(8192).
 02 v-held-model-mismatch.
 03 v-held-model-mismatch-presence usage binary-long unsigned.
 03 v-held-model-mismatch-value usage binary-long unsigned.
 02 v-input-tokens.
 03 v-input-tokens-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-input-tokens-value usage binary-double unsigned.
 02 v-largest-request-bytes usage binary-double unsigned.
 02 v-largest-request-estimated-input-tokens.
 03 v-largest-request-estimated-input-tokens-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-largest-request-estimated-input-tokens-value usage binary-double unsigned.
 02 v-model.
 03 v-model-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-model-value.
 04 v-model-value-length usage binary-double unsigned.
 04 v-model-value-text pic x(8192).
 02 v-output-tokens.
 03 v-output-tokens-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-output-tokens-value usage binary-double unsigned.
 02 v-records usage binary-double unsigned.
 02 v-requests-sent usage binary-double unsigned.
 02 v-seconds usage float-long.
 02 v-token-estimate-method.
 03 v-token-estimate-method-length usage binary-double unsigned.
 03 v-token-estimate-method-text pic x(8192).
 02 v-usage-persistence.
 03 v-usage-persistence-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-usage-persistence-value usage pointer.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-cobol-AnswerStatus based.
 02 v-code usage binary-double signed.
 02 v-retryable usage binary-double unsigned.
 02 v-failure-kind usage binary-double unsigned.
 02 v-message.
 03 v-message-length usage binary-double unsigned.
 03 v-message-text pic x(8192).
 02 v-facts-present usage binary-double unsigned.
 02 v-facts.
 03 v-facts-attempts.
 04 v-facts-attempts-presence usage binary-long unsigned.
 04 filler pic x(4).
 04 v-facts-attempts-value.
 05 v-facts-attempts-value-data usage pointer.
 05 v-facts-attempts-value-len usage binary-double unsigned.
 03 v-facts-cache-answers usage binary-double unsigned.
 03 v-facts-call-id usage pointer.
 03 v-facts-estimated-cost-usd.
 04 v-facts-estimated-cost-usd-presence usage binary-long unsigned.
 04 filler pic x(4).
 04 v-facts-estimated-cost-usd-value.
 05 v-facts-estimated-cost-usd-value-length usage binary-double unsigned.
 05 v-facts-estimated-cost-usd-value-text pic x(8192).
 03 v-facts-held-model-mismatch.
 04 v-facts-held-model-mismatch-presence usage binary-long unsigned.
 04 v-facts-held-model-mismatch-value usage binary-long unsigned.
 03 v-facts-input-tokens.
 04 v-facts-input-tokens-presence usage binary-long unsigned.
 04 filler pic x(4).
 04 v-facts-input-tokens-value usage binary-double unsigned.
 03 v-facts-largest-request-bytes usage binary-double unsigned.
 03 v-facts-largest-request-estimated-input-tokens.
 04 v-facts-largest-request-estimated-input-tokens-presence usage binary-long unsigned.
 04 filler pic x(4).
 04 v-facts-largest-request-estimated-input-tokens-value usage binary-double unsigned.
 03 v-facts-model.
 04 v-facts-model-presence usage binary-long unsigned.
 04 filler pic x(4).
 04 v-facts-model-value.
 05 v-facts-model-value-length usage binary-double unsigned.
 05 v-facts-model-value-text pic x(8192).
 03 v-facts-output-tokens.
 04 v-facts-output-tokens-presence usage binary-long unsigned.
 04 filler pic x(4).
 04 v-facts-output-tokens-value usage binary-double unsigned.
 03 v-facts-records usage binary-double unsigned.
 03 v-facts-requests-sent usage binary-double unsigned.
 03 v-facts-seconds usage float-long.
 03 v-facts-token-estimate-method.
 04 v-facts-token-estimate-method-length usage binary-double unsigned.
 04 v-facts-token-estimate-method-text pic x(8192).
 03 v-facts-usage-persistence.
 04 v-facts-usage-persistence-presence usage binary-long unsigned.
 04 filler pic x(4).
 04 v-facts-usage-persistence-value usage pointer.
 03 v-facts-extensions.
 04 v-facts-extensions-data usage pointer.
 04 v-facts-extensions-len usage binary-double unsigned.
 02 v-immediate-facts usage pointer.
 02 v-details usage pointer.
 02 v-owner usage pointer.
01 tt-n-cobol-AnswerTable based.
 02 v-owner usage pointer.
 02 v-rows usage pointer.
 02 v-count usage binary-double unsigned.
 02 v-edges usage pointer.
 02 v-edge-count usage binary-double unsigned.
01 tt-n-cobol-AnswerNumber based.
 02 v-present usage binary-double unsigned.
 02 v-value usage float-long.
01 tt-n-cobol-AnswerProbabilityList based.
 02 v-data usage pointer.
 02 v-count usage binary-double unsigned.
01 tt-n-cobol-AnswerProbability based.
 02 v-scalar usage binary-double unsigned.
 02 v-first.
 03 v-first-present usage binary-double unsigned.
 03 v-first-value usage float-long.
 02 v-data usage pointer.
 02 v-count usage binary-double unsigned.
 02 v-lists usage pointer.
 02 v-list-count usage binary-double unsigned.
01 tt-n-cobol-ReceiptRelation based.
 02 v-answer usage pointer.
 02 v-answer-id usage pointer.
 02 v-file.
 03 v-file-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-file-value.
 04 v-file-value-length usage binary-double unsigned.
 04 v-file-value-text pic x(8192).
 02 v-first-line.
 03 v-first-line-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-first-line-value usage binary-double unsigned.
 02 v-index.
 03 v-index-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-index-value usage binary-double unsigned.
 02 v-input.
 03 v-input-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-input-value usage pointer.
 02 v-input-sources.
 03 v-input-sources-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-input-sources-value.
 04 v-input-sources-value-data usage pointer.
 04 v-input-sources-value-len usage binary-double unsigned.
 02 v-last-line.
 03 v-last-line-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-last-line-value usage binary-double unsigned.
 02 v-meta usage pointer.
 02 v-position.
 03 v-position-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-position-value usage pointer.
 02 v-question usage pointer.
 02 v-schema usage pointer.
 02 v-value.
 03 v-value-data usage pointer.
 03 v-value-len usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptExtensions based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptExtension based.
 02 v-name.
 03 v-name-length usage binary-double unsigned.
 03 v-name-text pic x(8192).
 02 v-value usage pointer.
01 tt-n-cobol-ReceiptRelatedEntityEdgeList based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptRelatedEntityEdge based.
 02 v-either.
 03 v-either-presence usage binary-long unsigned.
 03 v-either-value usage binary-long unsigned.
 02 v-probability usage float-long.
 02 v-relation.
 03 v-relation-length usage binary-double unsigned.
 03 v-relation-text pic x(8192).
 02 v-source usage pointer.
 02 v-target usage pointer.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptRelationEndpoint based.
 02 v-kind usage binary-long unsigned.
 02 filler pic x(4).
 02 v-data.
 03 v-data-related-entity usage pointer.
 03 v-data-source usage pointer.
01 tt-n-cobol-ReceiptRelationEndpointData based.
 02 v-related-entity usage pointer.
 02 v-source usage pointer.
01 tt-n-cobol-ReceiptRelationEndpointSource based.
 02 v-file.
 03 v-file-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-file-value.
 04 v-file-value-length usage binary-double unsigned.
 04 v-file-value-text pic x(8192).
 02 v-first-line.
 03 v-first-line-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-first-line-value usage binary-double unsigned.
 02 v-kind.
 03 v-kind-length usage binary-double unsigned.
 03 v-kind-text pic x(8192).
 02 v-last-line.
 03 v-last-line-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-last-line-value usage binary-double unsigned.
 02 v-name.
 03 v-name-length usage binary-double unsigned.
 03 v-name-text pic x(8192).
 02 v-ordinal usage binary-double unsigned.
 02 v-record.
 03 v-record-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-record-value usage pointer.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptJsonPresence based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-cobol-ReceiptU64Presence based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage binary-double unsigned.
01 tt-n-cobol-ReceiptUtf8Presence based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-value-length usage binary-double unsigned.
 03 v-value-text pic x(8192).
01 tt-n-cobol-ReceiptRelationEndpointRelatedEntity based.
 02 v-kind.
 03 v-kind-length usage binary-double unsigned.
 03 v-kind-text pic x(8192).
 02 v-name.
 03 v-name-length usage binary-double unsigned.
 03 v-name-text pic x(8192).
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptU32Presence based.
 02 v-presence usage binary-long unsigned.
 02 v-value usage binary-long unsigned.
01 tt-n-cobol-ReceiptVersion based.
 02 v-kind usage binary-long unsigned.
01 tt-n-cobol-ReceiptRelateQuestionDocument based.
 02 v-batch.
 03 v-batch-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-batch-value usage pointer.
 02 v-context-schema.
 03 v-context-schema-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-context-schema-value usage pointer.
 02 v-fields.
 03 v-fields-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-fields-value usage pointer.
 02 v-item-schema.
 03 v-item-schema-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-item-schema-value usage pointer.
 02 v-label-details.
 03 v-label-details-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-label-details-value.
 04 v-label-details-value-data usage pointer.
 04 v-label-details-value-len usage binary-double unsigned.
 02 v-model.
 03 v-model-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-model-value.
 04 v-model-value-length usage binary-double unsigned.
 04 v-model-value-text pic x(8192).
 02 v-name.
 03 v-name-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-name-value usage pointer.
 02 v-on.
 03 v-on-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-on-value.
 04 v-on-value-data usage pointer.
 04 v-on-value-len usage binary-double unsigned.
 02 v-profile.
 03 v-profile-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-profile-value.
 04 v-profile-value-length usage binary-double unsigned.
 04 v-profile-value-text pic x(8192).
 02 v-relations.
 03 v-relations-data usage pointer.
 03 v-relations-len usage binary-double unsigned.
 02 v-threshold usage pointer.
 02 v-verb.
 03 v-verb-length usage binary-double unsigned.
 03 v-verb-text pic x(8192).
 02 v-wording-version.
 03 v-wording-version-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-wording-version-value usage pointer.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptWordingVersionPresence based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-cobol-ReceiptWordingVersion based.
 02 v-value usage binary-double unsigned.
01 tt-n-cobol-ReceiptThreshold based.
 02 v-kind usage binary-long unsigned.
 02 filler pic x(4).
 02 v-data.
 03 v-data-number usage float-long.
 03 v-data-string.
 04 v-data-string-length usage binary-double unsigned.
 04 v-data-string-text pic x(8192).
01 tt-n-cobol-ReceiptThresholdData based.
 02 v-number usage float-long.
 02 v-string.
 03 v-string-length usage binary-double unsigned.
 03 v-string-text pic x(8192).
01 tt-n-cobol-ReceiptRelationRuleList based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptRelationRule based.
 02 v-either usage binary-long unsigned.
 02 filler pic x(4).
 02 v-name.
 03 v-name-length usage binary-double unsigned.
 03 v-name-text pic x(8192).
 02 v-reads.
 03 v-reads-length usage binary-double unsigned.
 03 v-reads-text pic x(8192).
 02 v-single.
 03 v-single-presence usage binary-long unsigned.
 03 v-single-value usage binary-long unsigned.
 02 v-source.
 03 v-source-length usage binary-double unsigned.
 03 v-source-text pic x(8192).
 02 v-target.
 03 v-target-length usage binary-double unsigned.
 03 v-target-text pic x(8192).
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptStringListPresence based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-value-data usage pointer.
 03 v-value-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptStringList based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptQuestionNamePresence based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-cobol-ReceiptQuestionName based.
 02 v-value.
 03 v-value-length usage binary-double unsigned.
 03 v-value-text pic x(8192).
01 tt-n-cobol-ReceiptLabelListPresence based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-value-data usage pointer.
 03 v-value-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptLabelList based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptLabel based.
 02 v-description.
 03 v-description-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-description-value usage pointer.
 02 v-name.
 03 v-name-length usage binary-double unsigned.
 03 v-name-text pic x(8192).
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptInputDeclarationPresence based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-cobol-ReceiptInputDeclaration based.
 02 v-kind usage binary-long unsigned.
 02 filler pic x(4).
 02 v-data.
 03 v-data-string-root usage pointer.
 03 v-data-object-root usage pointer.
01 tt-n-cobol-ReceiptInputDeclarationData based.
 02 v-string-root usage pointer.
 02 v-object-root usage pointer.
01 tt-n-cobol-ReceiptInputDeclarationObjectRoot based.
 02 v-properties.
 03 v-properties-data usage pointer.
 03 v-properties-len usage binary-double unsigned.
 02 v-required.
 03 v-required-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-required-value.
 04 v-required-value-data usage pointer.
 04 v-required-value-len usage binary-double unsigned.
 02 v-type usage pointer.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptObjectType based.
 02 v-kind usage binary-long unsigned.
01 tt-n-cobol-ReceiptInputPropertyTypeMap based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptInputPropertyTypeMapEntry based.
 02 v-name.
 03 v-name-length usage binary-double unsigned.
 03 v-name-text pic x(8192).
 02 v-value usage pointer.
01 tt-n-cobol-ReceiptInputPropertyType based.
 02 v-kind usage binary-long unsigned.
 02 filler pic x(4).
 02 v-data.
 03 v-data-string usage pointer.
 03 v-data-number usage pointer.
 03 v-data-boolean usage pointer.
 03 v-data-array usage pointer.
01 tt-n-cobol-ReceiptInputPropertyTypeData based.
 02 v-string usage pointer.
 02 v-number usage pointer.
 02 v-boolean usage pointer.
 02 v-array usage pointer.
01 tt-n-cobol-ReceiptInputPropertyTypeArray based.
 02 v-items usage pointer.
 02 v-type.
 03 v-type-length usage binary-double unsigned.
 03 v-type-text pic x(8192).
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptStringRoot based.
 02 v-type usage pointer.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptStringType based.
 02 v-kind usage binary-long unsigned.
01 tt-n-cobol-ReceiptInputPropertyTypeBoolean based.
 02 v-type.
 03 v-type-length usage binary-double unsigned.
 03 v-type-text pic x(8192).
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptInputPropertyTypeNumber based.
 02 v-type.
 03 v-type-length usage binary-double unsigned.
 03 v-type-text pic x(8192).
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptInputPropertyTypeString based.
 02 v-type.
 03 v-type-length usage binary-double unsigned.
 03 v-type-text pic x(8192).
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptInputDeclarationStringRoot based.
 02 v-type usage pointer.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptRelateFieldsPresence based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-cobol-ReceiptRelateFields based.
 02 v-kind.
 03 v-kind-length usage binary-double unsigned.
 03 v-kind-text pic x(8192).
 02 v-name.
 03 v-name-length usage binary-double unsigned.
 03 v-name-text pic x(8192).
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptBatchPresence based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-cobol-ReceiptBatch based.
 02 v-kind usage binary-long unsigned.
 02 filler pic x(4).
 02 v-data.
 03 v-data-integer usage binary-double unsigned.
 03 v-data-string usage pointer.
01 tt-n-cobol-ReceiptBatchData based.
 02 v-integer usage binary-double unsigned.
 02 v-string usage pointer.
01 tt-n-cobol-ReceiptBatchString based.
 02 v-kind usage binary-long unsigned.
01 tt-n-cobol-ReceiptPositionPresence based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-cobol-ReceiptPosition based.
 02 v-file.
 03 v-file-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-file-value.
 04 v-file-value-length usage binary-double unsigned.
 04 v-file-value-text pic x(8192).
 02 v-first.
 03 v-first-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-first-value usage binary-double unsigned.
 02 v-images.
 03 v-images-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-images-value.
 04 v-images-value-data usage pointer.
 04 v-images-value-len usage binary-double unsigned.
 02 v-last.
 03 v-last-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-last-value usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptMeta based.
 02 v-answered-by.
 03 v-answered-by-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-answered-by-value.
 04 v-answered-by-value-length usage binary-double unsigned.
 04 v-answered-by-value-text pic x(8192).
 02 v-attempts.
 03 v-attempts-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-attempts-value.
 04 v-attempts-value-data usage pointer.
 04 v-attempts-value-len usage binary-double unsigned.
 02 v-batch-setting.
 03 v-batch-setting-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-batch-setting-value usage pointer.
 02 v-batch-warning.
 03 v-batch-warning-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-batch-warning-value usage pointer.
 02 v-cached usage binary-long unsigned.
 02 filler pic x(4).
 02 v-context-sha256.
 03 v-context-sha256-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-context-sha256-value.
 04 v-context-sha256-value-length usage binary-double unsigned.
 04 v-context-sha256-value-text pic x(8192).
 02 v-failed-questions usage binary-double unsigned.
 02 v-model.
 03 v-model-length usage binary-double unsigned.
 03 v-model-text pic x(8192).
 02 v-observations.
 03 v-observations-data usage pointer.
 03 v-observations-len usage binary-double unsigned.
 02 v-origin.
 03 v-origin-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-origin-value usage pointer.
 02 v-profile-warning.
 03 v-profile-warning-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-profile-warning-value usage pointer.
 02 v-question-sha256.
 03 v-question-sha256-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-question-sha256-value.
 04 v-question-sha256-value-length usage binary-double unsigned.
 04 v-question-sha256-value-text pic x(8192).
 02 v-question-sources.
 03 v-question-sources-data usage pointer.
 03 v-question-sources-len usage binary-double unsigned.
 02 v-questions-sha256.
 03 v-questions-sha256-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-questions-sha256-value.
 04 v-questions-sha256-value-length usage binary-double unsigned.
 04 v-questions-sha256-value-text pic x(8192).
 02 v-requests.
 03 v-requests-data usage pointer.
 03 v-requests-len usage binary-double unsigned.
 02 v-requests-sent usage binary-double unsigned.
 02 v-tool.
 03 v-tool-length usage binary-double unsigned.
 03 v-tool-text pic x(8192).
 02 v-url.
 03 v-url-length usage binary-double unsigned.
 03 v-url-text pic x(8192).
 02 v-usage.
 03 v-usage-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-usage-value usage pointer.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptUsagePresence based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-cobol-ReceiptUsage based.
 02 v-input-tokens.
 03 v-input-tokens-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-input-tokens-value usage binary-double unsigned.
 02 v-output-tokens.
 03 v-output-tokens-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-output-tokens-value usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptQuestionSourceList based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptQuestionSource based.
 02 v-answered-by.
 03 v-answered-by-length usage binary-double unsigned.
 03 v-answered-by-text pic x(8192).
 02 v-batch-size.
 03 v-batch-size-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-batch-size-value usage binary-double unsigned.
 02 v-origin usage pointer.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptOrigin based.
 02 v-kind usage binary-long unsigned.
01 tt-n-cobol-ReceiptProfileWarningPresence based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-cobol-ReceiptProfileWarning based.
 02 v-running.
 03 v-running-length usage binary-double unsigned.
 03 v-running-text pic x(8192).
 02 v-tuned-for.
 03 v-tuned-for-length usage binary-double unsigned.
 03 v-tuned-for-text pic x(8192).
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptOriginPresence based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-cobol-ReceiptObservationList based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptObservation based.
 02 v-kind usage binary-long unsigned.
 02 filler pic x(4).
 02 v-data.
 03 v-data-observation-id usage pointer.
 03 v-data-failure-id usage pointer.
01 tt-n-cobol-ReceiptObservationData based.
 02 v-observation-id usage pointer.
 02 v-failure-id usage pointer.
01 tt-n-cobol-ReceiptObservationFailureId based.
 02 v-failure-id usage pointer.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptFailureId based.
 02 v-value.
 03 v-value-length usage binary-double unsigned.
 03 v-value-text pic x(8192).
01 tt-n-cobol-ReceiptObservationObservationId based.
 02 v-observation-id usage pointer.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptObservationId based.
 02 v-value.
 03 v-value-length usage binary-double unsigned.
 03 v-value-text pic x(8192).
01 tt-n-cobol-ReceiptBatchWarningPresence based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-cobol-ReceiptBatchWarning based.
 02 v-running usage pointer.
 02 v-tuned-for usage pointer.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptBatchSetting based.
 02 v-kind usage binary-long unsigned.
 02 filler pic x(4).
 02 v-data.
 03 v-data-integer usage binary-double unsigned.
 03 v-data-string.
 04 v-data-string-length usage binary-double unsigned.
 04 v-data-string-text pic x(8192).
01 tt-n-cobol-ReceiptBatchSettingData based.
 02 v-integer usage binary-double unsigned.
 02 v-string.
 03 v-string-length usage binary-double unsigned.
 03 v-string-text pic x(8192).
01 tt-n-cobol-ReceiptBatchSettingPresence based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-cobol-ReceiptAttemptListPresence based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-value-data usage pointer.
 03 v-value-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptAttemptList based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptAttempt based.
 02 v-ordinal usage binary-double unsigned.
 02 v-outcome usage pointer.
 02 v-request-id.
 03 v-request-id-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-request-id-value.
 04 v-request-id-value-length usage binary-double unsigned.
 04 v-request-id-value-text pic x(8192).
 02 v-request-sha256.
 03 v-request-sha256-length usage binary-double unsigned.
 03 v-request-sha256-text pic x(8192).
 02 v-sdk-request-id usage pointer.
 02 v-server-ms.
 03 v-server-ms-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-server-ms-value usage binary-double unsigned.
 02 v-status.
 03 v-status-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-status-value usage binary-double unsigned.
 02 v-wall-ms usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptSdkRequestId based.
 02 v-value.
 03 v-value-length usage binary-double unsigned.
 03 v-value-text pic x(8192).
01 tt-n-cobol-ReceiptAttemptOutcome based.
 02 v-kind usage binary-long unsigned.
01 tt-n-cobol-ReceiptSessionInputSourceListPresence based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-value-data usage pointer.
 03 v-value-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptSessionInputSourceList based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptSessionInputSource based.
 02 v-index usage binary-double unsigned.
 02 v-source usage pointer.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptPhysicalSource based.
 02 v-file.
 03 v-file-length usage binary-double unsigned.
 03 v-file-text pic x(8192).
 02 v-first-line.
 03 v-first-line-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-first-line-value usage binary-double unsigned.
 02 v-last-line.
 03 v-last-line-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-last-line-value usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptAnswerId based.
 02 v-value.
 03 v-value-length usage binary-double unsigned.
 03 v-value-text pic x(8192).
01 tt-n-cobol-ReceiptAnswers based.
 02 v-questions.
 03 v-questions-data usage pointer.
 03 v-questions-len usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptRelationMemberList based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptRelationMember based.
 02 v-kind usage binary-long unsigned.
 02 filler pic x(4).
 02 v-data.
 03 v-data-answer-id usage pointer.
 03 v-data-failure-id usage pointer.
01 tt-n-cobol-ReceiptRelationMemberData based.
 02 v-answer-id usage pointer.
 02 v-failure-id usage pointer.
01 tt-n-cobol-ReceiptRelationMemberFailureId based.
 02 v-direction usage pointer.
 02 v-method usage pointer.
 02 v-observations.
 03 v-observations-data usage pointer.
 03 v-observations-len usage binary-double unsigned.
 02 v-question usage pointer.
 02 v-question-sources.
 03 v-question-sources-data usage pointer.
 03 v-question-sources-len usage binary-double unsigned.
 02 v-reads.
 03 v-reads-length usage binary-double unsigned.
 03 v-reads-text pic x(8192).
 02 v-relation.
 03 v-relation-length usage binary-double unsigned.
 03 v-relation-text pic x(8192).
 02 v-request.
 03 v-request-length usage binary-double unsigned.
 03 v-request-text pic x(8192).
 02 v-source usage pointer.
 02 v-target.
 03 v-target-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-target-value usage pointer.
 02 v-threshold usage pointer.
 02 v-usage.
 03 v-usage-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-usage-value usage pointer.
 02 v-failure usage pointer.
 02 v-failure-id usage pointer.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptFailure based.
 02 v-cause usage pointer.
 02 v-kind usage pointer.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptFailureFieldKind based.
 02 v-kind usage binary-long unsigned.
01 tt-n-cobol-ReceiptFailureCause based.
 02 v-kind usage binary-long unsigned.
01 tt-n-cobol-ReceiptRelatedEntityPresence based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-cobol-ReceiptRelatedEntity based.
 02 v-kind.
 03 v-kind-length usage binary-double unsigned.
 03 v-kind-text pic x(8192).
 02 v-name.
 03 v-name-length usage binary-double unsigned.
 03 v-name-text pic x(8192).
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptQuestionDocument based.
 02 v-kind usage binary-long unsigned.
 02 filler pic x(4).
 02 v-data.
 03 v-data-decide usage pointer.
 03 v-data-choose usage pointer.
 03 v-data-tag usage pointer.
 03 v-data-score usage pointer.
01 tt-n-cobol-ReceiptQuestionDocumentData based.
 02 v-decide usage pointer.
 02 v-choose usage pointer.
 02 v-tag usage pointer.
 02 v-score usage pointer.
01 tt-n-cobol-ReceiptQuestionDocumentScore based.
 02 v-batch.
 03 v-batch-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-batch-value usage pointer.
 02 v-context-schema.
 03 v-context-schema-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-context-schema-value usage pointer.
 02 v-item-schema.
 03 v-item-schema-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-item-schema-value usage pointer.
 02 v-label-details.
 03 v-label-details-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-label-details-value.
 04 v-label-details-value-data usage pointer.
 04 v-label-details-value-len usage binary-double unsigned.
 02 v-model.
 03 v-model-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-model-value.
 04 v-model-value-length usage binary-double unsigned.
 04 v-model-value-text pic x(8192).
 02 v-name.
 03 v-name-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-name-value usage pointer.
 02 v-on.
 03 v-on-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-on-value.
 04 v-on-value-data usage pointer.
 04 v-on-value-len usage binary-double unsigned.
 02 v-profile.
 03 v-profile-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-profile-value.
 04 v-profile-value-length usage binary-double unsigned.
 04 v-profile-value-text pic x(8192).
 02 v-wording-version.
 03 v-wording-version-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-wording-version-value usage pointer.
 02 v-levels.
 03 v-levels-data usage pointer.
 03 v-levels-len usage binary-double unsigned.
 02 v-text.
 03 v-text-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-text-value usage pointer.
 02 v-verb.
 03 v-verb-length usage binary-double unsigned.
 03 v-verb-text pic x(8192).
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptQuestionDocumentTag based.
 02 v-batch.
 03 v-batch-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-batch-value usage pointer.
 02 v-context-schema.
 03 v-context-schema-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-context-schema-value usage pointer.
 02 v-item-schema.
 03 v-item-schema-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-item-schema-value usage pointer.
 02 v-label-details.
 03 v-label-details-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-label-details-value.
 04 v-label-details-value-data usage pointer.
 04 v-label-details-value-len usage binary-double unsigned.
 02 v-model.
 03 v-model-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-model-value.
 04 v-model-value-length usage binary-double unsigned.
 04 v-model-value-text pic x(8192).
 02 v-name.
 03 v-name-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-name-value usage pointer.
 02 v-on.
 03 v-on-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-on-value.
 04 v-on-value-data usage pointer.
 04 v-on-value-len usage binary-double unsigned.
 02 v-profile.
 03 v-profile-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-profile-value.
 04 v-profile-value-length usage binary-double unsigned.
 04 v-profile-value-text pic x(8192).
 02 v-wording-version.
 03 v-wording-version-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-wording-version-value usage pointer.
 02 v-labels.
 03 v-labels-data usage pointer.
 03 v-labels-len usage binary-double unsigned.
 02 v-text.
 03 v-text-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-text-value usage pointer.
 02 v-verb.
 03 v-verb-length usage binary-double unsigned.
 03 v-verb-text pic x(8192).
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptQuestionDocumentChoose based.
 02 v-batch.
 03 v-batch-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-batch-value usage pointer.
 02 v-context-schema.
 03 v-context-schema-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-context-schema-value usage pointer.
 02 v-item-schema.
 03 v-item-schema-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-item-schema-value usage pointer.
 02 v-label-details.
 03 v-label-details-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-label-details-value.
 04 v-label-details-value-data usage pointer.
 04 v-label-details-value-len usage binary-double unsigned.
 02 v-model.
 03 v-model-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-model-value.
 04 v-model-value-length usage binary-double unsigned.
 04 v-model-value-text pic x(8192).
 02 v-name.
 03 v-name-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-name-value usage pointer.
 02 v-on.
 03 v-on-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-on-value.
 04 v-on-value-data usage pointer.
 04 v-on-value-len usage binary-double unsigned.
 02 v-profile.
 03 v-profile-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-profile-value.
 04 v-profile-value-length usage binary-double unsigned.
 04 v-profile-value-text pic x(8192).
 02 v-wording-version.
 03 v-wording-version-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-wording-version-value usage pointer.
 02 v-options.
 03 v-options-data usage pointer.
 03 v-options-len usage binary-double unsigned.
 02 v-text.
 03 v-text-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-text-value usage pointer.
 02 v-verb.
 03 v-verb-length usage binary-double unsigned.
 03 v-verb-text pic x(8192).
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptQuestionDocumentDecide based.
 02 v-batch.
 03 v-batch-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-batch-value usage pointer.
 02 v-context-schema.
 03 v-context-schema-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-context-schema-value usage pointer.
 02 v-item-schema.
 03 v-item-schema-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-item-schema-value usage pointer.
 02 v-label-details.
 03 v-label-details-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-label-details-value.
 04 v-label-details-value-data usage pointer.
 04 v-label-details-value-len usage binary-double unsigned.
 02 v-model.
 03 v-model-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-model-value.
 04 v-model-value-length usage binary-double unsigned.
 04 v-model-value-text pic x(8192).
 02 v-name.
 03 v-name-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-name-value usage pointer.
 02 v-on.
 03 v-on-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-on-value.
 04 v-on-value-data usage pointer.
 04 v-on-value-len usage binary-double unsigned.
 02 v-profile.
 03 v-profile-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-profile-value.
 04 v-profile-value-length usage binary-double unsigned.
 04 v-profile-value-text pic x(8192).
 02 v-wording-version.
 03 v-wording-version-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-wording-version-value usage pointer.
 02 v-false.
 03 v-false--presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-false--value usage pointer.
 02 v-text.
 03 v-text-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-text-value usage pointer.
 02 v-true.
 03 v-true--presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-true--value usage pointer.
 02 v-verb.
 03 v-verb-length usage binary-double unsigned.
 03 v-verb-text pic x(8192).
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptRelationMethod based.
 02 v-kind usage binary-long unsigned.
01 tt-n-cobol-ReceiptRelationDirection based.
 02 v-kind usage binary-long unsigned.
01 tt-n-cobol-ReceiptRelationMemberAnswerId based.
 02 v-direction usage pointer.
 02 v-method usage pointer.
 02 v-observations.
 03 v-observations-data usage pointer.
 03 v-observations-len usage binary-double unsigned.
 02 v-question usage pointer.
 02 v-question-sources.
 03 v-question-sources-data usage pointer.
 03 v-question-sources-len usage binary-double unsigned.
 02 v-reads.
 03 v-reads-length usage binary-double unsigned.
 03 v-reads-text pic x(8192).
 02 v-relation.
 03 v-relation-length usage binary-double unsigned.
 03 v-relation-text pic x(8192).
 02 v-request.
 03 v-request-length usage binary-double unsigned.
 03 v-request-text pic x(8192).
 02 v-source usage pointer.
 02 v-target.
 03 v-target-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-target-value usage pointer.
 02 v-threshold usage pointer.
 02 v-usage.
 03 v-usage-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-usage-value usage pointer.
 02 v-accepted usage binary-long unsigned.
 02 filler pic x(4).
 02 v-answer usage pointer.
 02 v-answer-id usage pointer.
 02 v-probability usage float-long.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptAnswer based.
 02 v-kind usage binary-long unsigned.
 02 filler pic x(4).
 02 v-data.
 03 v-data-yes-no usage pointer.
 03 v-data-choice usage pointer.
 03 v-data-tag usage pointer.
 03 v-data-score usage pointer.
01 tt-n-cobol-ReceiptAnswerData based.
 02 v-yes-no usage pointer.
 02 v-choice usage pointer.
 02 v-tag usage pointer.
 02 v-score usage pointer.
01 tt-n-cobol-ReceiptAnswerScore based.
 02 v-confidence.
 03 v-confidence-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-confidence-value usage float-long.
 02 v-kind.
 03 v-kind-length usage binary-double unsigned.
 03 v-kind-text pic x(8192).
 02 v-level.
 03 v-level-length usage binary-double unsigned.
 03 v-level-text pic x(8192).
 02 v-probabilities.
 03 v-probabilities-data usage pointer.
 03 v-probabilities-len usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptNumberMap based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptNumberMapEntry based.
 02 v-name.
 03 v-name-length usage binary-double unsigned.
 03 v-name-text pic x(8192).
 02 v-value usage float-long.
01 tt-n-cobol-ReceiptF64Presence based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage float-long.
01 tt-n-cobol-ReceiptAnswerTag based.
 02 v-kind.
 03 v-kind-length usage binary-double unsigned.
 03 v-kind-text pic x(8192).
 02 v-probabilities.
 03 v-probabilities-data usage pointer.
 03 v-probabilities-len usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptAnswerChoice based.
 02 v-confidence.
 03 v-confidence-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-confidence-value usage float-long.
 02 v-kind.
 03 v-kind-length usage binary-double unsigned.
 03 v-kind-text pic x(8192).
 02 v-pick.
 03 v-pick-length usage binary-double unsigned.
 03 v-pick-text pic x(8192).
 02 v-probabilities.
 03 v-probabilities-data usage pointer.
 03 v-probabilities-len usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptAnswerYesNo based.
 02 v-kind.
 03 v-kind-length usage binary-double unsigned.
 03 v-kind-text pic x(8192).
 02 v-probability usage float-long.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptRecognitionList based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptRecognition based.
 02 v-answer usage pointer.
 02 v-answer-id usage pointer.
 02 v-file.
 03 v-file-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-file-value.
 04 v-file-value-length usage binary-double unsigned.
 04 v-file-value-text pic x(8192).
 02 v-first-line.
 03 v-first-line-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-first-line-value usage binary-double unsigned.
 02 v-index.
 03 v-index-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-index-value usage binary-double unsigned.
 02 v-input.
 03 v-input-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-input-value usage pointer.
 02 v-last-line.
 03 v-last-line-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-last-line-value usage binary-double unsigned.
 02 v-meta usage pointer.
 02 v-position.
 03 v-position-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-position-value usage pointer.
 02 v-question usage pointer.
 02 v-schema usage pointer.
 02 v-source.
 03 v-source-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-source-value usage pointer.
 02 v-value usage pointer.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptRecognize based.
 02 v-kind usage binary-long unsigned.
 02 filler pic x(4).
 02 v-data.
 03 v-data-entities usage pointer.
 03 v-data-mode-proposals usage pointer.
01 tt-n-cobol-ReceiptRecognizeData based.
 02 v-entities usage pointer.
 02 v-mode-proposals usage pointer.
01 tt-n-cobol-ReceiptRecognizeModeProposals based.
 02 v-mode usage pointer.
 02 v-proposals.
 03 v-proposals-data usage pointer.
 03 v-proposals-len usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptBoundaryProposalList based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptBoundaryProposal based.
 02 v-end usage binary-double unsigned.
 02 v-length usage binary-double unsigned.
 02 v-probability usage float-long.
 02 v-start usage binary-double unsigned.
 02 v-text.
 03 v-text-length usage binary-double unsigned.
 03 v-text-text pic x(8192).
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptBoundaryMode based.
 02 v-kind usage binary-long unsigned.
01 tt-n-cobol-ReceiptRecognizeEntities based.
 02 v-entities.
 03 v-entities-data usage pointer.
 03 v-entities-len usage binary-double unsigned.
 02 v-relations.
 03 v-relations-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-relations-value.
 04 v-relations-value-data usage pointer.
 04 v-relations-value-len usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptEntityEdgeListPresence based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-value-data usage pointer.
 03 v-value-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptEntityEdgeList based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptEntityEdge based.
 02 v-either.
 03 v-either-presence usage binary-long unsigned.
 03 v-either-value usage binary-long unsigned.
 02 v-probability usage float-long.
 02 v-relation.
 03 v-relation-length usage binary-double unsigned.
 03 v-relation-text pic x(8192).
 02 v-source usage pointer.
 02 v-target usage pointer.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptEntity based.
 02 v-end usage binary-double unsigned.
 02 v-file.
 03 v-file-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-file-value.
 04 v-file-value-length usage binary-double unsigned.
 04 v-file-value-text pic x(8192).
 02 v-first-line.
 03 v-first-line-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-first-line-value usage binary-double unsigned.
 02 v-kind.
 03 v-kind-length usage binary-double unsigned.
 03 v-kind-text pic x(8192).
 02 v-last-line.
 03 v-last-line-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-last-line-value usage binary-double unsigned.
 02 v-length usage binary-double unsigned.
 02 v-start usage binary-double unsigned.
 02 v-strength usage float-long.
 02 v-text.
 03 v-text-length usage binary-double unsigned.
 03 v-text-text pic x(8192).
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptEntityList based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptPhysicalSourcePresence based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-cobol-ReceiptRecognizeQuestionDocument based.
 02 v-batch.
 03 v-batch-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-batch-value usage pointer.
 02 v-context-schema.
 03 v-context-schema-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-context-schema-value usage pointer.
 02 v-entity-definition.
 03 v-entity-definition-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-entity-definition-value usage pointer.
 02 v-instructions.
 03 v-instructions-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-instructions-value usage pointer.
 02 v-item-schema.
 03 v-item-schema-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-item-schema-value usage pointer.
 02 v-kinds.
 03 v-kinds-data usage pointer.
 03 v-kinds-len usage binary-double unsigned.
 02 v-label-details.
 03 v-label-details-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-label-details-value.
 04 v-label-details-value-data usage pointer.
 04 v-label-details-value-len usage binary-double unsigned.
 02 v-mode.
 03 v-mode-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-mode-value usage pointer.
 02 v-model.
 03 v-model-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-model-value.
 04 v-model-value-length usage binary-double unsigned.
 04 v-model-value-text pic x(8192).
 02 v-name.
 03 v-name-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-name-value usage pointer.
 02 v-on.
 03 v-on-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-on-value.
 04 v-on-value-data usage pointer.
 04 v-on-value-len usage binary-double unsigned.
 02 v-profile.
 03 v-profile-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-profile-value.
 04 v-profile-value-length usage binary-double unsigned.
 04 v-profile-value-text pic x(8192).
 02 v-relation-threshold.
 03 v-relation-threshold-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-relation-threshold-value usage pointer.
 02 v-relations.
 03 v-relations-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-relations-value.
 04 v-relations-value-data usage pointer.
 04 v-relations-value-len usage binary-double unsigned.
 02 v-snippet-pieces.
 03 v-snippet-pieces-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-snippet-pieces-value usage binary-double unsigned.
 02 v-stage-context.
 03 v-stage-context-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-stage-context-value usage pointer.
 02 v-threshold usage pointer.
 02 v-verb usage pointer.
 02 v-wording-version.
 03 v-wording-version-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-wording-version-value usage pointer.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptVerb based.
 02 v-kind usage binary-long unsigned.
01 tt-n-cobol-ReceiptRecognitionStageContextPresence based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-cobol-ReceiptRecognitionStageContext based.
 02 v-boundary.
 03 v-boundary-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-boundary-value.
 04 v-boundary-value-length usage binary-double unsigned.
 04 v-boundary-value-text pic x(8192).
 02 v-kind-edge.
 03 v-kind-edge-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-kind-edge-value.
 04 v-kind-edge-value-length usage binary-double unsigned.
 04 v-kind-edge-value-text pic x(8192).
 02 v-relation.
 03 v-relation-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-relation-value.
 04 v-relation-value-length usage binary-double unsigned.
 04 v-relation-value-text pic x(8192).
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptRelationRuleListPresence based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-value-data usage pointer.
 03 v-value-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptThresholdPresence based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-cobol-ReceiptRecognitionModePresence based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-cobol-ReceiptRecognitionMode based.
 02 v-kind usage binary-long unsigned.
01 tt-n-cobol-ReceiptJsonMap based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptJsonMapEntry based.
 02 v-name.
 03 v-name-length usage binary-double unsigned.
 03 v-name-text pic x(8192).
 02 v-value usage pointer.
01 tt-n-cobol-ReceiptRecognitionOdds based.
 02 v-kind usage binary-long unsigned.
 02 filler pic x(4).
 02 v-data.
 03 v-data-recognize-answer usage pointer.
 03 v-data-boundary-odds usage pointer.
01 tt-n-cobol-ReceiptRecognitionOddsData based.
 02 v-recognize-answer usage pointer.
 02 v-boundary-odds usage pointer.
01 tt-n-cobol-ReceiptRecognitionOddsBoundaryOdds based.
 02 v-pieces.
 03 v-pieces-data usage pointer.
 03 v-pieces-len usage binary-double unsigned.
 02 v-proposals.
 03 v-proposals-data usage pointer.
 03 v-proposals-len usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptPieceOddsList based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptPieceOdds based.
 02 v-end usage binary-double unsigned.
 02 v-start usage binary-double unsigned.
 02 v-tags.
 03 v-tags-data usage pointer.
 03 v-tags-len usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptRecognitionOddsRecognizeAnswer based.
 02 v-names.
 03 v-names-data usage pointer.
 03 v-names-len usage binary-double unsigned.
 02 v-pairs.
 03 v-pairs-data usage pointer.
 03 v-pairs-len usage binary-double unsigned.
 02 v-pieces.
 03 v-pieces-data usage pointer.
 03 v-pieces-len usage binary-double unsigned.
 02 v-proposals.
 03 v-proposals-data usage pointer.
 03 v-proposals-len usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptRecognitionProposalList based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptRecognitionProposal based.
 02 v-end usage binary-double unsigned.
 02 v-kept usage binary-long unsigned.
 02 filler pic x(4).
 02 v-kind.
 03 v-kind-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-kind-value.
 04 v-kind-value-length usage binary-double unsigned.
 04 v-kind-value-text pic x(8192).
 02 v-selected.
 03 v-selected-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-selected-value usage pointer.
 02 v-span-probability usage float-long.
 02 v-start usage binary-double unsigned.
 02 v-strength.
 03 v-strength-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-strength-value usage float-long.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptPlacePresence based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-cobol-ReceiptPlace based.
 02 v-end usage binary-double unsigned.
 02 v-start usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptPairOddsList based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptPairOdds based.
 02 v-probability usage float-long.
 02 v-relation.
 03 v-relation-length usage binary-double unsigned.
 03 v-relation-text pic x(8192).
 02 v-source usage pointer.
 02 v-target usage pointer.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptNameOddsList based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptNameOdds based.
 02 v-edges.
 03 v-edges-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-edges-value.
 04 v-edges-value-data usage pointer.
 04 v-edges-value-len usage binary-double unsigned.
 02 v-end usage binary-double unsigned.
 02 v-kinds.
 03 v-kinds-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-kinds-value.
 04 v-kinds-value-data usage pointer.
 04 v-kinds-value-len usage binary-double unsigned.
 02 v-start usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptNumberMapPresence based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-value-data usage pointer.
 03 v-value-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptAnnotationList based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptAnnotation based.
 02 v-answer-id usage pointer.
 02 v-answers.
 03 v-answers-data usage pointer.
 03 v-answers-len usage binary-double unsigned.
 02 v-file.
 03 v-file-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-file-value.
 04 v-file-value-length usage binary-double unsigned.
 04 v-file-value-text pic x(8192).
 02 v-first-line.
 03 v-first-line-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-first-line-value usage binary-double unsigned.
 02 v-index.
 03 v-index-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-index-value usage binary-double unsigned.
 02 v-input.
 03 v-input-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-input-value usage pointer.
 02 v-last-line.
 03 v-last-line-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-last-line-value usage binary-double unsigned.
 02 v-meta usage pointer.
 02 v-position.
 03 v-position-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-position-value usage pointer.
 02 v-schema usage pointer.
 02 v-source.
 03 v-source-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-source-value usage pointer.
 02 v-value usage pointer.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptAnnotatedRow based.
 02 v-value.
 03 v-value-data usage pointer.
 03 v-value-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptAnnotatedFieldMap based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptAnnotatedFieldMapEntry based.
 02 v-name.
 03 v-name-length usage binary-double unsigned.
 03 v-name-text pic x(8192).
 02 v-value usage pointer.
01 tt-n-cobol-ReceiptAnnotatedField based.
 02 v-kind usage binary-long unsigned.
 02 filler pic x(4).
 02 v-data.
 03 v-data-boolean usage binary-long unsigned.
 03 v-data-null usage binary-long unsigned.
 03 v-data-string.
 04 v-data-string-length usage binary-double unsigned.
 04 v-data-string-text pic x(8192).
 03 v-data-array.
 04 v-data-array-data usage pointer.
 04 v-data-array-len usage binary-double unsigned.
 03 v-data-number usage float-long.
 03 v-data-object usage pointer.
01 tt-n-cobol-ReceiptAnnotatedFieldData based.
 02 v-boolean usage binary-long unsigned.
 02 v-null usage binary-long unsigned.
 02 v-string.
 03 v-string-length usage binary-double unsigned.
 03 v-string-text pic x(8192).
 02 v-array.
 03 v-array-data usage pointer.
 03 v-array-len usage binary-double unsigned.
 02 v-number usage float-long.
 02 v-object usage pointer.
01 tt-n-cobol-ReceiptFailed based.
 02 v-failed usage pointer.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptAnnotationMemberMap based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptAnnotationMemberMapEntry based.
 02 v-name.
 03 v-name-length usage binary-double unsigned.
 03 v-name-text pic x(8192).
 02 v-value usage pointer.
01 tt-n-cobol-ReceiptAnnotationMember based.
 02 v-kind usage binary-long unsigned.
 02 filler pic x(4).
 02 v-data.
 03 v-data-answer-id usage pointer.
 03 v-data-failure-id usage pointer.
01 tt-n-cobol-ReceiptAnnotationMemberData based.
 02 v-answer-id usage pointer.
 02 v-failure-id usage pointer.
01 tt-n-cobol-ReceiptAnnotationMemberFailureId based.
 02 v-failure usage pointer.
 02 v-failure-id usage pointer.
 02 v-observations.
 03 v-observations-data usage pointer.
 03 v-observations-len usage binary-double unsigned.
 02 v-question usage pointer.
 02 v-question-sources.
 03 v-question-sources-data usage pointer.
 03 v-question-sources-len usage binary-double unsigned.
 02 v-request.
 03 v-request-length usage binary-double unsigned.
 03 v-request-text pic x(8192).
 02 v-threshold.
 03 v-threshold-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-threshold-value usage pointer.
 02 v-usage.
 03 v-usage-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-usage-value usage pointer.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptAnnotationMemberAnswerId based.
 02 v-answer usage pointer.
 02 v-answer-id usage pointer.
 02 v-observations.
 03 v-observations-data usage pointer.
 03 v-observations-len usage binary-double unsigned.
 02 v-question usage pointer.
 02 v-question-sources.
 03 v-question-sources-data usage pointer.
 03 v-question-sources-len usage binary-double unsigned.
 02 v-request.
 03 v-request-length usage binary-double unsigned.
 03 v-request-text pic x(8192).
 02 v-threshold.
 03 v-threshold-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-threshold-value usage pointer.
 02 v-usage.
 03 v-usage-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-usage-value usage pointer.
 02 v-value.
 03 v-value-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value-value usage pointer.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptValuePresence based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-cobol-ReceiptValue based.
 02 v-kind usage binary-long unsigned.
 02 filler pic x(4).
 02 v-data.
 03 v-data-boolean usage binary-long unsigned.
 03 v-data-null usage binary-long unsigned.
 03 v-data-string.
 04 v-data-string-length usage binary-double unsigned.
 04 v-data-string-text pic x(8192).
 03 v-data-array.
 04 v-data-array-data usage pointer.
 04 v-data-array-len usage binary-double unsigned.
 03 v-data-number usage float-long.
01 tt-n-cobol-ReceiptValueData based.
 02 v-boolean usage binary-long unsigned.
 02 v-null usage binary-long unsigned.
 02 v-string.
 03 v-string-length usage binary-double unsigned.
 03 v-string-text pic x(8192).
 02 v-array.
 03 v-array-data usage pointer.
 03 v-array-len usage binary-double unsigned.
 02 v-number usage float-long.
01 tt-n-cobol-ReceiptFind based.
 02 v-answer usage pointer.
 02 v-answer-id usage pointer.
 02 v-candidates.
 03 v-candidates-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-candidates-value.
 04 v-candidates-value-data usage pointer.
 04 v-candidates-value-len usage binary-double unsigned.
 02 v-file.
 03 v-file-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-file-value.
 04 v-file-value-length usage binary-double unsigned.
 04 v-file-value-text pic x(8192).
 02 v-first-line.
 03 v-first-line-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-first-line-value usage binary-double unsigned.
 02 v-index.
 03 v-index-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-index-value usage binary-double unsigned.
 02 v-last-line.
 03 v-last-line-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-last-line-value usage binary-double unsigned.
 02 v-meta usage pointer.
 02 v-position.
 03 v-position-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-position-value usage pointer.
 02 v-question usage pointer.
 02 v-schema usage pointer.
 02 v-threshold.
 03 v-threshold-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-threshold-value usage pointer.
 02 v-value.
 03 v-value-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value-value usage pointer.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptFindQuestionDocument based.
 02 v-batch.
 03 v-batch-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-batch-value usage pointer.
 02 v-context-schema.
 03 v-context-schema-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-context-schema-value usage pointer.
 02 v-item-schema.
 03 v-item-schema-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-item-schema-value usage pointer.
 02 v-label-details.
 03 v-label-details-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-label-details-value.
 04 v-label-details-value-data usage pointer.
 04 v-label-details-value-len usage binary-double unsigned.
 02 v-model.
 03 v-model-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-model-value.
 04 v-model-value-length usage binary-double unsigned.
 04 v-model-value-text pic x(8192).
 02 v-name.
 03 v-name-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-name-value usage pointer.
 02 v-none usage binary-long unsigned.
 02 filler pic x(4).
 02 v-on.
 03 v-on-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-on-value.
 04 v-on-value-data usage pointer.
 04 v-on-value-len usage binary-double unsigned.
 02 v-profile.
 03 v-profile-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-profile-value.
 04 v-profile-value-length usage binary-double unsigned.
 04 v-profile-value-text pic x(8192).
 02 v-text.
 03 v-text-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-text-value usage pointer.
 02 v-verb.
 03 v-verb-length usage binary-double unsigned.
 03 v-verb-text pic x(8192).
 02 v-wording-version.
 03 v-wording-version-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-wording-version-value usage pointer.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptFindCandidateListPresence based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-value-data usage pointer.
 03 v-value-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptFindCandidateList based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptFindCandidate based.
 02 v-index.
 03 v-index-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-index-value usage binary-double unsigned.
 02 v-input.
 03 v-input-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-input-value usage pointer.
 02 v-probability usage float-long.
 02 v-source.
 03 v-source-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-source-value usage pointer.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptFindAnswer based.
 02 v-confidence.
 03 v-confidence-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-confidence-value usage float-long.
 02 v-kind.
 03 v-kind-length usage binary-double unsigned.
 03 v-kind-text pic x(8192).
 02 v-pick.
 03 v-pick-length usage binary-double unsigned.
 03 v-pick-text pic x(8192).
 02 v-probabilities.
 03 v-probabilities-data usage pointer.
 03 v-probabilities-len usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptAtomicNonZeroUsizeList based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptAtomicNonZeroUsize based.
 02 v-answer usage pointer.
 02 v-answer-id usage pointer.
 02 v-images.
 03 v-images-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-images-value.
 04 v-images-value-data usage pointer.
 04 v-images-value-len usage binary-double unsigned.
 02 v-index.
 03 v-index-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-index-value usage binary-double unsigned.
 02 v-input.
 03 v-input-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-input-value usage pointer.
 02 v-members.
 03 v-members-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-members-value.
 04 v-members-value-data usage pointer.
 04 v-members-value-len usage binary-double unsigned.
 02 v-meta usage pointer.
 02 v-question usage pointer.
 02 v-question-name.
 03 v-question-name-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-question-name-value.
 04 v-question-name-value-length usage binary-double unsigned.
 04 v-question-name-value-text pic x(8192).
 02 v-schema usage pointer.
 02 v-source.
 03 v-source-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-source-value usage pointer.
 02 v-threshold.
 03 v-threshold-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-threshold-value usage pointer.
 02 v-value usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptRankMemberListPresence based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-value-data usage pointer.
 03 v-value-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptRankMemberList based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptRankMember based.
 02 v-name.
 03 v-name-length usage binary-double unsigned.
 03 v-name-text pic x(8192).
 02 v-result usage pointer.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptRankMemberResult based.
 02 v-answer usage pointer.
 02 v-answer-id usage pointer.
 02 v-images.
 03 v-images-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-images-value.
 04 v-images-value-data usage pointer.
 04 v-images-value-len usage binary-double unsigned.
 02 v-meta usage pointer.
 02 v-question usage pointer.
 02 v-schema usage pointer.
 02 v-source.
 03 v-source-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-source-value usage pointer.
 02 v-threshold.
 03 v-threshold-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-threshold-value usage pointer.
 02 v-value usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptImageListPresence based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-value-data usage pointer.
 03 v-value-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptImageList based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptImage based.
 02 v-base64.
 03 v-base64-length usage binary-double unsigned.
 03 v-base64-text pic x(8192).
 02 v-height usage binary-double unsigned.
 02 v-media usage pointer.
 02 v-width usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptImageMedia based.
 02 v-kind usage binary-long unsigned.
01 tt-n-cobol-ReceiptAtomicBooleanList based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptAtomicBoolean based.
 02 v-answer usage pointer.
 02 v-answer-id usage pointer.
 02 v-images.
 03 v-images-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-images-value.
 04 v-images-value-data usage pointer.
 04 v-images-value-len usage binary-double unsigned.
 02 v-index.
 03 v-index-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-index-value usage binary-double unsigned.
 02 v-input.
 03 v-input-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-input-value usage pointer.
 02 v-members.
 03 v-members-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-members-value.
 04 v-members-value-data usage pointer.
 04 v-members-value-len usage binary-double unsigned.
 02 v-meta usage pointer.
 02 v-question usage pointer.
 02 v-question-name.
 03 v-question-name-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-question-name-value.
 04 v-question-name-value-length usage binary-double unsigned.
 04 v-question-name-value-text pic x(8192).
 02 v-schema usage pointer.
 02 v-source.
 03 v-source-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-source-value usage pointer.
 02 v-threshold.
 03 v-threshold-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-threshold-value usage pointer.
 02 v-value usage binary-long unsigned.
 02 filler pic x(4).
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptAtomicDoubleList based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptAtomicDouble based.
 02 v-answer usage pointer.
 02 v-answer-id usage pointer.
 02 v-images.
 03 v-images-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-images-value.
 04 v-images-value-data usage pointer.
 04 v-images-value-len usage binary-double unsigned.
 02 v-index.
 03 v-index-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-index-value usage binary-double unsigned.
 02 v-input.
 03 v-input-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-input-value usage pointer.
 02 v-members.
 03 v-members-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-members-value.
 04 v-members-value-data usage pointer.
 04 v-members-value-len usage binary-double unsigned.
 02 v-meta usage pointer.
 02 v-question usage pointer.
 02 v-question-name.
 03 v-question-name-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-question-name-value.
 04 v-question-name-value-length usage binary-double unsigned.
 04 v-question-name-value-text pic x(8192).
 02 v-schema usage pointer.
 02 v-source.
 03 v-source-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-source-value usage pointer.
 02 v-threshold.
 03 v-threshold-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-threshold-value usage pointer.
 02 v-value usage float-long.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptAtomicArrayOfStringList based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptAtomicArrayOfString based.
 02 v-answer usage pointer.
 02 v-answer-id usage pointer.
 02 v-images.
 03 v-images-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-images-value.
 04 v-images-value-data usage pointer.
 04 v-images-value-len usage binary-double unsigned.
 02 v-index.
 03 v-index-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-index-value usage binary-double unsigned.
 02 v-input.
 03 v-input-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-input-value usage pointer.
 02 v-members.
 03 v-members-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-members-value.
 04 v-members-value-data usage pointer.
 04 v-members-value-len usage binary-double unsigned.
 02 v-meta usage pointer.
 02 v-question usage pointer.
 02 v-question-name.
 03 v-question-name-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-question-name-value.
 04 v-question-name-value-length usage binary-double unsigned.
 04 v-question-name-value-text pic x(8192).
 02 v-schema usage pointer.
 02 v-source.
 03 v-source-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-source-value usage pointer.
 02 v-threshold.
 03 v-threshold-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-threshold-value usage pointer.
 02 v-value.
 03 v-value-data usage pointer.
 03 v-value-len usage binary-double unsigned.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptAtomicNullableStringList based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptAtomicNullableString based.
 02 v-answer usage pointer.
 02 v-answer-id usage pointer.
 02 v-images.
 03 v-images-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-images-value.
 04 v-images-value-data usage pointer.
 04 v-images-value-len usage binary-double unsigned.
 02 v-index.
 03 v-index-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-index-value usage binary-double unsigned.
 02 v-input.
 03 v-input-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-input-value usage pointer.
 02 v-members.
 03 v-members-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-members-value.
 04 v-members-value-data usage pointer.
 04 v-members-value-len usage binary-double unsigned.
 02 v-meta usage pointer.
 02 v-question usage pointer.
 02 v-question-name.
 03 v-question-name-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-question-name-value.
 04 v-question-name-value-length usage binary-double unsigned.
 04 v-question-name-value-text pic x(8192).
 02 v-schema usage pointer.
 02 v-source.
 03 v-source-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-source-value usage pointer.
 02 v-threshold.
 03 v-threshold-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-threshold-value usage pointer.
 02 v-value.
 03 v-value-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value-value.
 04 v-value-value-length usage binary-double unsigned.
 04 v-value-value-text pic x(8192).
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptAtomicDecideValueList based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptAtomicDecideValue based.
 02 v-answer usage pointer.
 02 v-answer-id usage pointer.
 02 v-images.
 03 v-images-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-images-value.
 04 v-images-value-data usage pointer.
 04 v-images-value-len usage binary-double unsigned.
 02 v-index.
 03 v-index-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-index-value usage binary-double unsigned.
 02 v-input.
 03 v-input-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-input-value usage pointer.
 02 v-members.
 03 v-members-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-members-value.
 04 v-members-value-data usage pointer.
 04 v-members-value-len usage binary-double unsigned.
 02 v-meta usage pointer.
 02 v-question usage pointer.
 02 v-question-name.
 03 v-question-name-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-question-name-value.
 04 v-question-name-value-length usage binary-double unsigned.
 04 v-question-name-value-text pic x(8192).
 02 v-schema usage pointer.
 02 v-source.
 03 v-source-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-source-value usage pointer.
 02 v-threshold.
 03 v-threshold-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-threshold-value usage pointer.
 02 v-value.
 03 v-value-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value-value usage pointer.
 02 v-extensions.
 03 v-extensions-data usage pointer.
 03 v-extensions-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptDecideValuePresence based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-cobol-ReceiptDecideValue based.
 02 v-value usage pointer.
01 tt-n-cobol-ReceiptFindList based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-ReceiptRelationList based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-AnswerTokenBand based.
 02 v-lower usage binary-double unsigned.
 02 v-upper usage binary-double unsigned.
01 tt-n-cobol-AnswerPlan based.
 02 v-estimated-bytes usage binary-double unsigned.
 02 v-estimated-input-tokens.
 03 v-estimated-input-tokens-lower usage binary-double unsigned.
 03 v-estimated-input-tokens-upper usage binary-double unsigned.
 02 v-first-body-utf8-presence usage binary-double unsigned.
 02 v-first-body-utf8.
 03 v-first-body-utf8-length usage binary-double unsigned.
 03 v-first-body-utf8-text pic x(8192).
 02 v-largest-request-bytes usage binary-double unsigned.
 02 v-largest-request-estimated-input-tokens usage binary-double unsigned.
 02 v-records usage binary-double unsigned.
 02 v-requests usage binary-double unsigned.
 02 v-token-estimate-method.
 03 v-token-estimate-method-length usage binary-double unsigned.
 03 v-token-estimate-method-text pic x(8192).
 02 v-upper-bound usage binary-double unsigned.
01 tt-n-cobol-AnswerDecideValue based.
 02 v-scalar usage binary-double unsigned.
 02 v-rows usage pointer.
 02 v-count usage binary-double unsigned.
 02 v-first.
 03 v-first-present usage binary-double unsigned.
 03 v-first-value usage pointer.
01 tt-n-cobol-AnswerDecideResult based.
 02 v-owner usage pointer.
 02 v-value.
 03 v-value-scalar usage binary-double unsigned.
 03 v-value-rows usage pointer.
 03 v-value-count usage binary-double unsigned.
 03 v-value-first.
 04 v-value-first-present usage binary-double unsigned.
 04 v-value-first-value usage pointer.
 02 v-probability.
 03 v-probability-scalar usage binary-double unsigned.
 03 v-probability-first.
 04 v-probability-first-present usage binary-double unsigned.
 04 v-probability-first-value usage float-long.
 03 v-probability-data usage pointer.
 03 v-probability-count usage binary-double unsigned.
 03 v-probability-lists usage pointer.
 03 v-probability-list-count usage binary-double unsigned.
 02 v-results.
 03 v-results-data usage pointer.
 03 v-results-len usage binary-double unsigned.
 02 v-facts-present usage binary-double unsigned.
 02 v-facts.
 03 v-facts-attempts.
 04 v-facts-attempts-presence usage binary-long unsigned.
 04 filler pic x(4).
 04 v-facts-attempts-value.
 05 v-facts-attempts-value-data usage pointer.
 05 v-facts-attempts-value-len usage binary-double unsigned.
 03 v-facts-cache-answers usage binary-double unsigned.
 03 v-facts-call-id usage pointer.
 03 v-facts-estimated-cost-usd.
 04 v-facts-estimated-cost-usd-presence usage binary-long unsigned.
 04 filler pic x(4).
 04 v-facts-estimated-cost-usd-value.
 05 v-facts-estimated-cost-usd-value-length usage binary-double unsigned.
 05 v-facts-estimated-cost-usd-value-text pic x(8192).
 03 v-facts-held-model-mismatch.
 04 v-facts-held-model-mismatch-presence usage binary-long unsigned.
 04 v-facts-held-model-mismatch-value usage binary-long unsigned.
 03 v-facts-input-tokens.
 04 v-facts-input-tokens-presence usage binary-long unsigned.
 04 filler pic x(4).
 04 v-facts-input-tokens-value usage binary-double unsigned.
 03 v-facts-largest-request-bytes usage binary-double unsigned.
 03 v-facts-largest-request-estimated-input-tokens.
 04 v-facts-largest-request-estimated-input-tokens-presence usage binary-long unsigned.
 04 filler pic x(4).
 04 v-facts-largest-request-estimated-input-tokens-value usage binary-double unsigned.
 03 v-facts-model.
 04 v-facts-model-presence usage binary-long unsigned.
 04 filler pic x(4).
 04 v-facts-model-value.
 05 v-facts-model-value-length usage binary-double unsigned.
 05 v-facts-model-value-text pic x(8192).
 03 v-facts-output-tokens.
 04 v-facts-output-tokens-presence usage binary-long unsigned.
 04 filler pic x(4).
 04 v-facts-output-tokens-value usage binary-double unsigned.
 03 v-facts-records usage binary-double unsigned.
 03 v-facts-requests-sent usage binary-double unsigned.
 03 v-facts-seconds usage float-long.
 03 v-facts-token-estimate-method.
 04 v-facts-token-estimate-method-length usage binary-double unsigned.
 04 v-facts-token-estimate-method-text pic x(8192).
 03 v-facts-usage-persistence.
 04 v-facts-usage-persistence-presence usage binary-long unsigned.
 04 filler pic x(4).
 04 v-facts-usage-persistence-value usage pointer.
 03 v-facts-extensions.
 04 v-facts-extensions-data usage pointer.
 04 v-facts-extensions-len usage binary-double unsigned.
01 tt-n-cobol-AnswerChooseValue based.
 02 v-scalar usage binary-double unsigned.
 02 v-rows usage pointer.
 02 v-count usage binary-double unsigned.
 02 v-first.
 03 v-first-present usage binary-double unsigned.
 03 v-first-value.
 04 v-first-value-length usage binary-double unsigned.
 04 v-first-value-text pic x(8192).
01 tt-n-cobol-AnswerChooseResult based.
 02 v-owner usage pointer.
 02 v-value.
 03 v-value-scalar usage binary-double unsigned.
 03 v-value-rows usage pointer.
 03 v-value-count usage binary-double unsigned.
 03 v-value-first.
 04 v-value-first-present usage binary-double unsigned.
 04 v-value-first-value.
 05 v-value-first-value-length usage binary-double unsigned.
 05 v-value-first-value-text pic x(8192).
 02 v-probability.
 03 v-probability-scalar usage binary-double unsigned.
 03 v-probability-first.
 04 v-probability-first-present usage binary-double unsigned.
 04 v-probability-first-value usage float-long.
 03 v-probability-data usage pointer.
 03 v-probability-count usage binary-double unsigned.
 03 v-probability-lists usage pointer.
 03 v-probability-list-count usage binary-double unsigned.
 02 v-results.
 03 v-results-data usage pointer.
 03 v-results-len usage binary-double unsigned.
 02 v-facts-present usage binary-double unsigned.
 02 v-facts.
 03 v-facts-attempts.
 04 v-facts-attempts-presence usage binary-long unsigned.
 04 filler pic x(4).
 04 v-facts-attempts-value.
 05 v-facts-attempts-value-data usage pointer.
 05 v-facts-attempts-value-len usage binary-double unsigned.
 03 v-facts-cache-answers usage binary-double unsigned.
 03 v-facts-call-id usage pointer.
 03 v-facts-estimated-cost-usd.
 04 v-facts-estimated-cost-usd-presence usage binary-long unsigned.
 04 filler pic x(4).
 04 v-facts-estimated-cost-usd-value.
 05 v-facts-estimated-cost-usd-value-length usage binary-double unsigned.
 05 v-facts-estimated-cost-usd-value-text pic x(8192).
 03 v-facts-held-model-mismatch.
 04 v-facts-held-model-mismatch-presence usage binary-long unsigned.
 04 v-facts-held-model-mismatch-value usage binary-long unsigned.
 03 v-facts-input-tokens.
 04 v-facts-input-tokens-presence usage binary-long unsigned.
 04 filler pic x(4).
 04 v-facts-input-tokens-value usage binary-double unsigned.
 03 v-facts-largest-request-bytes usage binary-double unsigned.
 03 v-facts-largest-request-estimated-input-tokens.
 04 v-facts-largest-request-estimated-input-tokens-presence usage binary-long unsigned.
 04 filler pic x(4).
 04 v-facts-largest-request-estimated-input-tokens-value usage binary-double unsigned.
 03 v-facts-model.
 04 v-facts-model-presence usage binary-long unsigned.
 04 filler pic x(4).
 04 v-facts-model-value.
 05 v-facts-model-value-length usage binary-double unsigned.
 05 v-facts-model-value-text pic x(8192).
 03 v-facts-output-tokens.
 04 v-facts-output-tokens-presence usage binary-long unsigned.
 04 filler pic x(4).
 04 v-facts-output-tokens-value usage binary-double unsigned.
 03 v-facts-records usage binary-double unsigned.
 03 v-facts-requests-sent usage binary-double unsigned.
 03 v-facts-seconds usage float-long.
 03 v-facts-token-estimate-method.
 04 v-facts-token-estimate-method-length usage binary-double unsigned.
 04 v-facts-token-estimate-method-text pic x(8192).
 03 v-facts-usage-persistence.
 04 v-facts-usage-persistence-presence usage binary-long unsigned.
 04 filler pic x(4).
 04 v-facts-usage-persistence-value usage pointer.
 03 v-facts-extensions.
 04 v-facts-extensions-data usage pointer.
 04 v-facts-extensions-len usage binary-double unsigned.
01 tt-n-cobol-AnswerTagValue based.
 02 v-scalar usage binary-double unsigned.
 02 v-rows usage pointer.
 02 v-count usage binary-double unsigned.
 02 v-first.
 03 v-first-owner usage pointer.
 03 v-first-rows usage pointer.
 03 v-first-count usage binary-double unsigned.
 03 v-first-edges usage pointer.
 03 v-first-edge-count usage binary-double unsigned.
01 tt-n-cobol-AnswerTagResult based.
 02 v-owner usage pointer.
 02 v-value.
 03 v-value-scalar usage binary-double unsigned.
 03 v-value-rows usage pointer.
 03 v-value-count usage binary-double unsigned.
 03 v-value-first.
 04 v-value-first-owner usage pointer.
 04 v-value-first-rows usage pointer.
 04 v-value-first-count usage binary-double unsigned.
 04 v-value-first-edges usage pointer.
 04 v-value-first-edge-count usage binary-double unsigned.
 02 v-probability.
 03 v-probability-scalar usage binary-double unsigned.
 03 v-probability-first.
 04 v-probability-first-present usage binary-double unsigned.
 04 v-probability-first-value usage float-long.
 03 v-probability-data usage pointer.
 03 v-probability-count usage binary-double unsigned.
 03 v-probability-lists usage pointer.
 03 v-probability-list-count usage binary-double unsigned.
 02 v-results.
 03 v-results-data usage pointer.
 03 v-results-len usage binary-double unsigned.
 02 v-facts-present usage binary-double unsigned.
 02 v-facts.
 03 v-facts-attempts.
 04 v-facts-attempts-presence usage binary-long unsigned.
 04 filler pic x(4).
 04 v-facts-attempts-value.
 05 v-facts-attempts-value-data usage pointer.
 05 v-facts-attempts-value-len usage binary-double unsigned.
 03 v-facts-cache-answers usage binary-double unsigned.
 03 v-facts-call-id usage pointer.
 03 v-facts-estimated-cost-usd.
 04 v-facts-estimated-cost-usd-presence usage binary-long unsigned.
 04 filler pic x(4).
 04 v-facts-estimated-cost-usd-value.
 05 v-facts-estimated-cost-usd-value-length usage binary-double unsigned.
 05 v-facts-estimated-cost-usd-value-text pic x(8192).
 03 v-facts-held-model-mismatch.
 04 v-facts-held-model-mismatch-presence usage binary-long unsigned.
 04 v-facts-held-model-mismatch-value usage binary-long unsigned.
 03 v-facts-input-tokens.
 04 v-facts-input-tokens-presence usage binary-long unsigned.
 04 filler pic x(4).
 04 v-facts-input-tokens-value usage binary-double unsigned.
 03 v-facts-largest-request-bytes usage binary-double unsigned.
 03 v-facts-largest-request-estimated-input-tokens.
 04 v-facts-largest-request-estimated-input-tokens-presence usage binary-long unsigned.
 04 filler pic x(4).
 04 v-facts-largest-request-estimated-input-tokens-value usage binary-double unsigned.
 03 v-facts-model.
 04 v-facts-model-presence usage binary-long unsigned.
 04 filler pic x(4).
 04 v-facts-model-value.
 05 v-facts-model-value-length usage binary-double unsigned.
 05 v-facts-model-value-text pic x(8192).
 03 v-facts-output-tokens.
 04 v-facts-output-tokens-presence usage binary-long unsigned.
 04 filler pic x(4).
 04 v-facts-output-tokens-value usage binary-double unsigned.
 03 v-facts-records usage binary-double unsigned.
 03 v-facts-requests-sent usage binary-double unsigned.
 03 v-facts-seconds usage float-long.
 03 v-facts-token-estimate-method.
 04 v-facts-token-estimate-method-length usage binary-double unsigned.
 04 v-facts-token-estimate-method-text pic x(8192).
 03 v-facts-usage-persistence.
 04 v-facts-usage-persistence-presence usage binary-long unsigned.
 04 filler pic x(4).
 04 v-facts-usage-persistence-value usage pointer.
 03 v-facts-extensions.
 04 v-facts-extensions-data usage pointer.
 04 v-facts-extensions-len usage binary-double unsigned.
01 tt-n-cobol-AnswerScoreValue based.
 02 v-scalar usage binary-double unsigned.
 02 v-rows usage pointer.
 02 v-count usage binary-double unsigned.
 02 v-first.
 03 v-first-value usage float-long.
01 tt-n-cobol-AnswerScoreResult based.
 02 v-owner usage pointer.
 02 v-value.
 03 v-value-scalar usage binary-double unsigned.
 03 v-value-rows usage pointer.
 03 v-value-count usage binary-double unsigned.
 03 v-value-first.
 04 v-value-first-value usage float-long.
 02 v-probability.
 03 v-probability-scalar usage binary-double unsigned.
 03 v-probability-first.
 04 v-probability-first-present usage binary-double unsigned.
 04 v-probability-first-value usage float-long.
 03 v-probability-data usage pointer.
 03 v-probability-count usage binary-double unsigned.
 03 v-probability-lists usage pointer.
 03 v-probability-list-count usage binary-double unsigned.
 02 v-results.
 03 v-results-data usage pointer.
 03 v-results-len usage binary-double unsigned.
 02 v-facts-present usage binary-double unsigned.
 02 v-facts.
 03 v-facts-attempts.
 04 v-facts-attempts-presence usage binary-long unsigned.
 04 filler pic x(4).
 04 v-facts-attempts-value.
 05 v-facts-attempts-value-data usage pointer.
 05 v-facts-attempts-value-len usage binary-double unsigned.
 03 v-facts-cache-answers usage binary-double unsigned.
 03 v-facts-call-id usage pointer.
 03 v-facts-estimated-cost-usd.
 04 v-facts-estimated-cost-usd-presence usage binary-long unsigned.
 04 filler pic x(4).
 04 v-facts-estimated-cost-usd-value.
 05 v-facts-estimated-cost-usd-value-length usage binary-double unsigned.
 05 v-facts-estimated-cost-usd-value-text pic x(8192).
 03 v-facts-held-model-mismatch.
 04 v-facts-held-model-mismatch-presence usage binary-long unsigned.
 04 v-facts-held-model-mismatch-value usage binary-long unsigned.
 03 v-facts-input-tokens.
 04 v-facts-input-tokens-presence usage binary-long unsigned.
 04 filler pic x(4).
 04 v-facts-input-tokens-value usage binary-double unsigned.
 03 v-facts-largest-request-bytes usage binary-double unsigned.
 03 v-facts-largest-request-estimated-input-tokens.
 04 v-facts-largest-request-estimated-input-tokens-presence usage binary-long unsigned.
 04 filler pic x(4).
 04 v-facts-largest-request-estimated-input-tokens-value usage binary-double unsigned.
 03 v-facts-model.
 04 v-facts-model-presence usage binary-long unsigned.
 04 filler pic x(4).
 04 v-facts-model-value.
 05 v-facts-model-value-length usage binary-double unsigned.
 05 v-facts-model-value-text pic x(8192).
 03 v-facts-output-tokens.
 04 v-facts-output-tokens-presence usage binary-long unsigned.
 04 filler pic x(4).
 04 v-facts-output-tokens-value usage binary-double unsigned.
 03 v-facts-records usage binary-double unsigned.
 03 v-facts-requests-sent usage binary-double unsigned.
 03 v-facts-seconds usage float-long.
 03 v-facts-token-estimate-method.
 04 v-facts-token-estimate-method-length usage binary-double unsigned.
 04 v-facts-token-estimate-method-text pic x(8192).
 03 v-facts-usage-persistence.
 04 v-facts-usage-persistence-presence usage binary-long unsigned.
 04 filler pic x(4).
 04 v-facts-usage-persistence-value usage pointer.
 03 v-facts-extensions.
 04 v-facts-extensions-data usage pointer.
 04 v-facts-extensions-len usage binary-double unsigned.
01 tt-n-cobol-AnswerFilterValue based.
 02 v-scalar usage binary-double unsigned.
 02 v-rows usage pointer.
 02 v-count usage binary-double unsigned.
 02 v-first.
 03 v-first-original usage pointer.
01 tt-n-cobol-AnswerFilterResult based.
 02 v-owner usage pointer.
 02 v-value.
 03 v-value-scalar usage binary-double unsigned.
 03 v-value-rows usage pointer.
 03 v-value-count usage binary-double unsigned.
 03 v-value-first.
 04 v-value-first-original usage pointer.
 02 v-probability.
 03 v-probability-scalar usage binary-double unsigned.
 03 v-probability-first.
 04 v-probability-first-present usage binary-double unsigned.
 04 v-probability-first-value usage float-long.
 03 v-probability-data usage pointer.
 03 v-probability-count usage binary-double unsigned.
 03 v-probability-lists usage pointer.
 03 v-probability-list-count usage binary-double unsigned.
 02 v-results.
 03 v-results-data usage pointer.
 03 v-results-len usage binary-double unsigned.
 02 v-facts-present usage binary-double unsigned.
 02 v-facts.
 03 v-facts-attempts.
 04 v-facts-attempts-presence usage binary-long unsigned.
 04 filler pic x(4).
 04 v-facts-attempts-value.
 05 v-facts-attempts-value-data usage pointer.
 05 v-facts-attempts-value-len usage binary-double unsigned.
 03 v-facts-cache-answers usage binary-double unsigned.
 03 v-facts-call-id usage pointer.
 03 v-facts-estimated-cost-usd.
 04 v-facts-estimated-cost-usd-presence usage binary-long unsigned.
 04 filler pic x(4).
 04 v-facts-estimated-cost-usd-value.
 05 v-facts-estimated-cost-usd-value-length usage binary-double unsigned.
 05 v-facts-estimated-cost-usd-value-text pic x(8192).
 03 v-facts-held-model-mismatch.
 04 v-facts-held-model-mismatch-presence usage binary-long unsigned.
 04 v-facts-held-model-mismatch-value usage binary-long unsigned.
 03 v-facts-input-tokens.
 04 v-facts-input-tokens-presence usage binary-long unsigned.
 04 filler pic x(4).
 04 v-facts-input-tokens-value usage binary-double unsigned.
 03 v-facts-largest-request-bytes usage binary-double unsigned.
 03 v-facts-largest-request-estimated-input-tokens.
 04 v-facts-largest-request-estimated-input-tokens-presence usage binary-long unsigned.
 04 filler pic x(4).
 04 v-facts-largest-request-estimated-input-tokens-value usage binary-double unsigned.
 03 v-facts-model.
 04 v-facts-model-presence usage binary-long unsigned.
 04 filler pic x(4).
 04 v-facts-model-value.
 05 v-facts-model-value-length usage binary-double unsigned.
 05 v-facts-model-value-text pic x(8192).
 03 v-facts-output-tokens.
 04 v-facts-output-tokens-presence usage binary-long unsigned.
 04 filler pic x(4).
 04 v-facts-output-tokens-value usage binary-double unsigned.
 03 v-facts-records usage binary-double unsigned.
 03 v-facts-requests-sent usage binary-double unsigned.
 03 v-facts-seconds usage float-long.
 03 v-facts-token-estimate-method.
 04 v-facts-token-estimate-method-length usage binary-double unsigned.
 04 v-facts-token-estimate-method-text pic x(8192).
 03 v-facts-usage-persistence.
 04 v-facts-usage-persistence-presence usage binary-long unsigned.
 04 filler pic x(4).
 04 v-facts-usage-persistence-value usage pointer.
 03 v-facts-extensions.
 04 v-facts-extensions-data usage pointer.
 04 v-facts-extensions-len usage binary-double unsigned.
01 tt-n-cobol-AnswerRankValue based.
 02 v-scalar usage binary-double unsigned.
 02 v-rows usage pointer.
 02 v-count usage binary-double unsigned.
 02 v-first.
 03 v-first-value usage binary-double unsigned.
01 tt-n-cobol-AnswerRankResult based.
 02 v-owner usage pointer.
 02 v-value.
 03 v-value-scalar usage binary-double unsigned.
 03 v-value-rows usage pointer.
 03 v-value-count usage binary-double unsigned.
 03 v-value-first.
 04 v-value-first-value usage binary-double unsigned.
 02 v-probability.
 03 v-probability-scalar usage binary-double unsigned.
 03 v-probability-first.
 04 v-probability-first-present usage binary-double unsigned.
 04 v-probability-first-value usage float-long.
 03 v-probability-data usage pointer.
 03 v-probability-count usage binary-double unsigned.
 03 v-probability-lists usage pointer.
 03 v-probability-list-count usage binary-double unsigned.
 02 v-results.
 03 v-results-data usage pointer.
 03 v-results-len usage binary-double unsigned.
 02 v-facts-present usage binary-double unsigned.
 02 v-facts.
 03 v-facts-attempts.
 04 v-facts-attempts-presence usage binary-long unsigned.
 04 filler pic x(4).
 04 v-facts-attempts-value.
 05 v-facts-attempts-value-data usage pointer.
 05 v-facts-attempts-value-len usage binary-double unsigned.
 03 v-facts-cache-answers usage binary-double unsigned.
 03 v-facts-call-id usage pointer.
 03 v-facts-estimated-cost-usd.
 04 v-facts-estimated-cost-usd-presence usage binary-long unsigned.
 04 filler pic x(4).
 04 v-facts-estimated-cost-usd-value.
 05 v-facts-estimated-cost-usd-value-length usage binary-double unsigned.
 05 v-facts-estimated-cost-usd-value-text pic x(8192).
 03 v-facts-held-model-mismatch.
 04 v-facts-held-model-mismatch-presence usage binary-long unsigned.
 04 v-facts-held-model-mismatch-value usage binary-long unsigned.
 03 v-facts-input-tokens.
 04 v-facts-input-tokens-presence usage binary-long unsigned.
 04 filler pic x(4).
 04 v-facts-input-tokens-value usage binary-double unsigned.
 03 v-facts-largest-request-bytes usage binary-double unsigned.
 03 v-facts-largest-request-estimated-input-tokens.
 04 v-facts-largest-request-estimated-input-tokens-presence usage binary-long unsigned.
 04 filler pic x(4).
 04 v-facts-largest-request-estimated-input-tokens-value usage binary-double unsigned.
 03 v-facts-model.
 04 v-facts-model-presence usage binary-long unsigned.
 04 filler pic x(4).
 04 v-facts-model-value.
 05 v-facts-model-value-length usage binary-double unsigned.
 05 v-facts-model-value-text pic x(8192).
 03 v-facts-output-tokens.
 04 v-facts-output-tokens-presence usage binary-long unsigned.
 04 filler pic x(4).
 04 v-facts-output-tokens-value usage binary-double unsigned.
 03 v-facts-records usage binary-double unsigned.
 03 v-facts-requests-sent usage binary-double unsigned.
 03 v-facts-seconds usage float-long.
 03 v-facts-token-estimate-method.
 04 v-facts-token-estimate-method-length usage binary-double unsigned.
 04 v-facts-token-estimate-method-text pic x(8192).
 03 v-facts-usage-persistence.
 04 v-facts-usage-persistence-presence usage binary-long unsigned.
 04 filler pic x(4).
 04 v-facts-usage-persistence-value usage pointer.
 03 v-facts-extensions.
 04 v-facts-extensions-data usage pointer.
 04 v-facts-extensions-len usage binary-double unsigned.
01 tt-n-cobol-AnswerFindValue based.
 02 v-scalar usage binary-double unsigned.
 02 v-rows usage pointer.
 02 v-count usage binary-double unsigned.
 02 v-first.
 03 v-first-present usage binary-double unsigned.
 03 v-first-index usage binary-double unsigned.
 03 v-first-original usage pointer.
01 tt-n-cobol-AnswerFindResult based.
 02 v-owner usage pointer.
 02 v-value.
 03 v-value-scalar usage binary-double unsigned.
 03 v-value-rows usage pointer.
 03 v-value-count usage binary-double unsigned.
 03 v-value-first.
 04 v-value-first-present usage binary-double unsigned.
 04 v-value-first-index usage binary-double unsigned.
 04 v-value-first-original usage pointer.
 02 v-probability.
 03 v-probability-scalar usage binary-double unsigned.
 03 v-probability-first.
 04 v-probability-first-present usage binary-double unsigned.
 04 v-probability-first-value usage float-long.
 03 v-probability-data usage pointer.
 03 v-probability-count usage binary-double unsigned.
 03 v-probability-lists usage pointer.
 03 v-probability-list-count usage binary-double unsigned.
 02 v-results.
 03 v-results-data usage pointer.
 03 v-results-len usage binary-double unsigned.
 02 v-facts-present usage binary-double unsigned.
 02 v-facts.
 03 v-facts-attempts.
 04 v-facts-attempts-presence usage binary-long unsigned.
 04 filler pic x(4).
 04 v-facts-attempts-value.
 05 v-facts-attempts-value-data usage pointer.
 05 v-facts-attempts-value-len usage binary-double unsigned.
 03 v-facts-cache-answers usage binary-double unsigned.
 03 v-facts-call-id usage pointer.
 03 v-facts-estimated-cost-usd.
 04 v-facts-estimated-cost-usd-presence usage binary-long unsigned.
 04 filler pic x(4).
 04 v-facts-estimated-cost-usd-value.
 05 v-facts-estimated-cost-usd-value-length usage binary-double unsigned.
 05 v-facts-estimated-cost-usd-value-text pic x(8192).
 03 v-facts-held-model-mismatch.
 04 v-facts-held-model-mismatch-presence usage binary-long unsigned.
 04 v-facts-held-model-mismatch-value usage binary-long unsigned.
 03 v-facts-input-tokens.
 04 v-facts-input-tokens-presence usage binary-long unsigned.
 04 filler pic x(4).
 04 v-facts-input-tokens-value usage binary-double unsigned.
 03 v-facts-largest-request-bytes usage binary-double unsigned.
 03 v-facts-largest-request-estimated-input-tokens.
 04 v-facts-largest-request-estimated-input-tokens-presence usage binary-long unsigned.
 04 filler pic x(4).
 04 v-facts-largest-request-estimated-input-tokens-value usage binary-double unsigned.
 03 v-facts-model.
 04 v-facts-model-presence usage binary-long unsigned.
 04 filler pic x(4).
 04 v-facts-model-value.
 05 v-facts-model-value-length usage binary-double unsigned.
 05 v-facts-model-value-text pic x(8192).
 03 v-facts-output-tokens.
 04 v-facts-output-tokens-presence usage binary-long unsigned.
 04 filler pic x(4).
 04 v-facts-output-tokens-value usage binary-double unsigned.
 03 v-facts-records usage binary-double unsigned.
 03 v-facts-requests-sent usage binary-double unsigned.
 03 v-facts-seconds usage float-long.
 03 v-facts-token-estimate-method.
 04 v-facts-token-estimate-method-length usage binary-double unsigned.
 04 v-facts-token-estimate-method-text pic x(8192).
 03 v-facts-usage-persistence.
 04 v-facts-usage-persistence-presence usage binary-long unsigned.
 04 filler pic x(4).
 04 v-facts-usage-persistence-value usage pointer.
 03 v-facts-extensions.
 04 v-facts-extensions-data usage pointer.
 04 v-facts-extensions-len usage binary-double unsigned.
01 tt-n-cobol-AnswerAnnotateValue based.
 02 v-scalar usage binary-double unsigned.
 02 v-rows usage pointer.
 02 v-count usage binary-double unsigned.
 02 v-first.
 03 v-first-record usage binary-double unsigned.
 03 v-first-name.
 04 v-first-name-length usage binary-double unsigned.
 04 v-first-name-text pic x(8192).
 03 v-first-kind usage binary-double unsigned.
 03 v-first-boolean usage binary-double unsigned.
 03 v-first-number usage float-long.
 03 v-first-text.
 04 v-first-text-length usage binary-double unsigned.
 04 v-first-text-text pic x(8192).
 03 v-first-tags usage pointer.
 03 v-first-tag-count usage binary-double unsigned.
 03 v-first-failure-kind usage binary-double unsigned.
 03 v-first-failure-cause usage binary-double unsigned.
01 tt-n-cobol-AnswerAnnotateResult based.
 02 v-owner usage pointer.
 02 v-value.
 03 v-value-scalar usage binary-double unsigned.
 03 v-value-rows usage pointer.
 03 v-value-count usage binary-double unsigned.
 03 v-value-first.
 04 v-value-first-record usage binary-double unsigned.
 04 v-value-first-name.
 05 v-value-first-name-length usage binary-double unsigned.
 05 v-value-first-name-text pic x(8192).
 04 v-value-first-kind usage binary-double unsigned.
 04 v-value-first-boolean usage binary-double unsigned.
 04 v-value-first-number usage float-long.
 04 v-value-first-text.
 05 v-value-first-text-length usage binary-double unsigned.
 05 v-value-first-text-text pic x(8192).
 04 v-value-first-tags usage pointer.
 04 v-value-first-tag-count usage binary-double unsigned.
 04 v-value-first-failure-kind usage binary-double unsigned.
 04 v-value-first-failure-cause usage binary-double unsigned.
 02 v-probability.
 03 v-probability-scalar usage binary-double unsigned.
 03 v-probability-first.
 04 v-probability-first-present usage binary-double unsigned.
 04 v-probability-first-value usage float-long.
 03 v-probability-data usage pointer.
 03 v-probability-count usage binary-double unsigned.
 03 v-probability-lists usage pointer.
 03 v-probability-list-count usage binary-double unsigned.
 02 v-results.
 03 v-results-data usage pointer.
 03 v-results-len usage binary-double unsigned.
 02 v-facts-present usage binary-double unsigned.
 02 v-facts.
 03 v-facts-attempts.
 04 v-facts-attempts-presence usage binary-long unsigned.
 04 filler pic x(4).
 04 v-facts-attempts-value.
 05 v-facts-attempts-value-data usage pointer.
 05 v-facts-attempts-value-len usage binary-double unsigned.
 03 v-facts-cache-answers usage binary-double unsigned.
 03 v-facts-call-id usage pointer.
 03 v-facts-estimated-cost-usd.
 04 v-facts-estimated-cost-usd-presence usage binary-long unsigned.
 04 filler pic x(4).
 04 v-facts-estimated-cost-usd-value.
 05 v-facts-estimated-cost-usd-value-length usage binary-double unsigned.
 05 v-facts-estimated-cost-usd-value-text pic x(8192).
 03 v-facts-held-model-mismatch.
 04 v-facts-held-model-mismatch-presence usage binary-long unsigned.
 04 v-facts-held-model-mismatch-value usage binary-long unsigned.
 03 v-facts-input-tokens.
 04 v-facts-input-tokens-presence usage binary-long unsigned.
 04 filler pic x(4).
 04 v-facts-input-tokens-value usage binary-double unsigned.
 03 v-facts-largest-request-bytes usage binary-double unsigned.
 03 v-facts-largest-request-estimated-input-tokens.
 04 v-facts-largest-request-estimated-input-tokens-presence usage binary-long unsigned.
 04 filler pic x(4).
 04 v-facts-largest-request-estimated-input-tokens-value usage binary-double unsigned.
 03 v-facts-model.
 04 v-facts-model-presence usage binary-long unsigned.
 04 filler pic x(4).
 04 v-facts-model-value.
 05 v-facts-model-value-length usage binary-double unsigned.
 05 v-facts-model-value-text pic x(8192).
 03 v-facts-output-tokens.
 04 v-facts-output-tokens-presence usage binary-long unsigned.
 04 filler pic x(4).
 04 v-facts-output-tokens-value usage binary-double unsigned.
 03 v-facts-records usage binary-double unsigned.
 03 v-facts-requests-sent usage binary-double unsigned.
 03 v-facts-seconds usage float-long.
 03 v-facts-token-estimate-method.
 04 v-facts-token-estimate-method-length usage binary-double unsigned.
 04 v-facts-token-estimate-method-text pic x(8192).
 03 v-facts-usage-persistence.
 04 v-facts-usage-persistence-presence usage binary-long unsigned.
 04 filler pic x(4).
 04 v-facts-usage-persistence-value usage pointer.
 03 v-facts-extensions.
 04 v-facts-extensions-data usage pointer.
 04 v-facts-extensions-len usage binary-double unsigned.
01 tt-n-cobol-AnswerRecognizeValue based.
 02 v-scalar usage binary-double unsigned.
 02 v-rows usage pointer.
 02 v-count usage binary-double unsigned.
 02 v-first.
 03 v-first-record usage binary-double unsigned.
 03 v-first-proposal usage binary-double unsigned.
 03 v-first-kind.
 04 v-first-kind-length usage binary-double unsigned.
 04 v-first-kind-text pic x(8192).
 03 v-first-text.
 04 v-first-text-length usage binary-double unsigned.
 04 v-first-text-text pic x(8192).
 03 v-first-start usage binary-double unsigned.
 03 v-first-end usage binary-double unsigned.
 03 v-first-length usage binary-double unsigned.
 03 v-first-strength usage float-long.
 03 v-first-mode usage binary-double unsigned.
 03 v-first-file-present usage binary-double unsigned.
 03 v-first-file.
 04 v-first-file-length usage binary-double unsigned.
 04 v-first-file-text pic x(8192).
 03 v-first-first-line-present usage binary-double unsigned.
 03 v-first-first-line usage binary-double unsigned.
 03 v-first-last-line-present usage binary-double unsigned.
 03 v-first-last-line usage binary-double unsigned.
 03 v-first-extensions usage pointer.
 02 v-edges usage pointer.
 02 v-edge-count usage binary-double unsigned.
01 tt-n-cobol-AnswerRecognizeResult based.
 02 v-owner usage pointer.
 02 v-value.
 03 v-value-scalar usage binary-double unsigned.
 03 v-value-rows usage pointer.
 03 v-value-count usage binary-double unsigned.
 03 v-value-first.
 04 v-value-first-record usage binary-double unsigned.
 04 v-value-first-proposal usage binary-double unsigned.
 04 v-value-first-kind.
 05 v-value-first-kind-length usage binary-double unsigned.
 05 v-value-first-kind-text pic x(8192).
 04 v-value-first-text.
 05 v-value-first-text-length usage binary-double unsigned.
 05 v-value-first-text-text pic x(8192).
 04 v-value-first-start usage binary-double unsigned.
 04 v-value-first-end usage binary-double unsigned.
 04 v-value-first-length usage binary-double unsigned.
 04 v-value-first-strength usage float-long.
 04 v-value-first-mode usage binary-double unsigned.
 04 v-value-first-file-present usage binary-double unsigned.
 04 v-value-first-file.
 05 v-value-first-file-length usage binary-double unsigned.
 05 v-value-first-file-text pic x(8192).
 04 v-value-first-first-line-present usage binary-double unsigned.
 04 v-value-first-first-line usage binary-double unsigned.
 04 v-value-first-last-line-present usage binary-double unsigned.
 04 v-value-first-last-line usage binary-double unsigned.
 04 v-value-first-extensions usage pointer.
 03 v-value-edges usage pointer.
 03 v-value-edge-count usage binary-double unsigned.
 02 v-probability.
 03 v-probability-scalar usage binary-double unsigned.
 03 v-probability-first.
 04 v-probability-first-present usage binary-double unsigned.
 04 v-probability-first-value usage float-long.
 03 v-probability-data usage pointer.
 03 v-probability-count usage binary-double unsigned.
 03 v-probability-lists usage pointer.
 03 v-probability-list-count usage binary-double unsigned.
 02 v-results.
 03 v-results-data usage pointer.
 03 v-results-len usage binary-double unsigned.
 02 v-facts-present usage binary-double unsigned.
 02 v-facts.
 03 v-facts-attempts.
 04 v-facts-attempts-presence usage binary-long unsigned.
 04 filler pic x(4).
 04 v-facts-attempts-value.
 05 v-facts-attempts-value-data usage pointer.
 05 v-facts-attempts-value-len usage binary-double unsigned.
 03 v-facts-cache-answers usage binary-double unsigned.
 03 v-facts-call-id usage pointer.
 03 v-facts-estimated-cost-usd.
 04 v-facts-estimated-cost-usd-presence usage binary-long unsigned.
 04 filler pic x(4).
 04 v-facts-estimated-cost-usd-value.
 05 v-facts-estimated-cost-usd-value-length usage binary-double unsigned.
 05 v-facts-estimated-cost-usd-value-text pic x(8192).
 03 v-facts-held-model-mismatch.
 04 v-facts-held-model-mismatch-presence usage binary-long unsigned.
 04 v-facts-held-model-mismatch-value usage binary-long unsigned.
 03 v-facts-input-tokens.
 04 v-facts-input-tokens-presence usage binary-long unsigned.
 04 filler pic x(4).
 04 v-facts-input-tokens-value usage binary-double unsigned.
 03 v-facts-largest-request-bytes usage binary-double unsigned.
 03 v-facts-largest-request-estimated-input-tokens.
 04 v-facts-largest-request-estimated-input-tokens-presence usage binary-long unsigned.
 04 filler pic x(4).
 04 v-facts-largest-request-estimated-input-tokens-value usage binary-double unsigned.
 03 v-facts-model.
 04 v-facts-model-presence usage binary-long unsigned.
 04 filler pic x(4).
 04 v-facts-model-value.
 05 v-facts-model-value-length usage binary-double unsigned.
 05 v-facts-model-value-text pic x(8192).
 03 v-facts-output-tokens.
 04 v-facts-output-tokens-presence usage binary-long unsigned.
 04 filler pic x(4).
 04 v-facts-output-tokens-value usage binary-double unsigned.
 03 v-facts-records usage binary-double unsigned.
 03 v-facts-requests-sent usage binary-double unsigned.
 03 v-facts-seconds usage float-long.
 03 v-facts-token-estimate-method.
 04 v-facts-token-estimate-method-length usage binary-double unsigned.
 04 v-facts-token-estimate-method-text pic x(8192).
 03 v-facts-usage-persistence.
 04 v-facts-usage-persistence-presence usage binary-long unsigned.
 04 filler pic x(4).
 04 v-facts-usage-persistence-value usage pointer.
 03 v-facts-extensions.
 04 v-facts-extensions-data usage pointer.
 04 v-facts-extensions-len usage binary-double unsigned.
01 tt-n-cobol-AnswerRelateValue based.
 02 v-scalar usage binary-double unsigned.
 02 v-rows usage pointer.
 02 v-count usage binary-double unsigned.
 02 v-first.
 03 v-first-record usage binary-double unsigned.
 03 v-first-relation.
 04 v-first-relation-length usage binary-double unsigned.
 04 v-first-relation-text pic x(8192).
 03 v-first-source.
 04 v-first-source-length usage binary-double unsigned.
 04 v-first-source-text pic x(8192).
 03 v-first-source-kind.
 04 v-first-source-kind-length usage binary-double unsigned.
 04 v-first-source-kind-text pic x(8192).
 03 v-first-target.
 04 v-first-target-length usage binary-double unsigned.
 04 v-first-target-text pic x(8192).
 03 v-first-target-kind.
 04 v-first-target-kind-length usage binary-double unsigned.
 04 v-first-target-kind-text pic x(8192).
 03 v-first-probability usage float-long.
 03 v-first-either-present usage binary-double unsigned.
 03 v-first-either usage binary-double unsigned.
01 tt-n-cobol-AnswerRelateResult based.
 02 v-owner usage pointer.
 02 v-value.
 03 v-value-scalar usage binary-double unsigned.
 03 v-value-rows usage pointer.
 03 v-value-count usage binary-double unsigned.
 03 v-value-first.
 04 v-value-first-record usage binary-double unsigned.
 04 v-value-first-relation.
 05 v-value-first-relation-length usage binary-double unsigned.
 05 v-value-first-relation-text pic x(8192).
 04 v-value-first-source.
 05 v-value-first-source-length usage binary-double unsigned.
 05 v-value-first-source-text pic x(8192).
 04 v-value-first-source-kind.
 05 v-value-first-source-kind-length usage binary-double unsigned.
 05 v-value-first-source-kind-text pic x(8192).
 04 v-value-first-target.
 05 v-value-first-target-length usage binary-double unsigned.
 05 v-value-first-target-text pic x(8192).
 04 v-value-first-target-kind.
 05 v-value-first-target-kind-length usage binary-double unsigned.
 05 v-value-first-target-kind-text pic x(8192).
 04 v-value-first-probability usage float-long.
 04 v-value-first-either-present usage binary-double unsigned.
 04 v-value-first-either usage binary-double unsigned.
 02 v-probability.
 03 v-probability-scalar usage binary-double unsigned.
 03 v-probability-first.
 04 v-probability-first-present usage binary-double unsigned.
 04 v-probability-first-value usage float-long.
 03 v-probability-data usage pointer.
 03 v-probability-count usage binary-double unsigned.
 03 v-probability-lists usage pointer.
 03 v-probability-list-count usage binary-double unsigned.
 02 v-results.
 03 v-results-data usage pointer.
 03 v-results-len usage binary-double unsigned.
 02 v-facts-present usage binary-double unsigned.
 02 v-facts.
 03 v-facts-attempts.
 04 v-facts-attempts-presence usage binary-long unsigned.
 04 filler pic x(4).
 04 v-facts-attempts-value.
 05 v-facts-attempts-value-data usage pointer.
 05 v-facts-attempts-value-len usage binary-double unsigned.
 03 v-facts-cache-answers usage binary-double unsigned.
 03 v-facts-call-id usage pointer.
 03 v-facts-estimated-cost-usd.
 04 v-facts-estimated-cost-usd-presence usage binary-long unsigned.
 04 filler pic x(4).
 04 v-facts-estimated-cost-usd-value.
 05 v-facts-estimated-cost-usd-value-length usage binary-double unsigned.
 05 v-facts-estimated-cost-usd-value-text pic x(8192).
 03 v-facts-held-model-mismatch.
 04 v-facts-held-model-mismatch-presence usage binary-long unsigned.
 04 v-facts-held-model-mismatch-value usage binary-long unsigned.
 03 v-facts-input-tokens.
 04 v-facts-input-tokens-presence usage binary-long unsigned.
 04 filler pic x(4).
 04 v-facts-input-tokens-value usage binary-double unsigned.
 03 v-facts-largest-request-bytes usage binary-double unsigned.
 03 v-facts-largest-request-estimated-input-tokens.
 04 v-facts-largest-request-estimated-input-tokens-presence usage binary-long unsigned.
 04 filler pic x(4).
 04 v-facts-largest-request-estimated-input-tokens-value usage binary-double unsigned.
 03 v-facts-model.
 04 v-facts-model-presence usage binary-long unsigned.
 04 filler pic x(4).
 04 v-facts-model-value.
 05 v-facts-model-value-length usage binary-double unsigned.
 05 v-facts-model-value-text pic x(8192).
 03 v-facts-output-tokens.
 04 v-facts-output-tokens-presence usage binary-long unsigned.
 04 filler pic x(4).
 04 v-facts-output-tokens-value usage binary-double unsigned.
 03 v-facts-records usage binary-double unsigned.
 03 v-facts-requests-sent usage binary-double unsigned.
 03 v-facts-seconds usage float-long.
 03 v-facts-token-estimate-method.
 04 v-facts-token-estimate-method-length usage binary-double unsigned.
 04 v-facts-token-estimate-method-text pic x(8192).
 03 v-facts-usage-persistence.
 04 v-facts-usage-persistence-presence usage binary-long unsigned.
 04 filler pic x(4).
 04 v-facts-usage-persistence-value usage pointer.
 03 v-facts-extensions.
 04 v-facts-extensions-data usage pointer.
 04 v-facts-extensions-len usage binary-double unsigned.
78 tt-n-cobol-annotatequestionchoose-kind-k value 2.
78 tt-n-cobol-annotatequestiondecide-kind-k value 1.
78 tt-n-cobol-annotatequestionscore-kind-k value 4.
78 tt-n-cobol-annotatequestiontag-kind-k value 3.
78 tt-n-cobol-cachedocument-disabledcache-kind-k value 2.
78 tt-n-cobol-cachedocument-text-kind-k value 1.
78 tt-n-cobol-contextschema-object-kind-k value 2.
78 tt-n-cobol-contextschema-text-kind-k value 1.
78 tt-n-cobol-criterion-array-kind-k value 3.
78 tt-n-cobol-criterion-null-kind-k value 4.
78 tt-n-cobol-criterion-object-kind-k value 2.
78 tt-n-cobol-criterion-text-kind-k value 1.
78 tt-n-cobol-cut-number-kind-k value 1.
78 tt-n-cobol-cut-text-kind-k value 2.
78 tt-n-cobol-definitionannotatedefinition-kind-k value 8.
78 tt-n-cobol-definitionchoose-kind-k value 2.
78 tt-n-cobol-definitiondecide-kind-k value 1.
78 tt-n-cobol-definitionfind-kind-k value 6.
78 tt-n-cobol-definitionrecognizedefinition-kind-k value 7.
78 tt-n-cobol-definitionrelate-kind-k value 5.
78 tt-n-cobol-definitionscore-kind-k value 4.
78 tt-n-cobol-definitiontag-kind-k value 3.
78 tt-n-cobol-description-array-kind-k value 3.
78 tt-n-cobol-description-null-kind-k value 4.
78 tt-n-cobol-description-object-kind-k value 2.
78 tt-n-cobol-description-text-kind-k value 1.
78 tt-n-cobol-imagemedia-image-jpeg-kind-k value 1.
78 tt-n-cobol-imagemedia-image-png-kind-k value 2.
78 tt-n-cobol-inputdeclarationobject-kind-k value 2.
78 tt-n-cobol-inputdeclarationstring-kind-k value 1.
78 tt-n-cobol-inputpropertyarray-kind-k value 4.
78 tt-n-cobol-inputpropertyboolean-kind-k value 3.
78 tt-n-cobol-inputpropertynumber-kind-k value 2.
78 tt-n-cobol-inputpropertystring-kind-k value 1.
78 tt-n-cobol-labels-array-kind-k value 1.
78 tt-n-cobol-labels-object-kind-k value 2.
78 tt-n-cobol-levels-array-kind-k value 1.
78 tt-n-cobol-levels-object-kind-k value 2.
78 tt-n-cobol-options-array-kind-k value 1.
78 tt-n-cobol-options-object-kind-k value 2.
78 tt-n-cobol-overflow-k value 1002.
78 tt-n-cobol-pointers-array-kind-k value 2.
78 tt-n-cobol-pointers-text-kind-k value 1.
78 tt-n-cobol-questionbatch-integer-kind-k value 2.
78 tt-n-cobol-questionbatch-max-kind-k value 1.
78 tt-n-cobol-questiontext-array-kind-k value 3.
78 tt-n-cobol-questiontext-object-kind-k value 2.
78 tt-n-cobol-questiontext-text-kind-k value 1.
78 tt-n-cobol-readermedia-image-kind-k value 2.
78 tt-n-cobol-readermedia-text-kind-k value 1.
78 tt-n-cobol-recognitionexample-recognitionexampletext-kind-k value 2.
78 tt-n-cobol-recognitionexample-text-kind-k value 1.
78 tt-n-cobol-recognitionmode-boundary-only-kind-k value 2.
78 tt-n-cobol-recognitionmode-whole-kind-k value 1.
78 tt-n-cobol-representation-k value 1001.
78 tt-n-cobol-requestbatch-integer-kind-k value 1.
78 tt-n-cobol-requestbatch-text-kind-k value 2.
78 tt-n-cobol-requestframing-csv-kind-k value 4.
78 tt-n-cobol-requestframing-document-kind-k value 1.
78 tt-n-cobol-requestframing-jsonl-kind-k value 3.
78 tt-n-cobol-requestframing-lines-kind-k value 2.
78 tt-n-cobol-requestframing-tsv-kind-k value 5.
78 tt-n-cobol-requestfunction-annotate-kind-k value 8.
78 tt-n-cobol-requestfunction-choose-kind-k value 2.
78 tt-n-cobol-requestfunction-decide-kind-k value 1.
78 tt-n-cobol-requestfunction-filter-kind-k value 5.
78 tt-n-cobol-requestfunction-find-kind-k value 7.
78 tt-n-cobol-requestfunction-rank-kind-k value 6.
78 tt-n-cobol-requestfunction-recognize-kind-k value 9.
78 tt-n-cobol-requestfunction-relate-kind-k value 10.
78 tt-n-cobol-requestfunction-score-kind-k value 4.
78 tt-n-cobol-requestfunction-tag-kind-k value 3.
78 tt-n-cobol-requestimagebytes-kind-k value 2.
78 tt-n-cobol-requestimagefile-kind-k value 1.
78 tt-n-cobol-requestinputentities-kind-k value 5.
78 tt-n-cobol-requestinputfeed-kind-k value 7.
78 tt-n-cobol-requestinputjson-kind-k value 2.
78 tt-n-cobol-requestinputrecords-kind-k value 3.
78 tt-n-cobol-requestinputsource-kind-k value 6.
78 tt-n-cobol-requestinputtext-kind-k value 1.
78 tt-n-cobol-requestinputunits-kind-k value 4.
78 tt-n-cobol-requestoriginaljson-kind-k value 2.
78 tt-n-cobol-requestoriginaltext-kind-k value 1.
78 tt-n-cobol-requestquestiondefinition-kind-k value 2.
78 tt-n-cobol-requestquestionfile-kind-k value 3.
78 tt-n-cobol-requestquestionname-kind-k value 4.
78 tt-n-cobol-requestquestionreference-kind-k value 5.
78 tt-n-cobol-requestquestiontext-kind-k value 1.
78 tt-n-cobol-requestreaderfailureinvalidinput-kind-k value 4.
78 tt-n-cobol-requestreaderfailureio-kind-k value 2.
78 tt-n-cobol-requestreaderfailureproducer-kind-k value 1.
78 tt-n-cobol-requestreaderfailureutf8-kind-k value 3.
78 tt-n-cobol-requestthreshold-number-kind-k value 1.
78 tt-n-cobol-requestthreshold-text-kind-k value 2.
78 tt-n-cobol-settingsinputtokenbudget-integer-kind-k value 1.
78 tt-n-cobol-settingsinputtokenbudget-null-kind-k value 2.
78 tt-n-cobol-settingsmaxrequeststotal-integer-kind-k value 1.
78 tt-n-cobol-settingsmaxrequeststotal-null-kind-k value 2.
78 tt-n-cobol-settingsmaxrequests-integer-kind-k value 1.
78 tt-n-cobol-settingsmaxrequests-null-kind-k value 2.
78 tt-n-cobol-sourceunit-file-kind-k value 3.
78 tt-n-cobol-sourceunit-line-kind-k value 1.
78 tt-n-cobol-sourceunit-window-kind-k value 2.
78 tt-n-cobol-text-capacity-k value 8192.
78 tt-n-cobol-threshold-number-kind-k value 1.
78 tt-n-cobol-threshold-text-kind-k value 2.
78 tt-n-complete-annotated-field-array-v1-k value 4.
78 tt-n-complete-annotated-field-boolean-v1-k value 1.
78 tt-n-complete-annotated-field-null-v1-k value 2.
78 tt-n-complete-annotated-field-number-v1-k value 5.
78 tt-n-complete-annotated-field-object-v1-k value 6.
78 tt-n-complete-annotated-field-string-v1-k value 3.
78 tt-n-complete-annotation-member-answer-id-v1-k value 1.
78 tt-n-complete-annotation-member-failure-id-v1-k value 2.
78 tt-n-complete-annotation-value-choice-v1-k value 2.
78 tt-n-complete-annotation-value-decision-v1-k value 1.
78 tt-n-complete-annotation-value-failed-v1-k value 5.
78 tt-n-complete-annotation-value-score-v1-k value 3.
78 tt-n-complete-annotation-value-tags-v1-k value 4.
78 tt-n-complete-answer-choice-v1-k value 2.
78 tt-n-complete-answer-score-v1-k value 4.
78 tt-n-complete-answer-tag-v1-k value 3.
78 tt-n-complete-answer-yes-no-v1-k value 1.
78 tt-n-complete-attempt-outcome-ok-v1-k value 1.
78 tt-n-complete-attempt-outcome-status-v1-k value 2.
78 tt-n-complete-attempt-outcome-transport-v1-k value 3.
78 tt-n-complete-batch-integer-v1-k value 1.
78 tt-n-complete-batch-setting-integer-v1-k value 1.
78 tt-n-complete-batch-setting-string-v1-k value 2.
78 tt-n-complete-batch-string-max-v1-k value 1.
78 tt-n-complete-batch-string-v1-k value 2.
78 tt-n-complete-boundary-mode-boundary-only-v1-k value 1.
78 tt-n-complete-failure-cause-invalid-distribution-v1-k value 5.
78 tt-n-complete-failure-cause-invalid-probability-v1-k value 4.
78 tt-n-complete-failure-cause-missing-answer-v1-k value 1.
78 tt-n-complete-failure-cause-missing-probability-v1-k value 3.
78 tt-n-complete-failure-cause-unexpected-probability-v1-k value 6.
78 tt-n-complete-failure-cause-wrong-kind-v1-k value 2.
78 tt-n-complete-failure-field-kind-backend-v1-k value 1.
78 tt-n-complete-failure-kind-backend-v1-k value 2.
78 tt-n-complete-failure-kind-cancelled-v1-k value 4.
78 tt-n-complete-failure-kind-deadline-v1-k value 5.
78 tt-n-complete-failure-kind-defect-v1-k value 6.
78 tt-n-complete-failure-kind-local-v1-k value 3.
78 tt-n-complete-failure-kind-usage-v1-k value 1.
78 tt-n-complete-image-media-image-jpeg-v1-k value 1.
78 tt-n-complete-image-media-image-png-v1-k value 2.
78 tt-n-complete-input-declaration-object-root-v1-k value 2.
78 tt-n-complete-input-declaration-string-root-v1-k value 1.
78 tt-n-complete-input-denial-additional-request-v1-k value 2.
78 tt-n-complete-input-denial-initial-request-v1-k value 1.
78 tt-n-complete-input-denial-retry-v1-k value 3.
78 tt-n-complete-input-property-type-array-v1-k value 4.
78 tt-n-complete-input-property-type-boolean-v1-k value 3.
78 tt-n-complete-input-property-type-number-v1-k value 2.
78 tt-n-complete-input-property-type-string-v1-k value 1.
78 tt-n-complete-json-array-v1-k value 5.
78 tt-n-complete-json-boolean-v1-k value 2.
78 tt-n-complete-json-null-v1-k value 1.
78 tt-n-complete-json-number-v1-k value 3.
78 tt-n-complete-json-object-v1-k value 6.
78 tt-n-complete-json-string-v1-k value 4.
78 tt-n-complete-object-type-object-v1-k value 1.
78 tt-n-complete-observation-failure-id-v1-k value 2.
78 tt-n-complete-observation-observation-id-v1-k value 1.
78 tt-n-complete-origin-cache-v1-k value 2.
78 tt-n-complete-origin-live-v1-k value 1.
78 tt-n-complete-origin-memory-v1-k value 5.
78 tt-n-complete-origin-proxy-v1-k value 4.
78 tt-n-complete-origin-replay-v1-k value 3.
78 tt-n-complete-presence-missing-v1-k value 0.
78 tt-n-complete-presence-null-v1-k value 1.
78 tt-n-complete-presence-value-v1-k value 2.
78 tt-n-complete-question-document-choose-v1-k value 2.
78 tt-n-complete-question-document-decide-v1-k value 1.
78 tt-n-complete-question-document-score-v1-k value 4.
78 tt-n-complete-question-document-tag-v1-k value 3.
78 tt-n-complete-recognition-mode-boundary-only-v1-k value 2.
78 tt-n-complete-recognition-mode-whole-v1-k value 1.
78 tt-n-complete-recognition-odds-boundary-odds-v1-k value 2.
78 tt-n-complete-recognition-odds-recognize-answer-v1-k value 1.
78 tt-n-complete-recognize-entities-v1-k value 1.
78 tt-n-complete-recognize-mode-proposals-v1-k value 2.
78 tt-n-complete-relation-direction-either-v1-k value 2.
78 tt-n-complete-relation-direction-source-to-target-v1-k value 1.
78 tt-n-complete-relation-endpoint-related-entity-v1-k value 1.
78 tt-n-complete-relation-endpoint-source-v1-k value 2.
78 tt-n-complete-relation-member-answer-id-v1-k value 1.
78 tt-n-complete-relation-member-failure-id-v1-k value 2.
78 tt-n-complete-relation-method-choice-v1-k value 2.
78 tt-n-complete-relation-method-yes-no-v1-k value 1.
78 tt-n-complete-request-function-annotate-v1-k value 8.
78 tt-n-complete-request-function-choose-v1-k value 2.
78 tt-n-complete-request-function-decide-v1-k value 1.
78 tt-n-complete-request-function-filter-v1-k value 5.
78 tt-n-complete-request-function-find-v1-k value 7.
78 tt-n-complete-request-function-rank-v1-k value 6.
78 tt-n-complete-request-function-recognize-v1-k value 9.
78 tt-n-complete-request-function-relate-v1-k value 10.
78 tt-n-complete-request-function-score-v1-k value 4.
78 tt-n-complete-request-function-tag-v1-k value 3.
78 tt-n-complete-send-denial-before-additional-send-v1-k value 2.
78 tt-n-complete-send-denial-before-first-send-v1-k value 1.
78 tt-n-complete-send-denial-before-retry-v1-k value 3.
78 tt-n-complete-session-judgment-choice-v1-k value 2.
78 tt-n-complete-session-judgment-decision-v1-k value 1.
78 tt-n-complete-session-judgment-score-v1-k value 3.
78 tt-n-complete-session-judgment-tags-v1-k value 4.
78 tt-n-complete-session-observation-question-v1-k value 1.
78 tt-n-complete-session-observation-row-v1-k value 2.
78 tt-n-complete-session-observed-row-annotated-v1-k value 2.
78 tt-n-complete-session-observed-row-find-v1-k value 4.
78 tt-n-complete-session-observed-row-judgment-v1-k value 1.
78 tt-n-complete-session-observed-row-recognized-v1-k value 3.
78 tt-n-complete-session-observed-row-relations-v1-k value 5.
78 tt-n-complete-session-packet-annotate-aggregate-v1-k value 14.
78 tt-n-complete-session-packet-annotate-row-v1-k value 6.
78 tt-n-complete-session-packet-choose-aggregate-v1-k value 8.
78 tt-n-complete-session-packet-choose-row-v1-k value 2.
78 tt-n-complete-session-packet-decide-aggregate-v1-k value 7.
78 tt-n-complete-session-packet-decide-row-v1-k value 1.
78 tt-n-complete-session-packet-filter-aggregate-v1-k value 11.
78 tt-n-complete-session-packet-filter-row-v1-k value 5.
78 tt-n-complete-session-packet-find-aggregate-v1-k value 13.
78 tt-n-complete-session-packet-observation-v1-k value 17.
78 tt-n-complete-session-packet-rank-aggregate-v1-k value 12.
78 tt-n-complete-session-packet-recognize-aggregate-v1-k value 15.
78 tt-n-complete-session-packet-relate-aggregate-v1-k value 16.
78 tt-n-complete-session-packet-score-aggregate-v1-k value 10.
78 tt-n-complete-session-packet-score-row-v1-k value 4.
78 tt-n-complete-session-packet-tag-aggregate-v1-k value 9.
78 tt-n-complete-session-packet-tag-row-v1-k value 3.
78 tt-n-complete-session-packet-terminal-v1-k value 18.
78 tt-n-complete-session-probabilities-named-v1-k value 2.
78 tt-n-complete-session-probabilities-yes-no-v1-k value 1.
78 tt-n-complete-stop-cause-backend-v1-k value 8.
78 tt-n-complete-stop-cause-cancelled-v1-k value 9.
78 tt-n-complete-stop-cause-deadline-v1-k value 10.
78 tt-n-complete-stop-cause-defect-v1-k value 11.
78 tt-n-complete-stop-cause-local-v1-k value 2.
78 tt-n-complete-stop-cause-no-key-v1-k value 3.
78 tt-n-complete-stop-cause-reply-v1-k value 7.
78 tt-n-complete-stop-cause-status-v1-k value 5.
78 tt-n-complete-stop-cause-too-large-v1-k value 6.
78 tt-n-complete-stop-cause-transport-v1-k value 4.
78 tt-n-complete-stop-cause-usage-v1-k value 1.
78 tt-n-complete-string-type-string-v1-k value 1.
78 tt-n-complete-threshold-number-v1-k value 1.
78 tt-n-complete-threshold-string-v1-k value 2.
78 tt-n-complete-usage-persistence-disabled-v1-k value 1.
78 tt-n-complete-usage-persistence-failed-v1-k value 4.
78 tt-n-complete-usage-persistence-pending-v1-k value 2.
78 tt-n-complete-usage-persistence-written-v1-k value 3.
78 tt-n-complete-value-array-v1-k value 4.
78 tt-n-complete-value-boolean-v1-k value 1.
78 tt-n-complete-value-null-v1-k value 2.
78 tt-n-complete-value-number-v1-k value 5.
78 tt-n-complete-value-string-v1-k value 3.
78 tt-n-complete-verb-recognize-v1-k value 1.
78 tt-n-complete-version-thinkthen-result-2-v1-k value 1.
78 tt-n-declaration-absent-v1-k value 0.
78 tt-n-declaration-object-v1-k value 2.
78 tt-n-declaration-string-v1-k value 1.
78 tt-n-ebackend-k value 2.
78 tt-n-ecancelled-k value 5.
78 tt-n-edeadline-k value 3.
78 tt-n-edefect-k value 6.
78 tt-n-elocal-k value 4.
78 tt-n-eusage-k value 1.
78 tt-n-load-atomic-v1-k value 1.
78 tt-n-load-dynamic-choose-v1-k value 3.
78 tt-n-load-find-v1-k value 8.
78 tt-n-load-rank-set-v1-k value 7.
78 tt-n-load-rank-v1-k value 6.
78 tt-n-load-recognize-v1-k value 4.
78 tt-n-load-relate-v1-k value 5.
78 tt-n-load-set-v1-k value 2.
78 tt-n-no-k value 0.
78 tt-n-no-deadline-k value -1.
78 tt-n-ok-k value 0.
78 tt-n-property-boolean-v1-k value 3.
78 tt-n-property-number-v1-k value 2.
78 tt-n-property-string-list-v1-k value 4.
78 tt-n-property-string-v1-k value 1.
78 tt-n-session-accepted-v1-k value 0.
78 tt-n-session-closed-v1-k value 2.
78 tt-n-session-end-v1-k value 2.
78 tt-n-session-full-v1-k value 1.
78 tt-n-session-pending-v1-k value 1.
78 tt-n-session-ready-cancelled-v1-k value 4.
78 tt-n-session-ready-closed-v1-k value 2.
78 tt-n-session-ready-end-v1-k value 3.
78 tt-n-session-ready-input-v1-k value 1.
78 tt-n-session-ready-result-v1-k value 0.
78 tt-n-session-ready-timeout-v1-k value 5.
78 tt-n-session-result-v1-k value 0.
78 tt-n-session-wait-input-v1-k value 1.
78 tt-n-session-wait-interruptible-settlement-v1-k value 4.
78 tt-n-session-wait-result-v1-k value 0.
78 tt-n-session-wait-settlement-arm-v1-k value 3.
78 tt-n-session-wait-settlement-v1-k value 2.
78 tt-n-unsure-k value 2.
78 tt-n-version-major-k value 0.
78 tt-n-version-minor-k value 2.
78 tt-n-version-patch-k value 0.
78 tt-n-yes-k value 1.
