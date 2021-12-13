use druid::{Data, Lens};

#[derive(Clone, Data, Lens)]
pub struct AppState {
  pub body: String
}
