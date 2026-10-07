//! An outside-in consumer of the native client-reader/keyed SQL workaround.

use std::error::Error;
#[path = "../../../crates/thinkthen/src/test_deadline/child.rs"]
mod child;

use serde_json::{Value, json};
use thinkthen::{ReaderOptions, SourceUnit, read_files};

const ORIGINAL: &str = "café 😀\r\n\r\nAda approves Acme.\r\n";

fn quote(text: &str) -> String {
    format!("'{}'", text.replace('\'', "''"))
}

fn number(value: &Value, field: &str) -> Result<usize, Box<dyn Error>> {
    value
        .get(field)
        .and_then(Value::as_u64)
        .and_then(|n| usize::try_from(n).ok())
        .ok_or_else(|| format!("missing integer {field}").into())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args().skip(1);
    let socket = args.next().ok_or("expected owned PostgreSQL socket")?;
    let path = args.next().ok_or("expected owned client fixture path")?;
    std::fs::write(&path, ORIGINAL)?;
    // Duplicate occurrences retain different host keys, even for identical text.
    let records = read_files(
        [path.clone(), path.clone()],
        ReaderOptions {
            unit: SourceUnit::File,
            window: None,
        },
    )?
    .collect::<Result<Vec<_>, _>>()?;
    assert_eq!(records.len(), 2);
    let mut inserts = String::new();
    for (at, source) in records.iter().enumerate() {
        assert_eq!(source.record, ORIGINAL);
        assert_eq!((source.first_line, source.last_line), (1, 3));
        inserts.push_str(&format!(
            "INSERT INTO source_rows VALUES ({}, {}, {}, {}, {});",
            at + 1,
            quote(&source.record),
            quote(&source.file),
            source.first_line,
            source.last_line,
        ));
    }
    let script = format!(
        r#"
SET thinkthen.cache = 'off';
CREATE TEMP TABLE source_rows(id integer, record text, file text, first_line integer, last_line integer);
{inserts}
SELECT jsonb_agg(jsonb_build_object('id',s.id,'rank',r.rank,'probability',r.probability,'file',s.file) ORDER BY r.rank)
 FROM thinkthen_rank('Which needs attention?', (SELECT jsonb_object_agg(id,record) FROM source_rows)) r
 JOIN source_rows s ON r.key=s.id::text;
WITH found AS (SELECT thinkthen_find('Which source?', (SELECT array_agg(record ORDER BY id) FROM source_rows)) AS value)
 SELECT jsonb_build_object('id',s.id,'file',s.file,'record',s.record) FROM found
 JOIN source_rows s ON s.id=(value->>'index')::integer+1;
CREATE TEMP TABLE entity_rows AS SELECT row_number() OVER (ORDER BY s.id,n.start)::bigint AS id,
 s.id AS source_id,n.text,n.start,n.end FROM source_rows s,
 LATERAL thinkthen_recognize(s.record,ARRAY['Person']) n;
SELECT jsonb_agg(jsonb_build_object('source_id',source_id,'text',text,'start',start,'end',"end") ORDER BY id) FROM entity_rows;
SELECT jsonb_agg(jsonb_build_array(a.source_id,b.source_id,a.text,b.text,sa.file,sb.file,e.probability) ORDER BY a.source_id,b.source_id)
 FROM thinkthen_relate('SELECT id,text,CASE WHEN text=''Ada approves'' THEN ''person'' ELSE ''organization'' END FROM entity_rows WHERE start > 0 ORDER BY id',ARRAY['supports=person:organization']) e
 JOIN entity_rows a ON a.id=e.source JOIN entity_rows b ON b.id=e.target
 JOIN source_rows sa ON sa.id=a.source_id JOIN source_rows sb ON sb.id=b.source_id;
"#
    );
    let output = child::command("psql", &[])
        .args([
            "-X",
            "-q",
            "-At",
            "-v",
            "ON_ERROR_STOP=1",
            "-h",
            &socket,
            "-U",
            "postgres",
            "-d",
            "postgres",
            "-c",
            &script,
        ])
        .output()?;
    if !output.status.success() {
        return Err(format!(
            "{}\n{}",
            String::from_utf8(output.stdout)?,
            String::from_utf8(output.stderr)?
        )
        .into());
    }
    let results = String::from_utf8(output.stdout)?
        .lines()
        .map(serde_json::from_str::<Value>)
        .collect::<Result<Vec<_>, _>>()?;
    verify(&results, &records, &path)
}

fn verify(
    results: &[Value],
    records: &[thinkthen::SourceRecord<String>],
    path: &str,
) -> Result<(), Box<dyn Error>> {
    assert_eq!(results.len(), 4);
    assert_eq!(
        results.first(),
        Some(&json!([
            {"id":1,"rank":1,"probability":0.9,"file":path},
            {"id":2,"rank":2,"probability":0.9,"file":path},
        ]))
    );
    assert_eq!(
        results.get(1),
        Some(&json!({"id":1,"file":path,"record":ORIGINAL}))
    );
    assert_eq!(
        results.get(2),
        Some(&json!([
            {"source_id":1,"text":"café 😀","start":0,"end":6},
            {"source_id":1,"text":"Ada approves","start":10,"end":22},
            {"source_id":1,"text":"Acme.","start":23,"end":28},
            {"source_id":2,"text":"café 😀","start":0,"end":6},
            {"source_id":2,"text":"Ada approves","start":10,"end":22},
            {"source_id":2,"text":"Acme.","start":23,"end":28},
        ]))
    );
    for (entity, line) in results
        .get(2)
        .and_then(Value::as_array)
        .ok_or("missing entities")?
        .iter()
        .zip([1, 3, 3, 1, 3, 3])
    {
        let source = records
            .get(
                number(entity, "source_id")?
                    .checked_sub(1)
                    .ok_or("zero source id")?,
            )
            .ok_or("unknown source id")?;
        assert_eq!(
            source.span_lines(number(entity, "start")?, number(entity, "end")?)?,
            (line, line)
        );
    }
    assert_eq!(
        results.get(3),
        Some(&json!([
            [1, 1, "Ada approves", "Acme.", path, path, 0.9],
            [1, 2, "Ada approves", "Acme.", path, path, 0.9],
            [2, 1, "Ada approves", "Acme.", path, path, 0.9],
            [2, 2, "Ada approves", "Acme.", path, path, 0.9],
        ]))
    );
    Ok(())
}
