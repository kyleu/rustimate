use crate::utils::onclick_event;

use maud::{html, Markup};
use rustimate_core::poll::PollStatus;

pub(crate) fn modals() -> Markup {
  html!(
    div#add-member-modal uk-modal? {
      div.uk-modal-dialog.uk-modal-body {
        h2.uk-modal-title { "Add Member" }
        "I'm working on it, for now just copy/paste the url"
        p.uk-text-right {
          button.uk-button.uk-button-default.uk-modal-close type="button" { "Cancel" }
        }
      }
    }
    div#add-poll-modal uk-modal? {
      div.uk-modal-dialog.uk-modal-body {
        h2.uk-modal-title { "Add Poll" }
        input.uk-input#poll-modal-input type="text" placeholder="Poll Question" name="poll" {}
        p.uk-text-right {
          button.uk-button.uk-button-default.uk-modal-close type="button" { "Cancel" }
          button.uk-button.uk-button-primary.uk-modal-close type="button" onclick=(onclick_event("submit-poll", "", "document.getElementById('poll-modal-input').value")) { "Add Poll" }
        }
      }
    }
    div#member-detail-modal uk-modal? {
      div.uk-modal-dialog.uk-modal-body {
        h2.uk-modal-title#member-detail-name { "..." }
        div#member-detail-content { "..." }
        p.uk-text-right {
          button.uk-button.uk-button-default.uk-modal-close type="button" { "Close" }
        }
      }
    }
    div#poll-detail-modal.uk-modal-container uk-modal? {
      div.uk-modal-dialog.uk-modal-body {
        h2.uk-modal-title#poll-detail-title { "..." }
        div#poll-detail-content {
          input#active-poll-id type="hidden" { }
          div.poll-section#poll-section-pending {
            h5 { "Poll is pending" }
            div.uk-margin-top {
              button.uk-button.uk-button-default#poll-reset-button onclick=(onclick_event("delete-poll", "", "")) { "Delete Poll" }
              " "
              button.uk-button.uk-button-primary#poll-start-button onclick=(onclick_event("set-poll-status", &format!("{}", PollStatus::Active), "")) { "Start Voting" }
            }
          }
          div.poll-section#poll-section-active {
            h5 { "Poll is active" }

            h4 { "Vote Status" }
            div#poll-vote-status {
            }
            h4 { "Your Vote" }
            div#poll-vote-choices {
              "TODO"
            }
            div.uk-margin-top {
              button.uk-button.uk-button-default#poll-reset-button onclick=(onclick_event("set-poll-status", &format!("{}", PollStatus::Pending), "")) { "Clear Votes" }
              " "
              button.uk-button.uk-button-primary#poll-complete-button onclick=(onclick_event("set-poll-status", &format!("{}", PollStatus::Complete), "")) { "Finish Voting" }
            }
          }
          div.poll-section#poll-section-complete {
            h5 { "Poll is complete" }
            h4 { "Results" }
            div#poll-complete-summary { }
            h4 { "Stats" }
            div#poll-complete-stats { }
            div.uk-margin-top {
              button.uk-button.uk-button-default#poll-reopen-button onclick=(onclick_event("set-poll-status", &format!("{}", PollStatus::Active), "")) { "Reopen Voting" }
            }
          }
        }
        p.uk-text-right {
          button.uk-button.uk-button-default.uk-modal-close type="button" { "Close" }
        }
      }
    }
    div#profile-detail-modal uk-modal? {
      div.uk-modal-dialog.uk-modal-body {
        h2.uk-modal-title { "Edit Profile" }
        input.uk-input#profile-detail-modal-input type="text" placeholder="Name" name="name" {}
        p.uk-text-right {
          button.uk-button.uk-button-default.uk-modal-close type="button" { "Cancel" }
          button.uk-button.uk-button-primary.uk-modal-close type="button" onclick=(onclick_event("update-profile", "", "document.getElementById('profile-detail-modal-input').value")) { "Change Name" }
        }
      }
    }
    div#session-detail-modal uk-modal? {
      div.uk-modal-dialog.uk-modal-body {
        h2.uk-modal-title { "Edit Session" }
        input.uk-input#session-detail-modal-input type="text" placeholder="Name" name="name" {}
        p.uk-text-right {
          button.uk-button.uk-button-default.uk-modal-close type="button" { "Cancel" }
          button.uk-button.uk-button-primary.uk-modal-close type="button" onclick=(onclick_event("update-session", "", "document.getElementById('session-detail-modal-input').value")) { "Change Name" }
        }
      }
    }
  )
}
