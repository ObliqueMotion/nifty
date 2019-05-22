use crate::dfa::DFABuilder;

#[macro_export]
macro_rules! dfa {
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
        recognizes { $description:expr }
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
        recognizes { $description:expr }
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
        recognizes { $description:expr }
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
        recognizes { $description:expr }
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
        recognizes { $description:expr }
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
        recognizes { $description:expr }
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
        recognizes { $description:expr }
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
        recognizes { $description:expr }
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
        recognizes { $description:expr }
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
        recognizes { $description:expr }
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
        recognizes { $description:expr }
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
        recognizes { $description:expr }
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
        recognizes { $description:expr }
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
        recognizes { $description:expr }
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
        recognizes { $description:expr }
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
        recognizes { $description:expr }
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
        recognizes { $description:expr }
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
        recognizes { $description:expr }
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
        recognizes { $description:expr }
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
        recognizes { $description:expr }
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
        recognizes { $description:expr }
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
        recognizes { $description:expr }
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
        recognizes { $description:expr }
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
        recognizes { $description:expr }
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
        recognizes { $description:expr }
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
        recognizes { $description:expr }
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
        recognizes { $description:expr }
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
        recognizes { $description:expr }
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
        recognizes { $description:expr }
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
        recognizes { $description:expr }
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
        recognizes { $description:expr }
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
        recognizes { $description:expr }
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
        recognizes { $description:expr }
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
        recognizes { $description:expr }
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
        recognizes { $description:expr }
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
        recognizes { $description:expr }
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
        recognizes { $description:expr }
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
        recognizes { $description:expr }
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
        recognizes { $description:expr }
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
        recognizes { $description:expr }
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
        recognizes { $description:expr }
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
        recognizes { $description:expr }
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
        recognizes { $description:expr }
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
        recognizes { $description:expr }
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
        recognizes { $description:expr }
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
        recognizes { $description:expr }
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
        recognizes { $description:expr }
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
        recognizes { $description:expr }
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
        recognizes { $description:expr }
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
        recognizes { $description:expr }
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
        recognizes { $description:expr }
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
        recognizes { $description:expr }
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
        recognizes { $description:expr }
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
        recognizes { $description:expr }
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
        recognizes { $description:expr }
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
        recognizes { $description:expr }
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
        recognizes { $description:expr }
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
        recognizes { $description:expr }
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
        recognizes { $description:expr }
    ) => {{
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
    ) => {{
            DFABuilder::default()
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
            $edge:expr => ($from:expr, $to:expr),
        )*}
    ) => {{
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
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
            DFABuilder::default()
            $(.add_state(&$state))*
            $(.mark_goal_state(&$goal))*
            .build()
    }};
}