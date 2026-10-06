      *> Unpublished 0430 counted native carriers, Linux x86_64.
      *> BASED groups borrow native storage or caller ALLOCATE storage.
      *> Overlay nested named fields with their matching typed BASED group.
       01 tt-entity-v1 based.
          02 v-text pic x(16).
          02 v-start usage binary-double unsigned.
          02 v-end usage binary-double unsigned.
          02 v-length usage binary-double unsigned.
          02 v-kind pic x(16).
          02 v-strength usage float-long.
       01 tt-entities-v1 based.
          02 v-data usage pointer.
          02 v-len usage binary-double unsigned.
       01 tt-entity-edge-v1 based.
          02 v-relation pic x(16).
          02 v-source pic x(64).
          02 v-target pic x(64).
          02 v-probability usage float-long.
          02 v-either usage binary-long signed.
          02 v-padding-156 pic x(4).
       01 tt-entity-edges-v1 based.
          02 v-data usage pointer.
          02 v-len usage binary-double unsigned.
       01 tt-optional-entity-edges-v1 based.
          02 v-present usage binary-long signed.
          02 v-padding-4 pic x(4).
          02 v-value pic x(16).
       01 tt-place-v1 based.
          02 v-start usage binary-double unsigned.
          02 v-end usage binary-double unsigned.
       01 tt-piece-v1 based.
          02 v-start usage binary-double unsigned.
          02 v-end usage binary-double unsigned.
          02 v-tags pic x(16).
       01 tt-pieces-v1 based.
          02 v-data usage pointer.
          02 v-len usage binary-double unsigned.
       01 tt-name-v1 based.
          02 v-start usage binary-double unsigned.
          02 v-end usage binary-double unsigned.
          02 v-kinds pic x(24).
          02 v-edges pic x(24).
       01 tt-names-v1 based.
          02 v-data usage pointer.
          02 v-len usage binary-double unsigned.
       01 tt-pair-v1 based.
          02 v-relation pic x(16).
          02 v-source pic x(16).
          02 v-target pic x(16).
          02 v-probability usage float-long.
       01 tt-pairs-v1 based.
          02 v-data usage pointer.
          02 v-len usage binary-double unsigned.
       01 tt-recognize-value-v1 based.
          02 v-entities pic x(16).
          02 v-relations pic x(24).
       01 tt-recognize-answer-v1 based.
          02 v-pieces pic x(16).
          02 v-names pic x(16).
          02 v-pairs pic x(16).
       01 tt-endpoint-v1 based.
          02 v-name pic x(16).
          02 v-kind pic x(16).
       01 tt-optional-endpoint-v1 based.
          02 v-present usage binary-long signed.
          02 v-padding-4 pic x(4).
          02 v-value pic x(32).
       01 tt-edge-v1 based.
          02 v-relation pic x(16).
          02 v-source pic x(32).
          02 v-target pic x(32).
          02 v-probability usage float-long.
          02 v-either usage binary-long signed.
          02 v-padding-92 pic x(4).
       01 tt-edges-v1 based.
          02 v-data usage pointer.
          02 v-len usage binary-double unsigned.
       01 tt-relation-success-v1 based.
          02 v-answer-id pic x(16).
          02 v-probability usage float-long.
          02 v-accepted usage binary-long signed.
          02 v-padding-28 pic x(4).
       01 tt-relation-answer-v1 based.
          02 v-relation pic x(16).
          02 v-reads pic x(16).
          02 v-method usage binary-long unsigned.
          02 v-direction usage binary-long unsigned.
          02 v-source pic x(32).
          02 v-target pic x(40).
          02 v-request pic x(16).
          02 v-state usage binary-long unsigned.
          02 v-padding-132 pic x(4).
          02 v-data pic x(32).
          02 v-success redefines v-data
             pic x(32).
          02 v-failure redefines v-data
             pic x(24).
       01 tt-relation-answers-v1 based.
          02 v-data usage pointer.
          02 v-len usage binary-double unsigned.
