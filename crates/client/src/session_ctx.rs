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
  pub(crate) fn new(session: EstimateSession, members: &[Member], connected: &[Uuid], polls: &[Poll], votes: &[Vote]) -> Self {
    Self {
      session,
      members: members.iter().map(|m| (*m.user_id(), m.clone())).collect(),
      connected: connected.iter().copied().collect(),
      polls: polls.iter().map(|p| (*p.id(), p.clone())).collect(),
      votes: votes.iter().map(|v| ((*v.poll_id(), *v.user_id()), v.clone())).collect()
    }
  }

  pub(crate) const fn session(&self) -> &EstimateSession {
    &self.session
  }

  pub(crate) fn set_session(&mut self, s: EstimateSession) {
    self.session = s
  }

  pub(crate) const fn members(&self) -> &HashMap<Uuid, Member> {
    &self.members
  }

  pub(crate) fn set_member(&mut self, m: Member) {
    let _ = self.members.insert(m.user_id().clone(), m);
  }

  pub(crate) const fn connected(&self) -> &HashSet<Uuid> {
    &self.connected
  }

  pub(crate) const fn polls(&self) -> &HashMap<Uuid, Poll> {
    &self.polls
  }

  pub(crate) fn set_poll(&mut self, p: Poll) {
    let _ = self.polls.insert(p.id().clone(), p);
  }

  pub(crate) const fn _votes(&self) -> &HashMap<(Uuid, Uuid), Vote> {
    &self.votes
  }
}
