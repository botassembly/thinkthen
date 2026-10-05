//! The reviews' exit-139 probes, pinned: a malformed view or offsets
//! buffer comes back as a refusal, never as a read past a buffer. The
//! arrays are hand-built over real, small buffers in the interface's
//! layouts, and each refusal pins its sentence, so a loosened check that
//! refuses for another reason fails.

use std::ffi::c_void;

use super::super::ffi::{ArrowArray, EMPTY_ARRAY};
use super::super::memory::Readable;
use super::{Text, borrow};

fn array_of(held: &[*const c_void], length: i64) -> ArrowArray {
    ArrowArray {
        length,
        n_buffers: held.len() as i64,
        buffers: held.as_ptr().cast_mut(),
        ..EMPTY_ARRAY
    }
}

fn pointers(buffers: &[&[u8]]) -> Vec<*const c_void> {
    buffers.iter().map(|one| one.as_ptr().cast()).collect()
}

fn one_view(size: u32, index: u32, offset: u32) -> Vec<u8> {
    let mut one = [0_u8; 16];
    one[0..4].copy_from_slice(&size.to_le_bytes());
    one[8..12].copy_from_slice(&index.to_le_bytes());
    one[12..16].copy_from_slice(&offset.to_le_bytes());
    one.to_vec()
}

fn snapshot() -> Readable {
    Readable::snapshot().expect("the memory map reads")
}

fn read(array: &ArrowArray, text: Text, skip: usize, rows: usize) -> Result<Vec<String>, String> {
    borrow(&(), &snapshot(), array, text, skip, rows).map(|texts| {
        texts
            .into_iter()
            .map(|text| text.unwrap().to_owned())
            .collect()
    })
}

/// A view column `[validity, views, data.., sizes]` with its sizes set by hand.
fn views_raw(
    views: &[u8],
    data: &[&[u8]],
    sizes: &[i64],
    rows: usize,
) -> Result<Vec<String>, String> {
    let sizes: Vec<u8> = sizes.iter().flat_map(|one| one.to_le_bytes()).collect();
    let mut table = vec![b"".as_slice(), views];
    table.extend_from_slice(data);
    table.push(&sizes);
    let held = pointers(&table);
    read(&array_of(&held, rows as i64), Text::View, 0, rows)
}

fn views_over(views: &[u8], data: &[&[u8]], rows: usize) -> Result<Vec<String>, String> {
    let sizes: Vec<i64> = data.iter().map(|one| one.len() as i64).collect();
    views_raw(views, data, &sizes, rows)
}

fn utf8_at(offsets: &[i64], values: *const u8, text: Text) -> Result<Vec<String>, String> {
    let bytes: Vec<u8> = match text {
        Text::LargeUtf8 => offsets.iter().flat_map(|one| one.to_le_bytes()).collect(),
        _ => offsets
            .iter()
            .flat_map(|one| (*one as i32).to_le_bytes())
            .collect(),
    };
    let held = [std::ptr::null(), bytes.as_ptr().cast(), values.cast()];
    let rows = offsets.len() - 1;
    read(&array_of(&held, rows as i64), text, 0, rows)
}

fn utf8_over(offsets: &[i64], values: &[u8]) -> Result<Vec<String>, String> {
    utf8_at(offsets, values.as_ptr(), Text::Utf8)
}

fn refusal<T: std::fmt::Debug>(outcome: Result<T, String>) -> String {
    outcome.expect_err("the shape must be refused, not read")
}

#[test]
fn a_view_past_its_declared_length_is_refused() {
    let data = vec![b'x'; 100];
    for (size, offset) in [(100, 60_000_000), (13, 200), (13, 90)] {
        assert_eq!(
            refusal(views_over(&one_view(size, 0, offset), &[&data], 1)),
            "a string view reaches past the length its data buffer declares",
            "{size} at {offset}"
        );
    }
    assert_eq!(
        refusal(views_over(&one_view(0x4000_0000, 0, 0), &[&data], 1)),
        "a row names a size no text column carries"
    );
}

#[test]
fn a_view_cannot_name_the_sizes_buffer() {
    let data = vec![b'x'; 100];
    for index in [1, 5] {
        assert_eq!(
            refusal(views_over(&one_view(13, index, 0), &[&data], 1)),
            "a string view points past its data buffers"
        );
    }
}

#[test]
fn a_utf8_row_past_the_final_offset_is_refused() {
    for offsets in [[0, 200, 3].as_slice(), &[3, 1], &[-5, 3]] {
        assert_eq!(
            refusal(utf8_over(offsets, b"abc")),
            "the column's offsets are reversed or negative"
        );
    }
    assert_eq!(
        refusal(utf8_over(&[0, 60_000_000], b"abc")),
        "a row names a size no text column carries"
    );
}

#[test]
fn a_declared_data_extent_past_the_cap_is_refused_and_a_big_slice_borrows() {
    assert_eq!(
        refusal(views_raw(
            &one_view(13, 0, u32::MAX),
            &[b"abc".as_slice()],
            &[i64::MAX],
            1
        )),
        "a string view's data buffer declares more bytes than a text column carries"
    );
    let past = "the column's offsets name more bytes than a text column carries";
    assert_eq!(
        refusal(utf8_at(
            &[i64::MAX - 13, i64::MAX],
            b"abc".as_ptr(),
            Text::LargeUtf8
        )),
        past
    );
    assert_eq!(
        refusal(utf8_over(
            &[i64::from(i32::MAX) - 13, i64::from(i32::MAX)],
            b"abc"
        )),
        past
    );
    assert_eq!(
        refusal(utf8_at(
            &[1 << 40, (1 << 40) + 13],
            b"abc".as_ptr(),
            Text::LargeUtf8
        )),
        past
    );
    // A slice's first row may start partway into its buffer, and a column
    // past the 4 MiB row cap still borrows.
    assert_eq!(
        utf8_over(&[5, 9], b"-----SEEN"),
        Ok(vec!["SEEN".to_owned()])
    );
    let values = vec![b'a'; 5 << 20];
    let rows: Vec<i64> = (0..=5).map(|one| one << 20).collect();
    assert_eq!(
        utf8_at(&rows, values.as_ptr(), Text::LargeUtf8).map(|texts| texts.len()),
        Ok(5)
    );
    assert_eq!(
        views_over(&one_view(13, 0, (5 << 20) - 13), &[&values], 1).map(|texts| texts.len()),
        Ok(1)
    );
}

#[test]
fn a_buffer_table_far_longer_than_a_text_column_is_refused() {
    let offsets = [0_i32, 3].map(i32::to_le_bytes).concat();
    let held = pointers(&[b"".as_slice(), &offsets, b"abc"]);
    let mut outer = array_of(&held, 1);
    outer.n_buffers = 1 << 40;
    assert_eq!(
        refusal(read(&outer, Text::Utf8, 0, 1)),
        "the column's buffer table names more buffers than a text column carries"
    );
}

#[test]
fn null_tables_and_row_claims_are_refused() {
    let outer = ArrowArray {
        length: 1,
        n_buffers: 3,
        ..EMPTY_ARRAY
    };
    assert_eq!(
        refusal(read(&outer, Text::View, 0, 1)),
        "the column carries no buffer table"
    );
    let values = b"abc".to_vec();
    let held = [std::ptr::null(), std::ptr::null(), values.as_ptr().cast()];
    assert_eq!(
        refusal(read(&array_of(&held, 1), Text::Utf8, 0, 1)),
        "the string array lacks its offsets or its values buffer"
    );
    let held = pointers(&[b"".as_slice(), &values]);
    assert_eq!(
        refusal(read(&array_of(&held, 1), Text::View, 0, 1)),
        "the string-view array lacks its views or its buffer-sizes buffer"
    );
    let offsets = [0_i32, 3].map(i32::to_le_bytes).concat();
    let held = pointers(&[b"".as_slice(), &offsets, &values]);
    let claims = [
        (
            1_i64 << 40,
            0,
            "the column claims more rows than any text column carries",
        ),
        (
            1,
            1 << 40,
            "the column claims more rows than any text column carries",
        ),
        (
            -1,
            0,
            "the column's length or offset is negative; a malformed array is refused, not read",
        ),
    ];
    for (length, offset, sentence) in claims {
        let outer = ArrowArray {
            offset,
            ..array_of(&held, length)
        };
        let skip = offset.max(0) as usize;
        assert_eq!(
            refusal(read(&outer, Text::Utf8, skip, length.max(0) as usize)),
            sentence
        );
    }
}

#[test]
fn a_conformant_view_column_round_trips() {
    let first = b"alpha runs longer than twentyfour".to_vec();
    let second = b"beta also runs long past twelve".to_vec();
    let mut inline = one_view(5, 0, 0);
    inline[4..9].copy_from_slice(b"gamma");
    let views = [one_view(24, 0, 3), inline, one_view(21, 1, 4)].concat();
    assert_eq!(
        views_over(&views, &[&first, &second], 3),
        Ok(vec![
            "ha runs longer than twen".to_owned(),
            "gamma".to_owned(),
            " also runs long past ".to_owned()
        ])
    );
}

#[cfg(target_os = "linux")]
mod guarded {
    use super::super::super::ffi::testing::{Guarded, PastEnd};
    use super::super::super::ffi::{ArrowSchema, EMPTY_SCHEMA};
    use super::super::super::write::{BAD_METADATA, SchemaNode, metadata};
    use super::super::{BAD_TEXT, MAX_TEXT, UNREADABLE, c_text, text_layout};
    use super::*;

    /// R3-4, R5-6 (a), R7-2: offsets that clear the row cap and the
    /// extent cap still land in unreadable memory, and the readability
    /// check refuses them first.
    #[test]
    fn an_extent_past_readable_memory_is_refused() {
        let (_region, abc) = Guarded::ending_with(b"abc");
        for text in [Text::Utf8, Text::LargeUtf8] {
            let past =
                [[60_000_000, 60_000_001], [0, 200]].map(|offsets| utf8_at(&offsets, abc, text));
            assert_eq!(
                past.map(refusal),
                [UNREADABLE, UNREADABLE].map(str::to_owned)
            );
            assert_eq!(utf8_at(&[0, 3], abc, text), Ok(vec!["abc".to_owned()]));
        }
    }

    /// R5-6 (b): the view names data buffer 1, whose size sits past a
    /// guard page, and a view whose 13 bytes run past the readable page.
    #[test]
    fn a_sizes_buffer_shorter_than_its_count_is_refused() {
        let data = [b'x'; 100];
        let (_region, sizes) = Guarded::ending_with(&100_i64.to_le_bytes());
        let views = one_view(13, 1, 0);
        let held = [
            std::ptr::null(),
            views.as_ptr().cast(),
            data.as_ptr().cast(),
            data.as_ptr().cast(),
            sizes.cast(),
        ];
        assert_eq!(
            refusal(read(&array_of(&held, 1), Text::View, 0, 1)),
            UNREADABLE
        );
        let (_short, tail) = Guarded::ending_with(&[b'y'; 20]);
        let sizes = [200_i64, 200].map(i64::to_le_bytes).concat();
        let views = one_view(13, 1, 10);
        let held = [
            std::ptr::null(),
            views.as_ptr().cast(),
            tail.cast(),
            tail.cast(),
            sizes.as_ptr().cast(),
        ];
        assert_eq!(
            refusal(read(&array_of(&held, 1), Text::View, 0, 1)),
            UNREADABLE
        );
    }

    /// R5-6 (c): a mapping longer than its file is listed readable, and a
    /// row, a string, or a metadata blob past the file's end raised SIGBUS.
    #[test]
    fn a_file_mapping_past_the_end_of_its_file_is_unreadable() {
        let words = |parts: &[i32]| {
            parts
                .iter()
                .flat_map(|one| one.to_le_bytes())
                .collect::<Vec<u8>>()
        };
        for (backing, file) in PastEnd::each(b"abc") {
            let memory = snapshot();
            assert!(
                memory.covers(file.tail(3).addr(), 3),
                "{backing}: the file's bytes read"
            );
            assert!(
                !memory.covers(file.tail(3).addr(), 4),
                "{backing}: one byte past the file"
            );
            assert_eq!(memory.reach(file.tail(3).addr(), 100), 3, "{backing}");
            assert_eq!(
                utf8_at(&[0, 3], file.tail(3), Text::Utf8),
                Ok(vec!["abc".to_owned()]),
                "{backing}"
            );
            assert_eq!(
                refusal(utf8_at(&[0, 200], file.tail(3), Text::Utf8)),
                UNREADABLE,
                "{backing}"
            );
            assert_eq!(
                c_text(&memory, file.past().cast()),
                Err(BAD_TEXT),
                "{backing}"
            );
            assert_eq!(
                metadata(&memory, file.past().cast()),
                Err(BAD_METADATA),
                "{backing}"
            );
            let blob = file.put(&words(&[1, 64]));
            assert_eq!(
                metadata(&memory, blob.cast()),
                Err(BAD_METADATA),
                "{backing}"
            );
        }
    }

    /// R5-6 (d): a buffer table and a schema's child table each shorter
    /// than their counts, against a guard page.
    #[test]
    fn a_pointer_table_shorter_than_its_count_is_refused() {
        let offsets = [0_i32, 3].map(i32::to_le_bytes).concat();
        let values = b"abc".to_vec();
        let slots = [0, offsets.as_ptr().addr()]
            .map(usize::to_le_bytes)
            .concat();
        let (_region, table) = Guarded::ending_with(&slots);
        let outer = ArrowArray {
            length: 1,
            n_buffers: 3,
            buffers: table.cast_mut().cast(),
            ..EMPTY_ARRAY
        };
        assert_eq!(refusal(read(&outer, Text::Utf8, 0, 1)), UNREADABLE);
        let whole = [0, offsets.as_ptr().addr(), values.as_ptr().addr()]
            .map(usize::to_le_bytes)
            .concat();
        let (_region, table) = Guarded::ending_with(&whole);
        let outer = ArrowArray {
            buffers: table.cast_mut().cast(),
            ..outer
        };
        assert_eq!(read(&outer, Text::Utf8, 0, 1), Ok(vec!["abc".to_owned()]));
        let mut leaf = ArrowSchema {
            format: c"u".as_ptr(),
            ..EMPTY_SCHEMA
        };
        let one = (&raw mut leaf).addr().to_le_bytes();
        let (_region, children) = Guarded::ending_with(&one);
        let root = ArrowSchema {
            format: c"+s".as_ptr(),
            n_children: 2,
            children: children.cast_mut().cast(),
            ..EMPTY_SCHEMA
        };
        assert_eq!(
            SchemaNode::copy(&snapshot(), &root, 0).map(|_| ()),
            Err(UNREADABLE.to_owned())
        );
        let root = ArrowSchema {
            n_children: 1,
            ..root
        };
        assert!(SchemaNode::copy(&snapshot(), &root, 0).is_ok());
        let (_region, cut) = Guarded::ending_with(&vec![0_u8; size_of::<ArrowSchema>() - 8]);
        assert_eq!(
            SchemaNode::copy(&snapshot(), cut.cast(), 0).map(|_| ()),
            Err(UNREADABLE.to_owned())
        );
    }

    /// A 10-row slice of a large column is checked for its own rows only:
    /// every byte outside the rows read here is unreadable.
    #[test]
    fn only_the_bytes_the_selected_rows_read_are_checked() {
        let region = Guarded::new();
        let page = region.place(0, b"alpha runs past twelve");
        let values = page.wrapping_sub(Guarded::PAGE);
        let at = Guarded::PAGE as i64;
        assert_eq!(
            utf8_at(&[at, at + 5], values, Text::Utf8),
            Ok(vec!["alpha".to_owned()])
        );
        let offsets = [0, at, at + 5, at + 10, at + 9000]
            .map(|one| (one as i32).to_le_bytes())
            .concat();
        let held = [std::ptr::null(), offsets.as_ptr().cast(), values.cast()];
        let outer = ArrowArray {
            offset: 1,
            ..array_of(&held, 2)
        };
        assert_eq!(
            read(&outer, Text::Utf8, 1, 2),
            Ok(vec!["alpha".to_owned(), " runs".to_owned()])
        );
        let views = one_view(22, 0, Guarded::PAGE as u32);
        let sizes = (64_i64 << 20).to_le_bytes();
        let held = [
            std::ptr::null(),
            views.as_ptr().cast(),
            values.cast(),
            sizes.as_ptr().cast(),
        ];
        assert_eq!(
            read(&array_of(&held, 1), Text::View, 0, 1),
            Ok(vec!["alpha runs past twelve".to_owned()])
        );
    }

    /// A format or name that runs into a guard page, or has no NUL within
    /// the cap, is refused. One that ends at the page's edge reads.
    #[test]
    fn a_producer_string_is_read_only_inside_readable_memory() {
        let (_region, open) = Guarded::ending_with(b"abc");
        assert_eq!(c_text(&snapshot(), open.cast()), Err(BAD_TEXT));
        let schema = ArrowSchema {
            format: open.cast(),
            ..EMPTY_SCHEMA
        };
        assert_eq!(text_layout(&snapshot(), &schema), Err(BAD_TEXT.to_owned()));
        let (_region, closed) = Guarded::ending_with(b"u\0");
        let schema = ArrowSchema {
            format: closed.cast(),
            ..EMPTY_SCHEMA
        };
        assert_eq!(text_layout(&snapshot(), &schema), Ok(Text::Utf8));
        let long = vec![b'x'; MAX_TEXT + 8];
        assert_eq!(c_text(&snapshot(), long.as_ptr().cast()), Err(BAD_TEXT));
    }
}
