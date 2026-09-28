scores=../../tables/functions.tsv
jq -nrR '
  def two: . * 1000 | round
    | if . % 20 == 5 then . - 1 else . end
    | (. + 5) / 10 | floor | tostring
    | "0." + ("0" + .)[-2:];
  def pad($n): . + " " * ($n - length);
  [inputs | split("\t") | select(.[3] == "yes")]
  | group_by(.[0])
  | map({
      strict: map(select(.[2] | test("top pick") | not))[0],
      picks: map(select(.[2] | test("top pick right")))
        | map((.[5] | tonumber | two) + " "
          + (.[2] | sub(" ?top pick right"; "")))
    })
  | sort_by(.strict[5] | tonumber) | reverse
  | ["function", "measure", "strict", "top pick"],
    (.[] | [.strict[0], .strict[2],
      (.strict[5] | tonumber | two), .picks[0]],
      (.picks[1:][] | ["", "", "", .]))
  | (.[0] | pad(10)) + (.[1] | pad(30))
    + (.[2] | pad(8)) + (.[3] // "")
  | sub(" +$"; "")
' "$scores"
