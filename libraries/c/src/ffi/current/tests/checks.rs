//! Assertions over retained private native projections.
use super::{Row, list, text};
fn assert_atomic(row: &crate::current::private::CurrentAtomicV1, index: usize, file_mode: bool) {
    assert_eq!(row.value.kind, index as u32 + 1);
    assert_eq!(row.common.position.present, i32::from(file_mode));
    if file_mode {
        assert_eq!(row.common.position.value.first_line.value, 1);
        assert_eq!(row.common.position.value.last_line.value, 4);
    }
    if index == 0 {
        assert_eq!(row.value.boolean, 1);
        assert_eq!(row.yes_probability.value, 0.9);
    }
    if index == 1 {
        assert_eq!(text(row.value.choice.value), "z-first");
    }
    if index == 2 {
        assert_eq!(
            list(row.value.tags.data, row.value.tags.len)
                .iter()
                .map(|v| text(*v))
                .collect::<Vec<_>>(),
            ["z-first", "a-second"]
        );
    }
    if index == 1 || index == 3 {
        assert_eq!(
            list(row.probabilities.data, row.probabilities.len)
                .iter()
                .map(|v| text(v.name))
                .collect::<Vec<_>>(),
            ["z-first", "a-second"]
        );
    }
    if index == 3 {
        assert_eq!(row.value.score, 0.1);
        assert_eq!(text(row.nearest.value), "z-first");
    }
}
pub(super) fn row(row: &Row, index: usize, file_mode: bool) {
    match row {
        Row::Atomic(row) => assert_atomic(row, index, file_mode),
        Row::Original(row) => {
            assert!(index == 4 || index == 5);
            assert_eq!(row.index.value, 0);
            if index == 5 {
                assert_eq!(row.probability.value, 0.9);
            }
        }
        Row::Find(row) => {
            assert_eq!(row.selected.index.value, 0);
            assert_eq!(row.candidates.len, 2);
            assert_eq!(
                list(row.candidates.data, row.candidates.len)[0].probability,
                0.9
            );
        }
        Row::Annotation(row) => {
            let members = list(row.members.data, row.members.len);
            assert_eq!(
                members.iter().map(|m| text(m.name)).collect::<Vec<_>>(),
                ["z_first", "a_second"]
            );
            assert_eq!(members[0].state, 1);
            assert_eq!(text(members[1].value.choice.value), "z-first");
        }
        Row::Recognition(row) => assert_recognition(row, file_mode),
        Row::Relations(row) => {
            let edges = list(row.edges.data, row.edges.len);
            assert_eq!(edges.len(), 2);
            assert_eq!(text(edges[0].relation), "supports");
            assert_eq!(edges[0].probability, 0.9);
            assert_eq!(row.inputs.len, 2);
        }
    }
}

fn assert_recognition(row: &crate::current::private::CurrentRecognitionV1, file_mode: bool) {
    assert_eq!(row.relations_present, 0);
    assert_eq!(row.relations.len, 0);
    assert!(row.relations.data.is_null());
    let entities = list(row.entities.data, row.entities.len);
    assert!(!entities.is_empty());
    if !file_mode {
        assert_eq!(entities.len(), 3);
        assert_eq!(text(entities[0].text), "Maria Chen");
        assert_eq!((entities[0].start, entities[0].end), (0, 10));
    }
    for entity in entities {
        assert_eq!(entity.end - entity.start, entity.length);
    }
}
