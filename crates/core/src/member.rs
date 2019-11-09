use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum MemberRole {
  Creator,
  Admin,
  Participant,
  Observer,
  Invited
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Member {
  user_id: Uuid,
  name: String,
  role: MemberRole
}

impl Member {
  pub fn user_id(&self) -> &Uuid {
    &self.user_id
  }

  pub fn name(&self) -> &String {
    &self.name
  }

  pub fn role(&self) -> &MemberRole {
    &self.role
  }
}
