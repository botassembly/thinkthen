//! One public-boundary driver for corpus replay and coverage-guided fuzzing.
use std::sync::OnceLock;
use thinkthen::{
    BackendName, Code, Decide, Engine, Error, Family, Question, Record, Settings, Toolbox,
};

fn usage<T>(result: Result<T, Error>) -> Result<T, Code> {
    result.map_err(|error| {
        assert_eq!(error.code().family(), Family::Usage);
        contract(&error);
        error.code()
    })
}

fn contract(error: &Error) {
    let encoded = serde_json::to_string(error).unwrap();
    let decoded: Error = serde_json::from_str(&encoded).unwrap();
    assert_eq!(decoded.code(), error.code());
    assert!(!decoded.message().is_empty());
}

/// Results let the kept corpus pin rejection and successful admission alike.
#[derive(Debug)]
pub(crate) struct Admission {
    pub(crate) question: Option<Result<(), Code>>,
    pub(crate) toolbox: Option<Result<(), Code>>,
    pub(crate) settings: Option<Result<(), Code>>,
    pub(crate) record: Option<Code>,
}

/// Exercise full inputs without truncation, recovery from panics, or real services.
pub(crate) fn exercise(data: &[u8]) -> Admission {
    static ENGINE: OnceLock<Engine> = OnceLock::new();
    static QUESTION: OnceLock<Decide> = OnceLock::new();
    let engine = ENGINE.get_or_init(|| {
        Settings::default()
            .with_backend(BackendName::Stub)
            .engine(|_| None)
            .unwrap()
    });
    let question = QUESTION.get_or_init(|| Decide::new("Is this evidence?").unwrap());
    // UTF-8 conversion failure is itself an admitted failed record.
    let record = match std::str::from_utf8(data) {
        Ok(text) => Record::from_json(text),
        Err(_) => Record::from_utf8(data),
    };
    let row = engine.run(question, record).unwrap();
    let record = row.error().map(|error| {
        assert_eq!(error.code().family(), Family::Record);
        assert!(error.code().is_record());
        contract(error);
        error.code()
    });
    let Ok(text) = std::str::from_utf8(data) else {
        return Admission {
            question: None,
            toolbox: None,
            settings: None,
            record,
        };
    };
    let question = usage(Question::from_json(text)).map(|_| ());
    let toolbox = usage(Toolbox::from_json(text)).map(|toolbox| {
        // Canonical defaults can grow an admitted toolbox past its save limit.
        if let Ok(saved) = usage(toolbox.to_json()) {
            Toolbox::from_json(&saved).unwrap();
        }
    });
    let settings = usage(Settings::from_json(text)).and_then(|settings| {
        // Resolve configuration without reading the process environment or a key.
        // Constructing an engine sends nothing, even for a remote backend name.
        usage(
            settings
                .api_key("admission-dummy".parse().unwrap())
                .engine(|_| None),
        )
        .map(|_| ())
    });
    Admission {
        question: Some(question),
        toolbox: Some(toolbox),
        settings: Some(settings),
        record,
    }
}
