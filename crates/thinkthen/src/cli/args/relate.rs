use clap::Args;

use super::Common;

#[derive(Args, Debug)]
pub(crate) struct RelateArguments {
    /// Relation rules as NAME=SOURCE_KIND:TARGET_KIND, bare NAME, or one @FILE.
    #[arg(value_name = "RELATION", required = true)]
    pub(crate) relations: Vec<String>,

    /// Treat every inline relation as unordered.
    #[arg(long)]
    pub(crate) either: bool,

    /// Keep edges whose model probability reaches this cut. [default: 0.5]
    #[arg(long, value_name = "T", allow_negative_numbers = true)]
    pub(crate) threshold: Option<String>,

    /// Read each structured entity's kind from this RFC 6901 pointer.
    #[arg(long, value_name = "POINTER")]
    pub(crate) kind_field: Option<String>,

    /// Split relation plans before a request exceeds N bytes. [default: 96000]
    #[arg(
        long,
        value_name = "N",
        hide_short_help = true,
        allow_negative_numbers = true
    )]
    pub(crate) max_request_bytes: Option<String>,

    #[command(flatten)]
    pub(crate) common: Common,
}
