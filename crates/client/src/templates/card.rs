use maud::{html, Markup};

pub(crate) fn card(ctx: &crate::ctx::ClientContext, content: Markup) -> Markup {
  html! {
    div.uk-margin.uk-card.uk-card-body.(ctx.profile().theme().card_class()) {
      (content)
    }
  }
}
