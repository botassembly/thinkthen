//! Map neutral variants into the existing JSON proof seam without numeric narrowing.
use thinkthen::RequestValueTree;

pub(crate) fn render(value: &RequestValueTree) -> Result<String, String> {
    Ok(match value {
        RequestValueTree::Map(fields) => {
            let values = fields
                .iter()
                .filter(|(_, value)| !absent(value))
                .map(|(name, value)| {
                    Ok(format!("{}:{}", quote(name)?, render(value)?))
                })
                .collect::<Result<Vec<_>, String>>()?;
            format!("{{{}}}", values.join(","))
        }
        RequestValueTree::List(values) => {
            let values = values.iter().map(render).collect::<Result<Vec<_>, _>>()?;
            format!("[{}]", values.join(","))
        }
        RequestValueTree::String(value) => quote(value)?,
        RequestValueTree::Number(value) => value.clone(),
        RequestValueTree::Bool(value) => value.to_string(),
        RequestValueTree::Null => "null".into(),
        RequestValueTree::Absent => {
            return Err("an absent list member cannot enter the JSON proof seam".into());
        }
    })
}

fn absent(value: &RequestValueTree) -> bool {
    matches!(value, RequestValueTree::Absent)
        || matches!(value, RequestValueTree::List(values)
            if !values.is_empty() && values.iter().all(|value| matches!(value, RequestValueTree::Absent)))
}

fn quote(value: &str) -> Result<String, String> {
    serde_json::to_string(value).map_err(|error| error.to_string())
}
