// Immutable admitted aggregate, with explicit denominators and scoped configurations.
const SOURCE = "https://github.com/botassembly/thinkthen/blob/ea615c0e6a3412bb4716ad97b704a3f698be30cb/sdlc/records/2026-10-05-reviewed-experiment-planning.md";
const MODEL = "unknown served identity; Jev1.13.0 price basis reported";
const definitions = {
  resolved: "Resolved fields; terminal completion includes fields without answers",
  tp: "Exact-value true positives",
  fp: "Wrong distinct predictions",
  fn: "Missing present values; present fields without answers remain in conservative recall",
  precision: "Exact-value precision",
};
const qualifications = {
  rules: "Frozen regexes, broad dictionary and candidate caps; not a best-possible rules baseline.",
  choose: "Tested choose confirmation only; equivalent decide performance was not established.",
  recognize: "One frozen custom-role recognize configuration; no generic recognize failure established.",
};
function measured(id,value,denominator,cohort,definition,resolved,unresolved,qualification,backend="TypeSafe",model=MODEL) {
  return { id, evidenceClass:"measured", value:String(value), denominator:String(denominator), cohort, definition,
    resolved:String(resolved), unresolved:String(unresolved), qualification, backend, model, source:SOURCE };
}
export const RULES_MEASURED = [];
// arm, resolved, TP, FP, FN, reported prediction denominator.
for (const [suffix, population, present, cohort, arms] of [
  ["full",858,575,"Full population: 858 fields on 145 documents",[
    ["rules",858,533,6876,42,7409], ["choose",857,507,124,68,631], ["recognize",850,17,424,558,441],
  ]],
  ["matched",849,569,"Matched cohort: 849 fields on 144 documents",[
    ["rules",849,528,6786,41,7314], ["choose",849,503,123,66,626], ["recognize",849,16,423,553,439],
  ]],
]) {
  for (const [arm,resolved,tp,fp,fn,predictions] of arms) {
    for (const [key,value,denominator,definition] of [
      ["resolved",`${resolved}/${population}`,population,definitions.resolved],
      ["tp",tp,population,definitions.tp], ["fp",fp,predictions,definitions.fp],
      ["fn",fn,present,definitions.fn], ["precision",`${tp}/${predictions}`,predictions,definitions.precision],
      ["recall",`${tp}/${present}`,present,suffix==="full"?"Conservative recall":"Matched exact-value recall"],
    ]) RULES_MEASURED.push(measured(`${arm}-${suffix}-${key}`,value,denominator,cohort,`${arm}: ${definition}`,
      resolved,population-resolved,qualifications[arm],arm==="rules"?"not applicable":"TypeSafe",arm==="rules"?"not applicable":MODEL));
  }
}
// Controls and accounting remain separate from matched exact-value measures.
const additional = [
  [
    "candidate-misses",
    "42/575",
    "575",
    "Present values absent from candidates",
    "Confirmation cannot recover values outside the pool."
  ],
  [
    "dictionary",
    "9919",
    "858",
    "Dictionary words in frozen rules",
    "Broad dictionary and fixed caps; not a best-possible baseline."
  ],
  [
    "choose-abstain",
    "211/283",
    "283",
    "Correct confirmation abstentions on absent receipt fields",
    "All absent fields were receipts; local absence and model none stay distinct."
  ],
  [
    "recognize-abstain",
    "277/283",
    "283",
    "Correct recognize abstentions on absent receipt fields",
    "Three faults without answers are not abstentions."
  ],
  [
    "rules-abstain",
    "0/283",
    "283",
    "Correct rules abstentions on absent receipt fields",
    "Outcomes without answers never become explicit-none answers."
  ],
  [
    "name-recognize",
    "3/5",
    "5",
    "Recognize name control recovery",
    "Small separate control, not generic capability."
  ],
  [
    "name-choose",
    "0/5",
    "5",
    "Confirmation name control recovery",
    "Discovery missed every complete target."
  ],
  [
    "date-recognize",
    "0/8",
    "8",
    "Recognize date control recovery",
    "Small separate control."
  ],
  [
    "date-choose",
    "5/8",
    "8",
    "Confirmation date control recovery",
    "Small separate control; candidate ceiling remains."
  ],
  [
    "usage",
    "$0.182553",
    "145",
    "New known usage-derived charge",
    "Not an independently verified invoice or public-fixture price promise."
  ],
  [
    "exposure",
    "$0.203782",
    "145",
    "Conservative exposure",
    "Logical reused shares do not multiply provider consumption."
  ],
  [
    "historical-charge",
    "$0.068566848",
    "145",
    "Historical earlier-work charge, retained separately",
    "Not new comparison predictions or an invoice."
  ],
  [
    "choose-hold",
    "$0.000380",
    "1",
    "Genuine confirmation fault hold",
    "Unknown usage remains held; no retry authorized."
  ],
  [
    "recognize-hold",
    "$0.021019",
    "1",
    "Genuine recognize fault hold",
    "Includes $0.000170 known partial usage; do not add it again."
  ]
];
for (const [id,value,denominator,definition,qualification] of additional) {
  RULES_MEASURED.push(measured(id,value,denominator,"Frozen comparison or separately identified control",
    definition,"not applicable","not applicable",qualification));
}
