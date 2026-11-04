//! Executable starting points for command help.
pub(super) fn examples(name: &str) -> String {
    let call = match name {
        "decide" => {
            "printf '%s\\n' 'Please refund my order.' | thinkthen decide 'Does this ask for a refund?'"
        }
        "filter" => {
            "printf '%s\\n' 'Please refund my order.' 'Where is my parcel?' | thinkthen filter 'Does this ask for a refund?'"
        }
        "choose" => {
            "printf '%s\\n' 'Please refund my order.' | thinkthen choose 'Which team?' --choice billing --choice shipping"
        }
        "tag" => {
            "printf '%s\\n' 'The order is late.' | thinkthen tag 'Which concerns?' --label delivery --label payment"
        }
        "score" => {
            "printf '%s\\n' 'Please respond today.' | thinkthen score 'How urgent?' --level low --level high"
        }
        "rank" => {
            "printf '%s\\n' 'Respond today.' 'Respond next month.' | thinkthen rank 'How urgent?' --level low --level high"
        }
        "find" => {
            "printf '%s\\n' 'Contact Ada at the help desk.' | thinkthen find 'Who should I contact?'"
        }
        "recognize" => {
            "printf '%s\\n' 'Ada works at Acme.' | thinkthen recognize --kind person --kind company"
        }
        "relate" => {
            "printf '%s\\n' 'Ada works at Acme.' | thinkthen relate --rule works_for person company"
        }
        "annotate" => "thinkthen annotate questions.json < messages.jsonl",
        _ => "thinkthen run toolbox.json --entry triage < messages.jsonl",
    };
    format!("Examples:\n  {call}\n")
}
