//! |     
//! # Description
//!
//! The goal of this crate is not to be an efficient parsing library. Use a regex for that.  
//! Rather, this crate aims to preserve the integrity of stepping through a DFA state diagram.   
//!
//! It allows you to build a DFA with states of a generic type `S` that recognizes a language
//! whose symbols are of a generic type `T`. You can traverse each transition symbol one by one, or you can
//! consume an iterator of symbols, which the DFA will either `Accept` or `Reject`.
//!
//! DFAs are created using a `DFABuilder`, or by using the `make_dfa!` macro.
//! Both methods ensure that the DFA has valid transitions for every symbol in its alphabet.
//!
//! # Documentation
//!
//! [nifty::dfa](dfa/index.html)  
//! [nifty::make_dfa!](macro.make_dfa!.html)
//!
//! # Examples
//!
//! ## Building a DFA
//!
//! <img src="https://raw.githubusercontent.com/ObliqueMotion/nifty/master/images/example.png">
//!
//! ### Code
//!
//! ```
//! use nifty::make_dfa;
//!
//! let q0 = "q0";
//! let q1 = "q1";
//! let q2 = "q2";
//! let q3 = "q3";
//!
//! let mut dfa = make_dfa! {
//!     states { q0, q1, q2, q3 }
//!     accept { q0, q1 }
//!     start  { q0 }
//!     dead   { q3 }
//!     transitions {
//!         'a' => (q0, q1)
//!         'b' => (q0, q3)
//!
//!         'a' => (q1, q1)
//!         'b' => (q1, q2)
//!
//!         'a' => (q2, q1)
//!         'b' => (q2, q2)
//!     }
//!     recognizes {
//!         "empty, or starts and ends with { a }"
//!     }
//! };
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
//! use nifty::make_dfa;
//!
//! fn main() {
//!     let q0 = "Seen { }";
//!     let q1 = "Seen { b }";
//!     let q2 = "Seen { ba }";
//!     let q3 = "Seen { bab }";
//!
//!     let mut dfa = make_dfa! {
//!         states { q0, q1, q2, q3 }
//!         start  { q0 }
//!         goal   { q3 }
//!         transitions {
//!             'a' => (q0, q0)
//!             'a' => (q1, q2)
//!             'a' => (q2, q0)
//!
//!             'b' => (q0, q1)
//!             'b' => (q1, q1)
//!             'b' => (q2, q3)
//!         }
//!         recognizes {
//!             "contains { bab }"
//!         }
//!     };  
//!
//!     let path = "abaababa".chars()
//!         .map(|c| (c, dfa.get_next(&c)))
//!         .collect::<Vec<_>>();
//!
//!     for tuple in &path {
//!         println!("{:?}", tuple);
//!     }   
//! }
//! ```
//!
//! ### Output
//!
//! ```text
//! ('a', "Seen { }")
//! ('b', "Seen { b }")
//! ('a', "Seen { ba }")
//! ('a', "Seen { }")
//! ('b', "Seen { b }")
//! ('a', "Seen { ba }")
//! ('b', "Seen { bab }")
//! ('a', "Seen { bab }")
//! ```

pub mod dfa;
/// Macro [make_dfa!](macro.make_dfa!.html) creates a [DFA](dfa/struct.DFA.html) using the [DFABuilder](dfa/struct.DFABuilder.html).
///
/// 1. `make_dfa!` **must** take `states{...}` first.
/// 2. `make_dfa!` **may** take any of `start{...}`, `accept{...}`, `dead{...}`, `goal{...}` in any order.
/// 3. `make_dfa!` **may** take any of `transitions{...}`, `recognizes{...}` in any order.
///
/// ### Example
///
/// <img src="https://raw.githubusercontent.com/ObliqueMotion/nifty/master/images/example2.png">
///
/// ### Code
///
/// ```
/// use nifty::make_dfa;
///
/// let q0 = "Seen { }";
/// let q1 = "Seen { b }";
/// let q2 = "Seen { ba }";
/// let q3 = "Seen { bab }";
///
/// let mut dfa = make_dfa! {
///     states { q0, q1, q2, q3 }
///     start  { q0 }
///     goal   { q3 }
///     transitions {
///         'a' => (q0, q0)
///         'a' => (q1, q2)
///         'a' => (q2, q0)
///
///         'b' => (q0, q1)
///         'b' => (q1, q1)
///         'b' => (q2, q3)
///     }
///     recognizes {
///         "contains { bab }"
///     }
/// };
/// ```
pub mod make_dfa;
