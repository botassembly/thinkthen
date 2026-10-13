      *> Generated from measured canonical C declarations; do not edit.
01 tt-n-complete-usage-persistence-v1 based.
 02 v-kind usage binary-long unsigned.
01 tt-n-complete-utf8-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-extension-v1 based.
 02 v-name.
 03 v-name-data usage pointer.
 03 v-name-len usage binary-double unsigned.
 02 v-json.
 03 v-json-data usage pointer.
 03 v-json-len usage binary-double unsigned.
01 tt-n-complete-extensions-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
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
01 tt-n-complete-u64-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage binary-double unsigned.
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
01 tt-n-complete-json-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-utf8-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-value-data usage pointer.
 03 v-value-len usage binary-double unsigned.
01 tt-n-complete-attempt-outcome-v1 based.
 02 v-kind usage binary-long unsigned.
01 tt-n-complete-sdk-request-id-v1 based.
 02 v-value.
 03 v-value-data usage pointer.
 03 v-value-len usage binary-double unsigned.
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
01 tt-n-complete-readable-question-decide-v1 based.
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
01 tt-n-complete-readable-question-choose-v1 based.
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
01 tt-n-complete-readable-question-tag-v1 based.
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
01 tt-n-complete-readable-question-score-v1 based.
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
01 tt-n-complete-readable-question-v1 based.
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
01 tt-n-complete-readable-question-2-v1 based.
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
01 tt-n-complete-u32-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 v-value usage binary-long unsigned.
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
01 tt-n-complete-readable-question-3-v1 based.
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
01 tt-n-complete-readable-question-4-v1 based.
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
01 tt-n-complete-call-id-v1 based.
 02 v-value.
 03 v-value-data usage pointer.
 03 v-value-len usage binary-double unsigned.
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
01 tt-n-cobol-ContextSchema based.
 02 v-kind usage binary-double unsigned.
 02 v-value usage pointer.
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
 02 v-m-details usage pointer.
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
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
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
01 tt-n-cobol-ChooseBatch based.
 02 v-kind usage binary-double unsigned.
 02 v-value usage pointer.
01 tt-n-cobol-ChooseName based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-ChooseWordingVersion based.
 02 v-value usage binary-double signed.
01 tt-n-cobol-CriterionArm1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-CriterionArm2 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-CriterionArm3 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-CriterionArm4 based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-CutArm1 based.
 02 v-value usage float-long.
01 tt-n-cobol-CutArm2 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-DecideBatch based.
 02 v-kind usage binary-double unsigned.
 02 v-value usage pointer.
01 tt-n-cobol-DecideName based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-DecideWordingVersion based.
 02 v-value usage binary-double signed.
01 tt-n-cobol-DescriptionArm1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-DescriptionArm2 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-DescriptionArm3 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-DescriptionArm4 based.
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
01 tt-n-cobol-LabelsArm1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-LabelsArm2 based.
 02 v-keys usage pointer.
 02 v-values usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-LevelsArm1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-LevelsArm2 based.
 02 v-keys usage pointer.
 02 v-values usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-OptionsArm1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-OptionsArm2 based.
 02 v-keys usage pointer.
 02 v-values usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-PointersArm1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-PointersArm2 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-QuestionTextArm1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-QuestionTextArm2 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-QuestionTextArm3 based.
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
01 tt-n-cobol-ScoreBatch based.
 02 v-kind usage binary-double unsigned.
 02 v-value usage pointer.
01 tt-n-cobol-ScoreName based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-ScoreWordingVersion based.
 02 v-value usage binary-double signed.
01 tt-n-cobol-TagBatch based.
 02 v-kind usage binary-double unsigned.
 02 v-value usage pointer.
01 tt-n-cobol-TagName based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-TagWordingVersion based.
 02 v-value usage binary-double signed.
01 tt-n-cobol-ThresholdArm1 based.
 02 v-value usage float-long.
01 tt-n-cobol-ThresholdArm2 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-ContextSchemaArm1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-ContextSchemaArm2 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-ImageMediaArm1 based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-ImageMediaArm2 based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-OptionSchemaDescription based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-OptionSchemaName based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-ReaderMediaArm1 based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-ReaderMediaArm2 based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RecognitionExampleArm1 based.
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
01 tt-n-cobol-RecognitionModeArm1 based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RecognitionModeArm2 based.
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
01 tt-n-cobol-RequestBatchArm1 based.
 02 v-value usage binary-double unsigned.
01 tt-n-cobol-RequestBatchArm2 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-DefinitionAnnotateDefinitionBatch based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-DefinitionAnnotateDefinitionQuestions based.
 02 v-keys usage pointer.
 02 v-values usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-DefinitionAnnotateDefinitionVersion based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-DefinitionChooseBatch based.
 02 v-kind usage binary-double unsigned.
 02 v-value usage pointer.
01 tt-n-cobol-DefinitionChooseName based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-DefinitionChooseWordingVersion based.
 02 v-value usage binary-double signed.
01 tt-n-cobol-DefinitionDecideBatch based.
 02 v-kind usage binary-double unsigned.
 02 v-value usage pointer.
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
01 tt-n-cobol-DefinitionScoreBatch based.
 02 v-kind usage binary-double unsigned.
 02 v-value usage pointer.
01 tt-n-cobol-DefinitionScoreName based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-DefinitionScoreWordingVersion based.
 02 v-value usage binary-double signed.
01 tt-n-cobol-DefinitionTagBatch based.
 02 v-kind usage binary-double unsigned.
 02 v-value usage pointer.
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
01 tt-n-cobol-RequestFramingArm1 based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestFramingArm2 based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestFramingArm3 based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestFramingArm4 based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestFramingArm5 based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestFunctionArm1 based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestFunctionArm2 based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestFunctionArm3 based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestFunctionArm4 based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestFunctionArm5 based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestFunctionArm6 based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestFunctionArm7 based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestFunctionArm8 based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestFunctionArm9 based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestFunctionArm10 based.
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
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
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
01 tt-n-cobol-RequestOptionsDetails based.
 02 v-value usage binary-double unsigned.
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
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
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
01 tt-n-cobol-RequestSourcePaths based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestThresholdArm1 based.
 02 v-value usage float-long.
01 tt-n-cobol-RequestThresholdArm2 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-SessionSourceLocationFile based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-SessionSourceLocationFirstLine based.
 02 v-value usage binary-double unsigned.
01 tt-n-cobol-SessionSourceLocationLastLine based.
 02 v-value usage binary-double unsigned.
01 tt-n-cobol-SourceUnitArm1 based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-SourceUnitArm2 based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-SourceUnitArm3 based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-ChooseBatchArm1 based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-ChooseBatchArm2 based.
 02 v-value usage binary-double signed.
01 tt-n-cobol-CriterionArm3Item based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-DecideBatchArm1 based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-DecideBatchArm2 based.
 02 v-value usage binary-double signed.
01 tt-n-cobol-DescriptionArm3Item based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-InputDeclarationObjectRequiredItem based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-InputPropertyArrayItemsType based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-PointersArm2Item based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-QuestionTextArm3Item based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-ScoreBatchArm1 based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-ScoreBatchArm2 based.
 02 v-value usage binary-double signed.
01 tt-n-cobol-TagBatchArm1 based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-TagBatchArm2 based.
 02 v-value usage binary-double signed.
01 tt-n-cobol-RecognitionExampleTextKindsItem based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-DefinitionChooseBatchArm1 based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-DefinitionChooseBatchArm2 based.
 02 v-value usage binary-double signed.
01 tt-n-cobol-DefinitionDecideBatchArm1 based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-DefinitionDecideBatchArm2 based.
 02 v-value usage binary-double signed.
01 tt-n-cobol-DefinitionScoreBatchArm1 based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-DefinitionScoreBatchArm2 based.
 02 v-value usage binary-double signed.
01 tt-n-cobol-DefinitionTagBatchArm1 based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-DefinitionTagBatchArm2 based.
 02 v-value usage binary-double signed.
01 tt-n-cobol-RequestOptionsFieldItem based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestSourcePathsItem based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
78 tt-n-answer-choice-v1 value 2.
78 tt-n-answer-find-v1 value 5.
78 tt-n-answer-score-v1 value 4.
78 tt-n-answer-tag-v1 value 3.
78 tt-n-answer-yes-no-v1 value 1.
78 tt-n-attempt-ok-v1 value 1.
78 tt-n-attempt-status-v1 value 2.
78 tt-n-attempt-transport-v1 value 3.
78 tt-n-batch-max-v1 value 2.
78 tt-n-batch-records-v1 value 1.
78 tt-n-cobol-annotatequestionchoose-kind value 2.
78 tt-n-cobol-annotatequestiondecide-kind value 1.
78 tt-n-cobol-annotatequestionscore-kind value 4.
78 tt-n-cobol-annotatequestiontag-kind value 3.
78 tt-n-cobol-choosebatcharm1-kind value 1.
78 tt-n-cobol-choosebatcharm2-kind value 2.
78 tt-n-cobol-contextschemaarm1-kind value 1.
78 tt-n-cobol-contextschemaarm2-kind value 2.
78 tt-n-cobol-criterionarm1-kind value 1.
78 tt-n-cobol-criterionarm2-kind value 2.
78 tt-n-cobol-criterionarm3-kind value 3.
78 tt-n-cobol-criterionarm4-kind value 4.
78 tt-n-cobol-cutarm1-kind value 1.
78 tt-n-cobol-cutarm2-kind value 2.
78 tt-n-cobol-decidebatcharm1-kind value 1.
78 tt-n-cobol-decidebatcharm2-kind value 2.
78 tt-n-cobol-definitionannotatedefinition-kind value 8.
78 tt-n-cobol-definitionchoosebatcharm1-kind value 1.
78 tt-n-cobol-definitionchoosebatcharm2-kind value 2.
78 tt-n-cobol-definitionchoose-kind value 2.
78 tt-n-cobol-definitiondecidebatcharm1-kind value 1.
78 tt-n-cobol-definitiondecidebatcharm2-kind value 2.
78 tt-n-cobol-definitiondecide-kind value 1.
78 tt-n-cobol-definitionfind-kind value 6.
78 tt-n-cobol-definitionrecognizedefinition-kind value 7.
78 tt-n-cobol-definitionrelate-kind value 5.
78 tt-n-cobol-definitionscorebatcharm1-kind value 1.
78 tt-n-cobol-definitionscorebatcharm2-kind value 2.
78 tt-n-cobol-definitionscore-kind value 4.
78 tt-n-cobol-definitiontagbatcharm1-kind value 1.
78 tt-n-cobol-definitiontagbatcharm2-kind value 2.
78 tt-n-cobol-definitiontag-kind value 3.
78 tt-n-cobol-descriptionarm1-kind value 1.
78 tt-n-cobol-descriptionarm2-kind value 2.
78 tt-n-cobol-descriptionarm3-kind value 3.
78 tt-n-cobol-descriptionarm4-kind value 4.
78 tt-n-cobol-imagemediaarm1-kind value 1.
78 tt-n-cobol-imagemediaarm2-kind value 2.
78 tt-n-cobol-inputdeclarationobject-kind value 2.
78 tt-n-cobol-inputdeclarationstring-kind value 1.
78 tt-n-cobol-inputpropertyarray-kind value 4.
78 tt-n-cobol-inputpropertyboolean-kind value 3.
78 tt-n-cobol-inputpropertynumber-kind value 2.
78 tt-n-cobol-inputpropertystring-kind value 1.
78 tt-n-cobol-labelsarm1-kind value 1.
78 tt-n-cobol-labelsarm2-kind value 2.
78 tt-n-cobol-levelsarm1-kind value 1.
78 tt-n-cobol-levelsarm2-kind value 2.
78 tt-n-cobol-optionsarm1-kind value 1.
78 tt-n-cobol-optionsarm2-kind value 2.
78 tt-n-cobol-overflow value 1002.
78 tt-n-cobol-pointersarm1-kind value 1.
78 tt-n-cobol-pointersarm2-kind value 2.
78 tt-n-cobol-questiontextarm1-kind value 1.
78 tt-n-cobol-questiontextarm2-kind value 2.
78 tt-n-cobol-questiontextarm3-kind value 3.
78 tt-n-cobol-readermediaarm1-kind value 1.
78 tt-n-cobol-readermediaarm2-kind value 2.
78 tt-n-cobol-recognitionexamplearm1-kind value 1.
78 tt-n-cobol-recognitionexample-arm-2-kind value 2.
78 tt-n-cobol-recognitionmodearm1-kind value 1.
78 tt-n-cobol-recognitionmodearm2-kind value 2.
78 tt-n-cobol-representation value 1001.
78 tt-n-cobol-requestbatcharm1-kind value 1.
78 tt-n-cobol-requestbatcharm2-kind value 2.
78 tt-n-cobol-requestframingarm1-kind value 1.
78 tt-n-cobol-requestframingarm2-kind value 2.
78 tt-n-cobol-requestframingarm3-kind value 3.
78 tt-n-cobol-requestframingarm4-kind value 4.
78 tt-n-cobol-requestframingarm5-kind value 5.
78 tt-n-cobol-requestfunctionarm10-kind value 10.
78 tt-n-cobol-requestfunctionarm1-kind value 1.
78 tt-n-cobol-requestfunctionarm2-kind value 2.
78 tt-n-cobol-requestfunctionarm3-kind value 3.
78 tt-n-cobol-requestfunctionarm4-kind value 4.
78 tt-n-cobol-requestfunctionarm5-kind value 5.
78 tt-n-cobol-requestfunctionarm6-kind value 6.
78 tt-n-cobol-requestfunctionarm7-kind value 7.
78 tt-n-cobol-requestfunctionarm8-kind value 8.
78 tt-n-cobol-requestfunctionarm9-kind value 9.
78 tt-n-cobol-requestimagebytes-kind value 2.
78 tt-n-cobol-requestimagefile-kind value 1.
78 tt-n-cobol-requestinputentities-kind value 5.
78 tt-n-cobol-requestinputfeed-kind value 7.
78 tt-n-cobol-requestinputjson-kind value 2.
78 tt-n-cobol-requestinputrecords-kind value 3.
78 tt-n-cobol-requestinputsource-kind value 6.
78 tt-n-cobol-requestinputtext-kind value 1.
78 tt-n-cobol-requestinputunits-kind value 4.
78 tt-n-cobol-requestoriginaljson-kind value 2.
78 tt-n-cobol-requestoriginaltext-kind value 1.
78 tt-n-cobol-requestquestiondefinition-kind value 2.
78 tt-n-cobol-requestquestionfile-kind value 3.
78 tt-n-cobol-requestquestionname-kind value 4.
78 tt-n-cobol-requestquestionreference-kind value 5.
78 tt-n-cobol-requestquestiontext-kind value 1.
78 tt-n-cobol-requestthresholdarm1-kind value 1.
78 tt-n-cobol-requestthresholdarm2-kind value 2.
78 tt-n-cobol-scorebatcharm1-kind value 1.
78 tt-n-cobol-scorebatcharm2-kind value 2.
78 tt-n-cobol-sourceunitarm1-kind value 1.
78 tt-n-cobol-sourceunitarm2-kind value 2.
78 tt-n-cobol-sourceunitarm3-kind value 3.
78 tt-n-cobol-tagbatcharm1-kind value 1.
78 tt-n-cobol-tagbatcharm2-kind value 2.
78 tt-n-cobol-text-capacity value 8192.
78 tt-n-cobol-thresholdarm1-kind value 1.
78 tt-n-cobol-thresholdarm2-kind value 2.
78 tt-n-complete-annotated-field-array-v1 value 4.
78 tt-n-complete-annotated-field-boolean-v1 value 1.
78 tt-n-complete-annotated-field-null-v1 value 2.
78 tt-n-complete-annotated-field-number-v1 value 5.
78 tt-n-complete-annotated-field-object-v1 value 6.
78 tt-n-complete-annotated-field-string-v1 value 3.
78 tt-n-complete-annotation-member-answer-id-v1 value 1.
78 tt-n-complete-annotation-member-failure-id-v1 value 2.
78 tt-n-complete-annotation-value-choice-v1 value 2.
78 tt-n-complete-annotation-value-decision-v1 value 1.
78 tt-n-complete-annotation-value-failed-v1 value 5.
78 tt-n-complete-annotation-value-score-v1 value 3.
78 tt-n-complete-annotation-value-tags-v1 value 4.
78 tt-n-complete-answer-choice-v1 value 2.
78 tt-n-complete-answer-score-v1 value 4.
78 tt-n-complete-answer-tag-v1 value 3.
78 tt-n-complete-answer-yes-no-v1 value 1.
78 tt-n-complete-attempt-outcome-ok-v1 value 1.
78 tt-n-complete-attempt-outcome-status-v1 value 2.
78 tt-n-complete-attempt-outcome-transport-v1 value 3.
78 tt-n-complete-batch-integer-v1 value 1.
78 tt-n-complete-batch-setting-integer-v1 value 1.
78 tt-n-complete-batch-setting-string-v1 value 2.
78 tt-n-complete-batch-string-max-v1 value 1.
78 tt-n-complete-batch-string-v1 value 2.
78 tt-n-complete-boundary-mode-boundary-only-v1 value 1.
78 tt-n-complete-failure-cause-invalid-distribution-v1 value 5.
78 tt-n-complete-failure-cause-invalid-probability-v1 value 4.
78 tt-n-complete-failure-cause-missing-answer-v1 value 1.
78 tt-n-complete-failure-cause-missing-probability-v1 value 3.
78 tt-n-complete-failure-cause-unexpected-probability-v1 value 6.
78 tt-n-complete-failure-cause-wrong-kind-v1 value 2.
78 tt-n-complete-failure-field-kind-backend-v1 value 1.
78 tt-n-complete-failure-kind-backend-v1 value 2.
78 tt-n-complete-failure-kind-cancelled-v1 value 4.
78 tt-n-complete-failure-kind-deadline-v1 value 5.
78 tt-n-complete-failure-kind-defect-v1 value 6.
78 tt-n-complete-failure-kind-local-v1 value 3.
78 tt-n-complete-failure-kind-usage-v1 value 1.
78 tt-n-complete-image-media-image-jpeg-v1 value 1.
78 tt-n-complete-image-media-image-png-v1 value 2.
78 tt-n-complete-input-declaration-object-root-v1 value 2.
78 tt-n-complete-input-declaration-string-root-v1 value 1.
78 tt-n-complete-input-denial-additional-request-v1 value 2.
78 tt-n-complete-input-denial-initial-request-v1 value 1.
78 tt-n-complete-input-denial-retry-v1 value 3.
78 tt-n-complete-input-property-type-array-v1 value 4.
78 tt-n-complete-input-property-type-boolean-v1 value 3.
78 tt-n-complete-input-property-type-number-v1 value 2.
78 tt-n-complete-input-property-type-string-v1 value 1.
78 tt-n-complete-json-array-v1 value 5.
78 tt-n-complete-json-boolean-v1 value 2.
78 tt-n-complete-json-null-v1 value 1.
78 tt-n-complete-json-number-v1 value 3.
78 tt-n-complete-json-object-v1 value 6.
78 tt-n-complete-json-string-v1 value 4.
78 tt-n-complete-object-type-object-v1 value 1.
78 tt-n-complete-observation-failure-id-v1 value 2.
78 tt-n-complete-observation-observation-id-v1 value 1.
78 tt-n-complete-origin-cache-v1 value 2.
78 tt-n-complete-origin-live-v1 value 1.
78 tt-n-complete-origin-memory-v1 value 5.
78 tt-n-complete-origin-proxy-v1 value 4.
78 tt-n-complete-origin-replay-v1 value 3.
78 tt-n-complete-presence-missing-v1 value 0.
78 tt-n-complete-presence-null-v1 value 1.
78 tt-n-complete-presence-value-v1 value 2.
78 tt-n-complete-readable-question-choose-v1 value 2.
78 tt-n-complete-readable-question-decide-v1 value 1.
78 tt-n-complete-readable-question-score-v1 value 4.
78 tt-n-complete-readable-question-tag-v1 value 3.
78 tt-n-complete-recognition-mode-boundary-only-v1 value 2.
78 tt-n-complete-recognition-mode-whole-v1 value 1.
78 tt-n-complete-recognition-odds-boundary-odds-v1 value 2.
78 tt-n-complete-recognition-odds-recognize-answer-v1 value 1.
78 tt-n-complete-recognize-entities-v1 value 1.
78 tt-n-complete-recognize-mode-proposals-v1 value 2.
78 tt-n-complete-relation-direction-either-v1 value 2.
78 tt-n-complete-relation-direction-source-to-target-v1 value 1.
78 tt-n-complete-relation-endpoint-related-entity-v1 value 1.
78 tt-n-complete-relation-endpoint-source-v1 value 2.
78 tt-n-complete-relation-member-answer-id-v1 value 1.
78 tt-n-complete-relation-member-failure-id-v1 value 2.
78 tt-n-complete-relation-method-choice-v1 value 2.
78 tt-n-complete-relation-method-yes-no-v1 value 1.
78 tt-n-complete-request-function-annotate-v1 value 8.
78 tt-n-complete-request-function-choose-v1 value 2.
78 tt-n-complete-request-function-decide-v1 value 1.
78 tt-n-complete-request-function-filter-v1 value 5.
78 tt-n-complete-request-function-find-v1 value 7.
78 tt-n-complete-request-function-rank-v1 value 6.
78 tt-n-complete-request-function-recognize-v1 value 9.
78 tt-n-complete-request-function-relate-v1 value 10.
78 tt-n-complete-request-function-score-v1 value 4.
78 tt-n-complete-request-function-tag-v1 value 3.
78 tt-n-complete-send-denial-before-additional-send-v1 value 2.
78 tt-n-complete-send-denial-before-first-send-v1 value 1.
78 tt-n-complete-send-denial-before-retry-v1 value 3.
78 tt-n-complete-session-judgment-choice-v1 value 2.
78 tt-n-complete-session-judgment-decision-v1 value 1.
78 tt-n-complete-session-judgment-score-v1 value 3.
78 tt-n-complete-session-judgment-tags-v1 value 4.
78 tt-n-complete-session-observation-question-v1 value 1.
78 tt-n-complete-session-observation-row-v1 value 2.
78 tt-n-complete-session-observed-row-annotated-v1 value 2.
78 tt-n-complete-session-observed-row-find-v1 value 4.
78 tt-n-complete-session-observed-row-judgment-v1 value 1.
78 tt-n-complete-session-observed-row-recognized-v1 value 3.
78 tt-n-complete-session-observed-row-relations-v1 value 5.
78 tt-n-complete-session-packet-annotate-aggregate-v1 value 14.
78 tt-n-complete-session-packet-annotate-row-v1 value 6.
78 tt-n-complete-session-packet-choose-aggregate-v1 value 8.
78 tt-n-complete-session-packet-choose-row-v1 value 2.
78 tt-n-complete-session-packet-decide-aggregate-v1 value 7.
78 tt-n-complete-session-packet-decide-row-v1 value 1.
78 tt-n-complete-session-packet-filter-aggregate-v1 value 11.
78 tt-n-complete-session-packet-filter-row-v1 value 5.
78 tt-n-complete-session-packet-find-aggregate-v1 value 13.
78 tt-n-complete-session-packet-observation-v1 value 17.
78 tt-n-complete-session-packet-rank-aggregate-v1 value 12.
78 tt-n-complete-session-packet-recognize-aggregate-v1 value 15.
78 tt-n-complete-session-packet-relate-aggregate-v1 value 16.
78 tt-n-complete-session-packet-score-aggregate-v1 value 10.
78 tt-n-complete-session-packet-score-row-v1 value 4.
78 tt-n-complete-session-packet-tag-aggregate-v1 value 9.
78 tt-n-complete-session-packet-tag-row-v1 value 3.
78 tt-n-complete-session-packet-terminal-v1 value 18.
78 tt-n-complete-session-probabilities-named-v1 value 2.
78 tt-n-complete-session-probabilities-yes-no-v1 value 1.
78 tt-n-complete-stop-cause-backend-v1 value 8.
78 tt-n-complete-stop-cause-cancelled-v1 value 9.
78 tt-n-complete-stop-cause-deadline-v1 value 10.
78 tt-n-complete-stop-cause-defect-v1 value 11.
78 tt-n-complete-stop-cause-local-v1 value 2.
78 tt-n-complete-stop-cause-no-key-v1 value 3.
78 tt-n-complete-stop-cause-reply-v1 value 7.
78 tt-n-complete-stop-cause-status-v1 value 5.
78 tt-n-complete-stop-cause-too-large-v1 value 6.
78 tt-n-complete-stop-cause-transport-v1 value 4.
78 tt-n-complete-stop-cause-usage-v1 value 1.
78 tt-n-complete-string-type-string-v1 value 1.
78 tt-n-complete-threshold-number-v1 value 1.
78 tt-n-complete-threshold-string-v1 value 2.
78 tt-n-complete-usage-persistence-disabled-v1 value 1.
78 tt-n-complete-usage-persistence-failed-v1 value 4.
78 tt-n-complete-usage-persistence-pending-v1 value 2.
78 tt-n-complete-usage-persistence-written-v1 value 3.
78 tt-n-complete-value-array-v1 value 4.
78 tt-n-complete-value-boolean-v1 value 1.
78 tt-n-complete-value-null-v1 value 2.
78 tt-n-complete-value-number-v1 value 5.
78 tt-n-complete-value-string-v1 value 3.
78 tt-n-complete-verb-recognize-v1 value 1.
78 tt-n-complete-version-thinkthen-result-2-v1 value 1.
78 tt-n-content-json-v1 value 2.
78 tt-n-content-text-v1 value 1.
78 tt-n-decide-authored-v1 value 2.
78 tt-n-decide-boolean-v1 value 1.
78 tt-n-decide-null-v1 value 0.
78 tt-n-declaration-absent-v1 value 0.
78 tt-n-declaration-object-v1 value 2.
78 tt-n-declaration-string-v1 value 1.
78 tt-n-direction-either-v1 value 2.
78 tt-n-direction-source-to-target-v1 value 1.
78 tt-n-ebackend value 2.
78 tt-n-ecancelled value 5.
78 tt-n-edeadline value 3.
78 tt-n-edefect value 6.
78 tt-n-elocal value 4.
78 tt-n-eusage value 1.
78 tt-n-event-question-v1 value 1.
78 tt-n-event-row-v1 value 2.
78 tt-n-function-annotate-v1 value 8.
78 tt-n-function-choose-v1 value 2.
78 tt-n-function-decide-v1 value 1.
78 tt-n-function-filter-v1 value 5.
78 tt-n-function-find-v1 value 7.
78 tt-n-function-rank-v1 value 6.
78 tt-n-function-recognize-v1 value 9.
78 tt-n-function-relate-v1 value 10.
78 tt-n-function-score-v1 value 4.
78 tt-n-function-tag-v1 value 3.
78 tt-n-id-failure-v1 value 2.
78 tt-n-id-observation-v1 value 1.
78 tt-n-image-jpeg-v1 value 1.
78 tt-n-image-png-v1 value 2.
78 tt-n-load-atomic-v1 value 1.
78 tt-n-load-dynamic-choose-v1 value 3.
78 tt-n-load-find-v1 value 8.
78 tt-n-load-rank-set-v1 value 7.
78 tt-n-load-rank-v1 value 6.
78 tt-n-load-recognize-v1 value 4.
78 tt-n-load-relate-v1 value 5.
78 tt-n-load-set-v1 value 2.
78 tt-n-member-failure-v1 value 2.
78 tt-n-member-invalid-distribution-v1 value 5.
78 tt-n-member-invalid-probability-v1 value 4.
78 tt-n-member-missing-answer-v1 value 1.
78 tt-n-member-missing-probability-v1 value 3.
78 tt-n-member-success-v1 value 1.
78 tt-n-member-unexpected-probability-v1 value 6.
78 tt-n-member-wrong-kind-v1 value 2.
78 tt-n-no value 0.
78 tt-n-no-deadline value -1.
78 tt-n-ok value 0.
78 tt-n-origin-cache-v1 value 2.
78 tt-n-origin-live-v1 value 1.
78 tt-n-origin-memory-v1 value 5.
78 tt-n-origin-proxy-v1 value 4.
78 tt-n-origin-replay-v1 value 3.
78 tt-n-probabilities-named-v1 value 2.
78 tt-n-probabilities-yes-v1 value 1.
78 tt-n-property-boolean-v1 value 3.
78 tt-n-property-number-v1 value 2.
78 tt-n-property-string-list-v1 value 4.
78 tt-n-property-string-v1 value 1.
78 tt-n-relation-choice-v1 value 2.
78 tt-n-relation-yes-no-v1 value 1.
78 tt-n-request-annotate-question-arm-1-v1 value 1.
78 tt-n-request-annotate-question-arm-2-v1 value 2.
78 tt-n-request-annotate-question-arm-3-v1 value 3.
78 tt-n-request-annotate-question-arm-4-v1 value 4.
78 tt-n-request-batch-arm-1-v1 value 1.
78 tt-n-request-batch-arm-2-v1 value 2.
78 tt-n-request-call-threshold-arm-1-v1 value 1.
78 tt-n-request-call-threshold-arm-2-v1 value 2.
78 tt-n-request-context-schema-arm-1-v1 value 1.
78 tt-n-request-context-schema-arm-2-v1 value 2.
78 tt-n-request-criterion-arm-1-v1 value 1.
78 tt-n-request-criterion-arm-2-v1 value 2.
78 tt-n-request-criterion-arm-3-v1 value 3.
78 tt-n-request-criterion-arm-4-v1 value 4.
78 tt-n-request-cut-arm-1-v1 value 1.
78 tt-n-request-cut-arm-2-v1 value 2.
78 tt-n-request-definition-arm-1-v1 value 1.
78 tt-n-request-definition-arm-2-v1 value 2.
78 tt-n-request-definition-arm-3-v1 value 3.
78 tt-n-request-definition-arm-4-v1 value 4.
78 tt-n-request-definition-arm-5-v1 value 5.
78 tt-n-request-definition-arm-6-v1 value 6.
78 tt-n-request-definition-arm-7-v1 value 7.
78 tt-n-request-definition-arm-8-v1 value 8.
78 tt-n-request-definition-authored-score-batch-arm-1-v1 value 1.
78 tt-n-request-definition-authored-score-batch-arm-2-v1 value 2.
78 tt-n-request-description-arm-1-v1 value 1.
78 tt-n-request-description-arm-2-v1 value 2.
78 tt-n-request-description-arm-3-v1 value 3.
78 tt-n-request-description-arm-4-v1 value 4.
78 tt-n-request-framing-arm-1-v1 value 1.
78 tt-n-request-framing-arm-2-v1 value 2.
78 tt-n-request-framing-arm-3-v1 value 3.
78 tt-n-request-framing-arm-4-v1 value 4.
78 tt-n-request-framing-arm-5-v1 value 5.
78 tt-n-request-image-arm-1-v1 value 1.
78 tt-n-request-image-arm-2-v1 value 2.
78 tt-n-request-image-media-arm-1-v1 value 1.
78 tt-n-request-image-media-arm-2-v1 value 2.
78 tt-n-request-input-arm-1-v1 value 1.
78 tt-n-request-input-arm-2-v1 value 2.
78 tt-n-request-input-arm-3-v1 value 3.
78 tt-n-request-input-arm-4-v1 value 4.
78 tt-n-request-input-arm-5-v1 value 5.
78 tt-n-request-input-arm-6-v1 value 6.
78 tt-n-request-input-arm-7-v1 value 7.
78 tt-n-request-input-declaration-arm-1-v1 value 1.
78 tt-n-request-input-declaration-arm-2-v1 value 2.
78 tt-n-request-input-property-arm-1-v1 value 1.
78 tt-n-request-input-property-arm-2-v1 value 2.
78 tt-n-request-input-property-arm-3-v1 value 3.
78 tt-n-request-input-property-arm-4-v1 value 4.
78 tt-n-request-labels-arm-1-v1 value 1.
78 tt-n-request-labels-arm-2-v1 value 2.
78 tt-n-request-levels-arm-1-v1 value 1.
78 tt-n-request-levels-arm-2-v1 value 2.
78 tt-n-request-options-arm-1-v1 value 1.
78 tt-n-request-options-arm-2-v1 value 2.
78 tt-n-request-original-arm-1-v1 value 1.
78 tt-n-request-original-arm-2-v1 value 2.
78 tt-n-request-pointers-arm-1-v1 value 1.
78 tt-n-request-pointers-arm-2-v1 value 2.
78 tt-n-request-question-selector-arm-1-v1 value 1.
78 tt-n-request-question-selector-arm-2-v1 value 2.
78 tt-n-request-question-selector-arm-3-v1 value 3.
78 tt-n-request-question-selector-arm-4-v1 value 4.
78 tt-n-request-question-selector-arm-5-v1 value 5.
78 tt-n-request-question-text-arm-1-v1 value 1.
78 tt-n-request-question-text-arm-2-v1 value 2.
78 tt-n-request-question-text-arm-3-v1 value 3.
78 tt-n-request-reader-failure-arm-1-v1 value 1.
78 tt-n-request-reader-failure-arm-2-v1 value 2.
78 tt-n-request-reader-failure-arm-3-v1 value 3.
78 tt-n-request-reader-failure-arm-4-v1 value 4.
78 tt-n-request-reader-media-arm-1-v1 value 1.
78 tt-n-request-reader-media-arm-2-v1 value 2.
78 tt-n-request-recognition-example-arm-1-v1 value 1.
78 tt-n-request-recognition-example-arm-2-v1 value 2.
78 tt-n-request-recognition-mode-arm-1-v1 value 1.
78 tt-n-request-recognition-mode-arm-2-v1 value 2.
78 tt-n-request-source-unit-arm-1-v1 value 1.
78 tt-n-request-source-unit-arm-2-v1 value 2.
78 tt-n-request-source-unit-arm-3-v1 value 3.
78 tt-n-request-threshold-arm-1-v1 value 1.
78 tt-n-request-threshold-arm-2-v1 value 2.
78 tt-n-result-failure-v1 value 2.
78 tt-n-result-success-v1 value 1.
78 tt-n-rule-band-v1 value 3.
78 tt-n-rule-cut-v1 value 2.
78 tt-n-rule-default-v1 value 0.
78 tt-n-rule-null-v1 value 1.
78 tt-n-session-accepted-v1 value 0.
78 tt-n-session-closed-v1 value 2.
78 tt-n-session-end-v1 value 2.
78 tt-n-session-full-v1 value 1.
78 tt-n-session-pending-v1 value 1.
78 tt-n-session-ready-cancelled-v1 value 4.
78 tt-n-session-ready-closed-v1 value 2.
78 tt-n-session-ready-end-v1 value 3.
78 tt-n-session-ready-input-v1 value 1.
78 tt-n-session-ready-result-v1 value 0.
78 tt-n-session-ready-timeout-v1 value 5.
78 tt-n-session-result-v1 value 0.
78 tt-n-session-wait-input-v1 value 1.
78 tt-n-session-wait-result-v1 value 0.
78 tt-n-session-wait-settlement-v1 value 2.
78 tt-n-source-file-v1 value 3.
78 tt-n-source-image-file-v1 value 4.
78 tt-n-source-jsonl-v1 value 5.
78 tt-n-source-line-v1 value 1.
78 tt-n-source-window-v1 value 2.
78 tt-n-stage-boundary-v1 value 1.
78 tt-n-stage-edge-v1 value 3.
78 tt-n-stage-kind-v1 value 2.
78 tt-n-stage-relation-v1 value 4.
78 tt-n-stop-backend-v1 value 8.
78 tt-n-stop-cancelled-v1 value 9.
78 tt-n-stop-deadline-v1 value 11.
78 tt-n-stop-defect-v1 value 10.
78 tt-n-stop-local-v1 value 2.
78 tt-n-stop-no-key-v1 value 3.
78 tt-n-stop-reply-v1 value 7.
78 tt-n-stop-status-v1 value 5.
78 tt-n-stop-too-large-v1 value 6.
78 tt-n-stop-transport-v1 value 4.
78 tt-n-stop-usage-v1 value 1.
78 tt-n-unsure value 2.
78 tt-n-version-major value 0.
78 tt-n-version-minor value 2.
78 tt-n-version-patch value 0.
78 tt-n-yes value 1.
