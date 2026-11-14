.tt_state <- new.env(parent = emptyenv())

.tt_native <- function(name, ...) {
  result <- .Call(paste0("wrap__", name), ..., PACKAGE = "thinkthen")
  if (!is.null(result$error)) {
    error <- result$error
    family <- error$family
    if (error$code %in% c("usage.key-missing", .tt_abi_code) ||
        (name == "r_client" && identical(error$code, "local.io"))) family <- "setup"
    if (name == "r_question") family <- "usage"
    rlang::abort(error$message,
      class = c(paste0("thinkthen_", family, "_error"),
                if (identical(error$code, "local.cancelled")) "thinkthen_cancelled_error",
                "thinkthen_error"),
      code = error$code, http_status = error$http_status, call = rlang::caller_env(),
      trace = rlang::trace_back(bottom = globalenv()))
  }
  result
}

.tt_r_error <- function(message, code) {
  rlang::abort(message, class = c("thinkthen_usage_error", "thinkthen_error"),
               code = "usage.argument", call = rlang::caller_env())
}

.tt_dots <- function(...) {
  values <- rlang::enquos(..., .ignore_empty = "none")
  if (length(values) && (is.null(names(values)) || anyNA(names(values)) || any(!nzchar(names(values))))) {
    .tt_r_error("Arguments in ... must be named.", "arguments")
  }
  list(...)
}

.tt_json <- function(value) {
  as.character(jsonlite::toJSON(value, auto_unbox = TRUE, null = "null",
                               na = "null", digits = I(17), force = FALSE))
}

# Configuration cannot use JSON null as a replacement for a missing or nonfinite R value.
.tt_json_finite <- function(value) {
  if (anyNA(names(value))) return(FALSE)
  if (is.list(value)) return(all(vapply(unclass(value), .tt_json_finite, logical(1))))
  if (is.atomic(value)) {
    value <- unclass(value)
    return(!anyNA(value) && (!is.numeric(value) || all(is.finite(value))))
  }
  TRUE
}

.tt_conversion <- function(helper, code) {
  tryCatch(force(code), error = function(error) {
    if (inherits(error, "thinkthen_error")) stop(error)
    .tt_native(helper)
  })
}

.tt_context_json <- function(value, helper) {
  force(value)
  .tt_conversion(helper, {
    if (!.tt_json_finite(value)) .tt_native(helper)
    .tt_json(value)
  })
}

.tt_options <- function(..., .dots = list()) {
  values <- list(...)
  c(values[!vapply(values, is.null, logical(1))], .dots)
}

#' Create an engine client; an omitted backend reads THINKTHEN_BACKEND.
#'
#' @rdname tt_client
#' @export
tt_client <- function(backend = NULL, ..., api_key = NULL, url = NULL,
                      model = NULL, batch = NULL, max_requests = NULL, retry = NULL) {
  .tt_check_abi()
  dots <- .tt_dots(...)
  if (is.null(backend)) {
    backend <- Sys.getenv("THINKTHEN_BACKEND", unset = NA_character_)
    if (identical(backend, NA_character_)) backend <- NULL
  }
  force(backend); force(api_key); force(url); force(model); force(batch); force(max_requests); force(retry)
  if (is.numeric(max_requests) && any(abs(max_requests) > 9007199254740991, na.rm = TRUE)) .tt_native("r_settings_conversion_error")
  if (inherits(retry, "tt_retry")) retry <- .tt_snapshot_value(retry)
  settings <- .tt_options(backend = backend, api_key = api_key, url = url,
                          model = model, batch = batch, max_requests = max_requests, retry = retry, .dots = dots)
  if (!length(settings)) settings <- structure(list(), names = character())
  structure(.tt_native("r_client", .tt_context_json(settings, "r_settings_conversion_error"))$value,
            class = "tt_client")
}

print.tt_client <- function(x, ...) { cat("<tt_client>\n"); invisible(x) }
#' Copy cumulative client usage, including unresolved work in an early snapshot.
#'
#' @rdname tt_client
#' @export
tt_usage <- function(client) {
  jsonlite::fromJSON(.tt_native("r_client_usage", client)$value,
                     simplifyVector = FALSE, bigint_as_char = TRUE)
}

#' Release a client; repeated close is safe and scoped defaults borrow clients.
#'
#' @rdname tt_client
#' @export
tt_close <- function(client) {
  .Call("wrap__r_client_close", client, PACKAGE = "thinkthen")
  invisible(NULL)
}

#' Set a scoped default and restore the previous client on normal or exceptional exit.
#'
#' @rdname tt_client
#' @export
local_tt_client <- function(client, .env = parent.frame()) {
  previous <- .tt_state$client
  .tt_state$client <- client
  withr::defer(.tt_state$client <- previous, envir = .env)
  invisible(client)
}

#' Evaluate an expression with a borrowed default client, then restore the previous default.
#'
#' @rdname tt_client
#' @export
with_tt_client <- function(client, code) {
  previous <- .tt_state$client
  on.exit(.tt_state$client <- previous)
  .tt_state$client <- client
  force(code)
}

#' Admit immutable retry settings with the shared engine.
#'
#' @rdname tt_client
#' @export
tt_retry <- function(..., max_retries = NULL, initial_backoff_ms = NULL,
                     attempt_timeout_ms = NULL) {
  dots <- .tt_dots(...)
  retry <- .tt_options(max_retries = max_retries, initial_backoff_ms = initial_backoff_ms,
                       attempt_timeout_ms = attempt_timeout_ms, .dots = dots)
  if (!length(retry)) retry <- structure(list(), names = character())
  canonical <- .tt_native("r_admit",
    .tt_context_json(list(retry = retry), "r_settings_conversion_error"), 1L)$value
  value <- jsonlite::fromJSON(canonical, simplifyVector = FALSE)$retry
  .tt_snapshot(.tt_json(value), "tt_retry")
}

.tt_check_abi <- function() {
  compatible <- tryCatch({
    symbols <- names(getDLLRegisteredRoutines("thinkthen")$.Call)
    if (!.tt_abi_version_export %in% symbols || !all(.tt_abi_exports %in% symbols)) {
      FALSE
    } else {
      actual <- .Call(.tt_abi_version_export, PACKAGE = "thinkthen")
      typeof(actual) == "integer" && length(actual) == 1L && !is.na(actual) &&
        bitwShiftR(actual, 16L) == bitwShiftR(.tt_abi_version, 16L) &&
        bitwAnd(actual, 65535L) >= bitwAnd(.tt_abi_version, 65535L)
    }
  }, error = function(error) FALSE)
  if (!isTRUE(compatible)) .tt_check_abi_failure()

  invisible(NULL)
}

.onLoad <- function(libname, pkgname) .tt_check_abi()

.tt_restore <- function(original, selected) {
  attrs <- attributes(original)
  for (name in union(setdiff(names(attributes(selected)), "class"),
                     intersect(names(attrs), c("names", "row.names", "dim", "dimnames")))) {
    attrs[name] <- list(attr(selected, name))
  }
  attributes(selected) <- attrs
  selected
}

.tt_subset <- function(x, positions) {
  if (is.data.frame(x)) {
    selected <- x[positions, , drop = FALSE]
    for (column in seq_along(selected)) {
      selected[[column]] <- .tt_restore(x[[column]], selected[[column]])
    }
  } else {
    selected <- x[positions]
  }
  selected <- .tt_restore(x, selected)
  attr(selected, ".thinkthen_function") <- NULL
  # An index restored from the input describes the input's rows, not this subset's.
  if (inherits(selected, "data.table")) {
    attr(selected, "index") <- NULL
    selected <- data.table::setalloccol(selected)
  }
  selected
}

