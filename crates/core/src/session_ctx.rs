use crate::session::EstimateSession;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use std::collections::{HashMap, HashSet};

#[derive(Debug, Deserialize, Serialize)]
pub struct SessionContext {
  session: EstimateSession,
  members: HashMap<Uuid, crate::member::Member>,
  connected: HashSet<Uuid>,
  polls: HashMap<Uuid, crate::poll::Poll>,
  votes: HashMap<(Uuid, Uuid), crate::poll::Vote>
}

impl SessionContext {
  pub fn new(
    session: Box<EstimateSession>,
    members: Vec<crate::member::Member>,
    connected: Vec<Uuid>,
    polls: Vec<crate::poll::Poll>,
    votes: Vec<crate::poll::Vote>
  ) -> SessionContext {
    SessionContext {
      session: *session,
      members: members.iter().map(|m| (m.user_id().clone(), m.clone())).collect(),
      connected: connected.iter().map(|u| u.clone()).collect(),
      polls: polls.iter().map(|p| (p.id().clone(), p.clone())).collect(),
      votes: votes.iter().map(|v| ((v.poll_id().clone(), v.user_id().clone()), v.clone())).collect()
    }
  }

  pub fn session(&self) -> &EstimateSession {
    &self.session
  }

  pub fn members(&self) -> &HashMap<Uuid, crate::member::Member> {
    &self.members
  }

  pub fn connected(&self) -> &HashSet<Uuid> {
    &self.connected
  }

  pub fn polls(&self) -> &HashMap<Uuid, crate::poll::Poll> {
    &self.polls
  }

  pub fn votes(&self) -> &HashMap<(Uuid, Uuid), crate::poll::Vote> {
    &self.votes
  }
}
