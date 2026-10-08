      *> Counted native carriers, Linux x86_64.
      *> BASED groups borrow native storage or caller ALLOCATE storage.
      *> Overlay nested named fields with their matching typed BASED group.
       01 tt-string-v1 based.
          02 v-data usage pointer.
          02 v-len usage binary-double unsigned.
       01 tt-strings-v1 based.
          02 v-data usage pointer.
          02 v-len usage binary-double unsigned.
       01 tt-optional-string-v1 based.
          02 v-present usage binary-long signed.
          02 v-padding-4 pic x(4).
          02 v-value pic x(16).
       01 tt-optional-size-v1 based.
          02 v-present usage binary-long signed.
          02 v-padding-4 pic x(4).
          02 v-value usage binary-double unsigned.
       01 tt-optional-u64-v1 based.
          02 v-present usage binary-long signed.
          02 v-padding-4 pic x(4).
          02 v-value usage binary-double unsigned.
       01 tt-optional-u16-v1 based.
          02 v-present usage binary-long signed.
          02 v-value usage binary-short unsigned.
          02 v-padding-6 pic x(2).
       01 tt-optional-double-v1 based.
          02 v-present usage binary-long signed.
          02 v-padding-4 pic x(4).
          02 v-value usage float-long.
       01 tt-optional-discriminator-v1 based.
          02 v-present usage binary-long signed.
          02 v-value usage binary-long unsigned.
       01 tt-content-v1 based.
          02 v-kind usage binary-long unsigned.
          02 v-padding-4 pic x(4).
          02 v-data pic x(16).
       01 tt-optional-content-v1 based.
          02 v-present usage binary-long signed.
          02 v-padding-4 pic x(4).
          02 v-value pic x(24).
       01 tt-rule-v1 based.
          02 v-kind usage binary-long unsigned.
          02 v-padding-4 pic x(4).
          02 v-low usage float-long.
          02 v-high usage float-long.
       01 tt-optional-rule-v1 based.
          02 v-present usage binary-long signed.
          02 v-padding-4 pic x(4).
          02 v-value pic x(24).
       01 tt-choice-v1 based.
          02 v-name pic x(16).
          02 v-description pic x(32).
          02 v-weight pic x(16).
       01 tt-choices-v1 based.
          02 v-data usage pointer.
          02 v-len usage binary-double unsigned.
       01 tt-relation-v1 based.
          02 v-name pic x(16).
          02 v-source pic x(16).
          02 v-target pic x(16).
          02 v-reads pic x(24).
          02 v-either usage binary-long signed.
          02 v-single usage binary-long signed.
       01 tt-relations-v1 based.
          02 v-data usage pointer.
          02 v-len usage binary-double unsigned.
       01 tt-member-spec-v1 based.
          02 v-name pic x(16).
          02 v-question usage pointer.
       01 tt-member-specs-v1 based.
          02 v-data usage pointer.
          02 v-len usage binary-double unsigned.
       01 tt-question-spec-v1 based.
          02 v-kind usage binary-long unsigned.
          02 v-padding-4 pic x(4).
          02 v-text pic x(24).
          02 v-yes pic x(32).
          02 v-no pic x(32).
          02 v-choices pic x(16).
          02 v-threshold pic x(24).
          02 v-relation-threshold pic x(24).
          02 v-model pic x(24).
          02 v-profile pic x(24).
          02 v-batch pic x(16).
          02 v-batch-max usage binary-long signed.
          02 v-none usage binary-long signed.
          02 v-on pic x(16).
          02 v-members pic x(16).
          02 v-kinds pic x(16).
          02 v-relations pic x(16).
          02 v-name-pointer pic x(24).
          02 v-kind-pointer pic x(24).
       01 tt-question-member-v1 based.
          02 v-name pic x(16).
          02 v-question usage pointer.
       01 tt-question-members-v1 based.
          02 v-data usage pointer.
          02 v-len usage binary-double unsigned.
       01 tt-question-view-v1 based.
          02 v-kind usage binary-long unsigned.
          02 v-padding-4 pic x(4).
          02 v-text pic x(24).
          02 v-yes pic x(32).
          02 v-no pic x(32).
          02 v-choices pic x(16).
          02 v-threshold pic x(24).
          02 v-relation-threshold pic x(24).
          02 v-model pic x(24).
          02 v-profile pic x(24).
          02 v-batch pic x(16).
          02 v-batch-max usage binary-long signed.
          02 v-none usage binary-long signed.
          02 v-on pic x(16).
          02 v-members pic x(16).
          02 v-kinds pic x(16).
          02 v-relations pic x(16).
          02 v-name-pointer pic x(24).
          02 v-kind-pointer pic x(24).
       01 tt-optional-question-v1 based.
          02 v-present usage binary-long signed.
          02 v-padding-4 pic x(4).
          02 v-value pic x(344).
       01 tt-images-v1 based.
          02 v-data usage pointer.
          02 v-len usage binary-double unsigned.
       01 tt-image-view-v1 based.
          02 v-media usage binary-long unsigned.
          02 v-padding-4 pic x(4).
          02 v-bytes usage pointer.
          02 v-bytes-len usage binary-double unsigned.
          02 v-width usage binary-long unsigned.
          02 v-height usage binary-long unsigned.
          02 v-filename pic x(24).
       01 tt-image-views-v1 based.
          02 v-data usage pointer.
          02 v-len usage binary-double unsigned.
       01 tt-optional-image-views-v1 based.
          02 v-present usage binary-long signed.
          02 v-padding-4 pic x(4).
          02 v-value pic x(16).
       01 tt-record-v1 based.
          02 v-original pic x(32).
          02 v-context pic x(32).
          02 v-options pic x(16).
          02 v-images pic x(16).
       01 tt-source-spec-v1 based.
          02 v-paths pic x(16).
          02 v-unit usage binary-long unsigned.
          02 v-padding-20 pic x(4).
          02 v-window usage binary-double unsigned.
       01 tt-controls-v1 based.
          02 v-deadline-ms usage binary-double signed.
          02 v-cancel usage pointer.
          02 v-context pic x(32).
          02 v-batch pic x(16).
          02 v-batch-max usage binary-long signed.
          02 v-attempts usage binary-long signed.
          02 v-surface pic x(16).
       01 tt-recognition-task-v1 based.
          02 v-instructions pic x(24).
          02 v-entity-definition pic x(24).
