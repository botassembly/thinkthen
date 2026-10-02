/-->/ {
  split($1, t, /[:.]/)
  if (t[1] > 0)
    stamp = sprintf("%d:%02d:%02d", t[1], t[2], t[3])
  else
    stamp = sprintf("%d:%02d", t[2], t[3])
  next
}
/<c>/ {
  gsub(/<[^>]*>/, "")
  gsub(/&gt;/, ">")
  gsub(/&lt;/, "<")
  gsub(/&#39;/, "\047")
  gsub(/&quot;/, "\"")
  gsub(/&amp;/, "\\&")
  if (length($0)) print "[" stamp "] " $0
}
