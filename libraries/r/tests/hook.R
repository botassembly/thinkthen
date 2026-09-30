# A user's options(error = ...) hook meets a thinkthen error the way it
# meets a plain stop (R7-13's first half), and the synthetic interrupt
# still stops a call when the real signal never lands (R5-12). R consults
# the hook at signal time, so .tt_call restores it before it raises.
source(file.path(Sys.getenv("TT_TESTS"), "helper.R"))

hooked <- child(c('options(error = function() cat("HOOK RAN\\n"))', 'tt_decide("Q?", "x", deadline_ms = NA)', 'cat("AFTER\\n")'))
plain <- child(c('options(error = function() cat("HOOK RAN\\n"))', 'stop("plain")', 'cat("AFTER\\n")'))
check("a thinkthen error under a hook runs it and goes on, as a plain stop does",
      grepl("HOOK RAN\nAFTER", hooked$text, fixed = TRUE) && grepl("HOOK RAN\nAFTER", plain$text, fixed = TRUE))

bare <- child(c('tt_decide("Q?", "x", deadline_ms = NA)', 'cat("NEVER\\n")'))
check("without a hook the error halts the script nonzero", bare$status != 0L && !grepl("NEVER", bare$text, fixed = TRUE))

# R5-12: pskill reports a signal it never sent, and the call still stops.
lost <- child(c('utils::assignInNamespace("pskill", function(pid, signal) TRUE, ns = "tools")',
                'r <- tryCatch({ thinkthen:::.tt_interrupt(); "continued" }, interrupt = function(e) "stopped")',
                'cat("OUTCOME", r, "\\n")'))
check("R5-12: an interrupt that never lands still stops the call", grepl("OUTCOME stopped", lost$text, fixed = TRUE))

finish("hook", 0L)
