use super::parts::Issue;

#[derive(Deserialize, Debug)]
pub struct SearchPayload {
    items: Vec<Issue>,
}

impl SearchPayload {
    pub fn items(&self) -> &Vec<Issue> {
        &self.items
    }
}
