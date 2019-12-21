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

impl std::fmt::Display for MemberRole {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let s = match self {
      Self::Creator => "Creator",
      Self::Admin => "Admin",
      Self::Participant => "Participant",
      Self::Observer => "Observer",
      Self::Invited => "Invited"
    };
    write!(f, "{}", s)
  }
}

#[derive(Clone, Debug, derive_more::Constructor, Deserialize, Serialize)]
pub struct Member {
  user_id: Uuid,
  name: String,
  role: MemberRole
}

impl Member {
  pub const fn user_id(&self) -> &Uuid {
    &self.user_id
  }

  pub const fn name(&self) -> &String {
    &self.name
  }

  pub fn set_name(&mut self, t: String) {
    self.name = t;
  }

  pub const fn role(&self) -> &MemberRole {
    &self.role
  }

  pub fn set_role(&mut self, r: MemberRole) {
    self.role = r;
  }
}
