//! Pointer/lifetime regressions for owned carriers and private native projections.
#![allow(
    unsafe_code,
    reason = "tests obey the C buffer and owner lifetime contract"
)]
#![deny(unsafe_op_in_unsafe_fn)]
#[path = "layout/ffi.rs"]
mod layout;
#[path = "native.rs"]
mod native;
use super::{
    carriers::{
        ChoiceV1, ChoicesV1, ContentV1, ImagesV1, MemberSpecV1, MemberSpecsV1, OptionalContentV1,
        OptionalDoubleV1, OptionalStringV1, QuestionSpecV1, RecordV1, SourceSpecV1, StringV1,
        StringsV1,
    },
    images, read,
};
use crate::{
    Door,
    current::{self, QuestionHandle, SourceHandle, Storage},
    failures::Held,
};
use thinkthen::Engine;

pub(super) fn door(base: &str) -> Door {
    Door(Held::new(
        Engine::builder()
            .base_url(base)
            .expect("loopback")
            .api_key("sk-c-door-loopback")
            .expect("fake key")
            .no_cache()
            .max_retries(0)
            .build()
            .expect("engine"),
    ))
}
pub(super) fn string(text: &str) -> StringV1 {
    StringV1 {
        data: text.as_ptr().cast(),
        len: text.len(),
    }
}
pub(super) fn content(text: &str) -> ContentV1 {
    ContentV1 {
        kind: 1,
        data: string(text),
    }
}
pub(super) fn question(door: &Door, spec: QuestionSpecV1) -> Box<QuestionHandle> {
    let mut out = std::ptr::null_mut();
    // SAFETY: all local descriptors and writable out live through return.
    assert_eq!(
        unsafe { super::thinkthen_question_new(door, &spec, &mut out) },
        0
    );
    // SAFETY: successful constructor returned one Rust allocation, owned once here.
    unsafe { Box::from_raw(out) }
}
pub(super) fn source(door: &Door, records: &[RecordV1]) -> Box<SourceHandle> {
    let mut out = std::ptr::null_mut();
    // SAFETY: descriptors and output have exactly the advertised extents.
    assert_eq!(
        unsafe { super::thinkthen_source_records(door, records.as_ptr(), records.len(), &mut out) },
        0
    );
    // SAFETY: successful constructor returned one allocation, freed once by this Box.
    unsafe { Box::from_raw(out) }
}
pub(super) fn text(value: StringV1) -> String {
    // SAFETY: test reads the view only while its owner lives.
    unsafe { read::string(value) }
        .expect("view text")
        .to_owned()
}
pub(super) fn record(text: &str) -> RecordV1 {
    RecordV1 {
        original: OptionalContentV1 {
            present: 1,
            value: content(text),
        },
        ..Default::default()
    }
}
pub(super) fn scratch(name: &str) -> std::path::PathBuf {
    static NEXT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("target/typed-tests")
        .join(format!(
            "{}-{}-{name}",
            std::process::id(),
            NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
        ));
    std::fs::create_dir_all(&path).expect("scratch");
    path
}
#[test]
fn cloned_question_members_and_descriptions_keep_order_after_all_inputs_are_freed() {
    let door = door("http://127.0.0.1:9/v1");
    let mut name = String::from("z-first");
    let mut description = String::from("{\"what\":\"private description\"}");
    let choices = [
        ChoiceV1 {
            name: string(&name),
            description: OptionalContentV1 {
                present: 1,
                value: ContentV1 {
                    kind: 2,
                    data: string(&description),
                },
            },
            ..Default::default()
        },
        ChoiceV1 {
            name: string("a-second"),
            ..Default::default()
        },
    ];
    let member = question(
        &door,
        QuestionSpecV1 {
            kind: 2,
            text: content("Which?"),
            choices: ChoicesV1 {
                data: choices.as_ptr(),
                len: choices.len(),
            },
            ..Default::default()
        },
    );
    let members = [
        MemberSpecV1 {
            name: string("z_first"),
            question: &*member,
        },
        MemberSpecV1 {
            name: string("a_second"),
            question: &*member,
        },
    ];
    let set = question(
        &door,
        QuestionSpecV1 {
            kind: 8,
            members: MemberSpecsV1 {
                data: members.as_ptr(),
                len: members.len(),
            },
            ..Default::default()
        },
    );
    name.clear();
    description.clear();
    drop(member);
    drop(door);
    let mut storage = Storage::default();
    let view = storage
        .question(set.descriptor.as_ref().expect("cloned descriptor"))
        .expect("view");
    drop(set);
    // SAFETY: view and all nested allocations are owned by storage.
    let members = unsafe { read::slice(view.members.data, view.members.len) }.expect("members");
    assert_eq!(
        members.iter().map(|m| text(m.name)).collect::<Vec<_>>(),
        ["z_first", "a_second"]
    );
    // SAFETY: nested descriptor remains in storage after every input/engine owner is gone.
    let nested = unsafe { read::reference(members[0].question) }.expect("nested question");
    let choices = unsafe { read::slice(nested.choices.data, nested.choices.len) }.expect("choices");
    assert_eq!(text(choices[0].name), "z-first");
    assert_eq!(text(choices[1].name), "a-second");
    assert_eq!(choices[0].description.value.kind, 2);
    assert_eq!(
        text(choices[0].description.value.data),
        "{\"what\":\"private description\"}"
    );
    assert_eq!(choices[1].description.present, 0);
}
#[test]
fn record_context_candidates_and_duplicate_images_are_independent_owned_snapshots() {
    let source = image_source_snapshot();
    let input = source
        .read()
        .expect("source")
        .next()
        .expect("record")
        .expect("input");
    drop(source);
    assert!(input.original.is_none());
    assert_eq!(
        input.context.as_ref().expect("context").text(),
        "private context"
    );
    assert_eq!(
        input
            .options
            .iter()
            .map(|c| c.name.as_str())
            .collect::<Vec<_>>(),
        ["duplicate", "duplicate"]
    );
    assert_eq!(
        input.options[0]
            .description
            .as_ref()
            .expect("description")
            .text(),
        "candidate description"
    );
    assert_eq!(input.options[1].weight, Some(7.5));
    let explicit = input.question_input().expect("native input");
    let thinkthen::QuestionInput::Images(explicit) = explicit else {
        panic!("images");
    };
    assert_eq!(explicit.images().len(), 2);
    assert!(explicit.text().is_none());
    let mut storage = Storage::default();
    let view = storage.images(&input.images);
    drop(input);
    drop(explicit);
    // SAFETY: immutable bytes and filenames remain owned by storage.
    let views = unsafe { read::slice(view.value.data, view.value.len) }.expect("images");
    assert_eq!(views.len(), 2);
    for image in views {
        assert_eq!((image.width, image.height, image.media), (1, 1, 2));
        assert_eq!(text(image.filename.value), "private-name.png");
        assert_eq!(
            unsafe { read::slice(image.bytes, image.bytes_len) }.expect("bytes"),
            include_bytes!("../../../../../../specification/fixtures/images/red.png")
        );
    }
}
#[test]
fn image_file_sources_keep_filename_and_absent_lines_in_owned_views() {
    let door = door("http://127.0.0.1:9/v1");
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../specification/fixtures/images/red.png"
    );
    let paths = [string(path), string(path)];
    let spec = SourceSpecV1 {
        paths: StringsV1 {
            data: paths.as_ptr(),
            len: 2,
        },
        unit: 4,
        window: 0,
    };
    let mut source = std::ptr::null_mut();
    // SAFETY: initialized paths/options/out remain live through cloning.
    assert_eq!(
        unsafe { super::thinkthen_source_files(&door, &spec, &mut source) },
        0
    );
    let source = unsafe { Box::from_raw(source) };
    let inputs = source
        .read()
        .expect("reader")
        .collect::<Result<Vec<_>, _>>()
        .expect("images");
    assert_eq!(inputs.len(), 2);
    assert_eq!(inputs[0].index, 0);
    assert_eq!(inputs[1].index, 1);
    let mut storage = Storage::default();
    let positions = inputs
        .iter()
        .map(|input| storage.position(input.position.as_ref()))
        .collect::<Vec<_>>();
    let images = storage.images(&inputs[0].images);
    drop(inputs);
    drop(source);
    drop(door);
    for position in positions {
        assert_eq!(position.present, 1);
        assert_eq!(position.value.first_line.present, 0);
        assert_eq!(position.value.last_line.present, 0);
        assert!(text(position.value.file.value).ends_with("red.png"));
    }
    let images = unsafe { read::slice(images.value.data, images.value.len) }.expect("owned images");
    assert_eq!((images[0].width, images[0].height), (1, 1));
}
#[test]
fn debug_withholds_question_record_context_image_bytes_and_filenames() {
    let door = door("http://127.0.0.1:9/v1");
    let q = question(
        &door,
        QuestionSpecV1 {
            kind: 1,
            text: content("private question marker"),
            ..Default::default()
        },
    );
    let source = source(&door, &[record("private record marker")]);
    let image = current::ImageHandle {
        native: thinkthen::ImageInput::new(
            thinkthen::ImageMedia::Png,
            include_bytes!("../../../../../../specification/fixtures/images/red.png").as_slice(),
        )
        .expect("image"),
        filename: Some("private filename marker".into()),
    };
    let debug = format!("{q:?} {source:?} {image:?}");
    for secret in [
        "private question marker",
        "private record marker",
        "private filename marker",
        "137, 80, 78, 71",
    ] {
        assert!(!debug.contains(secret));
    }
}
pub(super) fn list<T: Copy>(data: *const T, len: usize) -> Vec<T> {
    // SAFETY: caller uses views only while their owner lives; cloned elements retain that lifetime.
    unsafe { read::slice(data, len) }
        .expect("view array")
        .to_vec()
}
pub(super) fn files(door: &Door, path: &str, unit: u32) -> Box<SourceHandle> {
    let paths = [string(path)];
    let spec = SourceSpecV1 {
        paths: StringsV1 {
            data: paths.as_ptr(),
            len: 1,
        },
        unit,
        window: 0,
    };
    let mut out = std::ptr::null_mut();
    // SAFETY: counted path, descriptor, and output live through return.
    assert_eq!(
        unsafe { super::thinkthen_source_files(door, &spec, &mut out) },
        0
    );
    // SAFETY: constructor handed over one owned Rust allocation.
    unsafe { Box::from_raw(out) }
}
pub(super) fn saved_facts(door: &Door) -> String {
    let facts = crate::failures::facts(Some(&door.0));
    assert!(!facts.is_null());
    // SAFETY: the saved thread-local error remains live during this immediate clone.
    unsafe { std::ffi::CStr::from_ptr(facts) }
        .to_str()
        .expect("facts")
        .to_owned()
}

fn image_source_snapshot() -> Box<SourceHandle> {
    let door = door("http://127.0.0.1:9/v1");
    let mut bytes =
        include_bytes!("../../../../../../specification/fixtures/images/red.png").to_vec();
    let mut filename = String::from("private-name.png");
    let mut image = std::ptr::null_mut();
    // SAFETY: bytes/filename/out have their exact live extents.
    assert_eq!(
        unsafe {
            images::thinkthen_image_clone(
                &door,
                bytes.as_ptr(),
                bytes.len(),
                2,
                OptionalStringV1 {
                    present: 1,
                    value: string(&filename),
                },
                &mut image,
            )
        },
        0
    );
    bytes.fill(0);
    filename.clear();
    let images_list = [image.cast_const(), image.cast_const()];
    let mut context = String::from("private context");
    let mut description = String::from("candidate description");
    let choices = [
        ChoiceV1 {
            name: string("duplicate"),
            description: OptionalContentV1 {
                present: 1,
                value: content(&description),
            },
            ..Default::default()
        },
        ChoiceV1 {
            name: string("duplicate"),
            weight: OptionalDoubleV1 {
                present: 1,
                value: 7.5,
            },
            ..Default::default()
        },
    ];
    let source = source(
        &door,
        &[RecordV1 {
            context: OptionalContentV1 {
                present: 1,
                value: content(&context),
            },
            options: ChoicesV1 {
                data: choices.as_ptr(),
                len: 2,
            },
            images: ImagesV1 {
                data: images_list.as_ptr(),
                len: 2,
            },
            ..Default::default()
        }],
    );
    context.clear();
    description.clear();
    // SAFETY: source cloned the image; the original owner is freed once after cloning.
    unsafe {
        images::thinkthen_image_free(image);
    }
    drop(door);
    source
}
