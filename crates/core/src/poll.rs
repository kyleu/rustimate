use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum PollStatus {
  Pending,
  Active,
  Complete
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Poll {
  id: Uuid,
  session_key: String,
  idx: u32,
  author_id: Uuid,
  title: String,
  status: PollStatus,
  final_vote: Option<String>
}

impl Poll {
  pub fn new(id: Uuid, session_key: String, idx: u32, author_id: Uuid, title: String) -> Poll {
    Poll {
      id,
      session_key,
      idx,
      author_id,
      title,
      status: PollStatus::Pending,
      final_vote: None
    }
  }

  pub fn id(&self) -> &Uuid {
    &self.id
  }

  pub fn title(&self) -> &String {
    &self.title
  }

  pub fn set_title(&mut self, t: String) {
    self.title = t;
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum PollActionType {
  UpdateTitle,
  StatusChange,
  CastVote
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PollAction {
  id: Uuid,
  poll_id: Uuid,
  user_id: Uuid,
  t: PollActionType,
  ctx: std::collections::HashMap<String, String>,
  message: String
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Vote {
  session_id: Uuid,
  poll_id: Uuid,
  user_id: Uuid,
  choice: String
}

impl Vote {
  pub fn poll_id(&self) -> &Uuid {
    &self.poll_id
  }

  pub fn user_id(&self) -> &Uuid {
    &self.user_id
  }
}
