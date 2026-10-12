identification division.
program-id. sample.
data division.
working-storage section.
copy "tt-native-generated.cpy".
01 question pic x(37) value
   "Which line gives the refund deadline?".
01 policy.
   02 filler pic x(46) value
      "Returns need the original receipt.".
   02 filler pic x(46) value
      "Refunds are issued within 30 days of purchase.".
   02 filler pic x(46) value
      "Shipping is free on orders over $50.".
   02 filler pic x(46) value
      "Gift cards cannot be exchanged for cash.".
01 policy-table redefines policy.
   02 policy-line pic x(46) occurs 4.
01 line-address usage pointer occurs 4.
01 at-line usage binary-long.
01 refund-deadline pic x(8192) value spaces.
01 deadline-length usage binary-double unsigned.
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
    perform add-line varying at-line from 1 by 1
       until at-line > 4
    allocate tt-n-cobol-RequestInput initialized
    allocate tt-n-cobol-RequestInput-records initialized
    allocate tt-n-cobol-RequestInput-records-member-items
    set v-data of
       tt-n-cobol-RequestInput-records-member-items
       to address of line-address(1)
    move 4 to v-len of
       tt-n-cobol-RequestInput-records-member-items
    set v-m-items of tt-n-cobol-RequestInput-records
       to address of
       tt-n-cobol-RequestInput-records-member-items
    move tt-n-cobol-requestinput-records-kind-constant
       to v-kind of tt-n-cobol-RequestInput
    set v-value of tt-n-cobol-RequestInput
       to address of tt-n-cobol-RequestInput-records
    allocate tt-n-cobol-RequestCall-find initialized
    set v-m-question of tt-n-cobol-RequestCall-find
       to address of tt-n-cobol-RequestQuestion
    set v-m-input of tt-n-cobol-RequestCall-find
       to address of tt-n-cobol-RequestInput
    call "TT_SESSION_FIND" using by value engine
       by reference tt-n-cobol-RequestCall-find
       session-owner returning status-code
    if status-code not = 0 stop run returning 1 end-if
    perform read-packet
       until read-status = tt-n-session-end-v1-constant
    call "thinkthen_session_free"
       using by value session-owner returning omitted
    call "thinkthen_engine_free"
       using by value engine returning omitted
    if refund-deadline not = policy-line(2)
       stop run returning 1
    end-if
    stop run.
add-line.
    allocate tt-n-cobol-RequestItem initialized
    allocate tt-n-cobol-RequestOriginal
    allocate tt-n-cobol-RequestOriginal-text initialized
    allocate tt-n-cobol-RequestOriginal-text-member-text
    set v-data of
       tt-n-cobol-RequestOriginal-text-member-text
       to address of policy-line(at-line)
    move function length(function trim(
       policy-line(at-line) trailing)) to v-len of
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
    set line-address(at-line)
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
    if v-kind of tt-n-complete-session-packet-v1 not =
       tt-n-complete-session-packet-terminal-v1-constant
       and v-kind of tt-n-complete-session-packet-v1 not =
       tt-n-complete-session-packet-observation-v1-constant
       perform read-deadline
    end-if
    call "thinkthen_session_result_free"
       using by value packet-owner returning omitted.
read-deadline.
    set address of
       tt-n-complete-session-packet-find-aggregate-v1
       to v-data-find-aggregate of
       tt-n-complete-session-packet-v1
    set address of tt-n-complete-find-v1
       to v-value of
       tt-n-complete-session-packet-find-aggregate-v1
    if v-value-presence of tt-n-complete-find-v1 =
       tt-n-complete-presence-value-v1-constant
       set address of tt-n-complete-json-v1
          to v-value-value of tt-n-complete-find-v1
       call "TT_SESSION_TEXT" using by reference
          v-data-string of tt-n-complete-json-v1
          refund-deadline deadline-length
          returning status-code
    end-if.
