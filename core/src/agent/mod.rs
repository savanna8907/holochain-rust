pub mod keys;

use agent::keys::Keys;
use common::entry::Entry;
use source_chain::memory::SourceChain;
use state;
use std::sync::{mpsc::Sender, Arc};

#[derive(Clone, Debug, PartialEq, Default)]
pub struct AgentState {
    keys: Option<Keys>,
    source_chain: Option<Box<SourceChain>>,
}

impl AgentState {
    pub fn new() -> Self {
        AgentState {
            keys: None,
            source_chain: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Action {
    Commit(Entry),
}

pub fn reduce(
    old_state: Arc<AgentState>,
    action: &state::Action,
    _action_channel: &Sender<state::ActionWrapper>,
) -> Arc<AgentState> {
    match *action {
        state::Action::Agent(ref agent_action) => {
            let mut new_state: AgentState = (*old_state).clone();
            match *agent_action {
                Action::Commit(ref _entry) => {}
            }
            Arc::new(new_state)
        }
        _ => old_state,
    }
}
