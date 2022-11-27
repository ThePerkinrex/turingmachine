use std::{collections::HashMap, hash::Hash};

use crate::tape::Tape;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Move {
    Left,
    Right,
}

pub type NextState<State, Data> = (State, Data, Move);
pub type CurrState<State, Data> = (State, Data);

pub type Movements<State, Data> = HashMap<CurrState<State, Data>, NextState<State, Data>>;

pub struct TuringMachine<State, Data>
where
    Data: Default,
{
    movements: Movements<State, Data>,
    tape: Tape<Data>,
    state: State,
}

pub enum StepResult<State, Data: Default> {
    Stopped(State, Tape<Data>),
    Running(TuringMachine<State, Data>),
}

impl<Data, State> TuringMachine<State, Data>
where
    State: Hash + Eq + Clone,
    Data: Default + Clone + Hash + Eq,
{
    pub const fn new(movements: Movements<State, Data>, tape: Tape<Data>, state: State) -> Self {
        Self {
            movements,
            tape,
            state,
        }
    }

    pub fn run(mut self) -> StepResult<State, Data> {
        match self
            .movements
            .get(&(self.state.clone(), self.tape.read().clone()))
        {
            Some((new_state, new_data, m)) => {
                self.tape.write(new_data.clone());
                self.state = new_state.clone();
                match m {
                    Move::Left => self.tape.move_left(),
                    Move::Right => self.tape.move_right(),
                }
                StepResult::Running(self)
            }
            None => StepResult::Stopped(self.state, self.tape),
        }
    }

    pub const fn tape(&self) -> &Tape<Data> {
        &self.tape
    }

    pub const fn state(&self) -> &State {
        &self.state
    }
}
