//! Retained original admission through current native MCP carriers.
use super::super::{
    admission::{CallParams, Invocation},
    protocol::MAX_MESSAGE,
};
use crate::{
    CallOptions, InputReaderOptions, QuestionInput, ReaderMedia, ReaderOptions, SourceItems,
    SourceUnit,
};
use serde_json::json;

#[test]
fn image_sources_charge_ordered_duplicates_and_stop_before_the_tail() {
    let folder =
        std::env::temp_dir().join(format!("thinkthen-mcp-image-budget-{}", std::process::id()));
    std::fs::create_dir(&folder).unwrap();
    let image = folder.join("original.png");
    let bytes = std::fs::read(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../specification/fixtures/images/above-spike.png"),
    )
    .unwrap();
    std::fs::write(&image, &bytes).unwrap();
    let tail = folder.join("invalid-tail.png");
    std::fs::write(&tail, b"not an image").unwrap();
    let admitted = MAX_MESSAGE / bytes.len();
    let mut paths = vec![image.clone(); admitted + 1];
    paths.push(tail);
    let params: CallParams = serde_json::from_value(json!({"name":"decide","arguments":{
        "question":"q", "source":{"paths":paths,"unit":"file","media":"image"}
    }}))
    .unwrap();
    let invocation = Invocation::admit(params).unwrap();
    let prepared = invocation.question().unwrap();
    let mut records = invocation.records(&prepared, CallOptions::new()).unwrap();
    for _ in 0..admitted {
        let QuestionInput::Images(original) = records.next().unwrap().unwrap().original else {
            panic!("image source original");
        };
        assert_eq!(original.images()[0].bytes(), bytes);
    }
    assert_eq!(
        records.next().unwrap().unwrap_err().detail().message(),
        "retained attachments exceed the input byte ceiling"
    );
    assert!(records.next().is_none());
    assert!(records.next().is_none());

    let options = InputReaderOptions {
        reading: ReaderOptions {
            unit: SourceUnit::File,
            window: None,
        },
        media: ReaderMedia::Image,
    };
    // Native readers retain their own ceiling; the narrower aggregate applies only to MCP.
    assert_eq!(
        crate::read_inputs(std::iter::repeat_n(&image, admitted + 1), options)
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
            .len(),
        admitted + 1
    );
    for (limit, successes) in [(bytes.len() * 2, 2), (bytes.len() * 2 - 1, 1)] {
        let mut source = SourceItems::bounded_images([&image, &image], options, limit).unwrap();
        for _ in 0..successes {
            assert!(source.next().unwrap().is_ok());
        }
        if successes == 1 {
            assert_eq!(
                source.next().unwrap().unwrap_err().detail().message(),
                "retained attachments exceed the input byte ceiling"
            );
        }
        assert!(source.next().is_none());
    }
    std::fs::remove_dir_all(folder).unwrap();
}
