//! Custom parsing functions for Commitment.

use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::{escaped, is_not, take_till1},
    character::complete::{char, one_of, space1},
    combinator::all_consuming,
    multi::many0,
    sequence::{delimited, preceded, tuple},
    IResult,
};

fn section(input: &str) -> IResult<&str, &str> {
    alt((
        delimited(
            char('"'),
            escaped(is_not("\n\""), '\\', one_of(r#"\""#)),
            char('"'),
        ),
        delimited(
            char('\''),
            escaped(is_not("\n\'"), '\\', one_of(r#"\'"#)),
            char('\''),
        ),
        take_till1(|ch: char| ch.is_ascii_whitespace()),
    ))(input)
}

fn command(input: &str) -> IResult<&str, (&str, Vec<&str>)> {
    tuple((section, many0(preceded(space1, section))))(input)
}

/// Parse a command string and break separate it into its components.
///
/// This method returns a tuple, where the first item is the executable and the
/// second is a vector of zero or more arguments.
pub fn parse_command(input: &str) -> Result<(&str, Vec<&str>)> {
    #[allow(clippy::redundant_closure_for_method_calls)]
    let (_, result) = all_consuming(command)(input).map_err(|err| err.to_owned())?;

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_command_simple() -> Result<()> {
        let input = "cargo clippy -- -D warnings";
        assert_eq!(
            parse_command(input)?,
            ("cargo", vec!["clippy", "--", "-D", "warnings"])
        );

        Ok(())
    }

    #[test]
    fn test_parse_command_no_args() -> Result<()> {
        let input = "cargo";
        assert_eq!(parse_command(input)?, ("cargo", vec![]));

        Ok(())
    }

    #[test]
    fn test_parse_command_quoted() -> Result<()> {
        let input = "eslint 'src/**/*.{ts,tsx}' --max-warnings=0";
        assert_eq!(
            parse_command(input)?,
            ("eslint", vec!["src/**/*.{ts,tsx}", "--max-warnings=0"])
        );

        let input = r#"eslint "src/**/*.{ts,tsx}" --max-warnings=0"#;
        assert_eq!(
            parse_command(input)?,
            ("eslint", vec!["src/**/*.{ts,tsx}", "--max-warnings=0"])
        );

        Ok(())
    }

    #[test]
    fn test_parse_command_quoted_spaces() -> Result<()> {
        let input = r#"echo "Hello, world!" 'Hello, there!'"#;
        assert_eq!(
            parse_command(input)?,
            ("echo", vec!["Hello, world!", "Hello, there!"])
        );

        Ok(())
    }

    #[test]
    fn test_parse_command_quoted_unterminated() -> Result<()> {
        let input = r#"echo "Hello 'Hello, there!'"#;
        assert_eq!(
            parse_command(input)?,
            ("echo", vec!["\"Hello", "Hello, there!"])
        );

        Ok(())
    }
}
