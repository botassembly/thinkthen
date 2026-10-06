identification division.
program-id. TypedConsumer.
data division.
working-storage section.
copy "thinkthen-typed.cpy".
01 engine-ptr usage pointer.
01 question-ptr usage pointer.
01 source-ptr usage pointer.
01 image-ptr usage pointer.
01 result-ptr usage pointer.
01 code-value usage binary-long signed.
01 index-value usage binary-double unsigned value 0.
01 count-value usage binary-double unsigned value 1.
01 media-value usage binary-long unsigned value 2.
procedure division.
    *> Compile-only public entry consumer; this program is never executed.
    allocate tt-question-spec-v1
    allocate tt-record-v1
    allocate tt-source-spec-v1
    allocate tt-controls-v1
    allocate tt-optional-string-v1
    allocate tt-string-v1
    call "TT_QUESTION_NEW" using by value engine-ptr by reference
       tt-question-spec-v1 question-ptr returning code-value
    call "TT_QUESTION_LOAD" using by value engine-ptr by reference
       tt-string-v1 question-ptr returning code-value
    call "TT_IMAGE_CLONE" using by value engine-ptr image-ptr
       by value size is 8 count-value by value size is 4 media-value by reference tt-optional-string-v1 image-ptr returning code-value
    call "TT_SOURCE_RECORDS" using by value engine-ptr by reference
       tt-record-v1 by value size is 8 count-value by reference source-ptr returning code-value
    call "TT_SOURCE_FILES" using by value engine-ptr by reference
       tt-source-spec-v1 source-ptr returning code-value
    allocate tt-decide-view-v1
    call "TT_DECIDE" using by value engine-ptr question-ptr source-ptr
       by reference tt-controls-v1 result-ptr returning code-value
    call "thinkthen_result_decide" using by value result-ptr by value size is 8 index-value
       by reference tt-decide-view-v1 returning code-value
    allocate tt-choose-view-v1
    call "TT_CHOOSE" using by value engine-ptr question-ptr source-ptr
       by reference tt-controls-v1 result-ptr returning code-value
    call "thinkthen_result_choose" using by value result-ptr by value size is 8 index-value
       by reference tt-choose-view-v1 returning code-value
    allocate tt-tag-view-v1
    call "TT_TAG" using by value engine-ptr question-ptr source-ptr
       by reference tt-controls-v1 result-ptr returning code-value
    call "thinkthen_result_tag" using by value result-ptr by value size is 8 index-value
       by reference tt-tag-view-v1 returning code-value
    allocate tt-score-view-v1
    call "TT_SCORE" using by value engine-ptr question-ptr source-ptr
       by reference tt-controls-v1 result-ptr returning code-value
    call "thinkthen_result_score" using by value result-ptr by value size is 8 index-value
       by reference tt-score-view-v1 returning code-value
    allocate tt-filter-view-v1
    call "TT_FILTER" using by value engine-ptr question-ptr source-ptr
       by reference tt-controls-v1 result-ptr returning code-value
    call "thinkthen_result_filter" using by value result-ptr by value size is 8 index-value
       by reference tt-filter-view-v1 returning code-value
    allocate tt-rank-view-v1
    call "TT_RANK" using by value engine-ptr question-ptr source-ptr
       by reference tt-controls-v1 result-ptr returning code-value
    call "thinkthen_result_rank" using by value result-ptr by value size is 8 index-value
       by reference tt-rank-view-v1 returning code-value
    allocate tt-find-view-v1
    call "TT_FIND" using by value engine-ptr question-ptr source-ptr
       by reference tt-controls-v1 result-ptr returning code-value
    call "thinkthen_result_find" using by value result-ptr by value size is 8 index-value
       by reference tt-find-view-v1 returning code-value
    allocate tt-annotate-view-v1
    call "TT_ANNOTATE" using by value engine-ptr question-ptr source-ptr
       by reference tt-controls-v1 result-ptr returning code-value
    call "thinkthen_result_annotate" using by value result-ptr by value size is 8 index-value
       by reference tt-annotate-view-v1 returning code-value
    allocate tt-recognize-view-v1
    call "TT_RECOGNIZE" using by value engine-ptr question-ptr source-ptr
       by reference tt-controls-v1 result-ptr returning code-value
    call "thinkthen_result_recognize" using by value result-ptr by value size is 8 index-value
       by reference tt-recognize-view-v1 returning code-value
    allocate tt-relate-view-v1
    call "TT_RELATE" using by value engine-ptr question-ptr source-ptr
       by reference tt-controls-v1 result-ptr returning code-value
    call "thinkthen_result_relate" using by value result-ptr by value size is 8 index-value
       by reference tt-relate-view-v1 returning code-value
    allocate tt-summary-v1
    allocate tt-observation-v1
    call "thinkthen_result_summary" using by value result-ptr
       by reference tt-summary-v1 returning code-value
    call "thinkthen_result_observation" using by value result-ptr by value size is 8 index-value
       by reference tt-observation-v1 returning code-value
    call "thinkthen_error_complete" using by value engine-ptr
       by reference result-ptr returning code-value
    goback.
