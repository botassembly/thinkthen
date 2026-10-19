//! Command-owned profile-file wording from closed public classifications.
use std::path::Path;
use crate::{Error, ErrorDiagnostic, ProfileFailure};

pub(super) fn message(path: &Path, error: &Error) -> (u8, String) {
    let reason = match error.diagnostic() {
        Some(ErrorDiagnostic::Profile { failure, .. }) => reason(failure),
        _ => return (70, "defect: a profile failure has no profile classification".to_owned()),
    };
    (5, format!("the profile file `{}` {reason}", path.display()))
}

fn reason(failure: ProfileFailure) -> String {
    match failure {
        ProfileFailure::Json(_) => "is not valid JSON".to_owned(),
        ProfileFailure::Object => "is one JSON object".to_owned(),
        ProfileFailure::Schema => "has schema `thinkthen.backend-profile/1`".to_owned(),
        ProfileFailure::Name => "has a nonempty lowercase name using letters, digits, hyphens, or underscores".to_owned(),
        ProfileFailure::UnknownKey => "holds no unknown keys".to_owned(),
        ProfileFailure::Limit(name) => format!("has a positive integer `{name}`"),
        ProfileFailure::NoLimit => "names at least one limit".to_owned(),
        ProfileFailure::ImageProfile => "image_profile holds an admitted `id` and an explicit nonblank `model_alias` only".to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_diagnostics_keep_native_classifications_and_withhold_authored_values() {
        let secret = "distinctive-private-profile-value";
        let cases = [
            (format!("{{{secret}"), "is not valid JSON"),
            ("[]".to_owned(), "is one JSON object"),
            (format!(r#"{{"schema":"{secret}"}}"#), "has schema `thinkthen.backend-profile/1`"),
            (format!(r#"{{"schema":"thinkthen.backend-profile/1","name":"{secret}","max_questions":0}}"#), "has a positive integer `max_questions`"),
            (r#"{"schema":"thinkthen.backend-profile/1","name":"INVALID","max_questions":1}"#.to_owned(), "has a nonempty lowercase name using letters, digits, hyphens, or underscores"),
            (format!(r#"{{"schema":"thinkthen.backend-profile/1","name":"safe","{secret}":1}}"#), "holds no unknown keys"),
            (r#"{"schema":"thinkthen.backend-profile/1","name":"safe"}"#.to_owned(), "names at least one limit"),
            (format!(r#"{{"schema":"thinkthen.backend-profile/2","name":"safe","max_questions":1,"image_profile":{{"id":"{secret}"}}}}"#), "image_profile holds an admitted `id` and an explicit nonblank `model_alias` only"),
        ];
        for (text, expected) in cases {
            let error = crate::Engine::builder().profile_json(&text).unwrap_err();
            let (code, shown) = message(Path::new("profile.json"), &error);
            assert_eq!(code, 5);
            assert_eq!(shown, format!("the profile file `profile.json` {expected}"));
            assert!(!shown.contains(secret));
            assert!(!format!("{error:?}").contains(secret));
        }
    }
}
