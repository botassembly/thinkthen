//! Attach database comments to the same declarations pgrx registers.

use pgrx::pgrx_sql_entity_graph::{PgrxSql, SqlGraphEntity, ToSql};

pub(crate) const COMPLETE_TEXT: &str = "on explicit native input records encoded as text JSON and return the complete result/2 envelope as PostgreSQL json, including final facts and started failures. NULL question or inputs returns NULL before inspecting partners; NULL settings uses defaults. Questions support native catalog references and privileged/confined @files. Evidence and image files must be read by the client. Admission and backend failures return error envelopes.";
pub(crate) const REQUIRED_SCALAR: &str =
    "A required NULL operand returns NULL before inspecting partners, reading files or sending. ";
pub(crate) const REQUIRED_TABLE: &str =
    "A required NULL operand gives no rows before inspecting partners, reading files or sending. ";

pub(crate) fn scalar_text() -> String {
    ["Judge text with a literal/JSON question or privileged/confined @file. ", REQUIRED_SCALAR, "NULL optional settings uses defaults. Evidence files must be read by the client. Failures raise SQL errors."].concat()
}
pub(crate) const SCALAR_IMAGES: &str = "NULL question or collection returns NULL before inspecting partners; NULL text is absent and NULL settings uses defaults. Invalid collections and backend failures raise SQL errors. Questions may use privileged/confined @files; image files must be read by the client.";
pub(crate) fn keyed_text() -> String {
    [REQUIRED_TABLE, "NULL settings uses defaults. Questions may use privileged/confined @files. Evidence files must be read by the client; failures raise SQL errors."].concat()
}

pub(crate) type SqlResult = Result<String, Box<dyn std::error::Error + Send + Sync>>;

pub(crate) fn sql(entity: &SqlGraphEntity, context: &PgrxSql, description: &str) -> SqlResult {
    let SqlGraphEntity::Function(function) = entity else {
        return Err("a function description requires a registered function".into());
    };
    let mut sql = function.to_sql(context)?;
    let wrapper = format!("{}_wrapper", function.unaliased_name).replace('\'', "''");
    let description = description.replace('\'', "''");
    sql.push_str(&format!(
        r#"
DO $thinkthen_description$
DECLARE signature text;
BEGIN
    SELECT p.oid::regprocedure::text INTO STRICT signature
    FROM pg_catalog.pg_proc p
    JOIN pg_catalog.pg_depend d
      ON d.objid = p.oid AND d.classid = 'pg_catalog.pg_proc'::regclass
    JOIN pg_catalog.pg_extension e
      ON e.oid = d.refobjid AND d.refclassid = 'pg_catalog.pg_extension'::regclass
    WHERE e.extname = 'thinkthen' AND d.deptype = 'e' AND p.prosrc = '{wrapper}';
    EXECUTE format('COMMENT ON FUNCTION %s IS %L', signature, '{description}');
END
$thinkthen_description$;
"#
    ));
    Ok(sql)
}

macro_rules! describe {
    ($description:expr; [$($options:tt)*]; $(#[$attribute:meta])* fn $name:ident $($body:tt)*) => {
        $(#[$attribute])*
        #[pg_extern($($options)*, sql = $name::sql)]
        fn $name $($body)*

        mod $name {
            pub(super) fn sql(
                entity: &pgrx::pgrx_sql_entity_graph::SqlGraphEntity,
                context: &pgrx::pgrx_sql_entity_graph::PgrxSql,
            ) -> crate::descriptions::SqlResult {
                crate::descriptions::sql(entity, context, &$description)
            }
        }
    };
}

pub(crate) use describe;
