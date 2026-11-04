library(thinkthen)

employers <- tt_relate(
  "Morgan works for Example Group.\nPlease refund my order.",
  rules = tt_rule("person", "organization", "works_for")
)[[1]]
print(data.frame(source = employers$source$text,
                 relation = employers$relation,
                 target = employers$target$text))
