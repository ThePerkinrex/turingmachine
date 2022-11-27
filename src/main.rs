use std::{collections::HashMap, fmt::Display};

use automata::{Move, Movements};
use tape::Tape;

mod automata;
mod tape;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Data {
    Space,
    One,
    Zero,
    Plus,
    Equals,
}

impl Default for Data {
    fn default() -> Self {
        Self::Space
    }
}

impl Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Space => '#',
                Self::One => '1',
                Self::Zero => '0',
                Self::Plus => '+',
                Self::Equals => '=',
            }
        )
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[allow(non_camel_case_types)]
enum State {
    q0,
    q1,
    q2,
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

fn main() {
    {
        println!("#u+v=# >> #u+v=uv#");
        let tape = vec![Data::One, Data::Plus, Data::One, Data::One, Data::Equals];

        let mut movements: Movements<State, Data> = HashMap::with_capacity(10);
        movements.insert(
            (State::q0, Data::Plus),
            (State::q0, Data::Plus, Move::Right),
        );
        movements.insert((State::q0, Data::One), (State::q1, Data::Zero, Move::Right));

        movements.insert((State::q1, Data::One), (State::q1, Data::One, Move::Right));
        movements.insert(
            (State::q1, Data::Plus),
            (State::q1, Data::Plus, Move::Right),
        );
        movements.insert(
            (State::q1, Data::Equals),
            (State::q1, Data::Equals, Move::Right),
        );
        movements.insert((State::q1, Data::Space), (State::q2, Data::One, Move::Left));

        movements.insert((State::q2, Data::One), (State::q2, Data::One, Move::Left));
        movements.insert((State::q2, Data::Plus), (State::q2, Data::Plus, Move::Left));
        movements.insert(
            (State::q2, Data::Equals),
            (State::q2, Data::Equals, Move::Left),
        );
        movements.insert((State::q2, Data::Zero), (State::q0, Data::One, Move::Right));

        let mut automata =
            automata::TuringMachine::new(movements, Tape::new_with_data(tape), State::q0);
        loop {
            println!("{}", automata.tape().display(automata.state()));
            match automata.run() {
                automata::StepResult::Stopped(s, t) => {
                    println!("STOPPED: state {s}");
                    println!("{}", t);
                    break;
                }
                automata::StepResult::Running(a) => automata = a,
            }
        }
    }
}
