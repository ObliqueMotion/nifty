/// Creates a [DFA](dfa/struct.DFA.html) using the [DFABuilder](dfa/struct.DFABuilder.html).
///
/// 1. `make_dfa!` **must** take `states{...}` first.
/// 2. `make_dfa!` **may** take any of `start{...}`, `accept{...}`, `dead{...}`, `goal{...}` in any order.
/// 3. `make_dfa!` **may** take any of `transitions{...}`, `recognizes{...}` in any order.
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
#[macro_export]
macro_rules! make_dfa {
    (
        states {$(
            $state:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        start {
            $start:expr
        }
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_accept_state(&$accept))*
            $(.mark_dead_state(&$dead))*
            $(.mark_goal_state(&$goal))*
            .mark_start_state(&$start)
            $(.add_transition(&$from, &$edge, &$to))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        start {
            $start:expr
        }
        goal{$(
            $goal:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_accept_state(&$accept))*
            $(.mark_dead_state(&$dead))*
            .mark_start_state(&$start)
            $(.mark_goal_state(&$goal))*
            $(.add_transition(&$from, &$edge, &$to))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        start {
            $start:expr
        }
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_accept_state(&$accept))*
            $(.mark_goal_state(&$goal))*
            $(.mark_dead_state(&$dead))*
            .mark_start_state(&$start)
            $(.add_transition(&$from, &$edge, &$to))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        start {
            $start:expr
        }
        dead{$(
            $dead:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_accept_state(&$accept))*
            $(.mark_goal_state(&$goal))*
            .mark_start_state(&$start)
            $(.mark_dead_state(&$dead))*
            $(.add_transition(&$from, &$edge, &$to))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        start {
            $start:expr
        }
        dead{$(
            $dead:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_accept_state(&$accept))*
            .mark_start_state(&$start)
            $(.mark_dead_state(&$dead))*
            $(.mark_goal_state(&$goal))*
            $(.add_transition(&$from, &$edge, &$to))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        start {
            $start:expr
        }
        goal{$(
            $goal:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_accept_state(&$accept))*
            .mark_start_state(&$start)
            $(.mark_goal_state(&$goal))*
            $(.mark_dead_state(&$dead))*
            $(.add_transition(&$from, &$edge, &$to))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        start {
            $start:expr
        }
        goal{$(
            $goal:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_dead_state(&$dead))*
            $(.mark_accept_state(&$accept))*
            .mark_start_state(&$start)
            $(.mark_goal_state(&$goal))*
            $(.add_transition(&$from, &$edge, &$to))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        start {
            $start:expr
        }
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_dead_state(&$dead))*
            $(.mark_accept_state(&$accept))*
            $(.mark_goal_state(&$goal))*
            .mark_start_state(&$start)
            $(.add_transition(&$from, &$edge, &$to))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        start {
            $start:expr
        }
        accept {$(
            $accept:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_dead_state(&$dead))*
            $(.mark_goal_state(&$goal))*
            .mark_start_state(&$start)
            $(.mark_accept_state(&$accept))*
            $(.add_transition(&$from, &$edge, &$to))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        start {
            $start:expr
        }
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_dead_state(&$dead))*
            $(.mark_goal_state(&$goal))*
            $(.mark_accept_state(&$accept))*
            .mark_start_state(&$start)
            $(.add_transition(&$from, &$edge, &$to))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        start {
            $start:expr
        }
        goal{$(
            $goal:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_dead_state(&$dead))*
            .mark_start_state(&$start)
            $(.mark_goal_state(&$goal))*
            $(.mark_accept_state(&$accept))*
            $(.add_transition(&$from, &$edge, &$to))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        start {
            $start:expr
        }
        accept {$(
            $accept:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_dead_state(&$dead))*
            .mark_start_state(&$start)
            $(.mark_accept_state(&$accept))*
            $(.mark_goal_state(&$goal))*
            $(.add_transition(&$from, &$edge, &$to))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        start {
            $start:expr
        }
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_goal_state(&$goal))*
            $(.mark_accept_state(&$accept))*
            $(.mark_dead_state(&$dead))*
            .mark_start_state(&$start)
            $(.add_transition(&$from, &$edge, &$to))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        start {
            $start:expr
        }
        dead{$(
            $dead:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_goal_state(&$goal))*
            $(.mark_accept_state(&$accept))*
            .mark_start_state(&$start)
            $(.mark_dead_state(&$dead))*
            $(.add_transition(&$from, &$edge, &$to))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        start {
            $start:expr
        }
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_goal_state(&$goal))*
            $(.mark_dead_state(&$dead))*
            $(.mark_accept_state(&$accept))*
            .mark_start_state(&$start)
            $(.add_transition(&$from, &$edge, &$to))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        start {
            $start:expr
        }
        accept {$(
            $accept:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_goal_state(&$goal))*
            $(.mark_dead_state(&$dead))*
            .mark_start_state(&$start)
            $(.mark_accept_state(&$accept))*
            $(.add_transition(&$from, &$edge, &$to))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        start {
            $start:expr
        }
        accept {$(
            $accept:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_goal_state(&$goal))*
            .mark_start_state(&$start)
            $(.mark_accept_state(&$accept))*
            $(.mark_dead_state(&$dead))*
            $(.add_transition(&$from, &$edge, &$to))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            .mark_start_state(&$start)
            $(.mark_accept_state(&$accept))*
            $(.mark_goal_state(&$goal))*
            $(.mark_dead_state(&$dead))*
            $(.add_transition(&$from, &$edge, &$to))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        start {
            $start:expr
        }
        dead{$(
            $dead:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_goal_state(&$goal))*
            .mark_start_state(&$start)
            $(.mark_dead_state(&$dead))*
            $(.mark_accept_state(&$accept))*
            $(.add_transition(&$from, &$edge, &$to))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        start {
            $start:expr
        }
        accept {$(
            $accept:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            .mark_start_state(&$start)
            $(.mark_accept_state(&$accept))*
            $(.mark_goal_state(&$goal))*
            $(.mark_dead_state(&$dead))*
            $(.add_transition(&$from, &$edge, &$to))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            .mark_start_state(&$start)
            $(.mark_accept_state(&$accept))*
            $(.mark_goal_state(&$goal))*
            $(.mark_dead_state(&$dead))*
            $(.add_transition(&$from, &$edge, &$to))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        start {
            $start:expr
        }
        accept {$(
            $accept:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            .mark_start_state(&$start)
            $(.mark_accept_state(&$accept))*
            $(.mark_dead_state(&$dead))*
            $(.mark_goal_state(&$goal))*
            $(.add_transition(&$from, &$edge, &$to))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        start {
            $start:expr
        }
        dead{$(
            $dead:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            .mark_start_state(&$start)
            $(.mark_dead_state(&$dead))*
            $(.mark_goal_state(&$goal))*
            $(.mark_accept_state(&$accept))*
            $(.add_transition(&$from, &$edge, &$to))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        start {
            $start:expr
        }
        dead{$(
            $dead:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            .mark_start_state(&$start)
            $(.mark_dead_state(&$dead))*
            $(.mark_accept_state(&$accept))*
            $(.mark_goal_state(&$goal))*
            $(.add_transition(&$from, &$edge, &$to))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        start {
            $start:expr
        }
        goal{$(
            $goal:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            .mark_start_state(&$start)
            $(.mark_goal_state(&$goal))*
            $(.mark_dead_state(&$dead))*
            $(.mark_accept_state(&$accept))*
            $(.add_transition(&$from, &$edge, &$to))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        start {
            $start:expr
        }
        goal{$(
            $goal:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            .mark_start_state(&$start)
            $(.mark_goal_state(&$goal))*
            $(.mark_accept_state(&$accept))*
            $(.mark_dead_state(&$dead))*
            $(.add_transition(&$from, &$edge, &$to))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_accept_state(&$accept))*
            $(.mark_dead_state(&$dead))*
            $(.mark_goal_state(&$goal))*
            $(.add_transition(&$from, &$edge, &$to))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_accept_state(&$accept))*
            $(.mark_goal_state(&$goal))*
            $(.mark_dead_state(&$dead))*
            $(.add_transition(&$from, &$edge, &$to))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_dead_state(&$dead))*
            $(.mark_accept_state(&$accept))*
            $(.mark_goal_state(&$goal))*
            $(.add_transition(&$from, &$edge, &$to))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_dead_state(&$dead))*
            $(.mark_goal_state(&$goal))*
            $(.mark_accept_state(&$accept))*
            $(.add_transition(&$from, &$edge, &$to))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_goal_state(&$goal))*
            $(.mark_accept_state(&$accept))*
            $(.mark_dead_state(&$dead))*
            $(.add_transition(&$from, &$edge, &$to))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_goal_state(&$goal))*
            $(.mark_dead_state(&$dead))*
            $(.mark_accept_state(&$accept))*
            $(.add_transition(&$from, &$edge, &$to))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        start {
            $start:expr
        }
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_accept_state(&$accept))*
            $(.mark_dead_state(&$dead))*
            .mark_start_state(&$start)
            $(.add_transition(&$from, &$edge, &$to))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        start {
            $start:expr
        }
        dead{$(
            $dead:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_accept_state(&$accept))*
            .mark_start_state(&$start)
            $(.mark_dead_state(&$dead))*
            $(.add_transition(&$from, &$edge, &$to))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        start {
            $start:expr
        }
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_dead_state(&$dead))*
            $(.mark_accept_state(&$accept))*
            .mark_start_state(&$start)
            $(.add_transition(&$from, &$edge, &$to))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        start {
            $start:expr
        }
        accept {$(
            $accept:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_dead_state(&$dead))*
            .mark_start_state(&$start)
            $(.mark_accept_state(&$accept))*
            $(.add_transition(&$from, &$edge, &$to))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        start {
            $start:expr
        }
        accept {$(
            $accept:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            .mark_start_state(&$start)
            $(.mark_accept_state(&$accept))*
            $(.mark_dead_state(&$dead))*
            $(.add_transition(&$from, &$edge, &$to))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        start {
            $start:expr
        }
        dead{$(
            $dead:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            .mark_start_state(&$start)
            $(.mark_dead_state(&$dead))*
            $(.mark_accept_state(&$accept))*
            $(.add_transition(&$from, &$edge, &$to))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        start {
            $start:expr
        }
        goal{$(
            $goal:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_accept_state(&$accept))*
            .mark_start_state(&$start)
            $(.mark_goal_state(&$goal))*
            $(.add_transition(&$from, &$edge, &$to))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        start {
            $start:expr
        }
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_accept_state(&$accept))*
            $(.mark_goal_state(&$goal))*
            .mark_start_state(&$start)
            $(.add_transition(&$from, &$edge, &$to))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        start {
            $start:expr
        }
        accept {$(
            $accept:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_goal_state(&$goal))*
            .mark_start_state(&$start)
            $(.mark_accept_state(&$accept))*
            $(.add_transition(&$from, &$edge, &$to))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        start {
            $start:expr
        }
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_goal_state(&$goal))*
            $(.mark_accept_state(&$accept))*
            .mark_start_state(&$start)
            $(.add_transition(&$from, &$edge, &$to))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        start {
            $start:expr
        }
        goal{$(
            $goal:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            .mark_start_state(&$start)
            $(.mark_goal_state(&$goal))*
            $(.mark_accept_state(&$accept))*
            $(.add_transition(&$from, &$edge, &$to))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        start {
            $start:expr
        }
        accept {$(
            $accept:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            .mark_start_state(&$start)
            $(.mark_accept_state(&$accept))*
            $(.mark_goal_state(&$goal))*
            $(.add_transition(&$from, &$edge, &$to))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        start {
            $start:expr
        }
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_dead_state(&$dead))*
            $(.mark_goal_state(&$goal))*
            .mark_start_state(&$start)
            $(.add_transition(&$from, &$edge, &$to))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        start {
            $start:expr
        }
        goal{$(
            $goal:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_dead_state(&$dead))*
            .mark_start_state(&$start)
            $(.mark_goal_state(&$goal))*
            $(.add_transition(&$from, &$edge, &$to))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        start {
            $start:expr
        }
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_goal_state(&$goal))*
            $(.mark_dead_state(&$dead))*
            .mark_start_state(&$start)
            $(.add_transition(&$from, &$edge, &$to))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        start {
            $start:expr
        }
        dead{$(
            $dead:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_goal_state(&$goal))*
            .mark_start_state(&$start)
            $(.mark_dead_state(&$dead))*
            $(.add_transition(&$from, &$edge, &$to))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        start {
            $start:expr
        }
        dead{$(
            $dead:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            .mark_start_state(&$start)
            $(.mark_dead_state(&$dead))*
            $(.mark_goal_state(&$goal))*
            $(.add_transition(&$from, &$edge, &$to))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        start {
            $start:expr
        }
        goal{$(
            $goal:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            .mark_start_state(&$start)
            $(.mark_goal_state(&$goal))*
            $(.mark_dead_state(&$dead))*
            $(.add_transition(&$from, &$edge, &$to))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_dead_state(&$dead))*
            $(.mark_goal_state(&$goal))*
            $(.add_transition(&$from, &$edge, &$to))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_goal_state(&$goal))*
            $(.mark_dead_state(&$dead))*
            $(.add_transition(&$from, &$edge, &$to))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        start {
            $start:expr
        }
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_dead_state(&$dead))*
            .mark_start_state(&$start)
            $(.add_transition(&$from, &$edge, &$to))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        start {
            $start:expr
        }
        dead{$(
            $dead:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            .mark_start_state(&$start)
            $(.mark_dead_state(&$dead))*
            $(.add_transition(&$from, &$edge, &$to))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        start {
            $start:expr
        }
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_goal_state(&$goal))*
            .mark_start_state(&$start)
            $(.add_transition(&$from, &$edge, &$to))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        start {
            $start:expr
        }
        goal{$(
            $goal:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            .mark_start_state(&$start)
            $(.mark_goal_state(&$goal))*
            $(.add_transition(&$from, &$edge, &$to))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr

        ),*}
        dead{$(
            $dead:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_dead_state(&$dead))*
            $(.add_transition(&$from, &$edge, &$to))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        start {
            $start:expr
        }
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            .mark_start_state(&$start)
            $(.add_transition(&$from, &$edge, &$to))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_goal_state(&$goal))*
            $(.add_transition(&$from, &$edge, &$to))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        start {
            $start:expr
        }
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_accept_state(&$accept))*
            $(.mark_dead_state(&$dead))*
            $(.mark_goal_state(&$goal))*
            .mark_start_state(&$start)
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        start {
            $start:expr
        }
        goal{$(
            $goal:expr
        ),*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_accept_state(&$accept))*
            $(.mark_dead_state(&$dead))*
            .mark_start_state(&$start)
            $(.mark_goal_state(&$goal))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        start {
            $start:expr
        }
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_accept_state(&$accept))*
            $(.mark_goal_state(&$goal))*
            $(.mark_dead_state(&$dead))*
            .mark_start_state(&$start)
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        start {
            $start:expr
        }
        dead{$(
            $dead:expr
        ),*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_accept_state(&$accept))*
            $(.mark_goal_state(&$goal))*
            .mark_start_state(&$start)
            $(.mark_dead_state(&$dead))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        start {
            $start:expr
        }
        dead{$(
            $dead:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_accept_state(&$accept))*
            .mark_start_state(&$start)
            $(.mark_dead_state(&$dead))*
            $(.mark_goal_state(&$goal))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        start {
            $start:expr
        }
        goal{$(
            $goal:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_accept_state(&$accept))*
            .mark_start_state(&$start)
            $(.mark_goal_state(&$goal))*
            $(.mark_dead_state(&$dead))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        start {
            $start:expr
        }
        goal{$(
            $goal:expr
        ),*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_dead_state(&$dead))*
            $(.mark_accept_state(&$accept))*
            .mark_start_state(&$start)
            $(.mark_goal_state(&$goal))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        start {
            $start:expr
        }
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_dead_state(&$dead))*
            $(.mark_accept_state(&$accept))*
            $(.mark_goal_state(&$goal))*
            .mark_start_state(&$start)
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        start {
            $start:expr
        }
        accept {$(
            $accept:expr
        ),*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_dead_state(&$dead))*
            $(.mark_goal_state(&$goal))*
            .mark_start_state(&$start)
            $(.mark_accept_state(&$accept))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        start {
            $start:expr
        }
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_dead_state(&$dead))*
            $(.mark_goal_state(&$goal))*
            $(.mark_accept_state(&$accept))*
            .mark_start_state(&$start)
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        start {
            $start:expr
        }
        goal{$(
            $goal:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_dead_state(&$dead))*
            .mark_start_state(&$start)
            $(.mark_goal_state(&$goal))*
            $(.mark_accept_state(&$accept))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        start {
            $start:expr
        }
        accept {$(
            $accept:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_dead_state(&$dead))*
            .mark_start_state(&$start)
            $(.mark_accept_state(&$accept))*
            $(.mark_goal_state(&$goal))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        start {
            $start:expr
        }
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_goal_state(&$goal))*
            $(.mark_accept_state(&$accept))*
            $(.mark_dead_state(&$dead))*
            .mark_start_state(&$start)
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        start {
            $start:expr
        }
        dead{$(
            $dead:expr
        ),*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_goal_state(&$goal))*
            $(.mark_accept_state(&$accept))*
            .mark_start_state(&$start)
            $(.mark_dead_state(&$dead))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        start {
            $start:expr
        }
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_goal_state(&$goal))*
            $(.mark_dead_state(&$dead))*
            $(.mark_accept_state(&$accept))*
            .mark_start_state(&$start)
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        start {
            $start:expr
        }
        accept {$(
            $accept:expr
        ),*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_goal_state(&$goal))*
            $(.mark_dead_state(&$dead))*
            .mark_start_state(&$start)
            $(.mark_accept_state(&$accept))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        start {
            $start:expr
        }
        accept {$(
            $accept:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_goal_state(&$goal))*
            .mark_start_state(&$start)
            $(.mark_accept_state(&$accept))*
            $(.mark_dead_state(&$dead))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            .mark_start_state(&$start)
            $(.mark_accept_state(&$accept))*
            $(.mark_goal_state(&$goal))*
            $(.mark_dead_state(&$dead))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        start {
            $start:expr
        }
        dead{$(
            $dead:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_goal_state(&$goal))*
            .mark_start_state(&$start)
            $(.mark_dead_state(&$dead))*
            $(.mark_accept_state(&$accept))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        start {
            $start:expr
        }
        accept {$(
            $accept:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            .mark_start_state(&$start)
            $(.mark_accept_state(&$accept))*
            $(.mark_goal_state(&$goal))*
            $(.mark_dead_state(&$dead))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            .mark_start_state(&$start)
            $(.mark_accept_state(&$accept))*
            $(.mark_goal_state(&$goal))*
            $(.mark_dead_state(&$dead))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        start {
            $start:expr
        }
        accept {$(
            $accept:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            .mark_start_state(&$start)
            $(.mark_accept_state(&$accept))*
            $(.mark_dead_state(&$dead))*
            $(.mark_goal_state(&$goal))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        start {
            $start:expr
        }
        dead{$(
            $dead:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            .mark_start_state(&$start)
            $(.mark_dead_state(&$dead))*
            $(.mark_goal_state(&$goal))*
            $(.mark_accept_state(&$accept))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        start {
            $start:expr
        }
        dead{$(
            $dead:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            .mark_start_state(&$start)
            $(.mark_dead_state(&$dead))*
            $(.mark_accept_state(&$accept))*
            $(.mark_goal_state(&$goal))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        start {
            $start:expr
        }
        goal{$(
            $goal:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            .mark_start_state(&$start)
            $(.mark_goal_state(&$goal))*
            $(.mark_dead_state(&$dead))*
            $(.mark_accept_state(&$accept))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        start {
            $start:expr
        }
        goal{$(
            $goal:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            .mark_start_state(&$start)
            $(.mark_goal_state(&$goal))*
            $(.mark_accept_state(&$accept))*
            $(.mark_dead_state(&$dead))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_accept_state(&$accept))*
            $(.mark_dead_state(&$dead))*
            $(.mark_goal_state(&$goal))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_accept_state(&$accept))*
            $(.mark_goal_state(&$goal))*
            $(.mark_dead_state(&$dead))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_dead_state(&$dead))*
            $(.mark_accept_state(&$accept))*
            $(.mark_goal_state(&$goal))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_dead_state(&$dead))*
            $(.mark_goal_state(&$goal))*
            $(.mark_accept_state(&$accept))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_goal_state(&$goal))*
            $(.mark_accept_state(&$accept))*
            $(.mark_dead_state(&$dead))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_goal_state(&$goal))*
            $(.mark_dead_state(&$dead))*
            $(.mark_accept_state(&$accept))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        start {
            $start:expr
        }
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_accept_state(&$accept))*
            $(.mark_dead_state(&$dead))*
            .mark_start_state(&$start)
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        start {
            $start:expr
        }
        dead{$(
            $dead:expr
        ),*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_accept_state(&$accept))*
            .mark_start_state(&$start)
            $(.mark_dead_state(&$dead))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        start {
            $start:expr
        }
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_dead_state(&$dead))*
            $(.mark_accept_state(&$accept))*
            .mark_start_state(&$start)
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        start {
            $start:expr
        }
        accept {$(
            $accept:expr
        ),*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_dead_state(&$dead))*
            .mark_start_state(&$start)
            $(.mark_accept_state(&$accept))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        start {
            $start:expr
        }
        accept {$(
            $accept:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            .mark_start_state(&$start)
            $(.mark_accept_state(&$accept))*
            $(.mark_dead_state(&$dead))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        start {
            $start:expr
        }
        dead{$(
            $dead:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            .mark_start_state(&$start)
            $(.mark_dead_state(&$dead))*
            $(.mark_accept_state(&$accept))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        start {
            $start:expr
        }
        goal{$(
            $goal:expr
        ),*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_accept_state(&$accept))*
            .mark_start_state(&$start)
            $(.mark_goal_state(&$goal))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        start {
            $start:expr
        }
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_accept_state(&$accept))*
            $(.mark_goal_state(&$goal))*
            .mark_start_state(&$start)
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        start {
            $start:expr
        }
        accept {$(
            $accept:expr
        ),*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_goal_state(&$goal))*
            .mark_start_state(&$start)
            $(.mark_accept_state(&$accept))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        start {
            $start:expr
        }
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_goal_state(&$goal))*
            $(.mark_accept_state(&$accept))*
            .mark_start_state(&$start)
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        start {
            $start:expr
        }
        goal{$(
            $goal:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            .mark_start_state(&$start)
            $(.mark_goal_state(&$goal))*
            $(.mark_accept_state(&$accept))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        start {
            $start:expr
        }
        accept {$(
            $accept:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            .mark_start_state(&$start)
            $(.mark_accept_state(&$accept))*
            $(.mark_goal_state(&$goal))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        start {
            $start:expr
        }
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_dead_state(&$dead))*
            $(.mark_goal_state(&$goal))*
            .mark_start_state(&$start)
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        start {
            $start:expr
        }
        goal{$(
            $goal:expr
        ),*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_dead_state(&$dead))*
            .mark_start_state(&$start)
            $(.mark_goal_state(&$goal))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        start {
            $start:expr
        }
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_goal_state(&$goal))*
            $(.mark_dead_state(&$dead))*
            .mark_start_state(&$start)
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        start {
            $start:expr
        }
        dead{$(
            $dead:expr
        ),*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_goal_state(&$goal))*
            .mark_start_state(&$start)
            $(.mark_dead_state(&$dead))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        start {
            $start:expr
        }
        dead{$(
            $dead:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            .mark_start_state(&$start)
            $(.mark_dead_state(&$dead))*
            $(.mark_goal_state(&$goal))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        start {
            $start:expr
        }
        goal{$(
            $goal:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            .mark_start_state(&$start)
            $(.mark_goal_state(&$goal))*
            $(.mark_dead_state(&$dead))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_dead_state(&$dead))*
            $(.mark_goal_state(&$goal))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_goal_state(&$goal))*
            $(.mark_dead_state(&$dead))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        start {
            $start:expr
        }
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_dead_state(&$dead))*
            .mark_start_state(&$start)
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        start {
            $start:expr
        }
        dead{$(
            $dead:expr
        ),*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            .mark_start_state(&$start)
            $(.mark_dead_state(&$dead))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        start {
            $start:expr
        }
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_goal_state(&$goal))*
            .mark_start_state(&$start)
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        start {
            $start:expr
        }
        goal{$(
            $goal:expr
        ),*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            .mark_start_state(&$start)
            $(.mark_goal_state(&$goal))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr

        ),*}
        dead{$(
            $dead:expr
        ),*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_dead_state(&$dead))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        start {
            $start:expr
        }
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            .mark_start_state(&$start)
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        recognizes { $description:expr }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_goal_state(&$goal))*
            .recognizes($description)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        start {
            $start:expr
        }
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_accept_state(&$accept))*
            $(.mark_dead_state(&$dead))*
            $(.mark_goal_state(&$goal))*
            .mark_start_state(&$start)
            $(.add_transition(&$from, &$edge, &$to))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        start {
            $start:expr
        }
        goal{$(
            $goal:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_accept_state(&$accept))*
            $(.mark_dead_state(&$dead))*
            .mark_start_state(&$start)
            $(.mark_goal_state(&$goal))*
            $(.add_transition(&$from, &$edge, &$to))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        start {
            $start:expr
        }
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_accept_state(&$accept))*
            $(.mark_goal_state(&$goal))*
            $(.mark_dead_state(&$dead))*
            .mark_start_state(&$start)
            $(.add_transition(&$from, &$edge, &$to))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        start {
            $start:expr
        }
        dead{$(
            $dead:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_accept_state(&$accept))*
            $(.mark_goal_state(&$goal))*
            .mark_start_state(&$start)
            $(.mark_dead_state(&$dead))*
            $(.add_transition(&$from, &$edge, &$to))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        start {
            $start:expr
        }
        dead{$(
            $dead:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_accept_state(&$accept))*
            .mark_start_state(&$start)
            $(.mark_dead_state(&$dead))*
            $(.mark_goal_state(&$goal))*
            $(.add_transition(&$from, &$edge, &$to))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        start {
            $start:expr
        }
        goal{$(
            $goal:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_accept_state(&$accept))*
            .mark_start_state(&$start)
            $(.mark_goal_state(&$goal))*
            $(.mark_dead_state(&$dead))*
            $(.add_transition(&$from, &$edge, &$to))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        start {
            $start:expr
        }
        goal{$(
            $goal:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_dead_state(&$dead))*
            $(.mark_accept_state(&$accept))*
            .mark_start_state(&$start)
            $(.mark_goal_state(&$goal))*
            $(.add_transition(&$from, &$edge, &$to))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        start {
            $start:expr
        }
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_dead_state(&$dead))*
            $(.mark_accept_state(&$accept))*
            $(.mark_goal_state(&$goal))*
            .mark_start_state(&$start)
            $(.add_transition(&$from, &$edge, &$to))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        start {
            $start:expr
        }
        accept {$(
            $accept:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_dead_state(&$dead))*
            $(.mark_goal_state(&$goal))*
            .mark_start_state(&$start)
            $(.mark_accept_state(&$accept))*
            $(.add_transition(&$from, &$edge, &$to))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        start {
            $start:expr
        }
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_dead_state(&$dead))*
            $(.mark_goal_state(&$goal))*
            $(.mark_accept_state(&$accept))*
            .mark_start_state(&$start)
            $(.add_transition(&$from, &$edge, &$to))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        start {
            $start:expr
        }
        goal{$(
            $goal:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_dead_state(&$dead))*
            .mark_start_state(&$start)
            $(.mark_goal_state(&$goal))*
            $(.mark_accept_state(&$accept))*
            $(.add_transition(&$from, &$edge, &$to))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        start {
            $start:expr
        }
        accept {$(
            $accept:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_dead_state(&$dead))*
            .mark_start_state(&$start)
            $(.mark_accept_state(&$accept))*
            $(.mark_goal_state(&$goal))*
            $(.add_transition(&$from, &$edge, &$to))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        start {
            $start:expr
        }
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_goal_state(&$goal))*
            $(.mark_accept_state(&$accept))*
            $(.mark_dead_state(&$dead))*
            .mark_start_state(&$start)
            $(.add_transition(&$from, &$edge, &$to))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        start {
            $start:expr
        }
        dead{$(
            $dead:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_goal_state(&$goal))*
            $(.mark_accept_state(&$accept))*
            .mark_start_state(&$start)
            $(.mark_dead_state(&$dead))*
            $(.add_transition(&$from, &$edge, &$to))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        start {
            $start:expr
        }
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_goal_state(&$goal))*
            $(.mark_dead_state(&$dead))*
            $(.mark_accept_state(&$accept))*
            .mark_start_state(&$start)
            $(.add_transition(&$from, &$edge, &$to))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        start {
            $start:expr
        }
        accept {$(
            $accept:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_goal_state(&$goal))*
            $(.mark_dead_state(&$dead))*
            .mark_start_state(&$start)
            $(.mark_accept_state(&$accept))*
            $(.add_transition(&$from, &$edge, &$to))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        start {
            $start:expr
        }
        accept {$(
            $accept:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_goal_state(&$goal))*
            .mark_start_state(&$start)
            $(.mark_accept_state(&$accept))*
            $(.mark_dead_state(&$dead))*
            $(.add_transition(&$from, &$edge, &$to))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            .mark_start_state(&$start)
            $(.mark_accept_state(&$accept))*
            $(.mark_goal_state(&$goal))*
            $(.mark_dead_state(&$dead))*
            $(.add_transition(&$from, &$edge, &$to))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        start {
            $start:expr
        }
        dead{$(
            $dead:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_goal_state(&$goal))*
            .mark_start_state(&$start)
            $(.mark_dead_state(&$dead))*
            $(.mark_accept_state(&$accept))*
            $(.add_transition(&$from, &$edge, &$to))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        start {
            $start:expr
        }
        accept {$(
            $accept:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            .mark_start_state(&$start)
            $(.mark_accept_state(&$accept))*
            $(.mark_goal_state(&$goal))*
            $(.mark_dead_state(&$dead))*
            $(.add_transition(&$from, &$edge, &$to))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            .mark_start_state(&$start)
            $(.mark_accept_state(&$accept))*
            $(.mark_goal_state(&$goal))*
            $(.mark_dead_state(&$dead))*
            $(.add_transition(&$from, &$edge, &$to))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        start {
            $start:expr
        }
        accept {$(
            $accept:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            .mark_start_state(&$start)
            $(.mark_accept_state(&$accept))*
            $(.mark_dead_state(&$dead))*
            $(.mark_goal_state(&$goal))*
            $(.add_transition(&$from, &$edge, &$to))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        start {
            $start:expr
        }
        dead{$(
            $dead:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            .mark_start_state(&$start)
            $(.mark_dead_state(&$dead))*
            $(.mark_goal_state(&$goal))*
            $(.mark_accept_state(&$accept))*
            $(.add_transition(&$from, &$edge, &$to))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        start {
            $start:expr
        }
        dead{$(
            $dead:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            .mark_start_state(&$start)
            $(.mark_dead_state(&$dead))*
            $(.mark_accept_state(&$accept))*
            $(.mark_goal_state(&$goal))*
            $(.add_transition(&$from, &$edge, &$to))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        start {
            $start:expr
        }
        goal{$(
            $goal:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            .mark_start_state(&$start)
            $(.mark_goal_state(&$goal))*
            $(.mark_dead_state(&$dead))*
            $(.mark_accept_state(&$accept))*
            $(.add_transition(&$from, &$edge, &$to))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        start {
            $start:expr
        }
        goal{$(
            $goal:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            .mark_start_state(&$start)
            $(.mark_goal_state(&$goal))*
            $(.mark_accept_state(&$accept))*
            $(.mark_dead_state(&$dead))*
            $(.add_transition(&$from, &$edge, &$to))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_accept_state(&$accept))*
            $(.mark_dead_state(&$dead))*
            $(.mark_goal_state(&$goal))*
            $(.add_transition(&$from, &$edge, &$to))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_accept_state(&$accept))*
            $(.mark_goal_state(&$goal))*
            $(.mark_dead_state(&$dead))*
            $(.add_transition(&$from, &$edge, &$to))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_dead_state(&$dead))*
            $(.mark_accept_state(&$accept))*
            $(.mark_goal_state(&$goal))*
            $(.add_transition(&$from, &$edge, &$to))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_dead_state(&$dead))*
            $(.mark_goal_state(&$goal))*
            $(.mark_accept_state(&$accept))*
            $(.add_transition(&$from, &$edge, &$to))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_goal_state(&$goal))*
            $(.mark_accept_state(&$accept))*
            $(.mark_dead_state(&$dead))*
            $(.add_transition(&$from, &$edge, &$to))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_goal_state(&$goal))*
            $(.mark_dead_state(&$dead))*
            $(.mark_accept_state(&$accept))*
            $(.add_transition(&$from, &$edge, &$to))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        start {
            $start:expr
        }
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_accept_state(&$accept))*
            $(.mark_dead_state(&$dead))*
            .mark_start_state(&$start)
            $(.add_transition(&$from, &$edge, &$to))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        start {
            $start:expr
        }
        dead{$(
            $dead:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_accept_state(&$accept))*
            .mark_start_state(&$start)
            $(.mark_dead_state(&$dead))*
            $(.add_transition(&$from, &$edge, &$to))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        start {
            $start:expr
        }
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_dead_state(&$dead))*
            $(.mark_accept_state(&$accept))*
            .mark_start_state(&$start)
            $(.add_transition(&$from, &$edge, &$to))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        start {
            $start:expr
        }
        accept {$(
            $accept:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_dead_state(&$dead))*
            .mark_start_state(&$start)
            $(.mark_accept_state(&$accept))*
            $(.add_transition(&$from, &$edge, &$to))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        start {
            $start:expr
        }
        accept {$(
            $accept:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            .mark_start_state(&$start)
            $(.mark_accept_state(&$accept))*
            $(.mark_dead_state(&$dead))*
            $(.add_transition(&$from, &$edge, &$to))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        start {
            $start:expr
        }
        dead{$(
            $dead:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            .mark_start_state(&$start)
            $(.mark_dead_state(&$dead))*
            $(.mark_accept_state(&$accept))*
            $(.add_transition(&$from, &$edge, &$to))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        start {
            $start:expr
        }
        goal{$(
            $goal:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_accept_state(&$accept))*
            .mark_start_state(&$start)
            $(.mark_goal_state(&$goal))*
            $(.add_transition(&$from, &$edge, &$to))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        start {
            $start:expr
        }
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_accept_state(&$accept))*
            $(.mark_goal_state(&$goal))*
            .mark_start_state(&$start)
            $(.add_transition(&$from, &$edge, &$to))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        start {
            $start:expr
        }
        accept {$(
            $accept:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_goal_state(&$goal))*
            .mark_start_state(&$start)
            $(.mark_accept_state(&$accept))*
            $(.add_transition(&$from, &$edge, &$to))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        start {
            $start:expr
        }
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_goal_state(&$goal))*
            $(.mark_accept_state(&$accept))*
            .mark_start_state(&$start)
            $(.add_transition(&$from, &$edge, &$to))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        start {
            $start:expr
        }
        goal{$(
            $goal:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            .mark_start_state(&$start)
            $(.mark_goal_state(&$goal))*
            $(.mark_accept_state(&$accept))*
            $(.add_transition(&$from, &$edge, &$to))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        start {
            $start:expr
        }
        accept {$(
            $accept:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            .mark_start_state(&$start)
            $(.mark_accept_state(&$accept))*
            $(.mark_goal_state(&$goal))*
            $(.add_transition(&$from, &$edge, &$to))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        start {
            $start:expr
        }
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_dead_state(&$dead))*
            $(.mark_goal_state(&$goal))*
            .mark_start_state(&$start)
            $(.add_transition(&$from, &$edge, &$to))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        start {
            $start:expr
        }
        goal{$(
            $goal:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_dead_state(&$dead))*
            .mark_start_state(&$start)
            $(.mark_goal_state(&$goal))*
            $(.add_transition(&$from, &$edge, &$to))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        start {
            $start:expr
        }
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_goal_state(&$goal))*
            $(.mark_dead_state(&$dead))*
            .mark_start_state(&$start)
            $(.add_transition(&$from, &$edge, &$to))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        start {
            $start:expr
        }
        dead{$(
            $dead:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_goal_state(&$goal))*
            .mark_start_state(&$start)
            $(.mark_dead_state(&$dead))*
            $(.add_transition(&$from, &$edge, &$to))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        start {
            $start:expr
        }
        dead{$(
            $dead:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            .mark_start_state(&$start)
            $(.mark_dead_state(&$dead))*
            $(.mark_goal_state(&$goal))*
            $(.add_transition(&$from, &$edge, &$to))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        start {
            $start:expr
        }
        goal{$(
            $goal:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            .mark_start_state(&$start)
            $(.mark_goal_state(&$goal))*
            $(.mark_dead_state(&$dead))*
            $(.add_transition(&$from, &$edge, &$to))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_dead_state(&$dead))*
            $(.mark_goal_state(&$goal))*
            $(.add_transition(&$from, &$edge, &$to))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_goal_state(&$goal))*
            $(.mark_dead_state(&$dead))*
            $(.add_transition(&$from, &$edge, &$to))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        start {
            $start:expr
        }
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_dead_state(&$dead))*
            .mark_start_state(&$start)
            $(.add_transition(&$from, &$edge, &$to))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        start {
            $start:expr
        }
        dead{$(
            $dead:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            .mark_start_state(&$start)
            $(.mark_dead_state(&$dead))*
            $(.add_transition(&$from, &$edge, &$to))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        start {
            $start:expr
        }
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_goal_state(&$goal))*
            .mark_start_state(&$start)
            $(.add_transition(&$from, &$edge, &$to))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        start {
            $start:expr
        }
        goal{$(
            $goal:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            .mark_start_state(&$start)
            $(.mark_goal_state(&$goal))*
            $(.add_transition(&$from, &$edge, &$to))*
            .build()
    }};
    (
        states {$(
            $state:expr

        ),*}
        dead{$(
            $dead:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_dead_state(&$dead))*
            $(.add_transition(&$from, &$edge, &$to))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        start {
            $start:expr
        }
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            .mark_start_state(&$start)
            $(.add_transition(&$from, &$edge, &$to))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        transitions {$(
            $edge:expr => ($from:expr, $to:expr)
        )*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_goal_state(&$goal))*
            $(.add_transition(&$from, &$edge, &$to))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        start {
            $start:expr
        }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_accept_state(&$accept))*
            $(.mark_dead_state(&$dead))*
            $(.mark_goal_state(&$goal))*
            .mark_start_state(&$start)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        start {
            $start:expr
        }
        goal{$(
            $goal:expr
        ),*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_accept_state(&$accept))*
            $(.mark_dead_state(&$dead))*
            .mark_start_state(&$start)
            $(.mark_goal_state(&$goal))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        start {
            $start:expr
        }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_accept_state(&$accept))*
            $(.mark_goal_state(&$goal))*
            $(.mark_dead_state(&$dead))*
            .mark_start_state(&$start)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        start {
            $start:expr
        }
        dead{$(
            $dead:expr
        ),*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_accept_state(&$accept))*
            $(.mark_goal_state(&$goal))*
            .mark_start_state(&$start)
            $(.mark_dead_state(&$dead))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        start {
            $start:expr
        }
        dead{$(
            $dead:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_accept_state(&$accept))*
            .mark_start_state(&$start)
            $(.mark_dead_state(&$dead))*
            $(.mark_goal_state(&$goal))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        start {
            $start:expr
        }
        goal{$(
            $goal:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_accept_state(&$accept))*
            .mark_start_state(&$start)
            $(.mark_goal_state(&$goal))*
            $(.mark_dead_state(&$dead))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        start {
            $start:expr
        }
        goal{$(
            $goal:expr
        ),*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_dead_state(&$dead))*
            $(.mark_accept_state(&$accept))*
            .mark_start_state(&$start)
            $(.mark_goal_state(&$goal))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        start {
            $start:expr
        }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_dead_state(&$dead))*
            $(.mark_accept_state(&$accept))*
            $(.mark_goal_state(&$goal))*
            .mark_start_state(&$start)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        start {
            $start:expr
        }
        accept {$(
            $accept:expr
        ),*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_dead_state(&$dead))*
            $(.mark_goal_state(&$goal))*
            .mark_start_state(&$start)
            $(.mark_accept_state(&$accept))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        start {
            $start:expr
        }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_dead_state(&$dead))*
            $(.mark_goal_state(&$goal))*
            $(.mark_accept_state(&$accept))*
            .mark_start_state(&$start)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        start {
            $start:expr
        }
        goal{$(
            $goal:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_dead_state(&$dead))*
            .mark_start_state(&$start)
            $(.mark_goal_state(&$goal))*
            $(.mark_accept_state(&$accept))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        start {
            $start:expr
        }
        accept {$(
            $accept:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_dead_state(&$dead))*
            .mark_start_state(&$start)
            $(.mark_accept_state(&$accept))*
            $(.mark_goal_state(&$goal))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        start {
            $start:expr
        }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_goal_state(&$goal))*
            $(.mark_accept_state(&$accept))*
            $(.mark_dead_state(&$dead))*
            .mark_start_state(&$start)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        start {
            $start:expr
        }
        dead{$(
            $dead:expr
        ),*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_goal_state(&$goal))*
            $(.mark_accept_state(&$accept))*
            .mark_start_state(&$start)
            $(.mark_dead_state(&$dead))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        start {
            $start:expr
        }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_goal_state(&$goal))*
            $(.mark_dead_state(&$dead))*
            $(.mark_accept_state(&$accept))*
            .mark_start_state(&$start)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        start {
            $start:expr
        }
        accept {$(
            $accept:expr
        ),*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_goal_state(&$goal))*
            $(.mark_dead_state(&$dead))*
            .mark_start_state(&$start)
            $(.mark_accept_state(&$accept))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        start {
            $start:expr
        }
        accept {$(
            $accept:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_goal_state(&$goal))*
            .mark_start_state(&$start)
            $(.mark_accept_state(&$accept))*
            $(.mark_dead_state(&$dead))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            .mark_start_state(&$start)
            $(.mark_accept_state(&$accept))*
            $(.mark_goal_state(&$goal))*
            $(.mark_dead_state(&$dead))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        start {
            $start:expr
        }
        dead{$(
            $dead:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_goal_state(&$goal))*
            .mark_start_state(&$start)
            $(.mark_dead_state(&$dead))*
            $(.mark_accept_state(&$accept))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        start {
            $start:expr
        }
        accept {$(
            $accept:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            .mark_start_state(&$start)
            $(.mark_accept_state(&$accept))*
            $(.mark_goal_state(&$goal))*
            $(.mark_dead_state(&$dead))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            .mark_start_state(&$start)
            $(.mark_accept_state(&$accept))*
            $(.mark_goal_state(&$goal))*
            $(.mark_dead_state(&$dead))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        start {
            $start:expr
        }
        accept {$(
            $accept:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            .mark_start_state(&$start)
            $(.mark_accept_state(&$accept))*
            $(.mark_dead_state(&$dead))*
            $(.mark_goal_state(&$goal))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        start {
            $start:expr
        }
        dead{$(
            $dead:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            .mark_start_state(&$start)
            $(.mark_dead_state(&$dead))*
            $(.mark_goal_state(&$goal))*
            $(.mark_accept_state(&$accept))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        start {
            $start:expr
        }
        dead{$(
            $dead:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            .mark_start_state(&$start)
            $(.mark_dead_state(&$dead))*
            $(.mark_accept_state(&$accept))*
            $(.mark_goal_state(&$goal))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        start {
            $start:expr
        }
        goal{$(
            $goal:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            .mark_start_state(&$start)
            $(.mark_goal_state(&$goal))*
            $(.mark_dead_state(&$dead))*
            $(.mark_accept_state(&$accept))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        start {
            $start:expr
        }
        goal{$(
            $goal:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            .mark_start_state(&$start)
            $(.mark_goal_state(&$goal))*
            $(.mark_accept_state(&$accept))*
            $(.mark_dead_state(&$dead))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_accept_state(&$accept))*
            $(.mark_dead_state(&$dead))*
            $(.mark_goal_state(&$goal))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_accept_state(&$accept))*
            $(.mark_goal_state(&$goal))*
            $(.mark_dead_state(&$dead))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_dead_state(&$dead))*
            $(.mark_accept_state(&$accept))*
            $(.mark_goal_state(&$goal))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_dead_state(&$dead))*
            $(.mark_goal_state(&$goal))*
            $(.mark_accept_state(&$accept))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_goal_state(&$goal))*
            $(.mark_accept_state(&$accept))*
            $(.mark_dead_state(&$dead))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_goal_state(&$goal))*
            $(.mark_dead_state(&$dead))*
            $(.mark_accept_state(&$accept))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        start {
            $start:expr
        }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_accept_state(&$accept))*
            $(.mark_dead_state(&$dead))*
            .mark_start_state(&$start)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        start {
            $start:expr
        }
        dead{$(
            $dead:expr
        ),*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_accept_state(&$accept))*
            .mark_start_state(&$start)
            $(.mark_dead_state(&$dead))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        start {
            $start:expr
        }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_dead_state(&$dead))*
            $(.mark_accept_state(&$accept))*
            .mark_start_state(&$start)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        start {
            $start:expr
        }
        accept {$(
            $accept:expr
        ),*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_dead_state(&$dead))*
            .mark_start_state(&$start)
            $(.mark_accept_state(&$accept))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        start {
            $start:expr
        }
        accept {$(
            $accept:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            .mark_start_state(&$start)
            $(.mark_accept_state(&$accept))*
            $(.mark_dead_state(&$dead))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        start {
            $start:expr
        }
        dead{$(
            $dead:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            .mark_start_state(&$start)
            $(.mark_dead_state(&$dead))*
            $(.mark_accept_state(&$accept))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        start {
            $start:expr
        }
        goal{$(
            $goal:expr
        ),*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_accept_state(&$accept))*
            .mark_start_state(&$start)
            $(.mark_goal_state(&$goal))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        start {
            $start:expr
        }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_accept_state(&$accept))*
            $(.mark_goal_state(&$goal))*
            .mark_start_state(&$start)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        start {
            $start:expr
        }
        accept {$(
            $accept:expr
        ),*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_goal_state(&$goal))*
            .mark_start_state(&$start)
            $(.mark_accept_state(&$accept))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
        start {
            $start:expr
        }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_goal_state(&$goal))*
            $(.mark_accept_state(&$accept))*
            .mark_start_state(&$start)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        start {
            $start:expr
        }
        goal{$(
            $goal:expr
        ),*}
        accept {$(
            $accept:expr
        ),*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            .mark_start_state(&$start)
            $(.mark_goal_state(&$goal))*
            $(.mark_accept_state(&$accept))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        start {
            $start:expr
        }
        accept {$(
            $accept:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            .mark_start_state(&$start)
            $(.mark_accept_state(&$accept))*
            $(.mark_goal_state(&$goal))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        start {
            $start:expr
        }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_dead_state(&$dead))*
            $(.mark_goal_state(&$goal))*
            .mark_start_state(&$start)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        start {
            $start:expr
        }
        goal{$(
            $goal:expr
        ),*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_dead_state(&$dead))*
            .mark_start_state(&$start)
            $(.mark_goal_state(&$goal))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        start {
            $start:expr
        }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_goal_state(&$goal))*
            $(.mark_dead_state(&$dead))*
            .mark_start_state(&$start)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        start {
            $start:expr
        }
        dead{$(
            $dead:expr
        ),*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_goal_state(&$goal))*
            .mark_start_state(&$start)
            $(.mark_dead_state(&$dead))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        start {
            $start:expr
        }
        dead{$(
            $dead:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            .mark_start_state(&$start)
            $(.mark_dead_state(&$dead))*
            $(.mark_goal_state(&$goal))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        start {
            $start:expr
        }
        goal{$(
            $goal:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            .mark_start_state(&$start)
            $(.mark_goal_state(&$goal))*
            $(.mark_dead_state(&$dead))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_dead_state(&$dead))*
            $(.mark_goal_state(&$goal))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_goal_state(&$goal))*
            $(.mark_dead_state(&$dead))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        dead{$(
            $dead:expr
        ),*}
        start {
            $start:expr
        }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_dead_state(&$dead))*
            .mark_start_state(&$start)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        start {
            $start:expr
        }
        dead{$(
            $dead:expr
        ),*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            .mark_start_state(&$start)
            $(.mark_dead_state(&$dead))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
        start {
            $start:expr
        }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_goal_state(&$goal))*
            .mark_start_state(&$start)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        start {
            $start:expr
        }
        goal{$(
            $goal:expr
        ),*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            .mark_start_state(&$start)
            $(.mark_goal_state(&$goal))*
            .build()
    }};
    (
        states {$(
            $state:expr

        ),*}
        dead{$(
            $dead:expr
        ),*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_dead_state(&$dead))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        start {
            $start:expr
        }
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            .mark_start_state(&$start)
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
        goal{$(
            $goal:expr
        ),*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_goal_state(&$goal))*
            .build()
    }};
    (
        states {$(
            $state:expr
        ),*}
    ) => {{
            $crate::dfa::DFABuilder::default()
            $(.add_state(&$state))*
            .build()
    }};
}
