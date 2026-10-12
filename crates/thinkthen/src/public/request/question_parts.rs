//! Native composition of existing authored question fields.
use super::{RequestDefinition, RequestFunction, RequestQuestion};
use crate::{Description, Error};
impl RequestQuestion {
    /// Compose inline fields through the existing authored definition grammar.
    /// Retains authored ordering and introduces no alternate execution path.
    /// # Errors
    /// Refuses fields that do not form an admitted authored question.
    pub fn inline(
        function: RequestFunction,
        text: Option<String>,
        fields: Vec<(String, Description)>,
    ) -> Result<Self, Error> {
        if fields.is_empty()
            && let Some(text) = &text
        {
            return Ok(Self::Text { text: text.clone() });
        }
        let mut members = Vec::new();
        for (name, value) in fields {
            let value = if function == RequestFunction::Recognize
                && name == "kinds"
                && value.as_json().starts_with('[')
            {
                let names: Vec<String> = serde_json::from_str(value.as_json())
                    .map_err(|_| Error::usage("invalid canonical request"))?;
                let fields = names
                    .iter()
                    .map(|name| Ok(format!("{}:null", quoted(name)?)))
                    .collect::<Result<Vec<_>, Error>>()?;
                format!("{{{}}}", fields.join(","))
            } else {
                value.as_json().to_owned()
            };
            members.push(format!("{}:{value}", quoted(&name)?));
        }
        let definition = if matches!(
            function,
            RequestFunction::Recognize | RequestFunction::Relate
        ) && text.is_none()
        {
            let name = if function == RequestFunction::Recognize {
                "recognize"
            } else {
                "relate"
            };
            format!("{{\"version\":1,\"{name}\":{{{}}}}}", members.join(","))
        } else {
            let name = match function {
                RequestFunction::Filter | RequestFunction::Rank | RequestFunction::Decide => {
                    "decide"
                }
                RequestFunction::Choose => "choose",
                RequestFunction::Tag => "tag",
                RequestFunction::Score => "score",
                RequestFunction::Find => "find",
                _ => return Err(Error::usage("inline question requires a literal text")),
            };
            let text = text.map_or(Ok("null".into()), |text| quoted(&text))?;
            members.insert(0, format!("\"{name}\":{text}"));
            format!("{{{}}}", members.join(","))
        };
        RequestDefinition::from_request_json(&definition).map(|value| Self::Definition { value })
    }
    /// Validate and compose ordered labels with optional authored descriptions.
    /// # Errors
    /// Refuses duplicate names and descriptions for labels outside the set.
    pub fn labels(
        names: Vec<String>,
        descriptions: Vec<(String, Description)>,
        bare_levels: bool,
    ) -> Result<Description, Error> {
        let mut selected = std::collections::HashSet::new();
        if names.iter().any(|name| !selected.insert(name)) {
            return Err(Error::usage("labels must be distinct"));
        }
        if descriptions
            .iter()
            .any(|(name, _)| !selected.contains(name))
        {
            return Err(Error::usage("descriptions= names an unknown label"));
        }
        let value = if descriptions.is_empty() {
            serde_json::to_string(&names)
                .map_err(|_| Error::defect("label names could not be written"))?
        } else {
            let fields = names
                .iter()
                .map(|name| {
                    let value = match descriptions.iter().find(|(label, _)| label == name) {
                        Some((_, value)) => value.as_json().to_owned(),
                        None if bare_levels => quoted(name)?,
                        None => "null".into(),
                    };
                    Ok(format!("{}:{value}", quoted(name)?))
                })
                .collect::<Result<Vec<_>, Error>>()?;
            format!("{{{}}}", fields.join(","))
        };
        Description::from_json(&value)
    }
}
fn quoted(value: &str) -> Result<String, Error> {
    serde_json::to_string(value)
        .map_err(|_| Error::defect("inline field name could not be written"))
}
