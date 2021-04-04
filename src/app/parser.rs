use super::command::Command;
use super::command::Vector2;
use nom::combinator::opt;
use nom::{branch::alt, character::complete::*, combinator::recognize};
use nom::{bytes::complete::tag, IResult};
use nom::{combinator::map_res, sequence::*};
use std::str::FromStr;

type Result<'a, T> = IResult<&'a str, T>;

pub fn command(text: &str) -> Command {
    alt((move_mouse, left_click, right_click, scroll))(text)
        .unwrap()
        .1
}

fn move_mouse(text: &str) -> Result<Command> {
    let (rest, (x, y)) = preceded(
        pair(tag("move"), space1),
        pair(i32, preceded(char(','), i32)),
    )(text)?;
    Ok((rest, Command::Move(Vector2 { x, y })))
}

fn scroll(text: &str) -> Result<Command> {
    let (rest, (x, y)) = preceded(
        pair(tag("scroll"), space1),
        pair(i32, preceded(char(','), i32)),
    )(text)?;
    Ok((rest, Command::Scroll(Vector2 { x, y })))
}

fn left_click(text: &str) -> Result<Command> {
    let (rest, _) = tag("lclick")(text)?;
    Ok((rest, Command::LeftClick))
}

fn right_click(text: &str) -> Result<Command> {
    let (rest, _) = tag("rclick")(text)?;
    Ok((rest, Command::RightClick))
}

fn i32(input: &str) -> Result<i32> {
    map_res(recognize(pair(opt(char('-')), digit1)), FromStr::from_str)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move() {
        assert_eq!(
            move_mouse("move 2,0"),
            Ok(("", Command::Move(Vector2 { x: 2, y: 0 })))
        );
    }
}
