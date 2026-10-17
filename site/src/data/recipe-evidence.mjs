// The measured comparison behind the rules recipe: right, wrong and missed
// values for each method and cohort, with the record that holds them.
const SOURCE = "https://github.com/botassembly/thinkthen/blob/ea615c0e6a3412bb4716ad97b704a3f698be30cb/sdlc/records/2026-10-05-reviewed-experiment-planning.md";
const MODEL = "unknown served identity; Jev1.13.0 price basis reported";
const definitions = {
  tp: "Exact-value true positives",
  fp: "Wrong distinct predictions",
  fn: "Missing present values; present fields without answers remain in conservative recall",
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
      ["tp",tp,population,definitions.tp], ["fp",fp,predictions,definitions.fp],
      ["fn",fn,present,definitions.fn],
    ]) RULES_MEASURED.push(measured(`${arm}-${suffix}-${key}`,value,denominator,cohort,`${arm}: ${definition}`,
      resolved,population-resolved,qualifications[arm],arm==="rules"?"not applicable":"TypeSafe",arm==="rules"?"not applicable":MODEL));
  }
}
