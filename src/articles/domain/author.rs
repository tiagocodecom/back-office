use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Author {
    id: Uuid,
    // _first_name: String,
    // _last_name: String,
    // _nickname: String,
}

impl Author {
    pub fn new(id: Uuid, _first_name: String, _last_name: String, _nickname: String) -> Self {
        Self { id }
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }
}
