use uuid::Uuid;

pub struct AccountId(Uuid);

impl AccountId {
    pub fn new(uuid: Uuid) -> Self {
        AccountId(uuid)
    }

    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}

pub struct Account {
    pub id: AccountId,
    pub name: String,
}
