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
  session_id: Uuid,
  participant_id: Uuid,
  name: String,
  role: MemberRole
}
