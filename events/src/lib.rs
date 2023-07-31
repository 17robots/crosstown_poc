use borsh::{BorshSerialize, BorshDeserialize};

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct UserCreatedEventMessage {
    pub user_id: String,
    pub user_name: String,
}

