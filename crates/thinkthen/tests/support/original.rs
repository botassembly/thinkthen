#[derive(serde::Serialize)]
pub(crate) struct Original {
    pub(crate) z: bool,
    pub(crate) body: &'static str,
    pub(crate) a: Vec<i32>,
}
impl thinkthen::Evidence for Original {
    fn evidence(&self) -> &str {
        self.body
    }
}
