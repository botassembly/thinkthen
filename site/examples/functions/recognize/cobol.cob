identification division.
program-id. sample.
data division.
working-storage section.
copy "tt-native-generated.cpy".
01 names-file pic x(10) value "names.json".
01 text-in pic x(58) value
   "Maria Chen joined Northwind Freight, "
   & "a company in Chicago.".
01 row-address usage pointer based.
01 entity-at usage pointer.
01 entity-address usage pointer based.
01 one-text pic x(8192).
01 text-length usage binary-double unsigned.
01 one-kind pic x(8192).
01 kind-length usage binary-double unsigned.
01 names-end usage binary-long value 1.
01 entity-names pic x(80) value spaces.
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
       to address of names-file
    move length of names-file to v-len of
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
       to address of text-in
    move length of text-in to v-len of
       tt-n-cobol-RequestInput-text-member-text
    set v-m-text of tt-n-cobol-RequestInput-text
       to address of
       tt-n-cobol-RequestInput-text-member-text
    move tt-n-cobol-requestinput-text-kind-constant
       to v-kind of tt-n-cobol-RequestInput
    set v-value of tt-n-cobol-RequestInput
       to address of tt-n-cobol-RequestInput-text
    allocate tt-n-cobol-RequestCall-recognize initialized
    set v-m-question of tt-n-cobol-RequestCall-recognize
       to address of tt-n-cobol-RequestQuestion
    set v-m-input of tt-n-cobol-RequestCall-recognize
       to address of tt-n-cobol-RequestInput
    call "TT_SESSION_RECOGNIZE" using by value engine
       by reference tt-n-cobol-RequestCall-recognize
       session-owner returning status-code
    if status-code not = 0 stop run returning 1 end-if
    perform read-packet
       until read-status = tt-n-session-end-v1-constant
    call "thinkthen_session_free"
       using by value session-owner returning omitted
    call "thinkthen_engine_free"
       using by value engine returning omitted
    if entity-names not = "Maria Chen/person "
       & "Northwind Freight/organization Chicago/place"
       stop run returning 1
    end-if
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
    if v-kind of tt-n-complete-session-packet-v1 not =
       tt-n-complete-session-packet-terminal-v1-constant
       and v-kind of tt-n-complete-session-packet-v1 not =
       tt-n-complete-session-packet-observation-v1-constant
       perform read-names
    end-if
    call "thinkthen_session_result_free"
       using by value packet-owner returning omitted.
read-names.
    set address of
       tt-n-complete-session-packet-recognize-aggregate-v1
       to v-data-recognize-aggregate of
       tt-n-complete-session-packet-v1
    if v-value-len of
       tt-n-complete-session-packet-recognize-aggregate-v1
       = 0
       exit paragraph
    end-if
    set address of row-address to v-value-data of
       tt-n-complete-session-packet-recognize-aggregate-v1
    set address of tt-n-complete-recognition-v1
       to row-address
    set address of tt-n-complete-recognize-v1
       to v-value of tt-n-complete-recognition-v1
    set address of
       tt-n-complete-recognize-fields-entities-v1
       to v-data-fields-entities of
       tt-n-complete-recognize-v1
    set entity-at to v-entities-data of
       tt-n-complete-recognize-fields-entities-v1
    perform read-entity v-entities-len of
       tt-n-complete-recognize-fields-entities-v1 times.
read-entity.
    set address of entity-address to entity-at
    set address of tt-n-complete-entity-v1
       to entity-address
    call "TT_SESSION_TEXT" using by reference
       v-text of tt-n-complete-entity-v1
       one-text text-length returning status-code
    call "TT_SESSION_TEXT" using by reference
       v-kind of tt-n-complete-entity-v1
       one-kind kind-length returning status-code
    string one-text(1:text-length) "/"
       one-kind(1:kind-length) " "
       delimited by size into entity-names
       with pointer names-end
    set entity-at up by length of entity-at.
