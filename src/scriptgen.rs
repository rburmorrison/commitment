use std::fmt::Write;

use anyhow::Result;

use crate::config::Task;

enum Instruction<'a> {
    /// A verbatim command ran "as is".
    Command(&'a str),

    /// An `echo` that will be ignored for line numbers.
    LIgnore(&'a str),

    /// Resets the line number count.
    ResetLines,
}

impl<'a> ToString for Instruction<'a> {
    fn to_string(&self) -> String {
        match *self {
            Self::Command(command) => command.to_string(),
            Self::LIgnore(output) => format!("echo 'CMT-LIGNORE:{output}'"),
            Self::ResetLines => "echo 'CMT-RESET_LINES:'".to_string(),
        }
    }
}

macro_rules! write_instruction {
    ($string:expr, $instruction:expr) => {{
        let string = $instruction.to_string();
        writeln!($string, "{}", string.as_str())
    }};
}

pub fn generate_script(task: &Task) -> Result<String> {
    let mut s = String::new();

    writeln!(s, "#!/usr/bin/env sh")?;
    writeln!(s)?;
    writeln!(s, "set -e")?;
    writeln!(s)?;

    for (idx, command) in task.execute.iter().enumerate() {
        write_instruction!(s, Instruction::ResetLines)?;

        let header = format!(">>> {command} <<<");
        write_instruction!(s, Instruction::LIgnore(header.as_str()))?;

        let bar = "─".repeat(command.len() + 8);
        write_instruction!(s, Instruction::LIgnore(bar.as_str()))?;

        write_instruction!(s, Instruction::Command(command.as_str()))?;

        if idx != task.execute.len() - 1 {
            write_instruction!(s, Instruction::LIgnore(""))?;
        }
    }

    Ok(s)
}
