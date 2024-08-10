#[derive(Debug, Clone)]
pub struct Author {
    id: uuid::Uuid,
    first_name: String,
    last_name: String,
    username: String,
}

impl Author {
    pub fn new(id: uuid::Uuid, first_name: String, last_name: String, username: String) -> Self {
        Self {
            id,
            first_name,
            last_name,
            username,
        }
    }

    pub fn get_full_name(&self) -> String {
        String::from(format!("{} {}", &self.first_name, &self.last_name))
    }
}
