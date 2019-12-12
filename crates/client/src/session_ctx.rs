use rustimate_core::session::EstimateSession;

use rustimate_core::member::Member;
use rustimate_core::poll::Poll;
use rustimate_core::poll::Vote;

use uuid::Uuid;

use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub(crate) struct SessionContext {
  session: EstimateSession,
  members: HashMap<Uuid, Member>,
  connected: HashSet<Uuid>,
  polls: HashMap<Uuid, Poll>,
  votes: HashMap<(Uuid, Uuid), Vote>
}

impl SessionContext {
  pub(crate) fn new(
    session: EstimateSession,
    members: Vec<Member>,
    connected: Vec<Uuid>,
    polls: Vec<Poll>,
    votes: Vec<Vote>
  ) -> SessionContext {
    SessionContext {
      session,
      members: members.iter().map(|m| (*m.user_id(), m.clone())).collect(),
      connected: connected.iter().copied().collect(),
      polls: polls.iter().map(|p| (*p.id(), p.clone())).collect(),
      votes: votes.iter().map(|v| ((*v.poll_id(), *v.user_id()), v.clone())).collect()
    }
  }

  pub(crate) fn session(&self) -> &EstimateSession {
    &self.session
  }

  pub(crate) fn members(&self) -> &HashMap<Uuid, Member> {
    &self.members
  }

  pub(crate) fn connected(&self) -> &HashSet<Uuid> {
    &self.connected
  }

  pub(crate) fn polls(&self) -> &HashMap<Uuid, Poll> {
    &self.polls
  }

  pub(crate) fn _votes(&self) -> &HashMap<(Uuid, Uuid), Vote> {
    &self.votes
  }
}
