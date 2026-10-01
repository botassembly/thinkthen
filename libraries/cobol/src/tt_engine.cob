       >>SOURCE FORMAT FREE
identification division.
program-id. TT-ENGINE-NEW.
data division.
working-storage section.
01 idx usage binary-double unsigned.
*> The caller's text is copied here before the NUL; caller storage is never written.
01 settings-copy pic x(8192).
linkage section.
01 settings-text pic x(8192).
01 settings-length usage binary-double unsigned.
01 engine usage pointer.
01 failure-row.
   02 failure-code usage binary-long signed.
   02 failure-retryable usage binary-long signed.
   02 failure-message pic x(512).
   02 failure-facts-length usage binary-double unsigned.
   02 failure-facts-json pic x(8192).
procedure division using settings-text settings-length engine failure-row.
    set engine to null
    move 0 to failure-code failure-retryable failure-facts-length
    move spaces to failure-message failure-facts-json
    if settings-length = 0 or settings-length >= 8192
       move 1 to failure-code
       move "settings length out of range" to failure-message
       goback
    end-if
    perform varying idx from 1 by 1 until idx > settings-length
       if settings-text(idx:1) = x"00"
          move 1 to failure-code
          move "interior NUL in settings" to failure-message
          goback
       end-if
    end-perform
    move settings-text(1:settings-length) to settings-copy(1:settings-length)
    move x"00" to settings-copy(settings-length + 1:1)
    call "thinkthen_engine_new_with" using by reference settings-copy
       returning engine
    if engine = null
       call "TT-CAPTURE-ERROR" using engine failure-row
    end-if
    goback.
