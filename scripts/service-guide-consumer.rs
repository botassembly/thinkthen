//! External consumer of the public engine and saved JSON contract.
use engine::{Engine, Error, OffsetUnit, Question, Request, Settings, Toolbox, results::AnyRow};
use serde_json::Value;
use std::io::Read;
fn execute(mode: &str, value: Value) -> Result<Value, Box<dyn std::error::Error>> {
    let text = serde_json::to_string(&value)?;
    Ok(match mode {
        "request" => {
            let request = Request::from_json(&text)?;
            let call = Engine::from_env()?.open_request(&request, OffsetUnit::CodePoints)?;
            call.sender().close();
            let rows = call.rows().collect::<Result<Vec<_>, Error>>()?;
            serde_json::to_value(rows)?
        }
        "row" => serde_json::to_value(serde_json::from_str::<AnyRow>(&text)?)?,
        "error" => serde_json::to_value(serde_json::from_str::<Error>(&text)?)?,
        "question" => serde_json::to_value(serde_json::from_str::<Question>(&text)?)?,
        "toolbox" => serde_json::from_str(&Toolbox::from_json(&text)?.to_json()?)?,
        "settings" => serde_json::to_value(Settings::from_json(&text)?)?,
        _ => return Err("unknown guide consumer mode".into()),
    })
}
fn main() {
    let mut text = String::new();
    std::io::stdin().read_to_string(&mut text).unwrap();
    let value: Value = serde_json::from_str(&text).unwrap();
    match execute(std::env::args().nth(1).unwrap().as_str(), value) {
        Ok(value) => println!("{value}"),
        Err(error) => {
            if let Some(error) = error.downcast_ref::<Error>() {
                eprintln!("{}", serde_json::to_string(error).unwrap());
            } else {
                eprintln!("{error}");
            }
            std::process::exit(1);
        }
    }
}
