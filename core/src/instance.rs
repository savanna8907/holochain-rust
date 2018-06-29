//use error::HolochainError;
use state::*;
use std::{
    sync::{mpsc::*, Arc, RwLock, RwLockReadGuard},
    thread,
    time::Duration,
};

//#[derive(Clone)]
pub struct Instance {
    state: Arc<RwLock<State>>,
    action_channel: Sender<ActionWrapper>,
    observer_channel: Sender<Observer>,
}

type ClosureType = Box<FnMut(&State) -> bool + Send>;

pub struct Observer {
    sensor: ClosureType,
    done: bool,
}

impl Observer {
    fn check(&mut self, state: &State) {
        self.done = (self.sensor)(state);
    }
}

static DISPATCH_WITHOUT_CHANNELS: &str = "dispatch called without channels open";

impl Instance {
    pub fn dispatch(&mut self, action: Action) -> ActionWrapper {
        let wrapper = ActionWrapper::new(action);
        self.action_channel
            .send(wrapper.clone())
            .unwrap_or_else(|_| panic!(DISPATCH_WITHOUT_CHANNELS));
        wrapper
    }

    pub fn dispatch_and_wait(&mut self, action: Action) {
        let wrapper = ActionWrapper::new(action);
        let wrapper_clone = wrapper.clone();

        let (sender, receiver) = channel::<bool>();
        let closure = move |state: &State| {
            if state.history.contains(&wrapper_clone) {
                sender
                    .send(true)
                    .unwrap_or_else(|_| panic!(DISPATCH_WITHOUT_CHANNELS));
                true
            } else {
                false
            }
        };

        let observer = Observer {
            sensor: Box::new(closure),
            done: false,
        };

        self.observer_channel
            .send(observer)
            .unwrap_or_else(|_| panic!(DISPATCH_WITHOUT_CHANNELS));

        self.action_channel
            .send(wrapper)
            .unwrap_or_else(|_| panic!(DISPATCH_WITHOUT_CHANNELS));

        receiver
            .recv()
            .unwrap_or_else(|_| panic!(DISPATCH_WITHOUT_CHANNELS));
    }

    pub fn dispatch_with_observer<F>(&mut self, action: Action, closure: F)
    where
        F: 'static + FnMut(&State) -> bool + Send,
    {
        let observer = Observer {
            sensor: Box::new(closure),
            done: false,
        };

        self.observer_channel
            .send(observer)
            .expect("observer channel to be open");
        self.dispatch(action);
    }

    pub fn start_action_loop(&mut self) {
        let (tx_action, rx_action) = channel::<ActionWrapper>();
        let (tx_observer, rx_observer) = channel::<Observer>();
        self.action_channel = tx_action.clone();
        self.observer_channel = tx_observer.clone();

        let state_mutex = self.state.clone();

        thread::spawn(move || {
            let mut state_observers: Vec<Box<Observer>> = Vec::new();

            loop {
                match rx_action.recv_timeout(Duration::from_millis(400)) {
                    Ok(action_wrapper) => {
                        // Mutate state:
                        {
                            let mut state = state_mutex.write().unwrap();
                            *state = state.reduce(action_wrapper, &tx_action);
                        }

                        // Add new observers
                        while let Ok(observer) = rx_observer.try_recv() {
                            state_observers.push(Box::new(observer));
                        }

                        // Run all observer closures
                        {
                            let state = state_mutex.read().unwrap();
                            state_observers = state_observers
                                .into_iter()
                                .map(|mut observer| {
                                    observer.check(&state);
                                    observer
                                })
                                .filter(|observer| !observer.done)
                                .collect::<Vec<_>>();
                        }
                    }
                    Err(ref _recv_error) => {}
                }
            }
        });
    }

    pub fn new() -> Self {
        let (tx_action, _) = channel();
        let (tx_observer, _) = channel();
        Instance {
            state: Arc::new(RwLock::new(State::new())),
            action_channel: tx_action,
            observer_channel: tx_observer,
        }
    }

    pub fn state(&self) -> RwLockReadGuard<State> {
        self.state.read().unwrap()
    }
}

impl Default for Instance {
    fn default() -> Self {
        Self::new()
    }
}
