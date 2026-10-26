# Feature: loading an incompatible installed native module refuses its ABI.
# The runner supplies a real installed package with an incompatible engine.
error <- tryCatch({ loadNamespace("thinkthen"); NULL }, error = identity)
stopifnot(inherits(error, "thinkthen_error"),
          identical(as.character(error$code), "usage.abi"))
cat("R native ABI refused\n")
