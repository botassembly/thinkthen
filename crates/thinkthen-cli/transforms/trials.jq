# trials.jq — average repeated observations once per case before a metric.
#
# Reads: detailed scalar decide, choose, or score rows. Run it with
#   `jq -n -c -f trials.jq RUN...`.
# Several files may follow the filter. Rows with one `input.id` are trials of
# one case, wherever they occur in those files.
# Arguments: none.
# Policies:
#   - One derived row is written per id, in first-id order. A unique id still
#     becomes a one-trial derived row.
#   - Probabilities are averaged before the saved rule is applied again. A
#     derived row describes that average and never claims to be one call.
#   - Input, question, threshold, answer kind, and stable run facts must agree
#     within a case. Usage and stored provenance may differ.
#   - Stored provenance is `meta.cached`; rows written before the rename carry
#     it as `meta.replayed`, read only when `cached` is absent. A present
#     `cached` must be a boolean and wins even when false.
#   - Tag, annotate, and find need other grouping rules and are refused.
#   - Probability means remain unrounded. A score value alone is rounded to
#     twelve decimal places, matching the product.
#   - Nonempty ordinary input usually means `jq -s` was used by mistake. It is
#     refused before a plausible wrong report can print.

def valid_names($minimum; $maximum):
  type == "array"
  and length >= $minimum and length <= $maximum
  and length == (unique | length)
  and all(.[]; type == "string" and test("\\S") and (test("[[:cntrl:]]") | not));

def valid_band:
  type == "string"
  and (split(":") as $parts
       | ($parts | length) == 2
       and (try ($parts | map(tonumber)) catch null) as $sides
       | $sides != null
       and $sides[0] >= 0 and $sides[0] < $sides[1] and $sides[1] <= 1);

def valid_threshold:
  if .answer.kind == "yes_no" then
    ((.threshold | type) == "number" and .threshold > 0 and .threshold <= 1)
    or (.threshold | valid_band)
  elif .answer.kind == "choice" then
    .threshold == null
    or ((.threshold | type) == "number" and .threshold > 0 and .threshold <= 1)
  else .threshold == null
  end;

def valid_probability: type == "number" and . >= 0 and . <= 1;
def distribution_complete($names):
  (.answer.probabilities | type) == "object"
  and ((.answer.probabilities | keys) == ($names | sort))
  and (. as $row
       | all($names[]; . as $name | $row.answer.probabilities[$name] | valid_probability))
  and (([$names[] as $name | .answer.probabilities[$name]] | add) - 1 | fabs)
      <= (0.01 + (($names | length) * 2.220446049250313e-16));

def valid_probabilities:
  if .answer.kind == "yes_no" then .answer.probability | valid_probability
  elif .answer.kind == "choice" then
    (.question.options | valid_names(2; 255))
    and distribution_complete(.question.options)
  else
    (.question.levels | valid_names(2; 10))
    and distribution_complete(.question.levels)
  end;

def stored: if .meta | has("cached") then .meta.cached else .meta.replayed end;

def stable_facts:
  {schema, input, question, threshold, kind:.answer.kind,
   tool:.meta.tool, question_sha256:.meta.question_sha256,
   url:.meta.url, model:.meta.model};

def group_in_first_order($rows):
  reduce $rows[] as $row ([];
    ($row.input.id as $id | map(.id) | index($id)) as $at
    | if $at == null then . + [{id:$row.input.id, rows:[$row]}]
      else .[$at].rows += [$row]
      end);

def mean_yes_no($rows):
  [$rows[].answer.probability] as $values
  | ($values | add) / ($values | length);

def mean_distribution($rows; $names):
  reduce $names[] as $name ({};
    .[$name] = ([$rows[].answer.probabilities[$name]] as $values
                | ($values | add) / ($values | length)));

def first_maximum($probabilities; $names):
  ([$names[] as $name | $probabilities[$name]] | max) as $maximum
  | [$names[] as $name
     | select($probabilities[$name] == $maximum)
     | $name][0];

def choice_value($probabilities; $names; $threshold):
  ([$names[] as $name | select($probabilities[$name] ==
    ([$names[] as $candidate | $probabilities[$candidate]] | max)) | $name]) as $leaders
  | if ($leaders | length) != 1 then null
    elif $threshold != null and $probabilities[$leaders[0]] < $threshold then null
    else $leaders[0]
    end;

def decide_value($probability; $threshold):
  if ($threshold | type) == "number" then $probability >= $threshold
  else ($threshold | split(":") | map(tonumber)) as $sides
    | if $probability >= $sides[1] then true
      elif $probability < $sides[0] then false
      else null
      end
  end;

def derived_answer($rows; $first):
  if $first.answer.kind == "yes_no" then
    mean_yes_no($rows) as $probability
    | {value:decide_value($probability; $first.threshold),
       answer:{kind:"yes_no", probability:$probability}}
  elif $first.answer.kind == "choice" then
    mean_distribution($rows; $first.question.options) as $probabilities
    | (first_maximum($probabilities; $first.question.options)) as $pick
    | {value:choice_value($probabilities; $first.question.options; $first.threshold),
       answer:{kind:"choice", pick:$pick, probabilities:$probabilities}}
  else
    mean_distribution($rows; $first.question.levels) as $probabilities
    | (first_maximum($probabilities; $first.question.levels)) as $level
    | ([$first.question.levels[] as $name | $probabilities[$name]] | add) as $total
    | (reduce range(0; $first.question.levels | length) as $index (0;
        . + ($index * $probabilities[$first.question.levels[$index]])) / $total
       | . * 1000000000000 | round | . / 1000000000000) as $value
    | {value:$value,
       answer:{kind:"score", level:$level, probabilities:$probabilities}}
  end;

def derive($rows):
  $rows[0] as $first
  | derived_answer($rows; $first) as $derived
  | {schema:"thinkthen.trials/1", input:$first.input, value:$derived.value,
     question:$first.question, answer:$derived.answer, threshold:$first.threshold,
     trials:{count:($rows | length),
             live:([$rows[] | select(stored | not)] | length),
             replayed:([$rows[] | select(stored)] | length)},
     meta:{tool:$first.meta.tool, question_sha256:$first.meta.question_sha256,
           url:$first.meta.url, model:$first.meta.model}};

if input_filename != null
then "trials: run with jq -n\n" | halt_error(5)
else . end
| [inputs] as $rows
| if all($rows[]; type == "object") | not
  then error("trials: every row must be an object")
  elif all($rows[]; .schema? == "thinkthen.result/1") | not
  then error("trials: every row must use schema thinkthen.result/1")
  elif all($rows[]; (.input? | type) == "object" and (.input.id? | type) == "string") | not
  then error("trials: every row must carry a string input id")
  elif all($rows[]; (.question? | type) == "object" and (.question.text? | type) == "string") | not
  then error("trials: every row must carry a valid question")
  elif all($rows[]; (.answer? | type) == "object"
                    and ((.answer.kind? == "yes_no" and .question.verb? == "decide")
                         or (.answer.kind? == "choice" and .question.verb? == "choose")
                         or (.answer.kind? == "score" and .question.verb? == "score"))) | not
  then error("trials: answer kind must be yes_no, choice, or score and match the question verb")
  elif all($rows[]; valid_probabilities) | not
  then error("trials: every row must carry valid probabilities")
  elif all($rows[]; valid_threshold) | not
  then error("trials: every row must carry a valid threshold")
  elif all($rows[];
           (.meta? | type) == "object"
           and (.meta.tool? | type) == "string"
           and (.meta.url? | type) == "string"
           and (.meta.model? | type) == "string"
           and (.meta.question_sha256? | type) == "string"
           and (.meta.question_sha256 | test("\\A[0-9a-f]{64}\\z"))
           and (stored | type) == "boolean") | not
  then error("trials: every row must carry valid metadata")
  elif ([$rows[].answer.kind] | unique | length) > 1
  then error("trials: one input must carry one answer kind")
  else group_in_first_order($rows) as $groups
    | if all($groups[]; ([.rows[] | stable_facts] | unique | length) == 1) | not
      then error("trials: rows for one id must share their case, question, threshold, and run facts")
      else $groups[] | derive(.rows)
      end
  end
