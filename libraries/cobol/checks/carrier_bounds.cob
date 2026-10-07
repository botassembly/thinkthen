identification division.
program-id. CarrierBounds.
data division.
working-storage section.
copy "thinkthen-typed.cpy".
01 result-code usage binary-long signed.
01 index-value usage binary-double unsigned.
01 item-size usage binary-double unsigned.
01 item-alignment usage binary-double unsigned value 8.
01 element-ptr usage pointer.
01 input-text pic x(9000) value all "x".
01 description-text pic x(36) value
    '{"what":"keep","examples":["a","a"]}'.
01 name-text pic x(4) value "same".
01 pointer-text pic x(6) value "/field".
01 copied-text pic x(64).
01 capacity usage binary-double unsigned value 64.
01 written usage binary-double unsigned.
01 choices-table.
   02 choice-row pic x(64) occurs 2.
01 pointers-table.
   02 pointer-row pic x(16) occurs 2.
01 i usage binary-long unsigned.
01 function-kind usage binary-long unsigned.
procedure division.
    allocate tt-summary-v1
    allocate tt-decide-view-v1
    allocate tt-member-v1
    allocate tt-record-v1
    allocate tt-question-spec-v1
    move low-values to tt-question-spec-v1
    if length of tt-summary-v1 not = 712 or length of tt-record-v1 not = 96
       or length of tt-question-spec-v1 not = 344
       move 1 to return-code goback
    end-if
    call static "tt_carrier_case" using by reference tt-summary-v1
       tt-decide-view-v1 tt-member-v1 tt-record-v1 returning result-code
    if result-code not = 0 move 2 to return-code goback end-if
    set address of tt-optional-facts-v1 to address of v-facts of tt-summary-v1
    if v-present of tt-optional-facts-v1 not = 1
       move 3 to return-code goback
    end-if
    set address of tt-facts-v1 to address of v-value of tt-optional-facts-v1
    set address of tt-optional-u64-v1 to address of v-input-tokens of tt-facts-v1
    if v-present of tt-optional-u64-v1 not = 1 or
       v-value of tt-optional-u64-v1 not = 18446744073709551615
       move 4 to return-code goback
    end-if
    set address of tt-optional-u64-v1 to address of v-output-tokens of tt-facts-v1
    if v-present of tt-optional-u64-v1 not = 0 move 5 to return-code goback end-if
    set address of tt-optional-string-v1 to address of v-estimated-cost-usd of tt-facts-v1
    set address of tt-string-v1 to address of v-value of tt-optional-string-v1
    perform copy-string
    if written not = 8 or copied-text(1:8) not = "0.000123"
       move 6 to return-code goback
    end-if
    set address of tt-optional-attempts-v1 to address of v-attempts of tt-summary-v1
    set address of tt-attempts-v1 to address of v-value of tt-optional-attempts-v1
    if v-present of tt-optional-attempts-v1 not = 1 or v-len of tt-attempts-v1 not = 0
       move 7 to return-code goback
    end-if
    set address of tt-decide-value-v1 to address of v-value of tt-decide-view-v1
    if v-kind of tt-decide-value-v1 not = 1 or v-boolean of tt-decide-value-v1 not = 0
       move 8 to return-code goback
    end-if
    set address of tt-member-success-v1 to address of v-success of tt-member-v1
    set address of tt-member-value-v1 to address of v-value of tt-member-success-v1
    set address of tt-decide-value-v1 to address of v-decide of tt-member-value-v1
    if v-state of tt-member-v1 not = 1 or v-kind of tt-decide-value-v1 not = 0
       move 9 to return-code goback
    end-if
    set address of tt-answer-v1 to address of v-answer of tt-member-success-v1
    set address of tt-probabilities-v1 to address of v-tag of tt-answer-v1
    move 24 to item-size
    perform varying index-value from 0 by 1 until index-value = 2
       call static "TT_ELEMENT" using by value size is 8 v-data of tt-probabilities-v1
          v-len of tt-probabilities-v1 index-value item-size item-alignment
          by reference element-ptr returning result-code
       if result-code not = 0 move 10 to return-code goback end-if
       set address of tt-probability-v1 to element-ptr
       if v-probability of tt-probability-v1 not = index-value
          move 11 to return-code goback
       end-if
    end-perform
    set address of tt-optional-content-v1 to address of v-original of tt-record-v1
    set address of tt-content-v1 to address of v-value of tt-optional-content-v1
    set address of tt-string-v1 to address of v-data of tt-content-v1
    perform copy-string
    if written not = 6 or copied-text(1:6) not = x"c3a90d0a007a"
       move 12 to return-code goback
    end-if
    *> Build the same counted input shape through every named function tag.
    move low-values to choices-table pointers-table tt-record-v1
    perform varying i from 1 by 1 until i > 2
       set address of tt-choice-v1 to address of choice-row(i)
       set address of tt-string-v1 to address of v-name of tt-choice-v1
       set v-data of tt-string-v1 to address of name-text
       move 4 to v-len of tt-string-v1
       set address of tt-optional-content-v1 to address of v-description of tt-choice-v1
       move 1 to v-present of tt-optional-content-v1
       set address of tt-content-v1 to address of v-value of tt-optional-content-v1
       move 2 to v-kind of tt-content-v1
       set address of tt-string-v1 to address of v-data of tt-content-v1
       set v-data of tt-string-v1 to address of description-text
       move 36 to v-len of tt-string-v1
       set address of tt-string-v1 to address of pointer-row(i)
       set v-data of tt-string-v1 to address of pointer-text
       move 6 to v-len of tt-string-v1
    end-perform
    set address of tt-choices-v1 to address of v-choices of tt-question-spec-v1
    set v-data of tt-choices-v1 to address of choices-table
    move 2 to v-len of tt-choices-v1
    move v-choices of tt-question-spec-v1 to v-options of tt-record-v1
    set address of tt-strings-v1 to address of v-on of tt-question-spec-v1
    set v-data of tt-strings-v1 to address of pointers-table
    move 2 to v-len of tt-strings-v1
    set address of tt-optional-content-v1 to address of v-original of tt-record-v1
    move 1 to v-present of tt-optional-content-v1
    set address of tt-content-v1 to address of v-value of tt-optional-content-v1
    move 1 to v-kind of tt-content-v1
    set address of tt-string-v1 to address of v-data of tt-content-v1
    set v-data of tt-string-v1 to address of input-text
    move 9000 to v-len of tt-string-v1
    set address of tt-optional-content-v1 to address of v-context of tt-record-v1
    move 1 to v-present of tt-optional-content-v1
    set address of tt-content-v1 to address of v-value of tt-optional-content-v1
    move 1 to v-kind of tt-content-v1
    perform varying function-kind from 1 by 1 until function-kind > 10
       move function-kind to v-kind of tt-question-spec-v1
       call static "tt_carrier_request" using by reference tt-question-spec-v1
          tt-record-v1 by value size is 4 function-kind returning result-code
       if result-code not = 0 move 13 to return-code goback end-if
    end-perform
    free tt-summary-v1 tt-decide-view-v1 tt-member-v1 tt-record-v1 tt-question-spec-v1
    move 0 to return-code
    goback.
copy-string.
    call static "TT_COPY_COUNTED" using by value size is 8 v-data of tt-string-v1
       v-len of tt-string-v1 by reference copied-text by value size is 8 capacity
       by reference written returning result-code
    if result-code not = 0 move 14 to return-code goback end-if.
