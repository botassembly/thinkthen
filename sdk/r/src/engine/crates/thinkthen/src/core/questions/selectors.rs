//! Typed toolbox lookups use the same saved-domain admission as enum constructors.
use super::*;
impl Toolbox {
    fn named(&self, name: &str) -> Result<&SimpleQuestion, Error> {
        let question = self.get(name).ok_or_else(|| {
            Error::from_contract(
                crate::core::Code::UsageName,
                format!("the toolbox has no question named `{name}`"),
            )
        })?;
        Ok(question)
    }
}
macro_rules! selector {
    ($method:ident,$question:ident) => { impl Toolbox {
        #[doc=concat!("Select the named ",stringify!($question)," question.")]
        /// # Errors
        /// Refuses a missing name or another function.
        pub fn $method(&self,name:&str)->Result<$question,Error> {
            match self.named(name)? { SimpleQuestion::$question(question)=>Ok(question.clone()), _=>Err(wrong_function(name)) }
        }
    }};
    (generic $method:ident,$question:ident) => { impl Toolbox {
        #[doc=concat!("Select the named ",stringify!($question)," question with this enum's exact domain.")]
        /// # Errors
        /// Refuses a missing name, another function or a mismatched enum domain.
        pub fn $method<C:Choice>(&self,name:&str)->Result<$question<C>,Error> {
            match self.named(name)? { SimpleQuestion::$question(question)=>question.clone().typed(), _=>Err(wrong_function(name)) }
        }
    }};
}
fn wrong_function(name: &str) -> Error {
    Error::from_contract(
        crate::core::Code::UsageFunction,
        format!("the toolbox question `{name}` asks another function"),
    )
}
selector!(decide, Decide);
selector!(filter, Filter);
selector!(generic choose,Choose);
selector!(generic tag,Tag);
selector!(generic score,Score);
selector!(generic rank,Rank);
selector!(find, Find);
selector!(recognize, Recognize);
selector!(relate, Relate);
