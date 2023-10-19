#[derive(Clone, Debug)]
pub struct Dialog {
    pub name: Option<String>,
    pub age: Option<i32>,
    pub username: Option<String>,
    pub messages: Option<Vec<String>>,
}
