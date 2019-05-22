//! |     
//! # Description
//! 
//! The goal of this crate is not to be an efficient parsing library. Use a regex for that.  
//! Rather, this crate aims to preserve the integrity of stepping through a DFA state diagram.   
//! 
//! It allows you to build a DFA with states of a generic type `S` that recognizes a language  
//! whose symbols are of a generic type `T`. You can traverse each transition symbol one-by-one, or you can 
//! consume an iterator of symbols, which the DFA will either `Accept` or `Reject`.
//! 
//! DFAs are created through a DFABuilder, which ensures that the DFA has valid transitions for every symbol in the alphabet.
//!
//! # Documentation
//!
//! [nifty::dfa](dfa/index.html)
//!
//! # Examples
//!
//! ## Using the DFABuilder
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
//!     .recognizes("string is empty, or begins and ends with the letter { a }.")
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
//!
//! ## Tracing a Path
//!
//! <img src="https://raw.githubusercontent.com/ObliqueMotion/nifty/master/images/example2.png">
//!
//! ### Code
//!
//! ```
//! use nifty::dfa::DFABuilder;
//!
//! let q0 = "Seen { }";
//! let q1 = "Seen { b }";
//! let q2 = "Seen { ba }";
//! let q3 = "Seen { bab }";
//!
//! let mut dfa = DFABuilder::default()
//!     .add_state(&q0)
//!     .add_state(&q1)
//!     .add_state(&q2)
//!     .add_state(&q3)
//!     .mark_goal_state(&q3)
//!     .mark_start_state(&q0)
//!     .add_transition(&q0, &'a', &q0)
//!     .add_transition(&q1, &'a', &q2)
//!     .add_transition(&q2, &'a', &q0)
//!     .add_transition(&q0, &'b', &q1)
//!     .add_transition(&q1, &'b', &q1)
//!     .add_transition(&q2, &'b', &q3)
//!     .recognizes("input contains { bab }")
//!     .build();
//!
//! let path = "abaababa".chars()
//!     .map(|c| dfa.get_next(&c))
//!     .collect::<Vec<_>>();
//!
//! dbg!(&path);
//! ```
//!
//! ### Output
//!
//! ```text
//! [src/main.rs:30] &path = [
//!    "Seen { }",
//!    "Seen { b }",
//!    "Seen { ba }",
//!    "Seen { }",
//!    "Seen { b }",
//!    "Seen { ba }",
//!    "Seen { bab }",
//!    "Seen { bab }",
//! ]
//! ```

pub mod dfa;
