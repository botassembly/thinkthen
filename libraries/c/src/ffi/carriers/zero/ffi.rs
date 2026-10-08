//! Bounded whole-union initialization at the C unsafe boundary.
use super::{
    AnswerDataV1, DecideValueDataV1, MemberDataV1, MemberValueDataV1, ObservationDataV1,
    ObservationIdentityDataV1, ObservedProbabilitiesDataV1, QuestionObservationDataV1,
    RelationAnswerDataV1, RowObservationDataV1,
};
impl Default for DecideValueDataV1 {
    #[allow(
        unsafe_code,
        reason = "initialize the entire C union with zero-valid carrier storage"
    )]
    fn default() -> Self {
        // SAFETY: every arm contains only integers, floats, nullable raw pointers,
        // and repr(C) carriers recursively composed of those types. No reference,
        // enum, NonNull or nonzero integer occurs. Zero is valid for every arm.
        // The parent starts with discriminator zero and controls subsequent reads.
        unsafe { std::mem::zeroed() }
    }
}
impl Default for AnswerDataV1 {
    #[allow(
        unsafe_code,
        reason = "initialize the entire C union with zero-valid carrier storage"
    )]
    fn default() -> Self {
        // SAFETY: every arm contains only integers, floats, nullable raw pointers,
        // and repr(C) carriers recursively composed of those types. No reference,
        // enum, NonNull or nonzero integer occurs. Zero is valid for every arm.
        // The parent starts with discriminator zero and controls subsequent reads.
        unsafe { std::mem::zeroed() }
    }
}
impl Default for MemberValueDataV1 {
    #[allow(
        unsafe_code,
        reason = "initialize the entire C union with zero-valid carrier storage"
    )]
    fn default() -> Self {
        // SAFETY: every arm contains only integers, floats, nullable raw pointers,
        // and repr(C) carriers recursively composed of those types. No reference,
        // enum, NonNull or nonzero integer occurs. Zero is valid for every arm.
        // The parent starts with discriminator zero and controls subsequent reads.
        unsafe { std::mem::zeroed() }
    }
}
impl Default for MemberDataV1 {
    #[allow(
        unsafe_code,
        reason = "initialize the entire C union with zero-valid carrier storage"
    )]
    fn default() -> Self {
        // SAFETY: every arm contains only integers, floats, nullable raw pointers,
        // and repr(C) carriers recursively composed of those types. No reference,
        // enum, NonNull or nonzero integer occurs. Zero is valid for every arm.
        // The parent starts with discriminator zero and controls subsequent reads.
        unsafe { std::mem::zeroed() }
    }
}
impl Default for RelationAnswerDataV1 {
    #[allow(
        unsafe_code,
        reason = "initialize the entire C union with zero-valid carrier storage"
    )]
    fn default() -> Self {
        // SAFETY: every arm contains only integers, floats, nullable raw pointers,
        // and repr(C) carriers recursively composed of those types. No reference,
        // enum, NonNull or nonzero integer occurs. Zero is valid for every arm.
        // The parent starts with discriminator zero and controls subsequent reads.
        unsafe { std::mem::zeroed() }
    }
}
impl Default for ObservationIdentityDataV1 {
    #[allow(
        unsafe_code,
        reason = "initialize the entire C union with zero-valid carrier storage"
    )]
    fn default() -> Self {
        // SAFETY: every arm contains only integers, floats, nullable raw pointers,
        // and repr(C) carriers recursively composed of those types. No reference,
        // enum, NonNull or nonzero integer occurs. Zero is valid for every arm.
        // The parent starts with discriminator zero and controls subsequent reads.
        unsafe { std::mem::zeroed() }
    }
}
impl Default for ObservedProbabilitiesDataV1 {
    #[allow(
        unsafe_code,
        reason = "initialize the entire C union with zero-valid carrier storage"
    )]
    fn default() -> Self {
        // SAFETY: every arm contains only integers, floats, nullable raw pointers,
        // and repr(C) carriers recursively composed of those types. No reference,
        // enum, NonNull or nonzero integer occurs. Zero is valid for every arm.
        // The parent starts with discriminator zero and controls subsequent reads.
        unsafe { std::mem::zeroed() }
    }
}
impl Default for QuestionObservationDataV1 {
    #[allow(
        unsafe_code,
        reason = "initialize the entire C union with zero-valid carrier storage"
    )]
    fn default() -> Self {
        // SAFETY: every arm contains only integers, floats, nullable raw pointers,
        // and repr(C) carriers recursively composed of those types. No reference,
        // enum, NonNull or nonzero integer occurs. Zero is valid for every arm.
        // The parent starts with discriminator zero and controls subsequent reads.
        unsafe { std::mem::zeroed() }
    }
}
impl Default for RowObservationDataV1 {
    #[allow(
        unsafe_code,
        reason = "initialize the entire C union with zero-valid carrier storage"
    )]
    fn default() -> Self {
        // SAFETY: every arm contains only integers, floats, nullable raw pointers,
        // and repr(C) carriers recursively composed of those types. No reference,
        // enum, NonNull or nonzero integer occurs. Zero is valid for every arm.
        // The parent starts with discriminator zero and controls subsequent reads.
        unsafe { std::mem::zeroed() }
    }
}
impl Default for ObservationDataV1 {
    #[allow(
        unsafe_code,
        reason = "initialize the entire C union with zero-valid carrier storage"
    )]
    fn default() -> Self {
        // SAFETY: every arm contains only integers, floats, nullable raw pointers,
        // and repr(C) carriers recursively composed of those types. No reference,
        // enum, NonNull or nonzero integer occurs. Zero is valid for every arm.
        // The parent starts with discriminator zero and controls subsequent reads.
        unsafe { std::mem::zeroed() }
    }
}
