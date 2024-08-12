use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Author {
    id: Uuid,
    first_name: String,
    last_name: String,
    nickname: String,
}

impl Author {
    pub fn new(id: Uuid, first_name: String, last_name: String, nickname: String) -> Self {
        Self {
            id,
            first_name,
            last_name,
            nickname,
        }
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }
}
