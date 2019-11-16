use crate::member::Member;
use crate::poll::{Poll, Vote};
use crate::session::EstimateSession;
use crate::{Error, Result};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Sent from server to client, this shared model is used for all client communication
#[allow(variant_size_differences)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ResponseMessage {
  Connected {
    connection_id: Uuid,
    u: Box<crate::profile::UserProfile>,
    b: bool
  },
  ServerError {
    reason: String,
    content: String
  },
  Pong {
    v: i64
  },
  Notification {
    level: String,
    content: String
  },
  // Session messages
  SessionNotFound {
    id: Uuid
  },
  SessionJoined {
    session: Box<EstimateSession>,
    members: Vec<Member>,
    connected: Vec<Uuid>,
    polls: Vec<Poll>,
    votes: Vec<Vote>
  },
  SessionUpdate {
    session: EstimateSession
  },
  MemberStatusUpdate {
    user_id: Uuid,
    connected: bool
  },
  MemberUpdate {
    member: Member
  },
  PollUpdate {
    poll: Poll
  },
  VoteUpdate {
    vote: Vote
  }
}

impl ResponseMessage {
  pub fn from_json(s: &str) -> Result<ResponseMessage> {
    serde_json::from_str(&s).map_err(|e| Error::from(format!("Can't decode json ResponseMessage: {}", e)))
  }

  pub fn to_json(&self) -> Result<String> {
    serde_json::to_string_pretty(&self).map_err(|e| Error::from(format!("Can't encode json ResponseMessage: {}", e)))
  }

  pub fn from_binary(b: &[u8]) -> Result<ResponseMessage> {
    bincode::deserialize(&b).map_err(|e| Error::from(format!("Can't decode binary ResponseMessage: {}", e)))
  }

  pub fn to_binary(&self) -> Result<Vec<u8>> {
    bincode::serialize(&self).map_err(|e| Error::from(format!("Can't encode binary ResponseMessage: {}", e)))
  }
}
