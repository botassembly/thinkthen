identification division.
program-id. sample.
data division.
working-storage section.
copy "tt-native-generated.cpy".
01 question-file pic x(10) value "teams.json".
01 parcel pic x(36) value
   "My parcel went to the wrong address.".
01 team pic x(8192) value spaces.
01 team-length usage binary-double unsigned.
01 engine usage pointer.
01 session-owner usage pointer.
01 packet-owner usage pointer.
01 packet-view usage pointer.
01 read-status usage binary-long unsigned.
01 status-code usage binary-long signed.
procedure division.
    call "thinkthen_engine_new" returning engine
    allocate tt-n-cobol-RequestQuestion initialized
    allocate tt-n-cobol-RequestQuestion-file initialized
    allocate tt-n-cobol-RequestQuestion-file-member-path
    set v-data of
       tt-n-cobol-RequestQuestion-file-member-path
       to address of question-file
    move length of question-file to v-len of
       tt-n-cobol-RequestQuestion-file-member-path
    set v-m-path of tt-n-cobol-RequestQuestion-file
       to address of
       tt-n-cobol-RequestQuestion-file-member-path
    move tt-n-cobol-requestquestion-file-kind-constant
       to v-kind of tt-n-cobol-RequestQuestion
    set v-value of tt-n-cobol-RequestQuestion
       to address of tt-n-cobol-RequestQuestion-file
    allocate tt-n-cobol-RequestInput initialized
    allocate tt-n-cobol-RequestInput-text initialized
    allocate tt-n-cobol-RequestInput-text-member-text
    set v-data of tt-n-cobol-RequestInput-text-member-text
       to address of parcel
    move length of parcel to v-len of
       tt-n-cobol-RequestInput-text-member-text
    set v-m-text of tt-n-cobol-RequestInput-text
       to address of
       tt-n-cobol-RequestInput-text-member-text
    move tt-n-cobol-requestinput-text-kind-constant
       to v-kind of tt-n-cobol-RequestInput
    set v-value of tt-n-cobol-RequestInput
       to address of tt-n-cobol-RequestInput-text
    allocate tt-n-cobol-RequestCall-choose initialized
    set v-m-question of tt-n-cobol-RequestCall-choose
       to address of tt-n-cobol-RequestQuestion
    set v-m-input of tt-n-cobol-RequestCall-choose
       to address of tt-n-cobol-RequestInput
    call "TT_SESSION_CHOOSE" using by value engine
       by reference tt-n-cobol-RequestCall-choose
       session-owner returning status-code
    if status-code not = 0 stop run returning 1 end-if
    perform read-packet
       until read-status = tt-n-session-end-v1-constant
    call "thinkthen_session_free"
       using by value session-owner returning omitted
    call "thinkthen_engine_free"
       using by value engine returning omitted
    if team not = "shipping" stop run returning 1 end-if
    stop run.
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
       tt-n-complete-session-packet-choose-row-v1-constant
       perform read-team
    end-if
    call "thinkthen_session_result_free"
       using by value packet-owner returning omitted.
read-team.
    set address of
       tt-n-complete-session-packet-choose-row-v1
       to v-data-choose-row of
       tt-n-complete-session-packet-v1
    set address of tt-n-complete-atomic-nullable-string-v1
       to v-value of
       tt-n-complete-session-packet-choose-row-v1
    if v-value-presence of
       tt-n-complete-atomic-nullable-string-v1 =
       tt-n-complete-presence-value-v1-constant
       call "TT_SESSION_TEXT" using by reference
          v-value-value of
          tt-n-complete-atomic-nullable-string-v1
          team team-length returning status-code
    end-if.
