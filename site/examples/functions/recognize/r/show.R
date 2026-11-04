library(thinkthen)

mentions <- tt_recognize("Morgan works for Example Group.",
                         kinds = c("person", "organization"))[[1]]
print(data.frame(text = mentions$text, kind = mentions$kind))
