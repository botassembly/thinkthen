identification division.
program-id. sample.
data division.
working-storage section.
copy "tt-native-generated.cpy".
01 question pic x(20) value "Is this a complaint?".
01 reviews.
   02 filler pic x(43) value
      "Arrived a day early. Thank you!".
   02 filler pic x(43) value
      "The zipper broke the first time I used it.".
   02 filler pic x(43) value
      "Does this come in blue?".
   02 filler pic x(43) value
      "The strap snapped on day two.".
01 review-table redefines reviews.
   02 review pic x(43) occurs 4.
01 review-address usage pointer occurs 4.
01 at-review usage binary-long.
01 complaints pic x(4) value "NNNN".
01 engine usage pointer.
01 session-owner usage pointer.
01 packet-owner usage pointer.
01 packet-view usage pointer.
01 read-status usage binary-long unsigned.
01 status-code usage binary-long signed.
procedure division.
    call "thinkthen_engine_new" returning engine
    allocate tt-n-cobol-RequestQuestion initialized
    allocate tt-n-cobol-RequestQuestion-text initialized
    allocate tt-n-cobol-RequestQuestion-text-member-text
    set v-data of
       tt-n-cobol-RequestQuestion-text-member-text
       to address of question
    move length of question to v-len of
       tt-n-cobol-RequestQuestion-text-member-text
    set v-m-text of tt-n-cobol-RequestQuestion-text
       to address of
       tt-n-cobol-RequestQuestion-text-member-text
    move tt-n-cobol-requestquestion-text-kind-constant
       to v-kind of tt-n-cobol-RequestQuestion
    set v-value of tt-n-cobol-RequestQuestion
       to address of tt-n-cobol-RequestQuestion-text
    perform add-review varying at-review from 1 by 1
       until at-review > 4
    allocate tt-n-cobol-RequestInput initialized
    allocate tt-n-cobol-RequestInput-records initialized
    allocate tt-n-cobol-RequestInput-records-member-items
    set v-data of
       tt-n-cobol-RequestInput-records-member-items
       to address of review-address(1)
    move 4 to v-len of
       tt-n-cobol-RequestInput-records-member-items
    set v-m-items of tt-n-cobol-RequestInput-records
       to address of
       tt-n-cobol-RequestInput-records-member-items
    move tt-n-cobol-requestinput-records-kind-constant
       to v-kind of tt-n-cobol-RequestInput
    set v-value of tt-n-cobol-RequestInput
       to address of tt-n-cobol-RequestInput-records
    allocate tt-n-cobol-RequestCall-filter initialized
    set v-m-question of tt-n-cobol-RequestCall-filter
       to address of tt-n-cobol-RequestQuestion
    set v-m-input of tt-n-cobol-RequestCall-filter
       to address of tt-n-cobol-RequestInput
    call "TT_SESSION_FILTER" using by value engine
       by reference tt-n-cobol-RequestCall-filter
       session-owner returning status-code
    if status-code not = 0 stop run returning 1 end-if
    perform read-packet
       until read-status = tt-n-session-end-v1-constant
    call "thinkthen_session_free"
       using by value session-owner returning omitted
    call "thinkthen_engine_free"
       using by value engine returning omitted
    if complaints not = "NYNY" stop run returning 1 end-if
    stop run.
add-review.
    allocate tt-n-cobol-RequestItem initialized
    allocate tt-n-cobol-RequestOriginal
    allocate tt-n-cobol-RequestOriginal-text initialized
    allocate tt-n-cobol-RequestOriginal-text-member-text
    set v-data of
       tt-n-cobol-RequestOriginal-text-member-text
       to address of review(at-review)
    move function length(function trim(
       review(at-review) trailing)) to v-len of
       tt-n-cobol-RequestOriginal-text-member-text
    set v-m-text of tt-n-cobol-RequestOriginal-text
       to address of
       tt-n-cobol-RequestOriginal-text-member-text
    move tt-n-cobol-requestoriginal-text-kind-constant
       to v-kind of tt-n-cobol-RequestOriginal
    set v-value of tt-n-cobol-RequestOriginal
       to address of tt-n-cobol-RequestOriginal-text
    set v-m-original of tt-n-cobol-RequestItem
       to address of tt-n-cobol-RequestOriginal
    set review-address(at-review)
       to address of tt-n-cobol-RequestItem.
read-packet.
    call "TT_SESSION_NEXT" using by value session-owner
       by reference read-status packet-owner
       returning status-code
    if read-status not = tt-n-session-result-v1-constant
       exit paragraph
    end-if
    call "thinkthen_session_result_view"
       using by value packet-owner by reference packet-view
       returning status-code
    set address of tt-n-complete-session-packet-v1
       to packet-view
    if v-kind of tt-n-complete-session-packet-v1 =
       tt-n-complete-session-packet-filter-row-v1-constant
       perform read-complaint
    end-if
    call "thinkthen_session_result_free"
       using by value packet-owner returning omitted.
read-complaint.
    set address of
       tt-n-complete-session-packet-filter-row-v1
       to v-data-filter-row of
       tt-n-complete-session-packet-v1
    set address of tt-n-complete-atomic-boolean-v1
       to v-value of
       tt-n-complete-session-packet-filter-row-v1
    if v-value of tt-n-complete-atomic-boolean-v1 = 1
       compute at-review = 1 + v-index-value of
          tt-n-complete-atomic-boolean-v1
       move "Y" to complaints(at-review:1)
    end-if.
