      *> Unpublished 0430 counted native carriers, Linux x86_64.
      *> BASED groups borrow native storage or caller ALLOCATE storage.
      *> Overlay nested named fields with their matching typed BASED group.
       01 tt-row-v1 based.
          02 v-answer-id pic x(16).
          02 v-input pic x(32).
          02 v-question pic x(352).
          02 v-answer pic x(64).
          02 v-threshold pic x(32).
          02 v-position pic x(64).
          02 v-input-file pic x(24).
          02 v-meta pic x(376).
          02 v-images pic x(24).
       01 tt-decide-view-v1 based.
          02 v-common pic x(984).
          02 v-value pic x(32).
       01 tt-choose-view-v1 based.
          02 v-common pic x(984).
          02 v-value pic x(24).
       01 tt-tag-view-v1 based.
          02 v-common pic x(984).
          02 v-value pic x(16).
       01 tt-score-view-v1 based.
          02 v-common pic x(984).
          02 v-value usage float-long.
       01 tt-filter-view-v1 based.
          02 v-common pic x(984).
          02 v-value usage binary-long signed.
          02 v-padding-988 pic x(4).
       01 tt-rank-view-v1 based.
          02 v-common pic x(984).
          02 v-value pic x(16).
          02 v-question-name pic x(24).
       01 tt-find-view-v1 based.
          02 v-common pic x(984).
          02 v-value pic x(32).
          02 v-index pic x(16).
       01 tt-annotate-view-v1 based.
          02 v-common pic x(984).
          02 v-answers pic x(16).
       01 tt-recognize-view-v1 based.
          02 v-common pic x(984).
          02 v-value pic x(40).
          02 v-answer pic x(48).
       01 tt-relate-view-v1 based.
          02 v-common pic x(984).
          02 v-value pic x(16).
          02 v-questions pic x(16).
