use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;
use std::rc::Rc;

// ===================================================================================================
// Enums
// ===================================================================================================

/// A `DFA`'s evaluation of an input, which is either `Accept` or `Reject`.
#[derive(Debug)]
pub enum Evaluation {
    Accept,
    Reject,
}

// ===================================================================================================
// DFA struct
// ===================================================================================================

/// A Deterministic Finite State Automaton (DFA) \[[wikipedia](https://en.wikipedia.org/wiki/Deterministic_finite_automaton)\]
///
/// The `DFA` struct aims to preserve the integrity of a DFA state diagram.
///
/// * A *dead* state is defined as a **non-accept** state from which all transitions implicitly lead back to itself.
/// * A *goal* state is defined as an **accept** state from which all transitions implicitly lead back to itself.
#[derive(Debug, Default)]
pub struct DFA<S, T>
where
    S: Eq + Hash + Clone + Debug,
    T: Eq + Hash + Clone + Debug,
{
    recognizes: String,
    states: HashSet<Rc<S>>,
    accept_states: HashSet<Rc<S>>,
    dead_states: HashSet<Rc<S>>,
    goal_states: HashSet<Rc<S>>,
    transitions: HashMap<T, HashMap<Rc<S>, Rc<S>>>,
    start: Option<Rc<S>>,
    current: Rc<S>,
}

impl<S, T> DFA<S, T>
where
    S: Eq + Hash + Clone + Debug,
    T: Eq + Hash + Clone + Debug,
{

    /// Returns a description of the language that this `DFA` recognizes.
    pub fn recognizes(&self) -> &str {
        &self.recognizes
    }

    /// Returns an iterator over the `DFA`'s states in no particular order.
    pub fn states(&self) -> impl Iterator<Item = Rc<S>> + '_ {
        self.states.iter().cloned()
    }

    /// Returns an iterator over the `DFA`'s *dead* states in no particular order.
    pub fn dead_states(&self) -> impl Iterator<Item = Rc<S>> + '_ {
        self.dead_states.iter().cloned()
    }

    /// Returns an iterator over the `DFA`'s *goal* states in no particular order.
    pub fn goal_states(&self) -> impl Iterator<Item = Rc<S>> + '_ {
        self.goal_states.iter().cloned()
    }

    /// Returns an iterator over the `DFA`'s alphabet in no particular order.
    pub fn alphabet(&self) -> impl Iterator<Item = T> + '_ {
        self.transitions.keys().cloned()
    }

    /// Returns the start state of the `DFA`.
    /// 
    /// * Panics if no start state is defined for the DFA.
    pub fn start(&self) -> Rc<S> {
        self.start
            .as_ref()
            .expect("DFA::start(): No start state was defined for this DFA.")
            .clone()
    }

    /// Returns the current state that the DFA is in.
    pub fn current(&self) -> Rc<S> {
        self.current.clone()
    }

    /// Moves the current state back to the start state.
    pub fn restart(&mut self) {
        self.current = self.start.as_ref().unwrap().clone();
    }

    /// Moves the current state to the next state given some transition.
    ///
    /// * Panics if the transition is not part of the alphabet.  
    /// * Panics if the transition has no defined destination from the current state.
    pub fn next(&mut self, transition: &T) {
        match self.transitions.get(transition) {
            Some(state_pairs) => match state_pairs.get(&self.current) {
                Some(destination) => self.current = destination.clone(),
                None => {
                    if self.dead_states.get(&self.current).is_some() {
                        return;
                    }
                    if self.goal_states.get(&self.current).is_some() {
                        return;
                    }
                    panic!("DFA::next(): There is no path defined from state ({:?}) on transition ({:?})", self.current, transition);
                }
            },
            None => panic!(
                "DFA::next(): Attempted to move with unknown transition ({:?})",
                transition
            ),
        }
    }

    /// Moves the current state to the next state given a transition and returns new state.
    ///
    /// * Panics if the transition is not part of the alphabet.  
    /// * Panics if the transition has no defined destination from the current state.
    pub fn get_next(&mut self, transition: &T) -> Rc<S> {
        self.next(transition);
        self.current.clone()
    }

    /// Evaluates the current state on whether or not it is an accept state.
    pub fn eval_current(&self) -> Evaluation {
        if self.accept_states.get(&self.current).is_some() {
            Evaluation::Accept
        } else {
            Evaluation::Reject
        }
    }

    /// Evaluates an input that will be either accepted or rejected by the DFA.
    /// 
    /// * Panics if the input contains symbols that are not in the DFA's alphabet.
    pub fn evaluate(&mut self, inputs: impl Iterator<Item = T>) -> Evaluation {
        self.restart();
        for transition in inputs {
            self.next(&transition);
        }
        self.eval_current()
    }
}

/// A [builder](https://www.geeksforgeeks.org/builder-design-pattern/) for the `DFA` struct.
#[derive(Debug, Default)]
pub struct DFABuilder<S, T>
where
    S: Eq + Hash + Clone + Debug,
    T: Eq + Hash + Clone + Debug,
{
    dfa: DFA<S, T>,
}

impl<'a, S, T> DFABuilder<S, T>
where
    S: Eq + Hash + Clone + Debug,
    T: Eq + Hash + Clone + Debug,
{
    /// Adds a description of the language that this [DFA](struct.DFA.html) will recognize.
    pub fn recognizes(mut self, description: &str) -> Self {
        if !self.dfa.recognizes.is_empty() {
            panic!("DFABuilder::recognizes(): The language that a DFA recognizes may be defined only once.");
        }
        self.dfa.recognizes = description.to_owned();
        self
    }

    /// Adds a state to the `DFA`.
    /// 
    /// * Panics if a state is added after any transition has been added. Must add all states before transitions.
    pub fn add_state(mut self, state: &S) -> Self {
        if !self.dfa.transitions.is_empty() {
            panic!("DFABuilder::add_state(): All states must be added before any transitions are added.");
        }
        self.dfa.states.insert(Rc::new(state.clone()));
        self
    }

    /// Marks a state as an accept state.
    /// 
    /// * Panics if the provided state does not exist in this [DFA](struct.DFA.html).
    pub fn mark_accept_state(mut self, state: &S) -> Self {
        match self.dfa.states.get(state) {
            Some(state) => {
                if let Some(state) = self.dfa.dead_states.get(state) {
                    panic!("DFABuilder::mark_accept_state(): Attempted to mark a dead state ({:?}) as an accept state.", state);
                }
                self.dfa.accept_states.insert(state.clone());
            }
            None => panic!("DFABuilder::mark_accept_state(): Attempted to mark a non-existent state ({:?}) as an accept state.", state),
        }
        self
    }

    /// Marks a state as a goal state.
    /// 
    /// * This will automatically add the goal state to the list of accept states.
    /// * Panics if the provided state does not exist in this [DFA](struct.DFA.html).
    pub fn mark_goal_state(mut self, state: &S) -> Self {
        match self.dfa.states.get(state) {
            Some(state) => {
                if let Some(state) = self.dfa.dead_states.get(state) {
                    panic!(
                        "DFABuilder::mark_goal_state(): Attempted to mark a dead state ({:?}) as a goal state.",
                        state
                    );
                }
                self.dfa.accept_states.insert(state.clone());
                self.dfa.goal_states.insert(state.clone());
            }
            None => panic!(
                "DFABuilder::mark_goal_state(): Attempted to mark a non-existent state ({:?}) as a goal state.",
                state
            ),
        }
        self
    }

    /// Marks a state as a dead state.
    /// 
    /// * Panics if the provided state does not exist in this [DFA](struct.DFA.html).
    pub fn mark_dead_state(mut self, state: &S) -> Self {
        match self.dfa.states.get(state) {
            Some(state) => {
                if let Some(state) = self.dfa.accept_states.get(state) {
                    panic!("DFABuilder::mark_dead_state(): Attempted to mark an accept state ({:?}) as a dead state.", state);
                }
                self.dfa.dead_states.insert(state.clone());
            }
            None => panic!(
                "DFABuilder::mark_dead_state(): Attempted to mark a non-existent state ({:?}) as a dead state.",
                state
            ),
        }
        self
    }

    /// Marks a state as the start state.
    /// 
    /// * Panics if the provided state does not exist in this [DFA](struct.DFA.html).
    /// * Panics if the [DFA](struct.DFA.html) already has a defined start state.
    pub fn mark_start_state(mut self, state: &S) -> Self {
        if let Some(start) = self.dfa.start {
            panic!("DFABUilder::mark_start_state(): Attempted to mark state ({:?}) as the start state when the start state ({:?}) has already been defined.", state, start);
        }
        match self.dfa.states.get(state) {
            Some(state) => {
                self.dfa.start = Some(state.clone());
                self.dfa.current = state.clone();
            }
            None => panic!(
                "DFABuilder::mark_start_state(): Attempted to mark a non-existent state ({:?}) as a start state.",
                state
            ),
        }
        self
    }

    /// Adds a transition from one state to another.
    /// 
    /// * Panics if the transition is coming from a **dead** state. A dead state's transitions cannot be overwritten.
    /// * Panics if the transition is coming from a **goal** state. A goal state's transitions cannot be overwritten.
    /// * Panics if the *from* state does not exist in the [DFA](struct.DFA.html).
    /// * Panics if the *to* state does not exist in the [DFA](struct.DFA.html).
    pub fn add_transition(mut self, from: &S, transition: &T, to: &S) -> Self {
        if let Some(state) = self.dfa.dead_states.get(from) {
            panic!("DFABuilder::add_transition(): Attempted to create a transition from state ({:?}), which is a dead state and all transitions implicitly lead to itself.", state);
        }
        if let Some(state) = self.dfa.goal_states.get(from) {
            panic!("DFABuilder::add_transition(): Attempted to create a transition from state ({:?}), which is a goal state and all transitions implicitly lead to itself.", state);
        }
        match (self.dfa.states.get(from), self.dfa.states.get(to)) {
            (Some(from), Some(to)) => {
                if let Some(state_pairs) = self.dfa.transitions.get_mut(transition) {
                    state_pairs.insert(from.clone(), to.clone());
                } else {
                    let transition = transition.clone();
                    let mut state_pairs = HashMap::new();
                    state_pairs.insert(from.clone(), to.clone());
                    self.dfa.transitions.insert(transition, state_pairs);
                }
            }
            (None, _) => panic!(
                "DFABuilder::add_transition(): Attempted to create a transition from state ({:?}), which does not exist in this DFA.",
                from
            ),
            (_, None) => panic!(
                "DFABuilder::add_transition(): Attempt to create a transition to state ({:?}), which does not exist in this DFA.",
                to
            ),
        }
        self
    }

    /// Builds a [DFA](struct.dfa.html) and consumes the builder.
    /// 
    /// * Panics if the [DFA](struct.dfa.html) has no states.
    /// * Panics if the [DFA](struct.dfa.html) has no defined start state.
    /// * Panics if each state in the [DFA](struct.dfa.html) does not have a transition for every symbol in the alphabet.
    pub fn build(self) -> DFA<S, T> {
        if self.dfa.states.is_empty() {
            panic!("DFABuilder::build(): The DFA must have at least one state, but it is empty.");
        }
        if self.dfa.start.is_none() {
            panic!(
                "DFABuilder::build(): DFA must have a valid start state. No start state was defined."
            );
        }
        let transition_states =
            self.dfa.states.len() - self.dfa.dead_states.len() - self.dfa.goal_states.len();
        for state_pairs in self.dfa.transitions.values() {
            if state_pairs.keys().count() < transition_states {
                panic!("DFABuilder::build(): All transition states have a complete set of output edges for every transition.");
            }
        }
        self.dfa
    }
}
