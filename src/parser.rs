use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, multispace0, none_of},
    combinator::{map, recognize},
    error::ParseError,
    multi::{many0, many0_count},
    sequence::{delimited, pair, preceded, separated_pair},
    Err, IResult,
};
use turingmachine::automata::{Move, Movements};

use self::string::parse_string;

mod string;

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing whitespace, returning the output of `inner`.
fn ws<'a, F, O, E: ParseError<&'a str>>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Fn(&'a str) -> IResult<&'a str, O, E> + 'a,
{
    delimited(multispace0, inner, multispace0)
}

type State<'a> = &'a str;
pub fn identifier(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        alt((alpha1, tag("_"))),
        many0_count(alt((alphanumeric1, tag("_")))),
    ))(input)
}

type Data = String;
pub fn data(input: &str) -> IResult<&str, Data> {
    alt((
        parse_string,
        map(
            recognize(many0_count(none_of(" \t\n\r(),\""))),
            String::from,
        ),
    ))(input)
}

pub fn empty_identifier(input: &str) -> IResult<&str, Data> {
    preceded(pair(tag("EMPTY:"), multispace0), data)(input)
}

pub fn initial_state(input: &str) -> IResult<&str, State> {
    preceded(pair(tag("INITIAL_STATE:"), multispace0), identifier)(input)
}

type CurrState<'a> = (State<'a>, Data);
pub fn curr_state(input: &str) -> IResult<&str, CurrState> {
    delimited(
        tag("("),
        separated_pair(ws(identifier), tag(","), ws(data)),
        tag(")"),
    )(input)
}

pub fn movement(input: &str) -> IResult<&str, Move> {
    alt((
        map(tag("R"), |_| Move::Right),
        map(tag("L"), |_| Move::Left),
    ))(input)
}

type NextState<'a> = (State<'a>, Data, Move);
pub fn next_state(input: &str) -> IResult<&str, NextState> {
    map(
        delimited(
            tag("("),
            separated_pair(
                ws(identifier),
                tag(","),
                separated_pair(ws(data), tag(","), ws(movement)),
            ),
            tag(")"),
        ),
        |(a, (b, c))| (a, b, c),
    )(input)
}

type StateChange<'a> = (CurrState<'a>, NextState<'a>);
pub fn state_change(input: &str) -> IResult<&str, StateChange> {
    separated_pair(curr_state, tag(":"), ws(next_state))(input)
}

pub fn turing_machine_def(input: &str) -> IResult<&str, ((Data, State), Vec<StateChange>)> {
    pair(
        pair(ws(empty_identifier), ws(initial_state)),
        many0(ws(state_change)),
    )(input)
}

#[allow(clippy::type_complexity)]
pub fn turing_machine_movements(
    input: &str,
) -> Result<((Data, State), Movements<State, Data>), Err<nom::error::Error<&str>>> {
    turing_machine_def(input).map(|(res, (default, moves))| {
        if res.is_empty() {
            (default, moves.into_iter().collect())
        } else {
            panic!("NON EMPTY RESULT")
        }
    })
}

#[cfg(test)]
mod tests {
    pub use super::*;

    #[test]
    fn test_identifier() {
        assert!(identifier("q0")
            .map(|(r, x)| x == "q0" && r.is_empty())
            .unwrap_or_default())
    }

    #[test]
    fn test_empty() {
        assert!(empty_identifier("EMPTY: #")
            .map(|(r, x)| x == "#" && r.is_empty())
            .unwrap_or_default())
    }

    #[test]
    fn test_curr_state() {
        assert!(dbg!(curr_state("(q0, +)"))
            .map(|(r, x)| x == ("q0", "+".into()) && r.is_empty())
            .unwrap_or_default())
    }

    #[test]
    fn test_next_state() {
        assert!(dbg!(next_state("(q0, +, R)"))
            .map(|(r, x)| x == ("q0", "+".into(), Move::Right) && r.is_empty())
            .unwrap_or_default())
    }

    #[test]
    fn test_state_change() {
        assert!(dbg!(state_change("(q0, +): (q0, +, R)"))
            .map(
                |(r, x)| x == (("q0", "+".into()), ("q0", "+".into(), Move::Right)) && r.is_empty()
            )
            .unwrap_or_default())
    }
}
