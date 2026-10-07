       01 tt-input-property-v1 based.
          02 v-name pic x(16).
          02 v-kind usage binary-long unsigned.
          02 v-padding-20 pic x(4).
       01 tt-input-properties-v1 based.
          02 v-data usage pointer.
          02 v-len usage binary-double unsigned.
       01 tt-input-declaration-v1 based.
          02 v-kind usage binary-long unsigned.
          02 v-padding-4 pic x(4).
          02 v-properties pic x(16).
          02 v-required pic x(16).
       01 tt-question-author-v1 based.
          02 v-name pic x(24).
          02 v-wording-version pic x(16).
          02 v-item-schema pic x(40).
          02 v-context-schema pic x(40).
       01 tt-reported-usage-v1 based.
          02 v-present usage binary-long signed.
          02 v-padding-4 pic x(4).
          02 v-input-tokens pic x(16).
          02 v-output-tokens pic x(16).
       01 tt-source-detail-v1 based.
          02 v-origin usage binary-long unsigned.
          02 v-padding-4 pic x(4).
          02 v-answered-by pic x(16).
          02 v-batch-size pic x(16).
       01 tt-source-details-v1 based.
          02 v-data usage pointer.
          02 v-len usage binary-double unsigned.
       01 tt-input-view-v1 based.
          02 v-original pic x(32).
          02 v-position pic x(64).
          02 v-images pic x(24).
       01 tt-input-views-v1 based.
          02 v-data usage pointer.
          02 v-len usage binary-double unsigned.
       01 tt-details-v1 based.
          02 v-question pic x(352).
          02 v-threshold pic x(32).
          02 v-raw-pick pic x(24).
          02 v-usage pic x(40).
          02 v-question-sources pic x(16).
          02 v-observations pic x(16).
          02 v-inputs pic x(16).
       01 tt-source-entity-v1 based.
          02 v-entity pic x(64).
          02 v-position pic x(64).
       01 tt-source-entities-v1 based.
          02 v-data usage pointer.
          02 v-len usage binary-double unsigned.
       01 tt-source-entity-edge-v1 based.
          02 v-relation pic x(16).
          02 v-source pic x(128).
          02 v-target pic x(128).
          02 v-probability usage float-long.
          02 v-either usage binary-long signed.
          02 v-padding-284 pic x(4).
       01 tt-source-entity-edges-v1 based.
          02 v-data usage pointer.
          02 v-len usage binary-double unsigned.
       01 tt-optional-source-entity-edges-v1 based.
          02 v-present usage binary-long signed.
          02 v-padding-4 pic x(4).
          02 v-value pic x(16).
       01 tt-source-recognition-v1 based.
          02 v-present usage binary-long signed.
          02 v-padding-4 pic x(4).
          02 v-entities pic x(16).
          02 v-relations pic x(24).
       01 tt-source-endpoint-v1 based.
          02 v-ordinal usage binary-double unsigned.
          02 v-endpoint pic x(32).
          02 v-record pic x(24).
          02 v-position pic x(64).
       01 tt-source-edge-v1 based.
          02 v-relation pic x(16).
          02 v-source pic x(128).
          02 v-target pic x(128).
          02 v-probability usage float-long.
          02 v-either usage binary-long signed.
          02 v-padding-284 pic x(4).
       01 tt-source-edges-v1 based.
          02 v-data usage pointer.
          02 v-len usage binary-double unsigned.
       01 tt-source-relations-v1 based.
          02 v-present usage binary-long signed.
          02 v-padding-4 pic x(4).
          02 v-edges pic x(16).
