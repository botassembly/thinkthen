//! Offline analysis of saved runs uses the existing audit and diff contracts.

use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub(crate) enum RunsCommand {
    /// Grade saved answers against an answer key and suggest a bar.
    ///
    /// RESULTS holds the lines `decide`, `filter`, `choose`, `tag`, `score`,
    /// `rank`, `find`, `annotate`, `recognize`, or `relate` printed with
    /// --details, and KEY holds one
    /// JSON object per record: its id, the right value, and an optional part of
    /// tune or held. audit prints agreement with its 95% interval, both kinds of
    /// disagreement, precision and f1, AUC, calibration, a coverage curve, and a
    /// suggested bar tuned on one part and checked on the other. An answer
    /// inside a band is not sure, and it counts apart from right and wrong.
    /// recognize names and relate edges get precision, recall, and f1.
    ///
    /// A key may give each record a part of tune or held; without parts audit
    /// splits the records itself and shows how steady its bar is.
    /// --cases instead prints one JSON line per saved case with its keyed
    /// outcome, probabilities, available usage and question identity.
    /// It keeps failed, unlabeled, unsure and tied cases distinct.
    ///
    /// --write changes one threshold in the file and prints the old value on
    /// standard error.
    /// --write QUESTIONS --write-to OUTPUT instead creates a tuned file at a
    /// new path and keeps QUESTIONS unchanged. An existing OUTPUT is refused.
    ///
    /// audit sends no request and reads no key.
    ///
    /// To grade a recording, replay it with --details and pass the output:
    ///
    /// thinkthen decide 'Is it red?' --jsonl --details --replay runs/red < records.jsonl | thinkthen runs audit - key.jsonl
    Audit(crate::cli::audit::AuditArguments),

    /// Show which saved answers changed between two runs or two cuts.
    ///
    /// A and B hold the lines `decide`, `choose`, `recognize`, or `relate` printed for the same
    /// records. Without B, diff compares A under --threshold with A under --compare-threshold.
    /// Two cuts on one run cost nothing. The probabilities are already saved. Each change prints
    /// one JSON line, or under --table one line and a line per changed item. A summary with its
    /// McNemar test prints last. With --key, a changed answer says whether it gained or lost a right
    /// answer, and a changed record counts the key names or edges each side matched. An answer
    /// inside a band is not sure.
    ///
    /// diff pairs answers by record id and answer name only. It compares
    /// question digests only when both runs saved --details.
    ///
    /// diff sends no request and reads no key.
    ///
    /// thinkthen runs diff runs/before.jsonl runs/after.jsonl --key key.jsonl --table
    Diff(crate::cli::diff::DiffArguments),
}
