identification division.
program-id. CountedBounds.
data division.
working-storage section.
copy "thinkthen-typed.cpy".
01 input-text pic x(9000) value all "x".
01 output-text pic x(9000) value all "z".
01 small-text pic x(8) value "sentinel".
01 source-ptr usage pointer.
01 out-ptr usage pointer.
01 saved-ptr usage pointer.
01 amount usage binary-double unsigned.
01 capacity usage binary-double unsigned.
01 written usage binary-double unsigned value 77.
01 count-value usage binary-double unsigned.
01 index-value usage binary-double unsigned.
01 item-size usage binary-double unsigned value 24.
01 item-alignment usage binary-double unsigned value 8.
01 code-value usage binary-long signed.
procedure division.
    set source-ptr to address of input-text
    move 9000 to amount capacity
    call static "TT_COPY_COUNTED" using by value size is 8 source-ptr amount
       by reference output-text by value size is 8 capacity by reference written
       returning code-value
    if code-value not = 0 or written not = 9000
       or output-text not = input-text
       move 1 to return-code goback
    end-if
    move 8 to capacity
    move 77 to written
    call static "TT_COPY_COUNTED" using by value size is 8 source-ptr amount
       by reference small-text by value size is 8 capacity by reference written
       returning code-value
    if code-value not = 1 or written not = 77 or small-text not = "sentinel"
       move 2 to return-code goback
    end-if
    set source-ptr to null
    move 1 to amount
    call static "TT_COPY_COUNTED" using by value size is 8 source-ptr amount
       by reference small-text by value size is 8 capacity by reference written
       returning code-value
    if code-value not = 1 or written not = 77
       move 3 to return-code goback
    end-if
    move 0 to amount capacity
    call static "TT_COPY_COUNTED" using by value size is 8 source-ptr amount
       source-ptr capacity by reference written returning code-value
    if code-value not = 0 or written not = 0
       move 4 to return-code goback
    end-if
    allocate tt-probability-v1
    initialize tt-probability-v1
    set source-ptr to address of tt-probability-v1
    set saved-ptr out-ptr to address of input-text
    move 18446744073709551615 to count-value
    move 0 to index-value
    call static "TT_ELEMENT" using by value size is 8 source-ptr count-value
       index-value item-size item-alignment by reference out-ptr
       returning code-value
    if code-value not = 1 or out-ptr not = saved-ptr
       move 5 to return-code goback
    end-if
    move 1 to count-value index-value
    call static "TT_ELEMENT" using by value size is 8 source-ptr count-value
       index-value item-size item-alignment by reference out-ptr
       returning code-value
    if code-value not = 1 or out-ptr not = saved-ptr
       move 6 to return-code goback
    end-if
    move 0 to index-value
    call static "TT_ELEMENT" using by value size is 8 source-ptr count-value
       index-value item-size item-alignment by reference out-ptr
       returning code-value
    if code-value not = 0 or out-ptr not = source-ptr
       move 7 to return-code goback
    end-if
    free tt-probability-v1
    move 0 to return-code
    goback.
