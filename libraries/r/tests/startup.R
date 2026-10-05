# Owned generic canaries exercise all three real nested launch paths.
source(file.path(Sys.getenv("TT_TESTS"), "helper.R"))
home <- tempfile("startup-")
dir.create(home)
writeLines('options(tt_startup_site_profile = TRUE)', file.path(home, "site.Rprofile"))
writeLines('options(tt_startup_user_profile = TRUE)', file.path(home, ".Rprofile"))
writeLines('TT_STARTUP_SITE_ENV=loaded', file.path(home, "site.Renviron"))
writeLines('TT_STARTUP_USER_ENV=loaded', file.path(home, ".Renviron"))
env <- c(paste0("HOME=", home), paste0("R_PROFILE=", file.path(home, "site.Rprofile")),
         paste0("R_PROFILE_USER=", file.path(home, ".Rprofile")),
         paste0("R_ENVIRON=", file.path(home, "site.Renviron")),
         paste0("R_ENVIRON_USER=", file.path(home, ".Renviron")))
code <- c('stopifnot(is.null(getOption("tt_startup_site_profile")), is.null(getOption("tt_startup_user_profile")),',
          'Sys.getenv("TT_STARTUP_SITE_ENV") == "", Sys.getenv("TT_STARTUP_USER_ENV") == "")',
          'cat("STARTUP SUPPRESSED\n")')
one <- child(code, env)
check("helper child suppresses startup", one$status == 0L && identical(one$text, "STARTUP SUPPRESSED"))
for (name in c("request_width.R", "interrupt.R")) {
  # Evaluate the actual launcher only, without running its backend campaign.
  expressions <- parse(file.path(Sys.getenv("TT_TESTS"), name))
  launcher <- Filter(function(x) is.call(x) && identical(x[[1L]], as.name("<-")) &&
                       identical(x[[2L]], as.name("spawn")), expressions)
  stopifnot(length(launcher) == 1L)
  scope <- new.env(parent = environment())
  eval(launcher[[1L]], scope)
  values <- child_values
  scope$child_values <- function(cache, ignored) values(cache, env)
  job <- scope$spawn(code)
  output <- if (name == "interrupt.R") job$out else job$output
  for (i in 1:600) {
    if (!tools::pskill(job$pid, 0L)) break
    Sys.sleep(0.02)
  }
  check(paste(name, "suppresses startup"), !tools::pskill(job$pid, 0L) &&
          identical(readLines(output, warn = FALSE), "STARTUP SUPPRESSED"))
}
finish("startup", 0L)
