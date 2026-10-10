      *> Generated from measured canonical C declarations; do not edit.
01 tt-n-string-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-content-v1 based.
 02 v-kind usage binary-long unsigned.
 02 filler pic x(4).
 02 v-data.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-optional-content-v1 based.
 02 v-present usage binary-long signed.
 02 filler pic x(4).
 02 v-value.
 03 v-kind usage binary-long unsigned.
 03 filler pic x(4).
 03 v-data.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
01 tt-n-optional-size-v1 based.
 02 v-present usage binary-long signed.
 02 filler pic x(4).
 02 v-value usage binary-double unsigned.
01 tt-n-controls-v1 based.
 02 v-deadline-ms usage binary-double signed.
 02 v-cancel usage pointer.
 02 v-context.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-value.
 04 v-kind usage binary-long unsigned.
 04 filler pic x(4).
 04 v-data.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 02 v-batch.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-value usage binary-double unsigned.
 02 v-batch-max usage binary-long signed.
 02 v-attempts usage binary-long signed.
 02 v-surface.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-answer based.
 02 v-outcome usage binary-long signed.
 02 filler pic x(4).
 02 v-probability usage float-long.
01 tt-n-optional-string-v1 based.
 02 v-present usage binary-long signed.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-image-view-v1 based.
 02 v-media usage binary-long unsigned.
 02 filler pic x(4).
 02 v-bytes usage pointer.
 02 v-bytes-len usage binary-double unsigned.
 02 v-width usage binary-long unsigned.
 02 v-height usage binary-long unsigned.
 02 v-filename.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
01 tt-n-optional-u64-v1 based.
 02 v-present usage binary-long signed.
 02 filler pic x(4).
 02 v-value usage binary-double unsigned.
01 tt-n-input-property-v1 based.
 02 v-name.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-kind usage binary-long unsigned.
 02 filler pic x(4).
01 tt-n-input-properties-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-strings-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-input-declaration-v1 based.
 02 v-kind usage binary-long unsigned.
 02 filler pic x(4).
 02 v-properties.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-required.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-question-author-v1 based.
 02 v-name.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-wording-version.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-value usage binary-double unsigned.
 02 v-item-schema.
 03 v-kind usage binary-long unsigned.
 03 filler pic x(4).
 03 v-properties.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 03 v-required.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-context-schema.
 03 v-kind usage binary-long unsigned.
 03 filler pic x(4).
 03 v-properties.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 03 v-required.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
01 tt-n-optional-double-v1 based.
 02 v-present usage binary-long signed.
 02 filler pic x(4).
 02 v-value usage float-long.
01 tt-n-choice-v1 based.
 02 v-name.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-description.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-value.
 04 v-kind usage binary-long unsigned.
 04 filler pic x(4).
 04 v-data.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 02 v-weight.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-value usage float-long.
01 tt-n-choices-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-rule-v1 based.
 02 v-kind usage binary-long unsigned.
 02 filler pic x(4).
 02 v-low usage float-long.
 02 v-high usage float-long.
01 tt-n-member-spec-v1 based.
 02 v-name.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-question usage pointer.
01 tt-n-member-specs-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-relation-v1 based.
 02 v-name.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-source.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-target.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-reads.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-either usage binary-long signed.
 02 v-single usage binary-long signed.
01 tt-n-relations-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-question-spec-v1 based.
 02 v-kind usage binary-long unsigned.
 02 filler pic x(4).
 02 v-text.
 03 v-kind usage binary-long unsigned.
 03 filler pic x(4).
 03 v-data.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-yes.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-value.
 04 v-kind usage binary-long unsigned.
 04 filler pic x(4).
 04 v-data.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 02 v-no.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-value.
 04 v-kind usage binary-long unsigned.
 04 filler pic x(4).
 04 v-data.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 02 v-choices.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-threshold.
 03 v-kind usage binary-long unsigned.
 03 filler pic x(4).
 03 v-low usage float-long.
 03 v-high usage float-long.
 02 v-relation-threshold.
 03 v-kind usage binary-long unsigned.
 03 filler pic x(4).
 03 v-low usage float-long.
 03 v-high usage float-long.
 02 v-model.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-profile.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-batch.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-value usage binary-double unsigned.
 02 v-batch-max usage binary-long signed.
 02 v-none usage binary-long signed.
 02 v-on.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-members.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-kinds.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-relations.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-name-pointer.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-kind-pointer.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
01 tt-n-recognition-task-v1 based.
 02 v-instructions.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-entity-definition.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
01 tt-n-question-member-v1 based.
 02 v-name.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-question usage pointer.
01 tt-n-question-members-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-question-view-v1 based.
 02 v-kind usage binary-long unsigned.
 02 filler pic x(4).
 02 v-text.
 03 v-kind usage binary-long unsigned.
 03 filler pic x(4).
 03 v-data.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-yes.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-value.
 04 v-kind usage binary-long unsigned.
 04 filler pic x(4).
 04 v-data.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 02 v-no.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-value.
 04 v-kind usage binary-long unsigned.
 04 filler pic x(4).
 04 v-data.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 02 v-choices.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-threshold.
 03 v-kind usage binary-long unsigned.
 03 filler pic x(4).
 03 v-low usage float-long.
 03 v-high usage float-long.
 02 v-relation-threshold.
 03 v-kind usage binary-long unsigned.
 03 filler pic x(4).
 03 v-low usage float-long.
 03 v-high usage float-long.
 02 v-model.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-profile.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-batch.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-value usage binary-double unsigned.
 02 v-batch-max usage binary-long signed.
 02 v-none usage binary-long signed.
 02 v-on.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-members.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-kinds.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-relations.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-name-pointer.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-kind-pointer.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
01 tt-n-optional-question-v1 based.
 02 v-present usage binary-long signed.
 02 filler pic x(4).
 02 v-value.
 03 v-kind usage binary-long unsigned.
 03 filler pic x(4).
 03 v-text.
 04 v-kind usage binary-long unsigned.
 04 filler pic x(4).
 04 v-data.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 03 v-yes.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-kind usage binary-long unsigned.
 05 filler pic x(4).
 05 v-data.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 03 v-no.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-kind usage binary-long unsigned.
 05 filler pic x(4).
 05 v-data.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 03 v-choices.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 03 v-threshold.
 04 v-kind usage binary-long unsigned.
 04 filler pic x(4).
 04 v-low usage float-long.
 04 v-high usage float-long.
 03 v-relation-threshold.
 04 v-kind usage binary-long unsigned.
 04 filler pic x(4).
 04 v-low usage float-long.
 04 v-high usage float-long.
 03 v-model.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 03 v-profile.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 03 v-batch.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value usage binary-double unsigned.
 03 v-batch-max usage binary-long signed.
 03 v-none usage binary-long signed.
 03 v-on.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 03 v-members.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 03 v-kinds.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 03 v-relations.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 03 v-name-pointer.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 03 v-kind-pointer.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
01 tt-n-probability-v1 based.
 02 v-name.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-probability usage float-long.
01 tt-n-probabilities-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-named-answer-v1 based.
 02 v-pick.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-probabilities.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-confidence.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-value usage float-long.
01 tt-n-score-answer-v1 based.
 02 v-level.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-probabilities.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-confidence.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-value usage float-long.
01 tt-n-answer-v1 based.
 02 v-kind usage binary-long unsigned.
 02 filler pic x(4).
 02 v-data.
 03 v-probability usage pointer.
 03 v-choice redefines v-probability usage pointer.
 03 v-tag redefines v-probability usage pointer.
 03 v-score redefines v-probability usage pointer.
 03 v-find redefines v-probability usage pointer.
01 tt-n-optional-answer-v1 based.
 02 v-present usage binary-long signed.
 02 filler pic x(4).
 02 v-value.
 03 v-kind usage binary-long unsigned.
 03 filler pic x(4).
 03 v-data.
 04 v-probability usage pointer.
 04 v-choice redefines v-probability usage pointer.
 04 v-tag redefines v-probability usage pointer.
 04 v-score redefines v-probability usage pointer.
 04 v-find redefines v-probability usage pointer.
01 tt-n-optional-rule-v1 based.
 02 v-present usage binary-long signed.
 02 filler pic x(4).
 02 v-value.
 03 v-kind usage binary-long unsigned.
 03 filler pic x(4).
 03 v-low usage float-long.
 03 v-high usage float-long.
01 tt-n-location-v1 based.
 02 v-file.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-first-line.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-value usage binary-double unsigned.
 02 v-last-line.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-value usage binary-double unsigned.
01 tt-n-optional-location-v1 based.
 02 v-present usage binary-long signed.
 02 filler pic x(4).
 02 v-value.
 03 v-file.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 03 v-first-line.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value usage binary-double unsigned.
 03 v-last-line.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value usage binary-double unsigned.
01 tt-n-usage-v1 based.
 02 v-input-tokens usage binary-double unsigned.
 02 v-output-tokens usage binary-double unsigned.
01 tt-n-optional-usage-v1 based.
 02 v-present usage binary-long signed.
 02 filler pic x(4).
 02 v-value.
 03 v-input-tokens usage binary-double unsigned.
 03 v-output-tokens usage binary-double unsigned.
01 tt-n-profile-warning-v1 based.
 02 v-tuned-for.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-running.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-optional-profile-warning-v1 based.
 02 v-present usage binary-long signed.
 02 filler pic x(4).
 02 v-value.
 03 v-tuned-for.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 03 v-running.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
01 tt-n-batch-v1 based.
 02 v-kind usage binary-long unsigned.
 02 filler pic x(4).
 02 v-records usage binary-double unsigned.
01 tt-n-optional-batch-v1 based.
 02 v-present usage binary-long signed.
 02 filler pic x(4).
 02 v-value.
 03 v-kind usage binary-long unsigned.
 03 filler pic x(4).
 03 v-records usage binary-double unsigned.
01 tt-n-batch-warning-v1 based.
 02 v-tuned-for.
 03 v-kind usage binary-long unsigned.
 03 filler pic x(4).
 03 v-records usage binary-double unsigned.
 02 v-running.
 03 v-kind usage binary-long unsigned.
 03 filler pic x(4).
 03 v-records usage binary-double unsigned.
01 tt-n-optional-batch-warning-v1 based.
 02 v-present usage binary-long signed.
 02 filler pic x(4).
 02 v-value.
 03 v-tuned-for.
 04 v-kind usage binary-long unsigned.
 04 filler pic x(4).
 04 v-records usage binary-double unsigned.
 03 v-running.
 04 v-kind usage binary-long unsigned.
 04 filler pic x(4).
 04 v-records usage binary-double unsigned.
01 tt-n-optional-u16-v1 based.
 02 v-present usage binary-long signed.
 02 v-value usage binary-short unsigned.
 02 filler pic x(2).
01 tt-n-attempt-v1 based.
 02 v-ordinal usage binary-double unsigned.
 02 v-request-sha256.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-wall-ms usage binary-double unsigned.
 02 v-outcome usage binary-long unsigned.
 02 filler pic x(4).
 02 v-sdk-request-id.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-status.
 03 v-present usage binary-long signed.
 03 v-value usage binary-short unsigned.
 03 filler pic x(2).
 02 v-server-ms.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-value usage binary-double unsigned.
 02 v-request-id.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
01 tt-n-attempts-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-optional-attempts-v1 based.
 02 v-present usage binary-long signed.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-optional-discriminator-v1 based.
 02 v-present usage binary-long signed.
 02 v-value usage binary-long unsigned.
01 tt-n-question-source-v1 based.
 02 v-origin usage binary-long unsigned.
 02 filler pic x(4).
 02 v-answered-by.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-question-sources-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-observation-identity-v1 based.
 02 v-kind usage binary-long unsigned.
 02 filler pic x(4).
 02 v-data.
 03 v-observation-id usage pointer.
 03 v-failure-id redefines v-observation-id usage pointer.
01 tt-n-observation-identities-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-meta-v1 based.
 02 v-tool.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-question-sha256.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-questions-sha256.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-url.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-model.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-usage.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-value.
 04 v-input-tokens usage binary-double unsigned.
 04 v-output-tokens usage binary-double unsigned.
 02 v-requests-sent usage binary-double unsigned.
 02 v-cached usage binary-long signed.
 02 filler pic x(4).
 02 v-requests.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-failed-questions usage binary-double unsigned.
 02 v-profile-warning.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-value.
 04 v-tuned-for.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-running.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 02 v-batch-setting.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-value.
 04 v-kind usage binary-long unsigned.
 04 filler pic x(4).
 04 v-records usage binary-double unsigned.
 02 v-batch-warning.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-value.
 04 v-tuned-for.
 05 v-kind usage binary-long unsigned.
 05 filler pic x(4).
 05 v-records usage binary-double unsigned.
 04 v-running.
 05 v-kind usage binary-long unsigned.
 05 filler pic x(4).
 05 v-records usage binary-double unsigned.
 02 v-context-sha256.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-attempts.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-origin.
 03 v-present usage binary-long signed.
 03 v-value usage binary-long unsigned.
 02 v-question-sources.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-observations.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-answered-by.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
01 tt-n-image-views-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-optional-image-views-v1 based.
 02 v-present usage binary-long signed.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-row-v1 based.
 02 v-answer-id.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-input.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-value.
 04 v-kind usage binary-long unsigned.
 04 filler pic x(4).
 04 v-data.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 02 v-question.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-value.
 04 v-kind usage binary-long unsigned.
 04 filler pic x(4).
 04 v-text.
 05 v-kind usage binary-long unsigned.
 05 filler pic x(4).
 05 v-data.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 04 v-yes.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-kind usage binary-long unsigned.
 06 filler pic x(4).
 06 v-data.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 04 v-no.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-kind usage binary-long unsigned.
 06 filler pic x(4).
 06 v-data.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 04 v-choices.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-threshold.
 05 v-kind usage binary-long unsigned.
 05 filler pic x(4).
 05 v-low usage float-long.
 05 v-high usage float-long.
 04 v-relation-threshold.
 05 v-kind usage binary-long unsigned.
 05 filler pic x(4).
 05 v-low usage float-long.
 05 v-high usage float-long.
 04 v-model.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 04 v-profile.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 04 v-batch.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value usage binary-double unsigned.
 04 v-batch-max usage binary-long signed.
 04 v-none usage binary-long signed.
 04 v-on.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-members.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-kinds.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-relations.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-name-pointer.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 04 v-kind-pointer.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 02 v-answer.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-value.
 04 v-kind usage binary-long unsigned.
 04 filler pic x(4).
 04 v-data.
 05 v-probability usage pointer.
 05 v-choice redefines v-probability usage pointer.
 05 v-tag redefines v-probability usage pointer.
 05 v-score redefines v-probability usage pointer.
 05 v-find redefines v-probability usage pointer.
 02 v-threshold.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-value.
 04 v-kind usage binary-long unsigned.
 04 filler pic x(4).
 04 v-low usage float-long.
 04 v-high usage float-long.
 02 v-position.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-value.
 04 v-file.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 04 v-first-line.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value usage binary-double unsigned.
 04 v-last-line.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value usage binary-double unsigned.
 02 v-input-file.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-meta.
 03 v-tool.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 03 v-question-sha256.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 03 v-questions-sha256.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 03 v-url.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 03 v-model.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 03 v-usage.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-input-tokens usage binary-double unsigned.
 05 v-output-tokens usage binary-double unsigned.
 03 v-requests-sent usage binary-double unsigned.
 03 v-cached usage binary-long signed.
 03 filler pic x(4).
 03 v-requests.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 03 v-failed-questions usage binary-double unsigned.
 03 v-profile-warning.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-tuned-for.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 05 v-running.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 03 v-batch-setting.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-kind usage binary-long unsigned.
 05 filler pic x(4).
 05 v-records usage binary-double unsigned.
 03 v-batch-warning.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-tuned-for.
 06 v-kind usage binary-long unsigned.
 06 filler pic x(4).
 06 v-records usage binary-double unsigned.
 05 v-running.
 06 v-kind usage binary-long unsigned.
 06 filler pic x(4).
 06 v-records usage binary-double unsigned.
 03 v-context-sha256.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 03 v-attempts.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 03 v-origin.
 04 v-present usage binary-long signed.
 04 v-value usage binary-long unsigned.
 03 v-question-sources.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 03 v-observations.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 03 v-answered-by.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 02 v-images.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
01 tt-n-decide-value-v1 based.
 02 v-kind usage binary-long unsigned.
 02 filler pic x(4).
 02 v-data.
 03 v-boolean usage pointer.
 03 v-authored redefines v-boolean usage pointer.
01 tt-n-member-value-v1 based.
 02 v-kind usage binary-long unsigned.
 02 filler pic x(4).
 02 v-data.
 03 v-decide usage pointer.
 03 v-choose redefines v-decide usage pointer.
 03 v-tag redefines v-decide usage pointer.
 03 v-score redefines v-decide usage pointer.
01 tt-n-member-success-v1 based.
 02 v-answer-id.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-value.
 03 v-kind usage binary-long unsigned.
 03 filler pic x(4).
 03 v-data.
 04 v-decide usage pointer.
 04 v-choose redefines v-decide usage pointer.
 04 v-tag redefines v-decide usage pointer.
 04 v-score redefines v-decide usage pointer.
 02 v-answer.
 03 v-kind usage binary-long unsigned.
 03 filler pic x(4).
 03 v-data.
 04 v-probability usage pointer.
 04 v-choice redefines v-probability usage pointer.
 04 v-tag redefines v-probability usage pointer.
 04 v-score redefines v-probability usage pointer.
 04 v-find redefines v-probability usage pointer.
 02 v-threshold.
 03 v-kind usage binary-long unsigned.
 03 filler pic x(4).
 03 v-low usage float-long.
 03 v-high usage float-long.
01 tt-n-member-failure-v1 based.
 02 v-failure-id.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-cause usage binary-long unsigned.
 02 filler pic x(4).
01 tt-n-member-v1 based.
 02 v-name.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-request.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-question.
 03 v-kind usage binary-long unsigned.
 03 filler pic x(4).
 03 v-text.
 04 v-kind usage binary-long unsigned.
 04 filler pic x(4).
 04 v-data.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 03 v-yes.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-kind usage binary-long unsigned.
 05 filler pic x(4).
 05 v-data.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 03 v-no.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-kind usage binary-long unsigned.
 05 filler pic x(4).
 05 v-data.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 03 v-choices.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 03 v-threshold.
 04 v-kind usage binary-long unsigned.
 04 filler pic x(4).
 04 v-low usage float-long.
 04 v-high usage float-long.
 03 v-relation-threshold.
 04 v-kind usage binary-long unsigned.
 04 filler pic x(4).
 04 v-low usage float-long.
 04 v-high usage float-long.
 03 v-model.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 03 v-profile.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 03 v-batch.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value usage binary-double unsigned.
 03 v-batch-max usage binary-long signed.
 03 v-none usage binary-long signed.
 03 v-on.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 03 v-members.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 03 v-kinds.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 03 v-relations.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 03 v-name-pointer.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 03 v-kind-pointer.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 02 v-state usage binary-long unsigned.
 02 filler pic x(4).
 02 v-data.
 03 v-success usage pointer.
 03 v-failure redefines v-success usage pointer.
01 tt-n-members-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-annotate-view-v1 based.
 02 v-common.
 03 v-answer-id.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 03 v-input.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-kind usage binary-long unsigned.
 05 filler pic x(4).
 05 v-data.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 03 v-question.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-kind usage binary-long unsigned.
 05 filler pic x(4).
 05 v-text.
 06 v-kind usage binary-long unsigned.
 06 filler pic x(4).
 06 v-data.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 05 v-yes.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value.
 07 v-kind usage binary-long unsigned.
 07 filler pic x(4).
 07 v-data.
 08 v-data usage pointer.
 08 v-len usage binary-double unsigned.
 05 v-no.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value.
 07 v-kind usage binary-long unsigned.
 07 filler pic x(4).
 07 v-data.
 08 v-data usage pointer.
 08 v-len usage binary-double unsigned.
 05 v-choices.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 05 v-threshold.
 06 v-kind usage binary-long unsigned.
 06 filler pic x(4).
 06 v-low usage float-long.
 06 v-high usage float-long.
 05 v-relation-threshold.
 06 v-kind usage binary-long unsigned.
 06 filler pic x(4).
 06 v-low usage float-long.
 06 v-high usage float-long.
 05 v-model.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 05 v-profile.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 05 v-batch.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value usage binary-double unsigned.
 05 v-batch-max usage binary-long signed.
 05 v-none usage binary-long signed.
 05 v-on.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 05 v-members.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 05 v-kinds.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 05 v-relations.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 05 v-name-pointer.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 05 v-kind-pointer.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 03 v-answer.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-kind usage binary-long unsigned.
 05 filler pic x(4).
 05 v-data.
 06 v-probability usage pointer.
 06 v-choice redefines v-probability usage pointer.
 06 v-tag redefines v-probability usage pointer.
 06 v-score redefines v-probability usage pointer.
 06 v-find redefines v-probability usage pointer.
 03 v-threshold.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-kind usage binary-long unsigned.
 05 filler pic x(4).
 05 v-low usage float-long.
 05 v-high usage float-long.
 03 v-position.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-file.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 05 v-first-line.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value usage binary-double unsigned.
 05 v-last-line.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value usage binary-double unsigned.
 03 v-input-file.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 03 v-meta.
 04 v-tool.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-question-sha256.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 04 v-questions-sha256.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 04 v-url.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-model.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-usage.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-input-tokens usage binary-double unsigned.
 06 v-output-tokens usage binary-double unsigned.
 04 v-requests-sent usage binary-double unsigned.
 04 v-cached usage binary-long signed.
 04 filler pic x(4).
 04 v-requests.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-failed-questions usage binary-double unsigned.
 04 v-profile-warning.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-tuned-for.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 06 v-running.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 04 v-batch-setting.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-kind usage binary-long unsigned.
 06 filler pic x(4).
 06 v-records usage binary-double unsigned.
 04 v-batch-warning.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-tuned-for.
 07 v-kind usage binary-long unsigned.
 07 filler pic x(4).
 07 v-records usage binary-double unsigned.
 06 v-running.
 07 v-kind usage binary-long unsigned.
 07 filler pic x(4).
 07 v-records usage binary-double unsigned.
 04 v-context-sha256.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 04 v-attempts.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 04 v-origin.
 05 v-present usage binary-long signed.
 05 v-value usage binary-long unsigned.
 04 v-question-sources.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-observations.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-answered-by.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 03 v-images.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 02 v-answers.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-choose-view-v1 based.
 02 v-common.
 03 v-answer-id.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 03 v-input.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-kind usage binary-long unsigned.
 05 filler pic x(4).
 05 v-data.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 03 v-question.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-kind usage binary-long unsigned.
 05 filler pic x(4).
 05 v-text.
 06 v-kind usage binary-long unsigned.
 06 filler pic x(4).
 06 v-data.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 05 v-yes.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value.
 07 v-kind usage binary-long unsigned.
 07 filler pic x(4).
 07 v-data.
 08 v-data usage pointer.
 08 v-len usage binary-double unsigned.
 05 v-no.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value.
 07 v-kind usage binary-long unsigned.
 07 filler pic x(4).
 07 v-data.
 08 v-data usage pointer.
 08 v-len usage binary-double unsigned.
 05 v-choices.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 05 v-threshold.
 06 v-kind usage binary-long unsigned.
 06 filler pic x(4).
 06 v-low usage float-long.
 06 v-high usage float-long.
 05 v-relation-threshold.
 06 v-kind usage binary-long unsigned.
 06 filler pic x(4).
 06 v-low usage float-long.
 06 v-high usage float-long.
 05 v-model.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 05 v-profile.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 05 v-batch.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value usage binary-double unsigned.
 05 v-batch-max usage binary-long signed.
 05 v-none usage binary-long signed.
 05 v-on.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 05 v-members.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 05 v-kinds.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 05 v-relations.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 05 v-name-pointer.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 05 v-kind-pointer.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 03 v-answer.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-kind usage binary-long unsigned.
 05 filler pic x(4).
 05 v-data.
 06 v-probability usage pointer.
 06 v-choice redefines v-probability usage pointer.
 06 v-tag redefines v-probability usage pointer.
 06 v-score redefines v-probability usage pointer.
 06 v-find redefines v-probability usage pointer.
 03 v-threshold.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-kind usage binary-long unsigned.
 05 filler pic x(4).
 05 v-low usage float-long.
 05 v-high usage float-long.
 03 v-position.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-file.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 05 v-first-line.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value usage binary-double unsigned.
 05 v-last-line.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value usage binary-double unsigned.
 03 v-input-file.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 03 v-meta.
 04 v-tool.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-question-sha256.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 04 v-questions-sha256.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 04 v-url.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-model.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-usage.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-input-tokens usage binary-double unsigned.
 06 v-output-tokens usage binary-double unsigned.
 04 v-requests-sent usage binary-double unsigned.
 04 v-cached usage binary-long signed.
 04 filler pic x(4).
 04 v-requests.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-failed-questions usage binary-double unsigned.
 04 v-profile-warning.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-tuned-for.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 06 v-running.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 04 v-batch-setting.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-kind usage binary-long unsigned.
 06 filler pic x(4).
 06 v-records usage binary-double unsigned.
 04 v-batch-warning.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-tuned-for.
 07 v-kind usage binary-long unsigned.
 07 filler pic x(4).
 07 v-records usage binary-double unsigned.
 06 v-running.
 07 v-kind usage binary-long unsigned.
 07 filler pic x(4).
 07 v-records usage binary-double unsigned.
 04 v-context-sha256.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 04 v-attempts.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 04 v-origin.
 05 v-present usage binary-long signed.
 05 v-value usage binary-long unsigned.
 04 v-question-sources.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-observations.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-answered-by.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 03 v-images.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 02 v-value.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
01 tt-n-decide-view-v1 based.
 02 v-common.
 03 v-answer-id.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 03 v-input.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-kind usage binary-long unsigned.
 05 filler pic x(4).
 05 v-data.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 03 v-question.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-kind usage binary-long unsigned.
 05 filler pic x(4).
 05 v-text.
 06 v-kind usage binary-long unsigned.
 06 filler pic x(4).
 06 v-data.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 05 v-yes.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value.
 07 v-kind usage binary-long unsigned.
 07 filler pic x(4).
 07 v-data.
 08 v-data usage pointer.
 08 v-len usage binary-double unsigned.
 05 v-no.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value.
 07 v-kind usage binary-long unsigned.
 07 filler pic x(4).
 07 v-data.
 08 v-data usage pointer.
 08 v-len usage binary-double unsigned.
 05 v-choices.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 05 v-threshold.
 06 v-kind usage binary-long unsigned.
 06 filler pic x(4).
 06 v-low usage float-long.
 06 v-high usage float-long.
 05 v-relation-threshold.
 06 v-kind usage binary-long unsigned.
 06 filler pic x(4).
 06 v-low usage float-long.
 06 v-high usage float-long.
 05 v-model.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 05 v-profile.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 05 v-batch.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value usage binary-double unsigned.
 05 v-batch-max usage binary-long signed.
 05 v-none usage binary-long signed.
 05 v-on.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 05 v-members.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 05 v-kinds.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 05 v-relations.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 05 v-name-pointer.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 05 v-kind-pointer.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 03 v-answer.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-kind usage binary-long unsigned.
 05 filler pic x(4).
 05 v-data.
 06 v-probability usage pointer.
 06 v-choice redefines v-probability usage pointer.
 06 v-tag redefines v-probability usage pointer.
 06 v-score redefines v-probability usage pointer.
 06 v-find redefines v-probability usage pointer.
 03 v-threshold.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-kind usage binary-long unsigned.
 05 filler pic x(4).
 05 v-low usage float-long.
 05 v-high usage float-long.
 03 v-position.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-file.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 05 v-first-line.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value usage binary-double unsigned.
 05 v-last-line.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value usage binary-double unsigned.
 03 v-input-file.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 03 v-meta.
 04 v-tool.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-question-sha256.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 04 v-questions-sha256.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 04 v-url.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-model.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-usage.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-input-tokens usage binary-double unsigned.
 06 v-output-tokens usage binary-double unsigned.
 04 v-requests-sent usage binary-double unsigned.
 04 v-cached usage binary-long signed.
 04 filler pic x(4).
 04 v-requests.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-failed-questions usage binary-double unsigned.
 04 v-profile-warning.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-tuned-for.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 06 v-running.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 04 v-batch-setting.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-kind usage binary-long unsigned.
 06 filler pic x(4).
 06 v-records usage binary-double unsigned.
 04 v-batch-warning.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-tuned-for.
 07 v-kind usage binary-long unsigned.
 07 filler pic x(4).
 07 v-records usage binary-double unsigned.
 06 v-running.
 07 v-kind usage binary-long unsigned.
 07 filler pic x(4).
 07 v-records usage binary-double unsigned.
 04 v-context-sha256.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 04 v-attempts.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 04 v-origin.
 05 v-present usage binary-long signed.
 05 v-value usage binary-long unsigned.
 04 v-question-sources.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-observations.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-answered-by.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 03 v-images.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 02 v-value.
 03 v-kind usage binary-long unsigned.
 03 filler pic x(4).
 03 v-data.
 04 v-boolean usage pointer.
 04 v-authored redefines v-boolean usage pointer.
01 tt-n-reported-usage-v1 based.
 02 v-present usage binary-long signed.
 02 filler pic x(4).
 02 v-input-tokens.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-value usage binary-double unsigned.
 02 v-output-tokens.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-value usage binary-double unsigned.
01 tt-n-source-detail-v1 based.
 02 v-origin usage binary-long unsigned.
 02 filler pic x(4).
 02 v-answered-by.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-batch-size.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-value usage binary-double unsigned.
01 tt-n-source-details-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-input-view-v1 based.
 02 v-original.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-value.
 04 v-kind usage binary-long unsigned.
 04 filler pic x(4).
 04 v-data.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 02 v-position.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-value.
 04 v-file.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 04 v-first-line.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value usage binary-double unsigned.
 04 v-last-line.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value usage binary-double unsigned.
 02 v-images.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
01 tt-n-input-views-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-details-v1 based.
 02 v-question.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-value.
 04 v-kind usage binary-long unsigned.
 04 filler pic x(4).
 04 v-text.
 05 v-kind usage binary-long unsigned.
 05 filler pic x(4).
 05 v-data.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 04 v-yes.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-kind usage binary-long unsigned.
 06 filler pic x(4).
 06 v-data.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 04 v-no.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-kind usage binary-long unsigned.
 06 filler pic x(4).
 06 v-data.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 04 v-choices.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-threshold.
 05 v-kind usage binary-long unsigned.
 05 filler pic x(4).
 05 v-low usage float-long.
 05 v-high usage float-long.
 04 v-relation-threshold.
 05 v-kind usage binary-long unsigned.
 05 filler pic x(4).
 05 v-low usage float-long.
 05 v-high usage float-long.
 04 v-model.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 04 v-profile.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 04 v-batch.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value usage binary-double unsigned.
 04 v-batch-max usage binary-long signed.
 04 v-none usage binary-long signed.
 04 v-on.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-members.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-kinds.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-relations.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-name-pointer.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 04 v-kind-pointer.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 02 v-threshold.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-value.
 04 v-kind usage binary-long unsigned.
 04 filler pic x(4).
 04 v-low usage float-long.
 04 v-high usage float-long.
 02 v-raw-pick.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-usage.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-input-tokens.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value usage binary-double unsigned.
 03 v-output-tokens.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value usage binary-double unsigned.
 02 v-question-sources.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-observations.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-inputs.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-filter-view-v1 based.
 02 v-common.
 03 v-answer-id.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 03 v-input.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-kind usage binary-long unsigned.
 05 filler pic x(4).
 05 v-data.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 03 v-question.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-kind usage binary-long unsigned.
 05 filler pic x(4).
 05 v-text.
 06 v-kind usage binary-long unsigned.
 06 filler pic x(4).
 06 v-data.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 05 v-yes.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value.
 07 v-kind usage binary-long unsigned.
 07 filler pic x(4).
 07 v-data.
 08 v-data usage pointer.
 08 v-len usage binary-double unsigned.
 05 v-no.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value.
 07 v-kind usage binary-long unsigned.
 07 filler pic x(4).
 07 v-data.
 08 v-data usage pointer.
 08 v-len usage binary-double unsigned.
 05 v-choices.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 05 v-threshold.
 06 v-kind usage binary-long unsigned.
 06 filler pic x(4).
 06 v-low usage float-long.
 06 v-high usage float-long.
 05 v-relation-threshold.
 06 v-kind usage binary-long unsigned.
 06 filler pic x(4).
 06 v-low usage float-long.
 06 v-high usage float-long.
 05 v-model.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 05 v-profile.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 05 v-batch.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value usage binary-double unsigned.
 05 v-batch-max usage binary-long signed.
 05 v-none usage binary-long signed.
 05 v-on.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 05 v-members.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 05 v-kinds.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 05 v-relations.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 05 v-name-pointer.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 05 v-kind-pointer.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 03 v-answer.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-kind usage binary-long unsigned.
 05 filler pic x(4).
 05 v-data.
 06 v-probability usage pointer.
 06 v-choice redefines v-probability usage pointer.
 06 v-tag redefines v-probability usage pointer.
 06 v-score redefines v-probability usage pointer.
 06 v-find redefines v-probability usage pointer.
 03 v-threshold.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-kind usage binary-long unsigned.
 05 filler pic x(4).
 05 v-low usage float-long.
 05 v-high usage float-long.
 03 v-position.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-file.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 05 v-first-line.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value usage binary-double unsigned.
 05 v-last-line.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value usage binary-double unsigned.
 03 v-input-file.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 03 v-meta.
 04 v-tool.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-question-sha256.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 04 v-questions-sha256.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 04 v-url.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-model.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-usage.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-input-tokens usage binary-double unsigned.
 06 v-output-tokens usage binary-double unsigned.
 04 v-requests-sent usage binary-double unsigned.
 04 v-cached usage binary-long signed.
 04 filler pic x(4).
 04 v-requests.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-failed-questions usage binary-double unsigned.
 04 v-profile-warning.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-tuned-for.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 06 v-running.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 04 v-batch-setting.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-kind usage binary-long unsigned.
 06 filler pic x(4).
 06 v-records usage binary-double unsigned.
 04 v-batch-warning.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-tuned-for.
 07 v-kind usage binary-long unsigned.
 07 filler pic x(4).
 07 v-records usage binary-double unsigned.
 06 v-running.
 07 v-kind usage binary-long unsigned.
 07 filler pic x(4).
 07 v-records usage binary-double unsigned.
 04 v-context-sha256.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 04 v-attempts.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 04 v-origin.
 05 v-present usage binary-long signed.
 05 v-value usage binary-long unsigned.
 04 v-question-sources.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-observations.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-answered-by.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 03 v-images.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 02 v-value usage binary-long signed.
 02 filler pic x(4).
01 tt-n-find-view-v1 based.
 02 v-common.
 03 v-answer-id.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 03 v-input.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-kind usage binary-long unsigned.
 05 filler pic x(4).
 05 v-data.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 03 v-question.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-kind usage binary-long unsigned.
 05 filler pic x(4).
 05 v-text.
 06 v-kind usage binary-long unsigned.
 06 filler pic x(4).
 06 v-data.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 05 v-yes.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value.
 07 v-kind usage binary-long unsigned.
 07 filler pic x(4).
 07 v-data.
 08 v-data usage pointer.
 08 v-len usage binary-double unsigned.
 05 v-no.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value.
 07 v-kind usage binary-long unsigned.
 07 filler pic x(4).
 07 v-data.
 08 v-data usage pointer.
 08 v-len usage binary-double unsigned.
 05 v-choices.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 05 v-threshold.
 06 v-kind usage binary-long unsigned.
 06 filler pic x(4).
 06 v-low usage float-long.
 06 v-high usage float-long.
 05 v-relation-threshold.
 06 v-kind usage binary-long unsigned.
 06 filler pic x(4).
 06 v-low usage float-long.
 06 v-high usage float-long.
 05 v-model.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 05 v-profile.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 05 v-batch.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value usage binary-double unsigned.
 05 v-batch-max usage binary-long signed.
 05 v-none usage binary-long signed.
 05 v-on.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 05 v-members.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 05 v-kinds.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 05 v-relations.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 05 v-name-pointer.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 05 v-kind-pointer.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 03 v-answer.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-kind usage binary-long unsigned.
 05 filler pic x(4).
 05 v-data.
 06 v-probability usage pointer.
 06 v-choice redefines v-probability usage pointer.
 06 v-tag redefines v-probability usage pointer.
 06 v-score redefines v-probability usage pointer.
 06 v-find redefines v-probability usage pointer.
 03 v-threshold.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-kind usage binary-long unsigned.
 05 filler pic x(4).
 05 v-low usage float-long.
 05 v-high usage float-long.
 03 v-position.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-file.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 05 v-first-line.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value usage binary-double unsigned.
 05 v-last-line.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value usage binary-double unsigned.
 03 v-input-file.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 03 v-meta.
 04 v-tool.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-question-sha256.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 04 v-questions-sha256.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 04 v-url.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-model.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-usage.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-input-tokens usage binary-double unsigned.
 06 v-output-tokens usage binary-double unsigned.
 04 v-requests-sent usage binary-double unsigned.
 04 v-cached usage binary-long signed.
 04 filler pic x(4).
 04 v-requests.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-failed-questions usage binary-double unsigned.
 04 v-profile-warning.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-tuned-for.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 06 v-running.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 04 v-batch-setting.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-kind usage binary-long unsigned.
 06 filler pic x(4).
 06 v-records usage binary-double unsigned.
 04 v-batch-warning.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-tuned-for.
 07 v-kind usage binary-long unsigned.
 07 filler pic x(4).
 07 v-records usage binary-double unsigned.
 06 v-running.
 07 v-kind usage binary-long unsigned.
 07 filler pic x(4).
 07 v-records usage binary-double unsigned.
 04 v-context-sha256.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 04 v-attempts.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 04 v-origin.
 05 v-present usage binary-long signed.
 05 v-value usage binary-long unsigned.
 04 v-question-sources.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-observations.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-answered-by.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 03 v-images.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 02 v-value.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-value.
 04 v-kind usage binary-long unsigned.
 04 filler pic x(4).
 04 v-data.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 02 v-index.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-value usage binary-double unsigned.
01 tt-n-observed-probabilities-v1 based.
 02 v-kind usage binary-long unsigned.
 02 filler pic x(4).
 02 v-data.
 03 v-yes usage pointer.
 03 v-named redefines v-yes usage pointer.
01 tt-n-observation-success-v1 based.
 02 v-answer-id.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-observation-id.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-value.
 03 v-kind usage binary-long unsigned.
 03 filler pic x(4).
 03 v-data.
 04 v-decide usage pointer.
 04 v-choose redefines v-decide usage pointer.
 04 v-tag redefines v-decide usage pointer.
 04 v-score redefines v-decide usage pointer.
 02 v-probabilities.
 03 v-kind usage binary-long unsigned.
 03 filler pic x(4).
 03 v-data.
 04 v-yes usage pointer.
 04 v-named redefines v-yes usage pointer.
 02 v-confidence.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-value usage float-long.
01 tt-n-question-observation-v1 based.
 02 v-index usage binary-double unsigned.
 02 v-member.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-stage.
 03 v-present usage binary-long signed.
 03 v-value usage binary-long unsigned.
 02 v-position usage binary-double unsigned.
 02 v-question-sha256.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-model.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-url.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-requests.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-requests-sent usage binary-double unsigned.
 02 v-cached usage binary-long signed.
 02 filler pic x(4).
 02 v-failed-questions usage binary-double unsigned.
 02 v-usage.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-value.
 04 v-input-tokens usage binary-double unsigned.
 04 v-output-tokens usage binary-double unsigned.
 02 v-question-sources.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-state usage binary-long unsigned.
 02 filler pic x(4).
 02 v-data.
 03 v-success usage pointer.
 03 v-failure redefines v-success usage pointer.
01 tt-n-tag-view-v1 based.
 02 v-common.
 03 v-answer-id.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 03 v-input.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-kind usage binary-long unsigned.
 05 filler pic x(4).
 05 v-data.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 03 v-question.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-kind usage binary-long unsigned.
 05 filler pic x(4).
 05 v-text.
 06 v-kind usage binary-long unsigned.
 06 filler pic x(4).
 06 v-data.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 05 v-yes.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value.
 07 v-kind usage binary-long unsigned.
 07 filler pic x(4).
 07 v-data.
 08 v-data usage pointer.
 08 v-len usage binary-double unsigned.
 05 v-no.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value.
 07 v-kind usage binary-long unsigned.
 07 filler pic x(4).
 07 v-data.
 08 v-data usage pointer.
 08 v-len usage binary-double unsigned.
 05 v-choices.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 05 v-threshold.
 06 v-kind usage binary-long unsigned.
 06 filler pic x(4).
 06 v-low usage float-long.
 06 v-high usage float-long.
 05 v-relation-threshold.
 06 v-kind usage binary-long unsigned.
 06 filler pic x(4).
 06 v-low usage float-long.
 06 v-high usage float-long.
 05 v-model.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 05 v-profile.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 05 v-batch.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value usage binary-double unsigned.
 05 v-batch-max usage binary-long signed.
 05 v-none usage binary-long signed.
 05 v-on.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 05 v-members.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 05 v-kinds.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 05 v-relations.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 05 v-name-pointer.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 05 v-kind-pointer.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 03 v-answer.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-kind usage binary-long unsigned.
 05 filler pic x(4).
 05 v-data.
 06 v-probability usage pointer.
 06 v-choice redefines v-probability usage pointer.
 06 v-tag redefines v-probability usage pointer.
 06 v-score redefines v-probability usage pointer.
 06 v-find redefines v-probability usage pointer.
 03 v-threshold.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-kind usage binary-long unsigned.
 05 filler pic x(4).
 05 v-low usage float-long.
 05 v-high usage float-long.
 03 v-position.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-file.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 05 v-first-line.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value usage binary-double unsigned.
 05 v-last-line.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value usage binary-double unsigned.
 03 v-input-file.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 03 v-meta.
 04 v-tool.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-question-sha256.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 04 v-questions-sha256.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 04 v-url.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-model.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-usage.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-input-tokens usage binary-double unsigned.
 06 v-output-tokens usage binary-double unsigned.
 04 v-requests-sent usage binary-double unsigned.
 04 v-cached usage binary-long signed.
 04 filler pic x(4).
 04 v-requests.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-failed-questions usage binary-double unsigned.
 04 v-profile-warning.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-tuned-for.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 06 v-running.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 04 v-batch-setting.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-kind usage binary-long unsigned.
 06 filler pic x(4).
 06 v-records usage binary-double unsigned.
 04 v-batch-warning.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-tuned-for.
 07 v-kind usage binary-long unsigned.
 07 filler pic x(4).
 07 v-records usage binary-double unsigned.
 06 v-running.
 07 v-kind usage binary-long unsigned.
 07 filler pic x(4).
 07 v-records usage binary-double unsigned.
 04 v-context-sha256.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 04 v-attempts.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 04 v-origin.
 05 v-present usage binary-long signed.
 05 v-value usage binary-long unsigned.
 04 v-question-sources.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-observations.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-answered-by.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 03 v-images.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-score-view-v1 based.
 02 v-common.
 03 v-answer-id.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 03 v-input.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-kind usage binary-long unsigned.
 05 filler pic x(4).
 05 v-data.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 03 v-question.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-kind usage binary-long unsigned.
 05 filler pic x(4).
 05 v-text.
 06 v-kind usage binary-long unsigned.
 06 filler pic x(4).
 06 v-data.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 05 v-yes.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value.
 07 v-kind usage binary-long unsigned.
 07 filler pic x(4).
 07 v-data.
 08 v-data usage pointer.
 08 v-len usage binary-double unsigned.
 05 v-no.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value.
 07 v-kind usage binary-long unsigned.
 07 filler pic x(4).
 07 v-data.
 08 v-data usage pointer.
 08 v-len usage binary-double unsigned.
 05 v-choices.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 05 v-threshold.
 06 v-kind usage binary-long unsigned.
 06 filler pic x(4).
 06 v-low usage float-long.
 06 v-high usage float-long.
 05 v-relation-threshold.
 06 v-kind usage binary-long unsigned.
 06 filler pic x(4).
 06 v-low usage float-long.
 06 v-high usage float-long.
 05 v-model.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 05 v-profile.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 05 v-batch.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value usage binary-double unsigned.
 05 v-batch-max usage binary-long signed.
 05 v-none usage binary-long signed.
 05 v-on.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 05 v-members.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 05 v-kinds.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 05 v-relations.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 05 v-name-pointer.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 05 v-kind-pointer.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 03 v-answer.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-kind usage binary-long unsigned.
 05 filler pic x(4).
 05 v-data.
 06 v-probability usage pointer.
 06 v-choice redefines v-probability usage pointer.
 06 v-tag redefines v-probability usage pointer.
 06 v-score redefines v-probability usage pointer.
 06 v-find redefines v-probability usage pointer.
 03 v-threshold.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-kind usage binary-long unsigned.
 05 filler pic x(4).
 05 v-low usage float-long.
 05 v-high usage float-long.
 03 v-position.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-file.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 05 v-first-line.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value usage binary-double unsigned.
 05 v-last-line.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value usage binary-double unsigned.
 03 v-input-file.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 03 v-meta.
 04 v-tool.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-question-sha256.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 04 v-questions-sha256.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 04 v-url.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-model.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-usage.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-input-tokens usage binary-double unsigned.
 06 v-output-tokens usage binary-double unsigned.
 04 v-requests-sent usage binary-double unsigned.
 04 v-cached usage binary-long signed.
 04 filler pic x(4).
 04 v-requests.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-failed-questions usage binary-double unsigned.
 04 v-profile-warning.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-tuned-for.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 06 v-running.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 04 v-batch-setting.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-kind usage binary-long unsigned.
 06 filler pic x(4).
 06 v-records usage binary-double unsigned.
 04 v-batch-warning.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-tuned-for.
 07 v-kind usage binary-long unsigned.
 07 filler pic x(4).
 07 v-records usage binary-double unsigned.
 06 v-running.
 07 v-kind usage binary-long unsigned.
 07 filler pic x(4).
 07 v-records usage binary-double unsigned.
 04 v-context-sha256.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 04 v-attempts.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 04 v-origin.
 05 v-present usage binary-long signed.
 05 v-value usage binary-long unsigned.
 04 v-question-sources.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-observations.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-answered-by.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 03 v-images.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 02 v-value usage float-long.
01 tt-n-rank-view-v1 based.
 02 v-common.
 03 v-answer-id.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 03 v-input.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-kind usage binary-long unsigned.
 05 filler pic x(4).
 05 v-data.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 03 v-question.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-kind usage binary-long unsigned.
 05 filler pic x(4).
 05 v-text.
 06 v-kind usage binary-long unsigned.
 06 filler pic x(4).
 06 v-data.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 05 v-yes.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value.
 07 v-kind usage binary-long unsigned.
 07 filler pic x(4).
 07 v-data.
 08 v-data usage pointer.
 08 v-len usage binary-double unsigned.
 05 v-no.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value.
 07 v-kind usage binary-long unsigned.
 07 filler pic x(4).
 07 v-data.
 08 v-data usage pointer.
 08 v-len usage binary-double unsigned.
 05 v-choices.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 05 v-threshold.
 06 v-kind usage binary-long unsigned.
 06 filler pic x(4).
 06 v-low usage float-long.
 06 v-high usage float-long.
 05 v-relation-threshold.
 06 v-kind usage binary-long unsigned.
 06 filler pic x(4).
 06 v-low usage float-long.
 06 v-high usage float-long.
 05 v-model.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 05 v-profile.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 05 v-batch.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value usage binary-double unsigned.
 05 v-batch-max usage binary-long signed.
 05 v-none usage binary-long signed.
 05 v-on.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 05 v-members.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 05 v-kinds.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 05 v-relations.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 05 v-name-pointer.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 05 v-kind-pointer.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 03 v-answer.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-kind usage binary-long unsigned.
 05 filler pic x(4).
 05 v-data.
 06 v-probability usage pointer.
 06 v-choice redefines v-probability usage pointer.
 06 v-tag redefines v-probability usage pointer.
 06 v-score redefines v-probability usage pointer.
 06 v-find redefines v-probability usage pointer.
 03 v-threshold.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-kind usage binary-long unsigned.
 05 filler pic x(4).
 05 v-low usage float-long.
 05 v-high usage float-long.
 03 v-position.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-file.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 05 v-first-line.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value usage binary-double unsigned.
 05 v-last-line.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value usage binary-double unsigned.
 03 v-input-file.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 03 v-meta.
 04 v-tool.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-question-sha256.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 04 v-questions-sha256.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 04 v-url.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-model.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-usage.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-input-tokens usage binary-double unsigned.
 06 v-output-tokens usage binary-double unsigned.
 04 v-requests-sent usage binary-double unsigned.
 04 v-cached usage binary-long signed.
 04 filler pic x(4).
 04 v-requests.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-failed-questions usage binary-double unsigned.
 04 v-profile-warning.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-tuned-for.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 06 v-running.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 04 v-batch-setting.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-kind usage binary-long unsigned.
 06 filler pic x(4).
 06 v-records usage binary-double unsigned.
 04 v-batch-warning.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-tuned-for.
 07 v-kind usage binary-long unsigned.
 07 filler pic x(4).
 07 v-records usage binary-double unsigned.
 06 v-running.
 07 v-kind usage binary-long unsigned.
 07 filler pic x(4).
 07 v-records usage binary-double unsigned.
 04 v-context-sha256.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 04 v-attempts.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 04 v-origin.
 05 v-present usage binary-long signed.
 05 v-value usage binary-long unsigned.
 04 v-question-sources.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-observations.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-answered-by.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 03 v-images.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 02 v-value.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-value usage binary-double unsigned.
 02 v-question-name.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
01 tt-n-entity-v1 based.
 02 v-text.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-start usage binary-double unsigned.
 02 v-end usage binary-double unsigned.
 02 v-length usage binary-double unsigned.
 02 v-kind.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-strength usage float-long.
01 tt-n-entities-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-entity-edge-v1 based.
 02 v-relation.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-source.
 03 v-text.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 03 v-start usage binary-double unsigned.
 03 v-end usage binary-double unsigned.
 03 v-length usage binary-double unsigned.
 03 v-kind.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 03 v-strength usage float-long.
 02 v-target.
 03 v-text.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 03 v-start usage binary-double unsigned.
 03 v-end usage binary-double unsigned.
 03 v-length usage binary-double unsigned.
 03 v-kind.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 03 v-strength usage float-long.
 02 v-probability usage float-long.
 02 v-either usage binary-long signed.
 02 filler pic x(4).
01 tt-n-entity-edges-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-optional-entity-edges-v1 based.
 02 v-present usage binary-long signed.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-recognize-value-v1 based.
 02 v-entities.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-relations.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
01 tt-n-piece-v1 based.
 02 v-start usage binary-double unsigned.
 02 v-end usage binary-double unsigned.
 02 v-tags.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-pieces-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-optional-probabilities-v1 based.
 02 v-present usage binary-long signed.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-name-v1 based.
 02 v-start usage binary-double unsigned.
 02 v-end usage binary-double unsigned.
 02 v-kinds.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-edges.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
01 tt-n-names-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-place-v1 based.
 02 v-start usage binary-double unsigned.
 02 v-end usage binary-double unsigned.
01 tt-n-pair-v1 based.
 02 v-relation.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-source.
 03 v-start usage binary-double unsigned.
 03 v-end usage binary-double unsigned.
 02 v-target.
 03 v-start usage binary-double unsigned.
 03 v-end usage binary-double unsigned.
 02 v-probability usage float-long.
01 tt-n-pairs-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-recognize-answer-v1 based.
 02 v-pieces.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-names.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-pairs.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-recognize-view-v1 based.
 02 v-common.
 03 v-answer-id.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 03 v-input.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-kind usage binary-long unsigned.
 05 filler pic x(4).
 05 v-data.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 03 v-question.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-kind usage binary-long unsigned.
 05 filler pic x(4).
 05 v-text.
 06 v-kind usage binary-long unsigned.
 06 filler pic x(4).
 06 v-data.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 05 v-yes.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value.
 07 v-kind usage binary-long unsigned.
 07 filler pic x(4).
 07 v-data.
 08 v-data usage pointer.
 08 v-len usage binary-double unsigned.
 05 v-no.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value.
 07 v-kind usage binary-long unsigned.
 07 filler pic x(4).
 07 v-data.
 08 v-data usage pointer.
 08 v-len usage binary-double unsigned.
 05 v-choices.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 05 v-threshold.
 06 v-kind usage binary-long unsigned.
 06 filler pic x(4).
 06 v-low usage float-long.
 06 v-high usage float-long.
 05 v-relation-threshold.
 06 v-kind usage binary-long unsigned.
 06 filler pic x(4).
 06 v-low usage float-long.
 06 v-high usage float-long.
 05 v-model.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 05 v-profile.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 05 v-batch.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value usage binary-double unsigned.
 05 v-batch-max usage binary-long signed.
 05 v-none usage binary-long signed.
 05 v-on.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 05 v-members.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 05 v-kinds.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 05 v-relations.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 05 v-name-pointer.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 05 v-kind-pointer.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 03 v-answer.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-kind usage binary-long unsigned.
 05 filler pic x(4).
 05 v-data.
 06 v-probability usage pointer.
 06 v-choice redefines v-probability usage pointer.
 06 v-tag redefines v-probability usage pointer.
 06 v-score redefines v-probability usage pointer.
 06 v-find redefines v-probability usage pointer.
 03 v-threshold.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-kind usage binary-long unsigned.
 05 filler pic x(4).
 05 v-low usage float-long.
 05 v-high usage float-long.
 03 v-position.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-file.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 05 v-first-line.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value usage binary-double unsigned.
 05 v-last-line.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value usage binary-double unsigned.
 03 v-input-file.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 03 v-meta.
 04 v-tool.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-question-sha256.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 04 v-questions-sha256.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 04 v-url.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-model.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-usage.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-input-tokens usage binary-double unsigned.
 06 v-output-tokens usage binary-double unsigned.
 04 v-requests-sent usage binary-double unsigned.
 04 v-cached usage binary-long signed.
 04 filler pic x(4).
 04 v-requests.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-failed-questions usage binary-double unsigned.
 04 v-profile-warning.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-tuned-for.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 06 v-running.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 04 v-batch-setting.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-kind usage binary-long unsigned.
 06 filler pic x(4).
 06 v-records usage binary-double unsigned.
 04 v-batch-warning.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-tuned-for.
 07 v-kind usage binary-long unsigned.
 07 filler pic x(4).
 07 v-records usage binary-double unsigned.
 06 v-running.
 07 v-kind usage binary-long unsigned.
 07 filler pic x(4).
 07 v-records usage binary-double unsigned.
 04 v-context-sha256.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 04 v-attempts.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 04 v-origin.
 05 v-present usage binary-long signed.
 05 v-value usage binary-long unsigned.
 04 v-question-sources.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-observations.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-answered-by.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 03 v-images.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 02 v-value.
 03 v-entities.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 03 v-relations.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 02 v-answer.
 03 v-pieces.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 03 v-names.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 03 v-pairs.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
01 tt-n-endpoint-v1 based.
 02 v-name.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-kind.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-edge-v1 based.
 02 v-relation.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-source.
 03 v-name.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 03 v-kind.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-target.
 03 v-name.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 03 v-kind.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-probability usage float-long.
 02 v-either usage binary-long signed.
 02 filler pic x(4).
01 tt-n-edges-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-optional-endpoint-v1 based.
 02 v-present usage binary-long signed.
 02 filler pic x(4).
 02 v-value.
 03 v-name.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 03 v-kind.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
01 tt-n-relation-success-v1 based.
 02 v-answer-id.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-probability usage float-long.
 02 v-accepted usage binary-long signed.
 02 filler pic x(4).
01 tt-n-relation-answer-v1 based.
 02 v-relation.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-reads.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-method usage binary-long unsigned.
 02 v-direction usage binary-long unsigned.
 02 v-source.
 03 v-name.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 03 v-kind.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-target.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-value.
 04 v-name.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-kind.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 02 v-request.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-state usage binary-long unsigned.
 02 filler pic x(4).
 02 v-data.
 03 v-success usage pointer.
 03 v-failure redefines v-success usage pointer.
01 tt-n-relation-answers-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-relate-view-v1 based.
 02 v-common.
 03 v-answer-id.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 03 v-input.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-kind usage binary-long unsigned.
 05 filler pic x(4).
 05 v-data.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 03 v-question.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-kind usage binary-long unsigned.
 05 filler pic x(4).
 05 v-text.
 06 v-kind usage binary-long unsigned.
 06 filler pic x(4).
 06 v-data.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 05 v-yes.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value.
 07 v-kind usage binary-long unsigned.
 07 filler pic x(4).
 07 v-data.
 08 v-data usage pointer.
 08 v-len usage binary-double unsigned.
 05 v-no.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value.
 07 v-kind usage binary-long unsigned.
 07 filler pic x(4).
 07 v-data.
 08 v-data usage pointer.
 08 v-len usage binary-double unsigned.
 05 v-choices.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 05 v-threshold.
 06 v-kind usage binary-long unsigned.
 06 filler pic x(4).
 06 v-low usage float-long.
 06 v-high usage float-long.
 05 v-relation-threshold.
 06 v-kind usage binary-long unsigned.
 06 filler pic x(4).
 06 v-low usage float-long.
 06 v-high usage float-long.
 05 v-model.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 05 v-profile.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 05 v-batch.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value usage binary-double unsigned.
 05 v-batch-max usage binary-long signed.
 05 v-none usage binary-long signed.
 05 v-on.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 05 v-members.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 05 v-kinds.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 05 v-relations.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 05 v-name-pointer.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 05 v-kind-pointer.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 03 v-answer.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-kind usage binary-long unsigned.
 05 filler pic x(4).
 05 v-data.
 06 v-probability usage pointer.
 06 v-choice redefines v-probability usage pointer.
 06 v-tag redefines v-probability usage pointer.
 06 v-score redefines v-probability usage pointer.
 06 v-find redefines v-probability usage pointer.
 03 v-threshold.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-kind usage binary-long unsigned.
 05 filler pic x(4).
 05 v-low usage float-long.
 05 v-high usage float-long.
 03 v-position.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-file.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 05 v-first-line.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value usage binary-double unsigned.
 05 v-last-line.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value usage binary-double unsigned.
 03 v-input-file.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 03 v-meta.
 04 v-tool.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-question-sha256.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 04 v-questions-sha256.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 04 v-url.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-model.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-usage.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-input-tokens usage binary-double unsigned.
 06 v-output-tokens usage binary-double unsigned.
 04 v-requests-sent usage binary-double unsigned.
 04 v-cached usage binary-long signed.
 04 filler pic x(4).
 04 v-requests.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-failed-questions usage binary-double unsigned.
 04 v-profile-warning.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-tuned-for.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 06 v-running.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 04 v-batch-setting.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-kind usage binary-long unsigned.
 06 filler pic x(4).
 06 v-records usage binary-double unsigned.
 04 v-batch-warning.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-tuned-for.
 07 v-kind usage binary-long unsigned.
 07 filler pic x(4).
 07 v-records usage binary-double unsigned.
 06 v-running.
 07 v-kind usage binary-long unsigned.
 07 filler pic x(4).
 07 v-records usage binary-double unsigned.
 04 v-context-sha256.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 04 v-attempts.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 04 v-origin.
 05 v-present usage binary-long signed.
 05 v-value usage binary-long unsigned.
 04 v-question-sources.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-observations.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-answered-by.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 03 v-images.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-questions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-row-observation-v1 based.
 02 v-index usage binary-double unsigned.
 02 v-function usage binary-long unsigned.
 02 filler pic x(4).
 02 v-data.
 03 v-decide usage pointer.
 03 v-choose redefines v-decide usage pointer.
 03 v-tag redefines v-decide usage pointer.
 03 v-score redefines v-decide usage pointer.
 03 v-filter redefines v-decide usage pointer.
 03 v-rank redefines v-decide usage pointer.
 03 v-find redefines v-decide usage pointer.
 03 v-annotate redefines v-decide usage pointer.
 03 v-recognize redefines v-decide usage pointer.
 03 v-relate redefines v-decide usage pointer.
01 tt-n-observation-v1 based.
 02 v-kind usage binary-long unsigned.
 02 filler pic x(4).
 02 v-data.
 03 v-question usage pointer.
 03 v-row redefines v-question usage pointer.
01 tt-n-source-entity-v1 based.
 02 v-entity.
 03 v-text.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 03 v-start usage binary-double unsigned.
 03 v-end usage binary-double unsigned.
 03 v-length usage binary-double unsigned.
 03 v-kind.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 03 v-strength usage float-long.
 02 v-position.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-value.
 04 v-file.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 04 v-first-line.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value usage binary-double unsigned.
 04 v-last-line.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value usage binary-double unsigned.
01 tt-n-source-entities-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-source-entity-edge-v1 based.
 02 v-relation.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-source.
 03 v-entity.
 04 v-text.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-start usage binary-double unsigned.
 04 v-end usage binary-double unsigned.
 04 v-length usage binary-double unsigned.
 04 v-kind.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-strength usage float-long.
 03 v-position.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-file.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 05 v-first-line.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value usage binary-double unsigned.
 05 v-last-line.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value usage binary-double unsigned.
 02 v-target.
 03 v-entity.
 04 v-text.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-start usage binary-double unsigned.
 04 v-end usage binary-double unsigned.
 04 v-length usage binary-double unsigned.
 04 v-kind.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-strength usage float-long.
 03 v-position.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-file.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 05 v-first-line.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value usage binary-double unsigned.
 05 v-last-line.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value usage binary-double unsigned.
 02 v-probability usage float-long.
 02 v-either usage binary-long signed.
 02 filler pic x(4).
01 tt-n-source-entity-edges-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-optional-source-entity-edges-v1 based.
 02 v-present usage binary-long signed.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-source-recognition-v1 based.
 02 v-present usage binary-long signed.
 02 filler pic x(4).
 02 v-entities.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-relations.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
01 tt-n-source-endpoint-v1 based.
 02 v-ordinal usage binary-double unsigned.
 02 v-endpoint.
 03 v-name.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 03 v-kind.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-record.
 03 v-kind usage binary-long unsigned.
 03 filler pic x(4).
 03 v-data.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-position.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-value.
 04 v-file.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 04 v-first-line.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value usage binary-double unsigned.
 04 v-last-line.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value usage binary-double unsigned.
01 tt-n-source-edge-v1 based.
 02 v-relation.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-source.
 03 v-ordinal usage binary-double unsigned.
 03 v-endpoint.
 04 v-name.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-kind.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 03 v-record.
 04 v-kind usage binary-long unsigned.
 04 filler pic x(4).
 04 v-data.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 03 v-position.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-file.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 05 v-first-line.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value usage binary-double unsigned.
 05 v-last-line.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value usage binary-double unsigned.
 02 v-target.
 03 v-ordinal usage binary-double unsigned.
 03 v-endpoint.
 04 v-name.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-kind.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 03 v-record.
 04 v-kind usage binary-long unsigned.
 04 filler pic x(4).
 04 v-data.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 03 v-position.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-file.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 05 v-first-line.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value usage binary-double unsigned.
 05 v-last-line.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value usage binary-double unsigned.
 02 v-probability usage float-long.
 02 v-either usage binary-long signed.
 02 filler pic x(4).
01 tt-n-source-edges-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-source-relations-v1 based.
 02 v-present usage binary-long signed.
 02 filler pic x(4).
 02 v-edges.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-optional-meta-v1 based.
 02 v-present usage binary-long signed.
 02 filler pic x(4).
 02 v-value.
 03 v-tool.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 03 v-question-sha256.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 03 v-questions-sha256.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 03 v-url.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 03 v-model.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 03 v-usage.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-input-tokens usage binary-double unsigned.
 05 v-output-tokens usage binary-double unsigned.
 03 v-requests-sent usage binary-double unsigned.
 03 v-cached usage binary-long signed.
 03 filler pic x(4).
 03 v-requests.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 03 v-failed-questions usage binary-double unsigned.
 03 v-profile-warning.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-tuned-for.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 05 v-running.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 03 v-batch-setting.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-kind usage binary-long unsigned.
 05 filler pic x(4).
 05 v-records usage binary-double unsigned.
 03 v-batch-warning.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-tuned-for.
 06 v-kind usage binary-long unsigned.
 06 filler pic x(4).
 06 v-records usage binary-double unsigned.
 05 v-running.
 06 v-kind usage binary-long unsigned.
 06 filler pic x(4).
 06 v-records usage binary-double unsigned.
 03 v-context-sha256.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 03 v-attempts.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 03 v-origin.
 04 v-present usage binary-long signed.
 04 v-value usage binary-long unsigned.
 03 v-question-sources.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 03 v-observations.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 03 v-answered-by.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
01 tt-n-facts-v1 based.
 02 v-call-id.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-cache-answers usage binary-double unsigned.
 02 v-estimated-cost-usd.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-input-tokens.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-value usage binary-double unsigned.
 02 v-model.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-output-tokens.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-value usage binary-double unsigned.
 02 v-records usage binary-double unsigned.
 02 v-requests-sent usage binary-double unsigned.
 02 v-seconds usage float-long.
 02 v-command-ms.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-value usage binary-double unsigned.
01 tt-n-optional-facts-v1 based.
 02 v-present usage binary-long signed.
 02 filler pic x(4).
 02 v-value.
 03 v-call-id.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 03 v-cache-answers usage binary-double unsigned.
 03 v-estimated-cost-usd.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 03 v-input-tokens.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value usage binary-double unsigned.
 03 v-model.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 03 v-output-tokens.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value usage binary-double unsigned.
 03 v-records usage binary-double unsigned.
 03 v-requests-sent usage binary-double unsigned.
 03 v-seconds usage float-long.
 03 v-command-ms.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value usage binary-double unsigned.
01 tt-n-stopped-v1 based.
 02 v-at.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-value usage binary-double unsigned.
 02 v-cause usage binary-long unsigned.
 02 v-status.
 03 v-present usage binary-long signed.
 03 v-value usage binary-short unsigned.
 03 filler pic x(2).
 02 v-retryable usage binary-long signed.
01 tt-n-optional-stopped-v1 based.
 02 v-present usage binary-long signed.
 02 filler pic x(4).
 02 v-value.
 03 v-at.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value usage binary-double unsigned.
 03 v-cause usage binary-long unsigned.
 03 v-status.
 04 v-present usage binary-long signed.
 04 v-value usage binary-short unsigned.
 04 filler pic x(2).
 03 v-retryable usage binary-long signed.
01 tt-n-error-v1 based.
 02 v-code usage binary-long signed.
 02 filler pic x(4).
 02 v-message.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-retryable usage binary-long signed.
 02 filler pic x(4).
 02 v-stopped.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-value.
 04 v-at.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value usage binary-double unsigned.
 04 v-cause usage binary-long unsigned.
 04 v-status.
 05 v-present usage binary-long signed.
 05 v-value usage binary-short unsigned.
 05 filler pic x(2).
 04 v-retryable usage binary-long signed.
01 tt-n-optional-error-v1 based.
 02 v-present usage binary-long signed.
 02 filler pic x(4).
 02 v-value.
 03 v-code usage binary-long signed.
 03 filler pic x(4).
 03 v-message.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 03 v-retryable usage binary-long signed.
 03 filler pic x(4).
 03 v-stopped.
 04 v-present usage binary-long signed.
 04 filler pic x(4).
 04 v-value.
 05 v-at.
 06 v-present usage binary-long signed.
 06 filler pic x(4).
 06 v-value usage binary-double unsigned.
 05 v-cause usage binary-long unsigned.
 05 v-status.
 06 v-present usage binary-long signed.
 06 v-value usage binary-short unsigned.
 06 filler pic x(2).
 05 v-retryable usage binary-long signed.
01 tt-n-summary-v1 based.
 02 v-state usage binary-long unsigned.
 02 filler pic x(4).
 02 v-schema.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-answer-id.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-function.
 03 v-present usage binary-long signed.
 03 v-value usage binary-long unsigned.
 02 v-count usage binary-double unsigned.
 02 v-observation-count usage binary-double unsigned.
 02 v-meta.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-value.
 04 v-tool.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-question-sha256.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 04 v-questions-sha256.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 04 v-url.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-model.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-usage.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-input-tokens usage binary-double unsigned.
 06 v-output-tokens usage binary-double unsigned.
 04 v-requests-sent usage binary-double unsigned.
 04 v-cached usage binary-long signed.
 04 filler pic x(4).
 04 v-requests.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-failed-questions usage binary-double unsigned.
 04 v-profile-warning.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-tuned-for.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 06 v-running.
 07 v-data usage pointer.
 07 v-len usage binary-double unsigned.
 04 v-batch-setting.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-kind usage binary-long unsigned.
 06 filler pic x(4).
 06 v-records usage binary-double unsigned.
 04 v-batch-warning.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-tuned-for.
 07 v-kind usage binary-long unsigned.
 07 filler pic x(4).
 07 v-records usage binary-double unsigned.
 06 v-running.
 07 v-kind usage binary-long unsigned.
 07 filler pic x(4).
 07 v-records usage binary-double unsigned.
 04 v-context-sha256.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 04 v-attempts.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 04 v-origin.
 05 v-present usage binary-long signed.
 05 v-value usage binary-long unsigned.
 04 v-question-sources.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-observations.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-answered-by.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 02 v-facts.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-value.
 04 v-call-id.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-cache-answers usage binary-double unsigned.
 04 v-estimated-cost-usd.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 04 v-input-tokens.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value usage binary-double unsigned.
 04 v-model.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-data usage pointer.
 06 v-len usage binary-double unsigned.
 04 v-output-tokens.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value usage binary-double unsigned.
 04 v-records usage binary-double unsigned.
 04 v-requests-sent usage binary-double unsigned.
 04 v-seconds usage float-long.
 04 v-command-ms.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value usage binary-double unsigned.
 02 v-attempts.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-error.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-value.
 04 v-code usage binary-long signed.
 04 filler pic x(4).
 04 v-message.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 04 v-retryable usage binary-long signed.
 04 filler pic x(4).
 04 v-stopped.
 05 v-present usage binary-long signed.
 05 filler pic x(4).
 05 v-value.
 06 v-at.
 07 v-present usage binary-long signed.
 07 filler pic x(4).
 07 v-value usage binary-double unsigned.
 06 v-cause usage binary-long unsigned.
 06 v-status.
 07 v-present usage binary-long signed.
 07 v-value usage binary-short unsigned.
 07 filler pic x(2).
 06 v-retryable usage binary-long signed.
01 tt-n-complete-utf8-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-extension-v1 based.
 02 v-name.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-json.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-extensions-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-answer-yes-no-v1 based.
 02 v-kind.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-probability usage float-long.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-answer-choice-field-confidence-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage float-long.
01 tt-n-complete-answer-choice-field-probabilities-entry-v1 based.
 02 v-name.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-value usage float-long.
01 tt-n-complete-answer-choice-field-probabilities-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-answer-choice-v1 based.
 02 v-confidence.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage float-long.
 02 v-kind.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-pick.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-probabilities.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-answer-tag-field-probabilities-entry-v1 based.
 02 v-name.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-value usage float-long.
01 tt-n-complete-answer-tag-field-probabilities-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-answer-tag-v1 based.
 02 v-kind.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-probabilities.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-answer-score-field-confidence-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage float-long.
01 tt-n-complete-answer-score-field-probabilities-entry-v1 based.
 02 v-name.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-value usage float-long.
01 tt-n-complete-answer-score-field-probabilities-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-answer-score-v1 based.
 02 v-confidence.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage float-long.
 02 v-kind.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-level.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-probabilities.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-answer-v1 based.
 02 v-kind usage binary-long unsigned.
 02 filler pic x(4).
 02 v-data.
 03 v-yes-no usage pointer.
 03 v-choice redefines v-yes-no usage pointer.
 03 v-tag redefines v-yes-no usage pointer.
 03 v-score redefines v-yes-no usage pointer.
01 tt-n-complete-answer-id-v1 based.
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-image-media-v1 based.
 02 v-kind usage binary-long unsigned.
01 tt-n-complete-image-v1 based.
 02 v-base64.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-height usage binary-double unsigned.
 02 v-media usage pointer.
 02 v-width usage binary-double unsigned.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-atomic-decide-value-field-images-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-atomic-decide-value-field-images-p-6049aaeb based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-atomic-decide-value-field-index-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage binary-double unsigned.
01 tt-n-complete-json-array-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-json-entry-v1 based.
 02 v-name.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-value usage pointer.
01 tt-n-complete-json-object-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-json-v1 based.
 02 v-kind usage binary-long unsigned.
 02 filler pic x(4).
 02 v-data.
 03 v-boolean usage pointer.
 03 v-number redefines v-boolean usage pointer.
 03 v-string redefines v-boolean usage pointer.
 03 v-array redefines v-boolean usage pointer.
 03 v-object redefines v-boolean usage pointer.
01 tt-n-complete-atomic-decide-value-field-input-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-rank-member-result-field-images-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-rank-member-result-field-images-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-meta-field-answered-by-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-attempt-outcome-v1 based.
 02 v-kind usage binary-long unsigned.
01 tt-n-complete-attempt-field-request-id-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-sdk-request-id-v1 based.
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-attempt-field-server-ms-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage binary-double unsigned.
01 tt-n-complete-attempt-field-status-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage binary-double unsigned.
01 tt-n-complete-attempt-v1 based.
 02 v-ordinal usage binary-double unsigned.
 02 v-outcome usage pointer.
 02 v-request-id.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-request-sha256.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-sdk-request-id usage pointer.
 02 v-server-ms.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage binary-double unsigned.
 02 v-status.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage binary-double unsigned.
 02 v-wall-ms usage binary-double unsigned.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-meta-field-attempts-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-meta-field-attempts-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-batch-setting-v1 based.
 02 v-kind usage binary-long unsigned.
 02 filler pic x(4).
 02 v-data.
 03 v-integer usage pointer.
 03 v-string redefines v-integer usage pointer.
01 tt-n-complete-meta-field-batch-setting-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-batch-warning-v1 based.
 02 v-running usage pointer.
 02 v-tuned-for usage pointer.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-meta-field-batch-warning-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-meta-field-context-sha-256-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-observation-id-v1 based.
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-observation-observation-id-v1 based.
 02 v-observation-id usage pointer.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-failure-id-v1 based.
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-observation-failure-id-v1 based.
 02 v-failure-id usage pointer.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-observation-v1 based.
 02 v-kind usage binary-long unsigned.
 02 filler pic x(4).
 02 v-data.
 03 v-observation-id usage pointer.
 03 v-failure-id redefines v-observation-id usage pointer.
01 tt-n-complete-meta-field-observations-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-origin-v1 based.
 02 v-kind usage binary-long unsigned.
01 tt-n-complete-meta-field-origin-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-profile-warning-v1 based.
 02 v-running.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-tuned-for.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-meta-field-profile-warning-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-meta-field-question-sha-256-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-question-source-field-batch-size-p-41d30d18 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage binary-double unsigned.
01 tt-n-complete-question-source-v1 based.
 02 v-answered-by.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-batch-size.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage binary-double unsigned.
 02 v-origin usage pointer.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-meta-field-question-sources-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-meta-field-questions-sha-256-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-meta-field-requests-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-usage-field-input-tokens-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage binary-double unsigned.
01 tt-n-complete-usage-field-output-tokens-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage binary-double unsigned.
01 tt-n-complete-usage-v1 based.
 02 v-input-tokens.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage binary-double unsigned.
 02 v-output-tokens.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage binary-double unsigned.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-meta-field-usage-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-meta-v1 based.
 02 v-answered-by.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-attempts.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-batch-setting.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-batch-warning.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-cached usage binary-long unsigned.
 02 filler pic x(4).
 02 v-context-sha256.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-failed-questions usage binary-double unsigned.
 02 v-model.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-observations.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-origin.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-profile-warning.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-question-sha256.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-question-sources.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-questions-sha256.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-requests.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-requests-sent usage binary-double unsigned.
 02 v-tool.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-url.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-usage.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-batch-string-v1 based.
 02 v-kind usage binary-long unsigned.
01 tt-n-complete-batch-v1 based.
 02 v-kind usage binary-long unsigned.
 02 filler pic x(4).
 02 v-data.
 03 v-integer usage pointer.
 03 v-string redefines v-integer usage pointer.
01 tt-n-complete-readable-question-decide-field-bat-7b340908 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-string-type-v1 based.
 02 v-kind usage binary-long unsigned.
01 tt-n-complete-input-declaration-string-v1 based.
 02 v-type usage pointer.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-input-property-type-string-v1 based.
 02 v-type.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-input-property-type-number-v1 based.
 02 v-type.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-input-property-type-boolean-v1 based.
 02 v-type.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-string-root-v1 based.
 02 v-type usage pointer.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-input-property-type-array-v1 based.
 02 v-items usage pointer.
 02 v-type.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-input-property-type-v1 based.
 02 v-kind usage binary-long unsigned.
 02 filler pic x(4).
 02 v-data.
 03 v-string usage pointer.
 03 v-number redefines v-string usage pointer.
 03 v-boolean redefines v-string usage pointer.
 03 v-array redefines v-string usage pointer.
01 tt-n-complete-input-declaration-object-field-pro-338622f0 based.
 02 v-name.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-value usage pointer.
01 tt-n-complete-input-declaration-object-field-pro-59f2100d based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-input-declaration-object-field-required-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-input-declaration-object-field-req-10275d32 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-object-type-v1 based.
 02 v-kind usage binary-long unsigned.
01 tt-n-complete-input-declaration-object-v1 based.
 02 v-properties.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-required.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-type usage pointer.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-input-declaration-v1 based.
 02 v-kind usage binary-long unsigned.
 02 filler pic x(4).
 02 v-data.
 03 v-string usage pointer.
 03 v-object redefines v-string usage pointer.
01 tt-n-complete-readable-question-decide-field-con-9de0fff0 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-readable-question-decide-field-ite-b0bb16c0 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-label-field-description-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-label-v1 based.
 02 v-description.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-name.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-readable-question-decide-field-lab-124d8f2b based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-readable-question-decide-field-lab-371b5406 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-readable-question-decide-field-mod-e4b39989 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-question-name-v1 based.
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-readable-question-decide-field-nam-0358e91d based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-readable-question-decide-field-on-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-readable-question-decide-field-on--37d188ab based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-readable-question-decide-field-pro-50db1d9b based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-wording-version-v1 based.
 02 v-value usage binary-double unsigned.
01 tt-n-complete-readable-question-decide-field-wor-b6d4c390 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-readable-question-decide-field-fal-8c5971d4 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-readable-question-decide-field-tex-a97e9136 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-readable-question-decide-field-tru-ffb12217 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-readable-question-decide-v1 based.
 02 v-batch.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-context-schema.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-item-schema.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-label-details.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-model.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-name.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-on.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-profile.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-wording-version.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-false.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-text.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-true.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-verb.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-readable-question-choose-field-bat-dca52f94 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-readable-question-choose-field-con-bf3637a4 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-readable-question-choose-field-ite-4c496912 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-readable-question-choose-field-lab-db3d092e based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-readable-question-choose-field-lab-3bc64cc9 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-readable-question-choose-field-mod-10392d69 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-readable-question-choose-field-nam-6c19a7fb based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-readable-question-choose-field-on-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-readable-question-choose-field-on--96b66078 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-readable-question-choose-field-pro-33f8bcaa based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-readable-question-choose-field-wor-aaf0f269 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-readable-question-choose-field-options-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-readable-question-choose-field-tex-acd2ab0c based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-readable-question-choose-v1 based.
 02 v-batch.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-context-schema.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-item-schema.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-label-details.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-model.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-name.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-on.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-profile.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-wording-version.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-options.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-text.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-verb.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-readable-question-tag-field-batch--8427b396 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-readable-question-tag-field-contex-ea19e9b2 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-readable-question-tag-field-item-s-ec66b087 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-readable-question-tag-field-label--ffc30543 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-readable-question-tag-field-label--836bf9d6 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-readable-question-tag-field-model--8530ceed based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-readable-question-tag-field-name-p-9650048f based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-readable-question-tag-field-on-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-readable-question-tag-field-on-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-readable-question-tag-field-profil-71257209 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-readable-question-tag-field-wordin-0db82431 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-readable-question-tag-field-labels-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-readable-question-tag-field-text-p-6504724d based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-readable-question-tag-v1 based.
 02 v-batch.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-context-schema.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-item-schema.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-label-details.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-model.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-name.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-on.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-profile.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-wording-version.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-labels.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-text.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-verb.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-readable-question-score-field-batc-af41a761 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-readable-question-score-field-cont-67bd8a25 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-readable-question-score-field-item-c036d576 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-readable-question-score-field-labe-9f3587f8 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-readable-question-score-field-labe-0cdf7290 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-readable-question-score-field-mode-df81fd3f based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-readable-question-score-field-name-60736bb1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-readable-question-score-field-on-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-readable-question-score-field-on-p-1b834b0e based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-readable-question-score-field-prof-68a05c07 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-readable-question-score-field-word-a9f1f1cb based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-readable-question-score-field-levels-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-readable-question-score-field-text-85617ef4 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-readable-question-score-v1 based.
 02 v-batch.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-context-schema.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-item-schema.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-label-details.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-model.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-name.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-on.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-profile.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-wording-version.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-levels.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-text.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-verb.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-readable-question-v1 based.
 02 v-kind usage binary-long unsigned.
 02 filler pic x(4).
 02 v-data.
 03 v-decide usage pointer.
 03 v-choose redefines v-decide usage pointer.
 03 v-tag redefines v-decide usage pointer.
 03 v-score redefines v-decide usage pointer.
01 tt-n-complete-version-v1 based.
 02 v-kind usage binary-long unsigned.
01 tt-n-complete-physical-source-field-first-line-p-dce18bca based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage binary-double unsigned.
01 tt-n-complete-physical-source-field-last-line-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage binary-double unsigned.
01 tt-n-complete-physical-source-v1 based.
 02 v-file.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-first-line.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage binary-double unsigned.
 02 v-last-line.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage binary-double unsigned.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-rank-member-result-field-source-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-rank-member-result-field-threshold-10d94d5b based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-rank-member-result-v1 based.
 02 v-answer usage pointer.
 02 v-answer-id usage pointer.
 02 v-images.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-meta usage pointer.
 02 v-question usage pointer.
 02 v-schema usage pointer.
 02 v-source.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-threshold.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-value usage binary-double unsigned.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-rank-member-v1 based.
 02 v-name.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-result usage pointer.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-atomic-decide-value-field-members-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-atomic-decide-value-field-members--f92e7ad6 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-atomic-decide-value-field-question-63df7df5 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-atomic-decide-value-field-source-p-8dec94a1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-threshold-v1 based.
 02 v-kind usage binary-long unsigned.
 02 filler pic x(4).
 02 v-data.
 03 v-number usage pointer.
 03 v-string redefines v-number usage pointer.
01 tt-n-complete-atomic-decide-value-field-threshol-08aafb7e based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-decide-value-v1 based.
 02 v-value usage pointer.
01 tt-n-complete-atomic-decide-value-field-value-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-atomic-decide-value-v1 based.
 02 v-answer usage pointer.
 02 v-answer-id usage pointer.
 02 v-images.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-index.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage binary-double unsigned.
 02 v-input.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-members.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-meta usage pointer.
 02 v-question usage pointer.
 02 v-question-name.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-schema usage pointer.
 02 v-source.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-threshold.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-value.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-session-packet-decide-row-v1 based.
 02 v-function.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-kind.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-value usage pointer.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-atomic-nullable-string-field-images-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-atomic-nullable-string-field-image-ffbcd52e based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-atomic-nullable-string-field-index-b9bf4ecd based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage binary-double unsigned.
01 tt-n-complete-atomic-nullable-string-field-input-332f1b09 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-atomic-nullable-string-field-members-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-atomic-nullable-string-field-membe-4272f6cb based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-atomic-nullable-string-field-quest-e214c29d based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-atomic-nullable-string-field-sourc-42bf083b based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-atomic-nullable-string-field-thres-c7e5e91f based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-atomic-nullable-string-field-value-d13f06ad based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-atomic-nullable-string-v1 based.
 02 v-answer usage pointer.
 02 v-answer-id usage pointer.
 02 v-images.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-index.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage binary-double unsigned.
 02 v-input.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-members.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-meta usage pointer.
 02 v-question usage pointer.
 02 v-question-name.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-schema usage pointer.
 02 v-source.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-threshold.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-value.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-session-packet-choose-row-v1 based.
 02 v-function.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-kind.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-value usage pointer.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-atomic-array-of-string-field-images-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-atomic-array-of-string-field-image-cc97bb10 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-atomic-array-of-string-field-index-c160c085 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage binary-double unsigned.
01 tt-n-complete-atomic-array-of-string-field-input-8bf9bda2 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-atomic-array-of-string-field-members-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-atomic-array-of-string-field-membe-33ce8448 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-atomic-array-of-string-field-quest-e0e3f4c8 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-atomic-array-of-string-field-sourc-ff23d29d based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-atomic-array-of-string-field-thres-9bdf2ee7 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-atomic-array-of-string-field-value-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-atomic-array-of-string-v1 based.
 02 v-answer usage pointer.
 02 v-answer-id usage pointer.
 02 v-images.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-index.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage binary-double unsigned.
 02 v-input.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-members.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-meta usage pointer.
 02 v-question usage pointer.
 02 v-question-name.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-schema usage pointer.
 02 v-source.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-threshold.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-session-packet-tag-row-v1 based.
 02 v-function.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-kind.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-value usage pointer.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-atomic-double-field-images-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-atomic-double-field-images-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-atomic-double-field-index-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage binary-double unsigned.
01 tt-n-complete-atomic-double-field-input-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-atomic-double-field-members-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-atomic-double-field-members-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-atomic-double-field-question-name--274033d4 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-atomic-double-field-source-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-atomic-double-field-threshold-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-atomic-double-v1 based.
 02 v-answer usage pointer.
 02 v-answer-id usage pointer.
 02 v-images.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-index.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage binary-double unsigned.
 02 v-input.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-members.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-meta usage pointer.
 02 v-question usage pointer.
 02 v-question-name.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-schema usage pointer.
 02 v-source.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-threshold.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-value usage float-long.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-session-packet-score-row-v1 based.
 02 v-function.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-kind.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-value usage pointer.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-atomic-boolean-field-images-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-atomic-boolean-field-images-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-atomic-boolean-field-index-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage binary-double unsigned.
01 tt-n-complete-atomic-boolean-field-input-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-atomic-boolean-field-members-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-atomic-boolean-field-members-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-atomic-boolean-field-question-name-e94df695 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-atomic-boolean-field-source-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-atomic-boolean-field-threshold-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-atomic-boolean-v1 based.
 02 v-answer usage pointer.
 02 v-answer-id usage pointer.
 02 v-images.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-index.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage binary-double unsigned.
 02 v-input.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-members.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-meta usage pointer.
 02 v-question usage pointer.
 02 v-question-name.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-schema usage pointer.
 02 v-source.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-threshold.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-value usage binary-long unsigned.
 02 filler pic x(4).
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-session-packet-filter-row-v1 based.
 02 v-function.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-kind.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-value usage pointer.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-annotation-member-answer-id-field--5f11b91c based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-annotation-member-answer-id-field--7966cb0f based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-annotation-member-answer-id-field--eda96048 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-annotation-member-answer-id-field--deded47c based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-value-array-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-value-v1 based.
 02 v-kind usage binary-long unsigned.
 02 filler pic x(4).
 02 v-data.
 03 v-boolean usage pointer.
 03 v-null redefines v-boolean usage pointer.
 03 v-string redefines v-boolean usage pointer.
 03 v-array redefines v-boolean usage pointer.
 03 v-number redefines v-boolean usage pointer.
01 tt-n-complete-annotation-member-answer-id-field--7081d607 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-annotation-member-answer-id-v1 based.
 02 v-answer usage pointer.
 02 v-answer-id usage pointer.
 02 v-observations.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-question usage pointer.
 02 v-question-sources.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-request.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-threshold.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-usage.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-value.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-failure-cause-v1 based.
 02 v-kind usage binary-long unsigned.
01 tt-n-complete-failure-field-kind-v1 based.
 02 v-kind usage binary-long unsigned.
01 tt-n-complete-failure-v1 based.
 02 v-cause usage pointer.
 02 v-kind usage pointer.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-annotation-member-failure-id-field-f869715c based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-annotation-member-failure-id-field-8bd4460f based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-annotation-member-failure-id-field-bbdcc43f based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-annotation-member-failure-id-field-ed16ad92 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-annotation-member-failure-id-v1 based.
 02 v-failure usage pointer.
 02 v-failure-id usage pointer.
 02 v-observations.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-question usage pointer.
 02 v-question-sources.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-request.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-threshold.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-usage.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-annotation-member-v1 based.
 02 v-kind usage binary-long unsigned.
 02 filler pic x(4).
 02 v-data.
 03 v-answer-id usage pointer.
 03 v-failure-id redefines v-answer-id usage pointer.
01 tt-n-complete-annotation-field-answers-entry-v1 based.
 02 v-name.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-value usage pointer.
01 tt-n-complete-annotation-field-answers-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-annotation-field-file-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-annotation-field-first-line-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage binary-double unsigned.
01 tt-n-complete-annotation-field-index-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage binary-double unsigned.
01 tt-n-complete-annotation-field-input-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-annotation-field-last-line-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage binary-double unsigned.
01 tt-n-complete-position-field-file-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-position-field-first-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage binary-double unsigned.
01 tt-n-complete-position-field-images-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-position-field-images-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-position-field-last-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage binary-double unsigned.
01 tt-n-complete-position-v1 based.
 02 v-file.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-first.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage binary-double unsigned.
 02 v-images.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-last.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage binary-double unsigned.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-annotation-field-position-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-annotation-field-source-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-annotated-field-array-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-failed-v1 based.
 02 v-failed usage pointer.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-annotated-field-v1 based.
 02 v-kind usage binary-long unsigned.
 02 filler pic x(4).
 02 v-data.
 03 v-boolean usage pointer.
 03 v-null redefines v-boolean usage pointer.
 03 v-string redefines v-boolean usage pointer.
 03 v-array redefines v-boolean usage pointer.
 03 v-number redefines v-boolean usage pointer.
 03 v-object redefines v-boolean usage pointer.
01 tt-n-complete-annotated-row-value-entry-v1 based.
 02 v-name.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-value usage pointer.
01 tt-n-complete-annotated-row-value-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-annotated-row-v1 based.
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-annotation-v1 based.
 02 v-answer-id usage pointer.
 02 v-answers.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-file.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-first-line.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage binary-double unsigned.
 02 v-index.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage binary-double unsigned.
 02 v-input.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-last-line.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage binary-double unsigned.
 02 v-meta usage pointer.
 02 v-position.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-schema usage pointer.
 02 v-source.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-value usage pointer.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-session-packet-annotate-row-v1 based.
 02 v-function.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-kind.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-value usage pointer.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-session-packet-decide-aggregate-fi-8fc6351f based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-session-packet-decide-aggregate-v1 based.
 02 v-function.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-kind.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-session-packet-choose-aggregate-fi-23f7ec70 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-session-packet-choose-aggregate-v1 based.
 02 v-function.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-kind.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-session-packet-tag-aggregate-field-value-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-session-packet-tag-aggregate-v1 based.
 02 v-function.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-kind.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-session-packet-score-aggregate-fie-24c5502e based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-session-packet-score-aggregate-v1 based.
 02 v-function.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-kind.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-session-packet-filter-aggregate-fi-b7d47aef based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-session-packet-filter-aggregate-v1 based.
 02 v-function.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-kind.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-atomic-non-zero-usize-field-images-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-atomic-non-zero-usize-field-images-9c9c80f3 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-atomic-non-zero-usize-field-index--54a3e969 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage binary-double unsigned.
01 tt-n-complete-atomic-non-zero-usize-field-input--1fa071d7 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-atomic-non-zero-usize-field-members-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-atomic-non-zero-usize-field-member-4bb17b44 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-atomic-non-zero-usize-field-questi-1e484acc based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-atomic-non-zero-usize-field-source-615a667b based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-atomic-non-zero-usize-field-thresh-cf910435 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-atomic-non-zero-usize-v1 based.
 02 v-answer usage pointer.
 02 v-answer-id usage pointer.
 02 v-images.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-index.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage binary-double unsigned.
 02 v-input.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-members.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-meta usage pointer.
 02 v-question usage pointer.
 02 v-question-name.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-schema usage pointer.
 02 v-source.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-threshold.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-value usage binary-double unsigned.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-session-packet-rank-aggregate-fiel-c877d197 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-session-packet-rank-aggregate-v1 based.
 02 v-function.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-kind.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-find-answer-field-confidence-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage float-long.
01 tt-n-complete-find-answer-field-probabilities-entry-v1 based.
 02 v-name.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-value usage float-long.
01 tt-n-complete-find-answer-field-probabilities-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-find-answer-v1 based.
 02 v-confidence.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage float-long.
 02 v-kind.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-pick.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-probabilities.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-find-candidate-field-index-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage binary-double unsigned.
01 tt-n-complete-find-candidate-field-input-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-find-candidate-field-source-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-find-candidate-v1 based.
 02 v-index.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage binary-double unsigned.
 02 v-input.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-probability usage float-long.
 02 v-source.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-find-field-candidates-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-find-field-candidates-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-find-field-file-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-find-field-first-line-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage binary-double unsigned.
01 tt-n-complete-find-field-index-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage binary-double unsigned.
01 tt-n-complete-find-field-last-line-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage binary-double unsigned.
01 tt-n-complete-find-field-position-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-readable-question-2-field-batch-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-readable-question-2-field-context--4a18f076 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-readable-question-2-field-item-sch-b05ad68d based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-readable-question-2-field-label-details-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-readable-question-2-field-label-de-9bd59e4a based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-readable-question-2-field-model-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-readable-question-2-field-name-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-readable-question-2-field-on-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-readable-question-2-field-on-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-readable-question-2-field-profile--14b8c376 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-readable-question-2-field-text-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-readable-question-2-field-wording--4d9c393a based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-readable-question-2-v1 based.
 02 v-batch.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-context-schema.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-item-schema.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-label-details.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-model.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-name.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-none usage binary-long unsigned.
 02 filler pic x(4).
 02 v-on.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-profile.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-text.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-verb.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-wording-version.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-find-field-threshold-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-find-field-value-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-find-v1 based.
 02 v-answer usage pointer.
 02 v-answer-id usage pointer.
 02 v-candidates.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-file.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-first-line.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage binary-double unsigned.
 02 v-index.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage binary-double unsigned.
 02 v-last-line.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage binary-double unsigned.
 02 v-meta usage pointer.
 02 v-position.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-question usage pointer.
 02 v-schema usage pointer.
 02 v-threshold.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-value.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-session-packet-find-aggregate-v1 based.
 02 v-function.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-kind.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-value usage pointer.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-session-packet-annotate-aggregate--6ec13838 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-session-packet-annotate-aggregate-v1 based.
 02 v-function.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-kind.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-name-odds-field-edges-entry-v1 based.
 02 v-name.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-value usage float-long.
01 tt-n-complete-name-odds-field-edges-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-name-odds-field-edges-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-name-odds-field-kinds-entry-v1 based.
 02 v-name.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-value usage float-long.
01 tt-n-complete-name-odds-field-kinds-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-name-odds-field-kinds-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-name-odds-v1 based.
 02 v-edges.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-end usage binary-double unsigned.
 02 v-kinds.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-start usage binary-double unsigned.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-recognition-odds-fields-names-pair-7cded0c1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-place-v1 based.
 02 v-end usage binary-double unsigned.
 02 v-start usage binary-double unsigned.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-pair-odds-v1 based.
 02 v-probability usage float-long.
 02 v-relation.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-source usage pointer.
 02 v-target usage pointer.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-recognition-odds-fields-names-pair-2ad450f0 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-piece-odds-field-tags-entry-v1 based.
 02 v-name.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-value usage float-long.
01 tt-n-complete-piece-odds-field-tags-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-piece-odds-v1 based.
 02 v-end usage binary-double unsigned.
 02 v-start usage binary-double unsigned.
 02 v-tags.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-recognition-odds-fields-names-pair-a334d002 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-recognition-proposal-field-kind-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-recognition-proposal-field-selecte-17e0d0f3 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-recognition-proposal-field-strengt-1ba5f90f based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage float-long.
01 tt-n-complete-recognition-proposal-v1 based.
 02 v-end usage binary-double unsigned.
 02 v-kept usage binary-long unsigned.
 02 filler pic x(4).
 02 v-kind.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-selected.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-span-probability usage float-long.
 02 v-start usage binary-double unsigned.
 02 v-strength.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage float-long.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-recognition-odds-fields-names-pair-160df007 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-recognition-odds-fields-names-pair-16621ffe based.
 02 v-names.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-pairs.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-pieces.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-proposals.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-recognition-odds-fields-pieces-pro-f0e824e2 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-boundary-proposal-v1 based.
 02 v-end usage binary-double unsigned.
 02 v-length usage binary-double unsigned.
 02 v-probability usage float-long.
 02 v-start usage binary-double unsigned.
 02 v-text.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-recognition-odds-fields-pieces-pro-d486e574 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-recognition-odds-fields-pieces-proposals-v1 based.
 02 v-pieces.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-proposals.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-recognition-odds-v1 based.
 02 v-kind usage binary-long unsigned.
 02 filler pic x(4).
 02 v-data.
 03 v-fields-names-pairs-pieces-proposals usage pointer.
 03 v-fields-pieces-proposals redefines v-fields-names-pairs-pieces-proposals usage pointer.
01 tt-n-complete-recognition-field-file-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-recognition-field-first-line-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage binary-double unsigned.
01 tt-n-complete-recognition-field-index-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage binary-double unsigned.
01 tt-n-complete-recognition-field-input-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-recognition-field-last-line-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage binary-double unsigned.
01 tt-n-complete-recognition-field-position-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-readable-question-3-field-batch-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-readable-question-3-field-context--bb33e4b1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-readable-question-3-field-entity-d-ace48894 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-readable-question-3-field-instruct-8f04842c based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-readable-question-3-field-item-sch-f7ef85b0 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-readable-question-3-field-kinds-entry-v1 based.
 02 v-name.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-value usage pointer.
01 tt-n-complete-readable-question-3-field-kinds-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-readable-question-3-field-label-details-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-readable-question-3-field-label-de-db3bb346 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-recognition-mode-v1 based.
 02 v-kind usage binary-long unsigned.
01 tt-n-complete-readable-question-3-field-mode-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-readable-question-3-field-model-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-readable-question-3-field-name-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-readable-question-3-field-on-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-readable-question-3-field-on-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-readable-question-3-field-profile--53340e5b based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-readable-question-3-field-relation-4a8ce819 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-relation-rule-field-single-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 v-value usage binary-long unsigned.
01 tt-n-complete-relation-rule-v1 based.
 02 v-either usage binary-long unsigned.
 02 filler pic x(4).
 02 v-name.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-reads.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-single.
 03 v-presence usage binary-long unsigned.
 03 v-value usage binary-long unsigned.
 02 v-source.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-target.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-readable-question-3-field-relations-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-readable-question-3-field-relation-ec422e28 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-readable-question-3-field-snippet--64eb313a based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage binary-double unsigned.
01 tt-n-complete-recognition-stage-context-field-bo-bf46ac6b based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-recognition-stage-context-field-ki-f11f80a2 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-recognition-stage-context-field-re-56d24b55 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-recognition-stage-context-v1 based.
 02 v-boundary.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-kind-edge.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-relation.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-readable-question-3-field-stage-co-f03fdd06 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-verb-v1 based.
 02 v-kind usage binary-long unsigned.
01 tt-n-complete-readable-question-3-field-wording--778faa61 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-readable-question-3-v1 based.
 02 v-batch.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-context-schema.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-entity-definition.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-instructions.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-item-schema.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-kinds.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-label-details.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-mode.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-model.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-name.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-on.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-profile.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-relation-threshold.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-relations.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-snippet-pieces.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage binary-double unsigned.
 02 v-stage-context.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-threshold usage pointer.
 02 v-verb usage pointer.
 02 v-wording-version.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-recognition-field-source-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-entity-field-file-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-entity-field-first-line-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage binary-double unsigned.
01 tt-n-complete-entity-field-last-line-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage binary-double unsigned.
01 tt-n-complete-entity-v1 based.
 02 v-end usage binary-double unsigned.
 02 v-file.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-first-line.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage binary-double unsigned.
 02 v-kind.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-last-line.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage binary-double unsigned.
 02 v-length usage binary-double unsigned.
 02 v-start usage binary-double unsigned.
 02 v-strength usage float-long.
 02 v-text.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-recognize-fields-entities-field-entities-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-entity-edge-field-either-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 v-value usage binary-long unsigned.
01 tt-n-complete-entity-edge-v1 based.
 02 v-either.
 03 v-presence usage binary-long unsigned.
 03 v-value usage binary-long unsigned.
 02 v-probability usage float-long.
 02 v-relation.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-source usage pointer.
 02 v-target usage pointer.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-recognize-fields-entities-field-re-d56c5743 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-recognize-fields-entities-field-re-4ee9b87c based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-recognize-fields-entities-v1 based.
 02 v-entities.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-relations.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-boundary-mode-v1 based.
 02 v-kind usage binary-long unsigned.
01 tt-n-complete-recognize-fields-mode-proposals-fi-4f01bea9 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-recognize-fields-mode-proposals-v1 based.
 02 v-mode usage pointer.
 02 v-proposals.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-recognize-v1 based.
 02 v-kind usage binary-long unsigned.
 02 filler pic x(4).
 02 v-data.
 03 v-fields-entities usage pointer.
 03 v-fields-mode-proposals redefines v-fields-entities usage pointer.
01 tt-n-complete-recognition-v1 based.
 02 v-answer usage pointer.
 02 v-answer-id usage pointer.
 02 v-file.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-first-line.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage binary-double unsigned.
 02 v-index.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage binary-double unsigned.
 02 v-input.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-last-line.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage binary-double unsigned.
 02 v-meta usage pointer.
 02 v-position.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-question usage pointer.
 02 v-schema usage pointer.
 02 v-source.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-value usage pointer.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-session-packet-recognize-aggregate-ceab54f9 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-session-packet-recognize-aggregate-v1 based.
 02 v-function.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-kind.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-relation-direction-v1 based.
 02 v-kind usage binary-long unsigned.
01 tt-n-complete-relation-method-v1 based.
 02 v-kind usage binary-long unsigned.
01 tt-n-complete-relation-member-answer-id-field-ob-fea0b065 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-relation-member-answer-id-field-qu-5a9cc189 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-related-entity-v1 based.
 02 v-kind.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-name.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-relation-member-answer-id-field-ta-c53e40d2 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-relation-member-answer-id-field-us-f6ba80ab based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-relation-member-answer-id-v1 based.
 02 v-direction usage pointer.
 02 v-method usage pointer.
 02 v-observations.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-question usage pointer.
 02 v-question-sources.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-reads.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-relation.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-request.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-source usage pointer.
 02 v-target.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-threshold usage pointer.
 02 v-usage.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-accepted usage binary-long unsigned.
 02 filler pic x(4).
 02 v-answer usage pointer.
 02 v-answer-id usage pointer.
 02 v-probability usage float-long.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-relation-member-failure-id-field-o-e0e28fb7 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-relation-member-failure-id-field-q-1c1a0bb1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-relation-member-failure-id-field-t-e42ec2b4 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-relation-member-failure-id-field-u-9db7c5db based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-relation-member-failure-id-v1 based.
 02 v-direction usage pointer.
 02 v-method usage pointer.
 02 v-observations.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-question usage pointer.
 02 v-question-sources.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-reads.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-relation.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-request.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-source usage pointer.
 02 v-target.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-threshold usage pointer.
 02 v-usage.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-failure usage pointer.
 02 v-failure-id usage pointer.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-relation-member-v1 based.
 02 v-kind usage binary-long unsigned.
 02 filler pic x(4).
 02 v-data.
 03 v-answer-id usage pointer.
 03 v-failure-id redefines v-answer-id usage pointer.
01 tt-n-complete-answers-field-questions-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-answers-v1 based.
 02 v-questions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-relation-field-file-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-relation-field-first-line-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage binary-double unsigned.
01 tt-n-complete-relation-field-index-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage binary-double unsigned.
01 tt-n-complete-relation-field-input-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-session-input-source-v1 based.
 02 v-index usage binary-double unsigned.
 02 v-source usage pointer.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-relation-field-input-sources-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-relation-field-input-sources-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-relation-field-last-line-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage binary-double unsigned.
01 tt-n-complete-relation-field-position-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-readable-question-4-field-batch-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-readable-question-4-field-context--ccb951bf based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-relate-fields-v1 based.
 02 v-kind.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-name.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-readable-question-4-field-fields-p-b47ce311 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-readable-question-4-field-item-sch-3d8a030b based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-readable-question-4-field-label-details-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-readable-question-4-field-label-de-2d046205 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-readable-question-4-field-model-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-readable-question-4-field-name-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-readable-question-4-field-on-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-readable-question-4-field-on-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-readable-question-4-field-profile--0c523516 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-readable-question-4-field-relations-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-readable-question-4-field-wording--0a2ad9ea based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-readable-question-4-v1 based.
 02 v-batch.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-context-schema.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-fields.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-item-schema.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-label-details.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-model.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-name.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-on.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-profile.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-relations.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-threshold usage pointer.
 02 v-verb.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-wording-version.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-related-entity-edge-field-either-p-4c12efed based.
 02 v-presence usage binary-long unsigned.
 02 v-value usage binary-long unsigned.
01 tt-n-complete-related-entity-edge-properties-sou-8e83a6b4 based.
 02 v-kind.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-name.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-related-entity-edge-properties-sou-a4572a79 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-related-entity-edge-properties-sou-01d78eda based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage binary-double unsigned.
01 tt-n-complete-related-entity-edge-properties-sou-bee5a90a based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage binary-double unsigned.
01 tt-n-complete-related-entity-edge-properties-sou-6f582a74 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-related-entity-edge-properties-sou-96b02c9c based.
 02 v-file.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-first-line.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage binary-double unsigned.
 02 v-kind.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-last-line.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage binary-double unsigned.
 02 v-name.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-ordinal usage binary-double unsigned.
 02 v-record.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-related-entity-edge-properties-source-v1 based.
 02 v-kind usage binary-long unsigned.
 02 filler pic x(4).
 02 v-data.
 03 v-fields-kind-name usage pointer.
 03 v-fields-file-kind-name-ordinal-record redefines v-fields-kind-name usage pointer.
01 tt-n-complete-related-entity-edge-v1 based.
 02 v-either.
 03 v-presence usage binary-long unsigned.
 03 v-value usage binary-long unsigned.
 02 v-probability usage float-long.
 02 v-relation.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-source usage pointer.
 02 v-target usage pointer.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-relation-field-value-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-relation-v1 based.
 02 v-answer usage pointer.
 02 v-answer-id usage pointer.
 02 v-file.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-first-line.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage binary-double unsigned.
 02 v-index.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage binary-double unsigned.
 02 v-input.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-input-sources.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-last-line.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage binary-double unsigned.
 02 v-meta usage pointer.
 02 v-position.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-question usage pointer.
 02 v-schema usage pointer.
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-session-packet-relate-aggregate-v1 based.
 02 v-function.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-kind.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-value usage pointer.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-request-function-v1 based.
 02 v-kind usage binary-long unsigned.
01 tt-n-complete-session-question-detail-field-answ-8f69faa2 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-session-question-detail-field-conf-fedb9951 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage float-long.
01 tt-n-complete-session-question-detail-field-fail-7f3aec61 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-session-question-detail-field-fail-54195448 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-session-question-detail-field-inpu-c50e32d5 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-session-question-detail-field-inpu-abbe3e25 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-session-question-detail-field-inpu-01984f3c based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-session-question-detail-field-inputs-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-session-question-detail-field-obse-93629d71 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-session-probabilities-yes-no-v1 based.
 02 v-kind.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-value usage float-long.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-session-named-probability-v1 based.
 02 v-name.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-probability usage float-long.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-session-probabilities-named-field-value-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-session-probabilities-named-v1 based.
 02 v-kind.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-session-probabilities-v1 based.
 02 v-kind usage binary-long unsigned.
 02 filler pic x(4).
 02 v-data.
 03 v-yes-no usage pointer.
 03 v-named redefines v-yes-no usage pointer.
01 tt-n-complete-session-question-detail-field-prob-e9e64b42 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-session-question-detail-field-ques-34547603 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-session-question-detail-field-raw--3cf8d548 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-session-question-detail-field-repo-c61f7f3e based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-session-question-detail-field-requests-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-session-question-detail-field-thre-c2b51608 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-token-usage-v1 based.
 02 v-input-tokens usage binary-double unsigned.
 02 v-output-tokens usage binary-double unsigned.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-session-question-detail-field-usag-a0ffba5d based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-session-question-detail-field-valu-5cc39287 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-session-question-detail-v1 based.
 02 v-answer-id.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-cached usage binary-long unsigned.
 02 filler pic x(4).
 02 v-confidence.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage float-long.
 02 v-failed-questions usage binary-double unsigned.
 02 v-failure.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-failure-id.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-input.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-input-source.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-input-sources.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-inputs.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-model.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-observations.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-probabilities.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-question usage pointer.
 02 v-question-sha256.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-question-sources.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-raw-pick.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-reported-usage.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-requests.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-requests-sent usage binary-double unsigned.
 02 v-threshold.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-url.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-usage.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-value.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-session-observation-question-field-67df6d42 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-session-observation-question-field-c611ae85 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-session-observation-question-v1 based.
 02 v-detail usage pointer.
 02 v-index usage binary-double unsigned.
 02 v-kind.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-member.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-position usage binary-double unsigned.
 02 v-stage.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-session-judgment-decision-field-va-fbb9cd9b based.
 02 v-presence usage binary-long unsigned.
 02 v-value usage binary-long unsigned.
01 tt-n-complete-session-judgment-decision-v1 based.
 02 v-kind.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-value.
 03 v-presence usage binary-long unsigned.
 03 v-value usage binary-long unsigned.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-session-judgment-choice-field-valu-73875c65 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-session-judgment-choice-v1 based.
 02 v-kind.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-value.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-session-judgment-score-v1 based.
 02 v-kind.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-value usage float-long.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-session-judgment-tags-field-value-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-session-judgment-tags-v1 based.
 02 v-kind.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-session-judgment-v1 based.
 02 v-kind usage binary-long unsigned.
 02 filler pic x(4).
 02 v-data.
 03 v-decision usage pointer.
 03 v-choice redefines v-decision usage pointer.
 03 v-score redefines v-decision usage pointer.
 03 v-tags redefines v-decision usage pointer.
01 tt-n-complete-session-observed-row-judgment-v1 based.
 02 v-kind.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-value usage pointer.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-annotation-value-decision-field-va-93bbb879 based.
 02 v-presence usage binary-long unsigned.
 02 v-value usage binary-long unsigned.
01 tt-n-complete-annotation-value-decision-v1 based.
 02 v-kind.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-value.
 03 v-presence usage binary-long unsigned.
 03 v-value usage binary-long unsigned.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-annotation-value-choice-field-valu-310865ea based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-annotation-value-choice-v1 based.
 02 v-kind.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-value.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-annotation-value-score-v1 based.
 02 v-kind.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-value usage float-long.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-annotation-value-tags-field-value-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-annotation-value-tags-v1 based.
 02 v-kind.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-annotation-value-failed-v1 based.
 02 v-kind.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-value usage pointer.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-annotation-value-v1 based.
 02 v-kind usage binary-long unsigned.
 02 filler pic x(4).
 02 v-data.
 03 v-decision usage pointer.
 03 v-choice redefines v-decision usage pointer.
 03 v-score redefines v-decision usage pointer.
 03 v-tags redefines v-decision usage pointer.
 03 v-failed redefines v-decision usage pointer.
01 tt-n-complete-session-annotation-v1 based.
 02 v-name.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-value usage pointer.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-session-observed-row-annotated-fie-e5b78708 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-session-observed-row-annotated-v1 based.
 02 v-kind.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-session-recognition-field-entities-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-session-recognition-field-proposals-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-session-recognition-field-proposal-8c6ed9cc based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-recognition-edge-document-v1 based.
 02 v-either usage binary-long unsigned.
 02 filler pic x(4).
 02 v-probability usage float-long.
 02 v-relation.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-source usage pointer.
 02 v-target usage pointer.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-session-recognition-field-relations-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-session-recognition-field-relation-81109d4e based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-session-recognition-v1 based.
 02 v-entities.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-mode usage pointer.
 02 v-proposals.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-relations.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-session-observed-row-recognized-v1 based.
 02 v-kind.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-value usage pointer.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-session-observed-row-find-field-va-40099317 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage binary-double unsigned.
01 tt-n-complete-session-observed-row-find-v1 based.
 02 v-kind.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-value.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage binary-double unsigned.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-entity-document-v1 based.
 02 v-kind.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-name.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-session-relation-edge-v1 based.
 02 v-either usage binary-long unsigned.
 02 filler pic x(4).
 02 v-probability usage float-long.
 02 v-relation.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-source usage pointer.
 02 v-target usage pointer.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-session-observed-row-relations-fie-db6b7be0 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-session-observed-row-relations-v1 based.
 02 v-kind.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-session-observed-row-v1 based.
 02 v-kind usage binary-long unsigned.
 02 filler pic x(4).
 02 v-data.
 03 v-judgment usage pointer.
 03 v-annotated redefines v-judgment usage pointer.
 03 v-recognized redefines v-judgment usage pointer.
 03 v-find redefines v-judgment usage pointer.
 03 v-relations redefines v-judgment usage pointer.
01 tt-n-complete-session-observation-row-v1 based.
 02 v-index usage binary-double unsigned.
 02 v-kind.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-value usage pointer.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-session-observation-v1 based.
 02 v-kind usage binary-long unsigned.
 02 filler pic x(4).
 02 v-data.
 03 v-question usage pointer.
 03 v-row redefines v-question usage pointer.
01 tt-n-complete-session-packet-observation-v1 based.
 02 v-function usage pointer.
 02 v-kind.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-value usage pointer.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-facts-field-attempts-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-complete-facts-field-attempts-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-call-id-v1 based.
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-facts-field-estimated-cost-usd-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-facts-field-held-model-mismatch-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 v-value usage binary-long unsigned.
01 tt-n-complete-facts-field-input-tokens-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage binary-double unsigned.
01 tt-n-complete-facts-field-largest-request-estima-50ed75bc based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage binary-double unsigned.
01 tt-n-complete-facts-field-model-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-facts-field-output-tokens-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage binary-double unsigned.
01 tt-n-complete-persistence-observation-field-advi-959f547f based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-usage-persistence-v1 based.
 02 v-kind usage binary-long unsigned.
01 tt-n-complete-persistence-observation-v1 based.
 02 v-advice.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-observed-at.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-state usage pointer.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-facts-field-usage-persistence-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-facts-v1 based.
 02 v-attempts.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-cache-answers usage binary-double unsigned.
 02 v-call-id usage pointer.
 02 v-estimated-cost-usd.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-held-model-mismatch.
 03 v-presence usage binary-long unsigned.
 03 v-value usage binary-long unsigned.
 02 v-input-tokens.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage binary-double unsigned.
 02 v-largest-request-bytes usage binary-double unsigned.
 02 v-largest-request-estimated-input-tokens.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage binary-double unsigned.
 02 v-model.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value.
 04 v-data usage pointer.
 04 v-len usage binary-double unsigned.
 02 v-output-tokens.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage binary-double unsigned.
 02 v-records usage binary-double unsigned.
 02 v-requests-sent usage binary-double unsigned.
 02 v-seconds usage float-long.
 02 v-token-estimate-method.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-usage-persistence.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-session-packet-terminal-field-fact-eafb4767 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-estimated-input-denial-initial-request-v1 based.
 02 v-kind.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-limit usage binary-double unsigned.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-estimated-input-denial-additional--4e0ac8d3 based.
 02 v-kind.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-limit usage binary-double unsigned.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-estimated-input-denial-retry-v1 based.
 02 v-kind.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-last-status usage binary-double unsigned.
 02 v-limit usage binary-double unsigned.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-estimated-input-denial-v1 based.
 02 v-kind usage binary-long unsigned.
 02 filler pic x(4).
 02 v-data.
 03 v-initial-request usage pointer.
 03 v-additional-request redefines v-initial-request usage pointer.
 03 v-retry redefines v-initial-request usage pointer.
01 tt-n-complete-error-field-estimated-input-denial-37744ba7 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-failure-kind-v1 based.
 02 v-kind usage binary-long unsigned.
01 tt-n-complete-send-budget-denial-before-first-send-v1 based.
 02 v-kind.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-send-budget-denial-before-addition-6de30b29 based.
 02 v-kind.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-send-budget-denial-before-retry-v1 based.
 02 v-kind.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-last-status usage binary-double unsigned.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-send-budget-denial-v1 based.
 02 v-kind usage binary-long unsigned.
 02 filler pic x(4).
 02 v-data.
 03 v-before-first-send usage pointer.
 03 v-before-additional-send redefines v-before-first-send usage pointer.
 03 v-before-retry redefines v-before-first-send usage pointer.
01 tt-n-complete-error-field-send-budget-denial-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-stopped-field-at-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage binary-double unsigned.
01 tt-n-complete-stop-cause-v1 based.
 02 v-kind usage binary-long unsigned.
01 tt-n-complete-stopped-field-status-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage binary-double unsigned.
01 tt-n-complete-stopped-v1 based.
 02 v-at.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage binary-double unsigned.
 02 v-cause usage pointer.
 02 v-retryable usage binary-long unsigned.
 02 filler pic x(4).
 02 v-status.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage binary-double unsigned.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-error-v1 based.
 02 v-estimated-input-denial.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-kind usage pointer.
 02 v-message.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-retryable usage binary-long unsigned.
 02 filler pic x(4).
 02 v-send-budget-denial.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-stopped usage pointer.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-call-error-field-facts-presence-v1 based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-call-error-v1 based.
 02 v-error usage pointer.
 02 v-facts.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-session-packet-terminal-field-fail-ac40530c based.
 02 v-presence usage binary-long unsigned.
 02 filler pic x(4).
 02 v-value usage pointer.
01 tt-n-complete-session-packet-terminal-v1 based.
 02 v-facts.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-failure.
 03 v-presence usage binary-long unsigned.
 03 filler pic x(4).
 03 v-value usage pointer.
 02 v-kind.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-extensions.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-complete-session-packet-v1 based.
 02 v-kind usage binary-long unsigned.
 02 filler pic x(4).
 02 v-data.
 03 v-decide-row usage pointer.
 03 v-choose-row redefines v-decide-row usage pointer.
 03 v-tag-row redefines v-decide-row usage pointer.
 03 v-score-row redefines v-decide-row usage pointer.
 03 v-filter-row redefines v-decide-row usage pointer.
 03 v-annotate-row redefines v-decide-row usage pointer.
 03 v-decide-aggregate redefines v-decide-row usage pointer.
 03 v-choose-aggregate redefines v-decide-row usage pointer.
 03 v-tag-aggregate redefines v-decide-row usage pointer.
 03 v-score-aggregate redefines v-decide-row usage pointer.
 03 v-filter-aggregate redefines v-decide-row usage pointer.
 03 v-rank-aggregate redefines v-decide-row usage pointer.
 03 v-find-aggregate redefines v-decide-row usage pointer.
 03 v-annotate-aggregate redefines v-decide-row usage pointer.
 03 v-recognize-aggregate redefines v-decide-row usage pointer.
 03 v-relate-aggregate redefines v-decide-row usage pointer.
 03 v-observation redefines v-decide-row usage pointer.
 03 v-terminal redefines v-decide-row usage pointer.
01 tt-n-source-spec-v1 based.
 02 v-paths.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-unit usage binary-long unsigned.
 02 filler pic x(4).
 02 v-window usage binary-double unsigned.
01 tt-n-images-v1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-record-v1 based.
 02 v-original.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-value.
 04 v-kind usage binary-long unsigned.
 04 filler pic x(4).
 04 v-data.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 02 v-context.
 03 v-present usage binary-long signed.
 03 filler pic x(4).
 03 v-value.
 04 v-kind usage binary-long unsigned.
 04 filler pic x(4).
 04 v-data.
 05 v-data usage pointer.
 05 v-len usage binary-double unsigned.
 02 v-options.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
 02 v-images.
 03 v-data usage pointer.
 03 v-len usage binary-double unsigned.
01 tt-n-cobol-text based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-Authored-choose based.
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
01 tt-n-cobol-Authored-criterion based.
 02 v-kind usage binary-double unsigned.
 02 v-value usage pointer.
01 tt-n-cobol-Authored-cut based.
 02 v-kind usage binary-double unsigned.
 02 v-value usage pointer.
01 tt-n-cobol-Authored-decide based.
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
01 tt-n-cobol-Authored-description based.
 02 v-kind usage binary-double unsigned.
 02 v-value usage pointer.
01 tt-n-cobol-Authored-find based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-context-schema usage pointer.
 02 v-m-find usage pointer.
 02 v-m-item-schema usage pointer.
 02 v-m-model usage pointer.
 02 v-m-name usage pointer.
 02 v-m-on usage pointer.
 02 v-m-profile usage pointer.
 02 v-m-wording-version usage pointer.
01 tt-n-cobol-Authored-inputDeclaration based.
 02 v-kind usage binary-double unsigned.
 02 v-value usage pointer.
01 tt-n-cobol-Authored-inputDeclaration-object based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-properties usage pointer.
 02 v-m-required usage pointer.
 02 v-m-type usage pointer.
01 tt-n-cobol-Authored-inputDeclaration-string based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-type usage pointer.
01 tt-n-cobol-Authored-inputProperty based.
 02 v-kind usage binary-double unsigned.
 02 v-value usage pointer.
01 tt-n-cobol-Authored-inputProperty-array based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-items usage pointer.
 02 v-m-type usage pointer.
01 tt-n-cobol-Authored-inputProperty-boolean based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-type usage pointer.
01 tt-n-cobol-Authored-inputProperty-number based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-type usage pointer.
01 tt-n-cobol-Authored-inputProperty-string based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-type usage pointer.
01 tt-n-cobol-Authored-labels based.
 02 v-kind usage binary-double unsigned.
 02 v-value usage pointer.
01 tt-n-cobol-Authored-levels based.
 02 v-kind usage binary-double unsigned.
 02 v-value usage pointer.
01 tt-n-cobol-Authored-name based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-Authored-options based.
 02 v-kind usage binary-double unsigned.
 02 v-value usage pointer.
01 tt-n-cobol-Authored-pointers based.
 02 v-kind usage binary-double unsigned.
 02 v-value usage pointer.
01 tt-n-cobol-Authored-profile based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-Authored-questionText based.
 02 v-kind usage binary-double unsigned.
 02 v-value usage pointer.
01 tt-n-cobol-Authored-relate based.
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
01 tt-n-cobol-Authored-relation based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-either usage pointer.
 02 v-m-name usage pointer.
 02 v-m-reads usage pointer.
 02 v-m-single usage pointer.
 02 v-m-source usage pointer.
 02 v-m-target usage pointer.
01 tt-n-cobol-Authored-score based.
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
01 tt-n-cobol-Authored-tag based.
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
01 tt-n-cobol-Authored-threshold based.
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
01 tt-n-cobol-Request based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-call usage pointer.
 02 v-m-schema usage pointer.
01 tt-n-cobol-RequestBatch based.
 02 v-kind usage binary-double unsigned.
 02 v-value usage pointer.
01 tt-n-cobol-RequestCall based.
 02 v-kind usage binary-double unsigned.
 02 v-value usage pointer.
01 tt-n-cobol-RequestCall-annotate based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-function usage pointer.
 02 v-m-input usage pointer.
 02 v-m-options usage pointer.
 02 v-m-question usage pointer.
01 tt-n-cobol-RequestCall-choose based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-function usage pointer.
 02 v-m-input usage pointer.
 02 v-m-options usage pointer.
 02 v-m-question usage pointer.
01 tt-n-cobol-RequestCall-decide based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-function usage pointer.
 02 v-m-input usage pointer.
 02 v-m-options usage pointer.
 02 v-m-question usage pointer.
01 tt-n-cobol-RequestCall-filter based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-function usage pointer.
 02 v-m-input usage pointer.
 02 v-m-options usage pointer.
 02 v-m-question usage pointer.
01 tt-n-cobol-RequestCall-find based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-function usage pointer.
 02 v-m-input usage pointer.
 02 v-m-options usage pointer.
 02 v-m-question usage pointer.
01 tt-n-cobol-RequestCall-rank based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-function usage pointer.
 02 v-m-input usage pointer.
 02 v-m-options usage pointer.
 02 v-m-question usage pointer.
01 tt-n-cobol-RequestCall-recognize based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-function usage pointer.
 02 v-m-input usage pointer.
 02 v-m-options usage pointer.
 02 v-m-question usage pointer.
01 tt-n-cobol-RequestCall-relate based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-function usage pointer.
 02 v-m-input usage pointer.
 02 v-m-options usage pointer.
 02 v-m-question usage pointer.
01 tt-n-cobol-RequestCall-score based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-function usage pointer.
 02 v-m-input usage pointer.
 02 v-m-options usage pointer.
 02 v-m-question usage pointer.
01 tt-n-cobol-RequestCall-tag based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-function usage pointer.
 02 v-m-input usage pointer.
 02 v-m-options usage pointer.
 02 v-m-question usage pointer.
01 tt-n-cobol-RequestDefinition based.
 02 v-kind usage binary-double unsigned.
 02 v-value usage pointer.
01 tt-n-cobol-RequestDefinition-anyOf-7-properties--f8b54b1b based.
 02 v-kind usage binary-double unsigned.
 02 v-value usage pointer.
01 tt-n-cobol-RequestDefinition-anyOf-7-properties--4966856c based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-choose usage pointer.
 02 v-m-context-schema usage pointer.
 02 v-m-item-schema usage pointer.
 02 v-m-name usage pointer.
 02 v-m-on usage pointer.
 02 v-m-options usage pointer.
 02 v-m-threshold usage pointer.
 02 v-m-wording-version usage pointer.
01 tt-n-cobol-RequestDefinition-anyOf-7-properties--28340b07 based.
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
01 tt-n-cobol-RequestDefinition-anyOf-7-properties--29a6b9aa based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-context-schema usage pointer.
 02 v-m-item-schema usage pointer.
 02 v-m-levels usage pointer.
 02 v-m-name usage pointer.
 02 v-m-on usage pointer.
 02 v-m-score usage pointer.
 02 v-m-wording-version usage pointer.
01 tt-n-cobol-RequestDefinition-anyOf-7-properties--d61b4ab4 based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-context-schema usage pointer.
 02 v-m-item-schema usage pointer.
 02 v-m-labels usage pointer.
 02 v-m-name usage pointer.
 02 v-m-on usage pointer.
 02 v-m-tag usage pointer.
 02 v-m-threshold usage pointer.
 02 v-m-wording-version usage pointer.
01 tt-n-cobol-RequestDefinition-fields-choose based.
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
01 tt-n-cobol-RequestDefinition-fields-decide based.
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
01 tt-n-cobol-RequestDefinition-fields-find based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-context-schema usage pointer.
 02 v-m-find usage pointer.
 02 v-m-item-schema usage pointer.
 02 v-m-model usage pointer.
 02 v-m-name usage pointer.
 02 v-m-on usage pointer.
 02 v-m-profile usage pointer.
 02 v-m-wording-version usage pointer.
01 tt-n-cobol-RequestDefinition-fields-questions-version based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-batch usage pointer.
 02 v-m-profile usage pointer.
 02 v-m-questions usage pointer.
 02 v-m-threshold usage pointer.
 02 v-m-version usage pointer.
01 tt-n-cobol-RequestDefinition-fields-recognize-version based.
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
01 tt-n-cobol-RequestDefinition-fields-relate-version based.
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
01 tt-n-cobol-RequestDefinition-fields-score based.
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
01 tt-n-cobol-RequestDefinition-fields-tag based.
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
01 tt-n-cobol-RequestFraming based.
 02 v-kind usage binary-double unsigned.
 02 v-value usage pointer.
01 tt-n-cobol-RequestImage based.
 02 v-kind usage binary-double unsigned.
 02 v-value usage pointer.
01 tt-n-cobol-RequestImage-bytes based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-bytes usage pointer.
 02 v-m-kind usage pointer.
 02 v-m-media usage pointer.
01 tt-n-cobol-RequestImage-file based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-kind usage pointer.
 02 v-m-media usage pointer.
 02 v-m-path usage pointer.
01 tt-n-cobol-RequestInput based.
 02 v-kind usage binary-double unsigned.
 02 v-value usage pointer.
01 tt-n-cobol-RequestInput-entities based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-items usage pointer.
 02 v-m-kind usage pointer.
01 tt-n-cobol-RequestInput-feed based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-framing usage pointer.
 02 v-m-images usage pointer.
 02 v-m-kind usage pointer.
 02 v-m-name usage pointer.
 02 v-m-reading usage pointer.
01 tt-n-cobol-RequestInput-json based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-images usage pointer.
 02 v-m-kind usage pointer.
 02 v-m-value usage pointer.
01 tt-n-cobol-RequestInput-records based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-items usage pointer.
 02 v-m-kind usage pointer.
01 tt-n-cobol-RequestInput-source based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-kind usage pointer.
 02 v-m-source usage pointer.
01 tt-n-cobol-RequestInput-text based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-images usage pointer.
 02 v-m-kind usage pointer.
 02 v-m-text usage pointer.
01 tt-n-cobol-RequestInput-units based.
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
01 tt-n-cobol-RequestOriginal-json based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-kind usage pointer.
 02 v-m-value usage pointer.
01 tt-n-cobol-RequestOriginal-text based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-kind usage pointer.
 02 v-m-text usage pointer.
01 tt-n-cobol-RequestQuestion based.
 02 v-kind usage binary-double unsigned.
 02 v-value usage pointer.
01 tt-n-cobol-RequestQuestion-definition based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-kind usage pointer.
 02 v-m-value usage pointer.
01 tt-n-cobol-RequestQuestion-file based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-kind usage pointer.
 02 v-m-path usage pointer.
01 tt-n-cobol-RequestQuestion-name based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-kind usage pointer.
 02 v-m-name usage pointer.
01 tt-n-cobol-RequestQuestion-reference based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-kind usage pointer.
 02 v-m-reference usage pointer.
01 tt-n-cobol-RequestQuestion-text based.
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
 02 v-m-media usage pointer.
 02 v-m-paths usage pointer.
 02 v-m-reading usage pointer.
01 tt-n-cobol-RequestThreshold based.
 02 v-kind usage binary-double unsigned.
 02 v-value usage pointer.
01 tt-n-cobol-RequestVersion based.
 02 v-kind usage binary-double unsigned.
 02 v-value usage pointer.
01 tt-n-cobol-SessionSourceLocation based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-file usage pointer.
 02 v-m-first-line usage pointer.
 02 v-m-last-line usage pointer.
01 tt-n-cobol-SourceUnit based.
 02 v-kind usage binary-double unsigned.
 02 v-value usage pointer.
01 tt-n-cobol-Authored-choose-member-batch based.
 02 v-kind usage binary-double unsigned.
 02 v-value usage pointer.
01 tt-n-cobol-Authored-choose-member-name based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-Authored-choose-member-wording-version based.
 02 v-value usage binary-double signed.
01 tt-n-cobol-Authored-criterion-arm-1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-Authored-criterion-arm-2 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-Authored-criterion-arm-3 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-Authored-criterion-arm-4 based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-Authored-cut-arm-1 based.
 02 v-value usage float-long.
01 tt-n-cobol-Authored-cut-arm-2 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-Authored-decide-member-batch based.
 02 v-kind usage binary-double unsigned.
 02 v-value usage pointer.
01 tt-n-cobol-Authored-decide-member-name based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-Authored-decide-member-wording-version based.
 02 v-value usage binary-double signed.
01 tt-n-cobol-Authored-description-arm-1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-Authored-description-arm-2 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-Authored-description-arm-3 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-Authored-description-arm-4 based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-Authored-find-member-name based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-Authored-find-member-wording-version based.
 02 v-value usage binary-double signed.
01 tt-n-cobol-Authored-inputDeclaration-object-memb-6cf70998 based.
 02 v-keys usage pointer.
 02 v-values usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-Authored-inputDeclaration-object-memb-04b342bf based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-Authored-inputDeclaration-object-member-type based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-Authored-inputDeclaration-string-member-type based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-Authored-inputProperty-array-member-items based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-type usage pointer.
01 tt-n-cobol-Authored-inputProperty-array-member-type based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-Authored-inputProperty-boolean-member-type based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-Authored-inputProperty-number-member-type based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-Authored-inputProperty-string-member-type based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-Authored-labels-arm-1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-Authored-labels-arm-2 based.
 02 v-keys usage pointer.
 02 v-values usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-Authored-levels-arm-1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-Authored-levels-arm-2 based.
 02 v-keys usage pointer.
 02 v-values usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-Authored-options-arm-1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-Authored-options-arm-2 based.
 02 v-keys usage pointer.
 02 v-values usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-Authored-pointers-arm-1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-Authored-pointers-arm-2 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-Authored-questionText-arm-1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-Authored-questionText-arm-2 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-Authored-questionText-arm-3 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-Authored-relate-member-name based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-Authored-relate-member-relate based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-fields usage pointer.
 02 v-m-relations usage pointer.
01 tt-n-cobol-Authored-relate-member-version based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-Authored-relate-member-wording-version based.
 02 v-value usage binary-double signed.
01 tt-n-cobol-Authored-relation-member-either based.
 02 v-value usage binary-double unsigned.
01 tt-n-cobol-Authored-relation-member-single based.
 02 v-value usage binary-double unsigned.
01 tt-n-cobol-Authored-score-member-batch based.
 02 v-kind usage binary-double unsigned.
 02 v-value usage pointer.
01 tt-n-cobol-Authored-score-member-name based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-Authored-score-member-wording-version based.
 02 v-value usage binary-double signed.
01 tt-n-cobol-Authored-tag-member-batch based.
 02 v-kind usage binary-double unsigned.
 02 v-value usage pointer.
01 tt-n-cobol-Authored-tag-member-name based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-Authored-tag-member-wording-version based.
 02 v-value usage binary-double signed.
01 tt-n-cobol-Authored-threshold-arm-1 based.
 02 v-value usage float-long.
01 tt-n-cobol-Authored-threshold-arm-2 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-ContextSchema-arm-1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-ContextSchema-arm-2 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-ImageMedia-arm-1 based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-ImageMedia-arm-2 based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-OptionSchema-member-description based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-OptionSchema-member-name based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-ReaderMedia-arm-1 based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-ReaderMedia-arm-2 based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RecognitionExample-arm-1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RecognitionExampleEntity-member-end based.
 02 v-value usage binary-double unsigned.
01 tt-n-cobol-RecognitionExampleEntity-member-kind based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RecognitionExampleEntity-member-start based.
 02 v-value usage binary-double unsigned.
01 tt-n-cobol-RecognitionExampleText-member-entities based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RecognitionExampleText-member-kinds based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RecognitionExampleText-member-text based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RecognitionMode-arm-1 based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RecognitionMode-arm-2 based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RecognitionSeedSpan-member-end based.
 02 v-value usage binary-double unsigned.
01 tt-n-cobol-RecognitionSeedSpan-member-kind based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RecognitionSeedSpan-member-start based.
 02 v-value usage binary-double unsigned.
01 tt-n-cobol-RecognitionStageContext-member-boundary based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RecognitionStageContext-member-kind-edge based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RecognitionStageContext-member-relation based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestBatch-arm-1 based.
 02 v-value usage binary-double unsigned.
01 tt-n-cobol-RequestBatch-arm-2 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestCall-annotate-member-function based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestCall-choose-member-function based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestCall-decide-member-function based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestCall-filter-member-function based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestCall-find-member-function based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestCall-rank-member-function based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestCall-recognize-member-function based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestCall-relate-member-function based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestCall-score-member-function based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestCall-tag-member-function based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestDefinition-anyOf-7-properties--47cb990f based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestDefinition-anyOf-7-properties--38543973 based.
 02 v-value usage binary-double signed.
01 tt-n-cobol-RequestDefinition-anyOf-7-properties--928e3246 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestDefinition-anyOf-7-properties--3b4e92dc based.
 02 v-value usage binary-double signed.
01 tt-n-cobol-RequestDefinition-anyOf-7-properties--038b85be based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestDefinition-anyOf-7-properties--b5dad632 based.
 02 v-value usage binary-double signed.
01 tt-n-cobol-RequestDefinition-anyOf-7-properties--89be50f1 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestDefinition-anyOf-7-properties--339d94b8 based.
 02 v-value usage binary-double signed.
01 tt-n-cobol-RequestDefinition-fields-choose-member-batch based.
 02 v-kind usage binary-double unsigned.
 02 v-value usage pointer.
01 tt-n-cobol-RequestDefinition-fields-choose-member-name based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestDefinition-fields-choose-membe-fd789a2b based.
 02 v-value usage binary-double signed.
01 tt-n-cobol-RequestDefinition-fields-decide-member-batch based.
 02 v-kind usage binary-double unsigned.
 02 v-value usage pointer.
01 tt-n-cobol-RequestDefinition-fields-decide-member-name based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestDefinition-fields-decide-membe-d85937c4 based.
 02 v-value usage binary-double signed.
01 tt-n-cobol-RequestDefinition-fields-find-member-name based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestDefinition-fields-find-member--1adc5e4d based.
 02 v-value usage binary-double signed.
01 tt-n-cobol-RequestDefinition-fields-questions-ve-0c906ac8 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestDefinition-fields-questions-ve-9ca136eb based.
 02 v-keys usage pointer.
 02 v-values usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestDefinition-fields-questions-ve-952e1517 based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestDefinition-fields-recognize-ve-4c92d405 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestDefinition-fields-recognize-ve-9b17d7c1 based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-entity-definition usage pointer.
 02 v-m-instructions usage pointer.
 02 v-m-kinds usage pointer.
 02 v-m-mode usage pointer.
 02 v-m-relations usage pointer.
 02 v-m-snippet-pieces usage pointer.
 02 v-m-stage-context usage pointer.
01 tt-n-cobol-RequestDefinition-fields-recognize-ve-c1b7f425 based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestDefinition-fields-recognize-ve-a87d0755 based.
 02 v-value usage binary-double signed.
01 tt-n-cobol-RequestDefinition-fields-relate-versi-717255f8 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestDefinition-fields-relate-versi-e07903d4 based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-fields usage pointer.
 02 v-m-relations usage pointer.
01 tt-n-cobol-RequestDefinition-fields-relate-versi-528f5945 based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestDefinition-fields-relate-versi-f91d0225 based.
 02 v-value usage binary-double signed.
01 tt-n-cobol-RequestDefinition-fields-score-member-batch based.
 02 v-kind usage binary-double unsigned.
 02 v-value usage pointer.
01 tt-n-cobol-RequestDefinition-fields-score-member-name based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestDefinition-fields-score-member-38513ba2 based.
 02 v-value usage binary-double signed.
01 tt-n-cobol-RequestDefinition-fields-tag-member-batch based.
 02 v-kind usage binary-double unsigned.
 02 v-value usage pointer.
01 tt-n-cobol-RequestDefinition-fields-tag-member-name based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestDefinition-fields-tag-member-w-1b6b7c8d based.
 02 v-value usage binary-double signed.
01 tt-n-cobol-RequestFraming-arm-1 based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestFraming-arm-2 based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestFraming-arm-3 based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestFraming-arm-4 based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestFraming-arm-5 based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestImage-bytes-member-bytes based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestImage-bytes-member-kind based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestImage-file-member-kind based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestImage-file-member-path based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestInput-entities-member-items based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestInput-entities-member-kind based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestInput-feed-member-images based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestInput-feed-member-kind based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestInput-feed-member-name based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestInput-json-member-images based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestInput-json-member-kind based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestInput-json-member-value based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestInput-records-member-items based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestInput-records-member-kind based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestInput-source-member-kind based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestInput-text-member-images based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestInput-text-member-kind based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestInput-text-member-text based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestInput-units-member-items based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestInput-units-member-kind based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestItem-member-examples based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestItem-member-images based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestItem-member-options based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestItem-member-seed-spans based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestOptions-member-attempts based.
 02 v-value usage binary-double unsigned.
01 tt-n-cobol-RequestOptions-member-context based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestOptions-member-context-field based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestOptions-member-deadline-ms based.
 02 v-value usage binary-double signed.
01 tt-n-cobol-RequestOptions-member-details based.
 02 v-value usage binary-double unsigned.
01 tt-n-cobol-RequestOptions-member-examples based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestOptions-member-examples-field based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestOptions-member-field based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestOptions-member-files-only based.
 02 v-value usage binary-double unsigned.
01 tt-n-cobol-RequestOptions-member-max-requests-total based.
 02 v-value usage binary-double unsigned.
01 tt-n-cobol-RequestOptions-member-model based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestOptions-member-none based.
 02 v-value usage binary-double unsigned.
01 tt-n-cobol-RequestOptions-member-options-field based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestOptions-member-seed-spans based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestOptions-member-seed-spans-field based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestOptions-member-snippet-pieces based.
 02 v-value usage binary-double unsigned.
01 tt-n-cobol-RequestOptions-member-top based.
 02 v-value usage binary-double unsigned.
01 tt-n-cobol-RequestOriginal-json-member-kind based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestOriginal-json-member-value based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestOriginal-text-member-kind based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestOriginal-text-member-text based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestQuestion-definition-member-kind based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestQuestion-file-member-kind based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestQuestion-file-member-path based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestQuestion-name-member-kind based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestQuestion-name-member-name based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestQuestion-reference-member-kind based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestQuestion-reference-member-reference based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestQuestion-text-member-kind based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestQuestion-text-member-text based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestReader-member-window based.
 02 v-value usage binary-double unsigned.
01 tt-n-cobol-RequestSource-member-paths based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestThreshold-arm-1 based.
 02 v-value usage float-long.
01 tt-n-cobol-RequestThreshold-arm-2 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestVersion-arm-1 based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-SessionSourceLocation-member-file based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-SessionSourceLocation-member-first-line based.
 02 v-value usage binary-double unsigned.
01 tt-n-cobol-SessionSourceLocation-member-last-line based.
 02 v-value usage binary-double unsigned.
01 tt-n-cobol-SourceUnit-arm-1 based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-SourceUnit-arm-2 based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-SourceUnit-arm-3 based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-Authored-choose-member-batch-arm-1 based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-Authored-choose-member-batch-arm-2 based.
 02 v-value usage binary-double signed.
01 tt-n-cobol-Authored-criterion-arm-3-item based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-Authored-decide-member-batch-arm-1 based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-Authored-decide-member-batch-arm-2 based.
 02 v-value usage binary-double signed.
01 tt-n-cobol-Authored-description-arm-3-item based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-Authored-inputDeclaration-object-memb-d08acaf0 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-Authored-inputProperty-array-member-i-2064227d based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-Authored-pointers-arm-2-item based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-Authored-questionText-arm-3-item based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-Authored-relate-member-relate-member-fields based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-kind usage pointer.
 02 v-m-name usage pointer.
01 tt-n-cobol-Authored-relate-member-relate-member-relations based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-Authored-score-member-batch-arm-1 based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-Authored-score-member-batch-arm-2 based.
 02 v-value usage binary-double signed.
01 tt-n-cobol-Authored-tag-member-batch-arm-1 based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-Authored-tag-member-batch-arm-2 based.
 02 v-value usage binary-double signed.
01 tt-n-cobol-RecognitionExampleText-member-kinds-item based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestDefinition-fields-choose-membe-bd798961 based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestDefinition-fields-choose-membe-98dad8c8 based.
 02 v-value usage binary-double signed.
01 tt-n-cobol-RequestDefinition-fields-decide-membe-3b2f6b4e based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestDefinition-fields-decide-membe-744fb6ed based.
 02 v-value usage binary-double signed.
01 tt-n-cobol-RequestDefinition-fields-recognize-ve-1dcf1ddc based.
 02 v-keys usage pointer.
 02 v-values usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestDefinition-fields-recognize-ve-8fe7e86e based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestDefinition-fields-recognize-ve-44998be7 based.
 02 v-value usage binary-double unsigned.
01 tt-n-cobol-RequestDefinition-fields-relate-versi-2c45451e based.
 02 v-reserved usage binary-double unsigned.
 02 v-m-kind usage pointer.
 02 v-m-name usage pointer.
01 tt-n-cobol-RequestDefinition-fields-relate-versi-68e0a829 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestDefinition-fields-score-member-6be1cb5f based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestDefinition-fields-score-member-6316884c based.
 02 v-value usage binary-double signed.
01 tt-n-cobol-RequestDefinition-fields-tag-member-b-32d27fbb based.
 02 v-reserved usage binary-double unsigned.
01 tt-n-cobol-RequestDefinition-fields-tag-member-b-4b29a182 based.
 02 v-value usage binary-double signed.
01 tt-n-cobol-RequestOptions-member-field-item based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestSource-member-paths-item based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-Authored-relate-member-relate-member--4b8c52bf based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-Authored-relate-member-relate-member--67387a66 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestDefinition-fields-relate-versi-5796c479 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
01 tt-n-cobol-RequestDefinition-fields-relate-versi-4d24e830 based.
 02 v-data usage pointer.
 02 v-len usage binary-double unsigned.
78 tt-n-text-capacity value 8192.
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
78 tt-n-complete-estimated-input-denial-additional--4e0ac8d3 value 2.
78 tt-n-complete-estimated-input-denial-initial-request-v1 value 1.
78 tt-n-complete-estimated-input-denial-retry-v1 value 3.
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
78 tt-n-complete-input-declaration-object-v1 value 2.
78 tt-n-complete-input-declaration-string-v1 value 1.
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
78 tt-n-complete-recognition-odds-fields-names-pair-16621ffe value 1.
78 tt-n-complete-recognition-odds-fields-pieces-proposals-v1 value 2.
78 tt-n-complete-recognize-fields-entities-v1 value 1.
78 tt-n-complete-recognize-fields-mode-proposals-v1 value 2.
78 tt-n-complete-related-entity-edge-properties-sou-96b02c9c value 2.
78 tt-n-complete-related-entity-edge-properties-sou-8e83a6b4 value 1.
78 tt-n-complete-relation-direction-either-v1 value 2.
78 tt-n-complete-relation-direction-source-to-target-v1 value 1.
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
78 tt-n-complete-send-budget-denial-before-addition-6de30b29 value 2.
78 tt-n-complete-send-budget-denial-before-first-send-v1 value 1.
78 tt-n-complete-send-budget-denial-before-retry-v1 value 3.
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
78 tt-n-session-result-v1 value 0.
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
