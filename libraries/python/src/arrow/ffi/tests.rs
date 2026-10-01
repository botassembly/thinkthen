//! The frame writer over a hand-built frame. It is a child of `ffi` because
//! only that module can build an `Imported` with no producer behind it.

use std::sync::Arc;

use super::super::read::{Batch, Frame};
use super::super::write::{ArrayNode, Cells, Output, frame};
use super::{ArrowArray, EMPTY_ARRAY, EMPTY_SCHEMA, Imported};

/// One output batch's columns: an alias as its offset and rows, a new
/// count column as its values.
fn cuts(batch: &ArrayNode) -> Vec<String> {
    let ArrayNode::Owned(root) = batch else {
        panic!("a frame batch's root is its own");
    };
    let one = |child: &ArrayNode| match child {
        ArrayNode::Alias(alias) => {
            format!("rows {}+{}", alias.array.offset, alias.array.length)
        }
        ArrayNode::Owned(owned) => {
            let values = owned.buffers[1].as_deref().unwrap_or_default();
            let values: Vec<i64> = values
                .chunks_exact(8)
                .map(|word| i64::from_le_bytes(word.try_into().unwrap()))
                .collect();
            format!("{values:?}")
        }
    };
    root.children.iter().map(one).collect()
}

/// The frame door over a hand-built two-batch stream. Each batch's new
/// column holds that batch's answers, and each input column keeps its
/// batch's offset and rows. Regression: a cut that restarts at row 0
/// gives the second batch the first batch's answers.
#[test]
fn each_batch_gets_its_own_rows_of_a_new_column() {
    let hold = Arc::new(Imported {
        stream: None,
        schema: EMPTY_SCHEMA,
        batches: Vec::new(),
        gated: false,
    });
    let column = ArrowArray {
        offset: 1,
        length: 9,
        ..EMPTY_ARRAY
    };
    let batch = |offset, length| Batch {
        offset,
        length,
        columns: vec![column],
    };
    let read = Frame {
        texts: Vec::new(),
        schemas: Vec::new(),
        batches: vec![batch(0, 2), batch(4, 3)],
    };
    let answers = [("n".to_owned(), Cells::Counts(vec![10, 11, 12, 13, 14]))];
    let Ok(Output::Frame(_, batches)) = frame(&hold, (Vec::new(), None), &read, &answers) else {
        panic!("the frame is written");
    };
    let cuts: Vec<Vec<String>> = batches.iter().map(cuts).collect();
    assert_eq!(
        cuts,
        [["rows 1+2", "[10, 11]"], ["rows 5+3", "[12, 13, 14]"]]
    );
}
