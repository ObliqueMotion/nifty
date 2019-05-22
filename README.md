# dfa-gen
A Generic Deterministic Finite-State Automaton Generator Written in Rust

## Example

![example](images/example.png)

**Code Represntation**
```rust
use dfagen::DFABuilder;

fn main() {
    let q0 = "q0";
    let q1 = "q1";
    let q2 = "q2";
    let q3 = "q3";

    let mut dfa = DFABuilder::default()
        .add_state(&q0)
        .add_state(&q1)
        .add_state(&q2)
        .add_state(&q3)
        .mark_dead_state(&q3)
        .mark_start_state(&q0)
        .mark_accept_state(&q0)
        .mark_accept_state(&q1)
        .add_transition(&q0, &'a', &q1)
        .add_transition(&q0, &'b', &q3)
        .add_transition(&q1, &'a', &q1)
        .add_transition(&q1, &'b', &q2)
        .add_transition(&q2, &'a', &q1)
        .add_transition(&q2, &'b', &q2)
        .build();

    dbg!(&dfa);

    dbg!(dfa.evaluate("".chars()));
    dbg!(dfa.evaluate("a".chars()));
    dbg!(dfa.evaluate("b".chars()));
    dbg!(dfa.evaluate("aa".chars()));
    dbg!(dfa.evaluate("ab".chars()));
    dbg!(dfa.evaluate("abb".chars()));
    dbg!(dfa.evaluate("aba".chars()));
    dbg!(dfa.evaluate("abba".chars()));
    dbg!(dfa.evaluate("babba".chars()));
}
```

**Output**
```
[src/main.rs:26] &dfa = DFA {
    states: {
        "q0",
        "q1",
        "q2",
        "q3"
    },
    accept_states: {
        "q0",
        "q1"
    },
    dead_states: {
        "q3"
    },
    goal_states: {},
    transitions: {
        'a': {
            "q0": "q1",
            "q1": "q1",
            "q2": "q1"
        },
        'b': {
            "q0": "q3",
            "q1": "q2",
            "q2": "q2"
        }
    },
    start: Some(
        "q0"
    ),
    current: "q0"
}
[src/main.rs:28] dfa.recognize("".chars()) = Accept
[src/main.rs:29] dfa.recognize("a".chars()) = Accept
[src/main.rs:30] dfa.recognize("b".chars()) = Reject
[src/main.rs:31] dfa.recognize("aa".chars()) = Accept
[src/main.rs:32] dfa.recognize("ab".chars()) = Reject
[src/main.rs:33] dfa.recognize("abb".chars()) = Reject
[src/main.rs:34] dfa.recognize("aba".chars()) = Accept
[src/main.rs:35] dfa.recognize("abba".chars()) = Accept
[src/main.rs:36] dfa.recognize("babba".chars()) = Reject
```
