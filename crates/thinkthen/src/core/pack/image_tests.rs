//! Image identity and grouping at the shared-input boundary.
use super::{Entry, PackLimits, Packer, QuestionKey, asks};
use crate::core::image::{ImageInput, ImageMedia};
use crate::core::{Descriptions, Evidence, ModelName, Plan, Question, QuestionText, Url};
use std::sync::Arc;

fn plan(image: Option<ImageInput>) -> Plan {
    Plan::new(
        Evidence::new("same text").expect("evidence"),
        ModelName::new("m").expect("model"),
        Descriptions::Authored,
        vec![Question::Decide {
            text: QuestionText::new("Visible?").expect("question"),
            yes: None,
            no: None,
        }],
    )
    .expect("plan")
    .with_image(image.map(|image| vec![image]))
}

#[test]
fn bytes_and_media_change_keys_and_groups_while_size_counts_the_sent_payload() {
    let url = Url::new("http://localhost:8080/v1/systemone").expect("url");
    let limits = PackLimits {
        ceiling: 96000,
        profile: None,
        inputs: 16,
        questions: None,
        context: false,
    };
    let mut packer = Packer::new(limits, "\"m\"".to_owned());
    let mut closed = vec![];
    let mut keys = vec![];
    for (position, image) in [
        None,
        Some((ImageMedia::Png, vec![1])),
        Some((ImageMedia::Png, vec![2])),
        Some((ImageMedia::Jpeg, vec![2])),
    ]
    .into_iter()
    .enumerate()
    {
        let image = image.map(|(media, bytes)| ImageInput {
            media,
            bytes: bytes.into(),
        });
        let plan = plan(image);
        let mut questions = asks(&url, &plan).expect("asks");
        let ask = questions.pop().expect("one ask");
        assert_eq!(
            QuestionKey::stored(&url, "\"m\"", ask.state.json(), &ask.question),
            ask.key
        );
        keys.push(ask.key);
        let body = crate::core::adapters::built_in::encode(&plan).expect("encoded");
        packer
            .add(
                vec![Entry {
                    state: ask.state,
                    question: Arc::clone(&ask.question),
                    options: 0,
                    item: (position, body),
                }],
                &mut closed,
            )
            .expect("fits");
    }
    closed.extend(packer.close());
    assert_eq!(closed.len(), 4, "different images cannot share a request");
    for request in closed {
        let [(_, expected)] = request.items.as_slice() else {
            panic!("one input per image")
        };
        assert_eq!(&request.body, expected);
    }
    keys.sort();
    keys.dedup();
    assert_eq!(keys.len(), 4);
}
