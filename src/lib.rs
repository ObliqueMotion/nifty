//! |     
//! # Description
//! A crate for generating Deterministic Finite State Automata (DFAs) \[[wikipedia](https://en.wikipedia.org/wiki/Deterministic_finite_automaton)\].
//! 
//! # Documentation
//! 
//! [nifty::dfa](dfa/index.html)
//! 
//! # Example
//! 
//! <img src="https://raw.githubusercontent.com/ObliqueMotion/nifty/master/images/example.png">
//! 
//! ### Code
//! 
//! ```
//! use nifty::dfa::DFABuilder;
//! 
//! let q0 = "q0";
//! let q1 = "q1";
//! let q2 = "q2";
//! let q3 = "q3";
//!
//! let mut dfa = DFABuilder::default()
//!     .add_state(&q0)
//!     .add_state(&q1)
//!     .add_state(&q2)
//!     .add_state(&q3)
//!     .mark_dead_state(&q3)
//!     .mark_start_state(&q0)
//!     .mark_accept_state(&q0)
//!     .mark_accept_state(&q1)
//!     .add_transition(&q0, &'a', &q1)
//!     .add_transition(&q0, &'b', &q3)
//!     .add_transition(&q1, &'a', &q1)
//!     .add_transition(&q1, &'b', &q2)
//!     .add_transition(&q2, &'a', &q1)
//!     .add_transition(&q2, &'b', &q2)
//!     .recognizes("string is empty, or begins and ends with the letter 'a'.")
//!     .build();
//! 
//! dfa.evaluate("".chars());      //    Accept
//! dfa.evaluate("a".chars());     //    Accept
//! dfa.evaluate("b".chars());     // Reject
//! dfa.evaluate("aa".chars());    //    Accept
//! dfa.evaluate("ab".chars());    // Reject
//! dfa.evaluate("abb".chars());   // Reject
//! dfa.evaluate("aba".chars());   //    Accept
//! dfa.evaluate("abba".chars());  //    Accept
//! dfa.evaluate("babba".chars()); // Reject
//!
//! ```
pub mod dfa;