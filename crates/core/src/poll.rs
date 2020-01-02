use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum PollStatus {
  Pending,
  Active,
  Complete
}

impl std::str::FromStr for PollStatus {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> anyhow::Result<Self> {
    match s {
      "Pending" => Ok(Self::Pending),
      "Active" => Ok(Self::Active),
      "Complete" => Ok(Self::Complete),
      _ => Err(anyhow::anyhow!("Invalid theme [{}]", s))
    }
  }
}

impl std::fmt::Display for PollStatus {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let s = match self {
      Self::Pending => "Pending",
      Self::Active => "Active",
      Self::Complete => "Complete"
    };
    write!(f, "{}", s)
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Poll {
  id: Uuid,
  idx: u32,
  author_id: Uuid,
  title: String,
  status: PollStatus,
  final_vote: Option<String>
}

impl Poll {
  pub const fn new(id: Uuid, idx: u32, author_id: Uuid, title: String, status: PollStatus) -> Self {
    Self {
      id,
      idx,
      author_id,
      title,
      status,
      final_vote: None
    }
  }

  pub const fn id(&self) -> &Uuid {
    &self.id
  }

  pub const fn idx(&self) -> u32 {
    self.idx
  }

  pub const fn title(&self) -> &String {
    &self.title
  }

  pub fn set_title(&mut self, t: String) {
    self.title = t;
  }

  pub const fn status(&self) -> &PollStatus {
    &self.status
  }

  pub fn set_status(&mut self, s: PollStatus) {
    self.status = s;
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
  poll_id: Uuid,
  user_id: Uuid,
  choice: String
}

impl Vote {
  pub const fn new(poll_id: Uuid, user_id: Uuid, choice: String) -> Self {
    Self { poll_id, user_id, choice }
  }

  pub const fn poll_id(&self) -> &Uuid {
    &self.poll_id
  }

  pub const fn user_id(&self) -> &Uuid {
    &self.user_id
  }

  pub const fn choice(&self) -> &String {
    &self.choice
  }

  pub fn set_choice(&mut self, c: String) {
    self.choice = c;
  }
}
