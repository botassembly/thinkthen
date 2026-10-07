//! Additive author/declaration descriptors; existing v1 layouts stay frozen.
use super::{OptionalStringV1, OptionalU64V1, StringV1, StringsV1};
layout!(InputPropertyV1 {
    name: StringV1,
    kind: u32
});
layout!(InputPropertiesV1 { data: *const InputPropertyV1, len: usize });
layout!(InputDeclarationV1 {
    kind: u32,
    properties: InputPropertiesV1,
    required: StringsV1
});
layout!(QuestionAuthorV1 {
    name: OptionalStringV1,
    wording_version: OptionalU64V1,
    item_schema: InputDeclarationV1,
    context_schema: InputDeclarationV1,
});
