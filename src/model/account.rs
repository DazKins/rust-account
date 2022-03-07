use uuid::Uuid;

#[derive(PartialEq, Eq)]
pub struct AccountId {
    uuid: Uuid
}

impl AccountId {
    pub const fn new(uuid: Uuid) -> Self {
        AccountId {
            uuid
        }
    }

    pub fn to_string(&self) -> String {
        self.uuid.to_string()
    }
}

pub struct Account {
    pub id: AccountId,
    pub name: String,
}
