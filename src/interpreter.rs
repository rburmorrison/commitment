use std::{
    io::{stderr, stdout, BufRead, BufReader, Write},
    process::{ChildStdin, Command, ExitStatus, Stdio},
    sync::mpsc,
    thread,
};

use anyhow::{bail, Result};
use crossterm::style::{Color, Print, ResetColor, SetForegroundColor, Stylize};
use indexmap::IndexMap;
use thiserror::Error;

use crate::config::{Config, Task};

#[derive(Debug, Error)]
pub enum Error {
    /// Occurs when a task returns a non-zero error code.
    #[error(r#"task "{0}" returned a non-zero exit code"#)]
    TaskFailed(String),
}

fn draw_box<S: AsRef<str>>(input: S) {
    let input = input.as_ref();

    println!("┌{}┐", "─".repeat(input.len() + 10));
    println!("│     {input}     │");
    println!("└{}┘", "─".repeat(input.len() + 10));
}

fn draw_thick_box<S: AsRef<str>>(input: S) {
    let input = input.as_ref();

    println!("╔{}╗", "═".repeat(input.len() + 20));
    println!("║          {}          ║", input.underlined());
    println!("╚{}╝", "═".repeat(input.len() + 20));
}

enum Output {
    Stdout(String),
    Stderr(String),
}

impl Output {
    fn print(&self, line_number: usize) -> Result<()> {
        let line_number = format!("{line_number:04}: ");

        match self {
            Self::Stdout(line) => {
                crossterm::execute!(
                    stdout(),
                    ResetColor,
                    Print(line_number),
                    ResetColor,
                    Print(line),
                    Print("\n")
                )?;
            }
            Self::Stderr(line) => crossterm::execute!(
                stderr(),
                SetForegroundColor(Color::Red),
                Print(line_number),
                ResetColor,
                Print(line),
                Print("\n")
            )?,
        };

        Ok(())
    }

    const fn data(&self) -> &String {
        match self {
            Self::Stdout(data) | Self::Stderr(data) => data,
        }
    }
}

/// Send commands to stdin and exit.
///
/// Certain directives are specified in echo statements that change the behavior
/// of the output. Directives are:
///
/// - `CMT-LIGNORE` - Don't print the line number along with the line.
/// - `CMT-RESET_LINES` - Reset the line number counter
fn inject_steps(task: &Task, stdin: &mut ChildStdin) -> Result<()> {
    writeln!(stdin, "set -e")?;
    for (idx, step) in task.execute.iter().enumerate() {
        writeln!(stdin, "echo 'CMT-RESET_LINES:'")?;
        writeln!(stdin, "echo 'CMT-LIGNORE:>>> {step} <<<'")?;
        writeln!(stdin, "echo 'CMT-LIGNORE:{}'", "-".repeat(step.len() + 8))?;
        writeln!(stdin, "{step}")?;

        if idx != task.execute.len() - 1 {
            writeln!(stdin, "echo 'CMT-LIGNORE:'")?;
        }
    }
    writeln!(stdin, "exit")?;

    Ok(())
}

fn execute_task(task: &Task) -> Result<ExitStatus> {
    let mut process = Command::new("sh")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    // To show both the stdout and stderr, we need to create two threads and get
    // the output lines via channels.

    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    let stdout = process.stdout.take();
    let stdout_thread = thread::spawn(move || {
        let reader = BufReader::new(stdout.unwrap());
        for line in reader.lines() {
            let line = line?;
            tx.send(Output::Stdout(line))?;
        }

        Ok::<(), anyhow::Error>(())
    });

    let stderr = process.stderr.take();
    let stderr_thread = thread::spawn(move || {
        let reader = BufReader::new(stderr.unwrap());
        for line in reader.lines() {
            let line = line?;
            tx1.send(Output::Stderr(line))?;
        }

        Ok::<(), anyhow::Error>(())
    });

    let stdin = process.stdin.as_mut().expect("failed to access stdin");
    inject_steps(task, stdin)?;

    let mut count = 1;
    while let Ok(output) = rx.recv() {
        let data = output.data();
        if let Some(data) = data.strip_prefix("CMT-LIGNORE:") {
            println!("{data}");
        } else if data.as_str() == "CMT-RESET_LINES:" {
            count = 1;
        } else {
            output.print(count)?;
            count += 1;
        }
    }

    stdout_thread.join().unwrap()?;
    stderr_thread.join().unwrap()?;

    Ok(process.wait()?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// The outcome of a task.
enum TaskResult {
    /// The task was fully successful.
    Success,

    /// The task failed to complete.
    Failure,

    // The task failed, but it was failable.
    Skipped,
}

fn display_results(results: &IndexMap<String, Option<TaskResult>>) {
    draw_thick_box("RESULTS");
    println!();

    let longest_name = results
        .keys()
        .map(std::string::String::len)
        .max()
        .unwrap_or_default();

    let mut successes = 0;
    for (name, result) in results {
        let colored_block = format!("{}{name}", " ".repeat(longest_name - name.len()))
            .black()
            .on_blue();

        let result = result.map_or_else(
            || "IGNORED".black().on_grey(),
            |result| match result {
                TaskResult::Success => {
                    successes += 1;
                    "SUCCESS".black().on_green()
                }
                TaskResult::Failure => "FAILURE".black().on_red(),
                TaskResult::Skipped => "SKIPPED".black().on_yellow(),
            },
        );

        println!("{colored_block:>longest_name$}....................{result}");
    }

    let tasks_count = results.len();

    #[allow(clippy::cast_precision_loss)]
    let pass_percent = (successes as f32) / (tasks_count as f32) * 100.0;

    println!();
    println!("PASSED: {successes}/{tasks_count} ({pass_percent:.2}%)",);
}

/// Execute the commitment file.
pub fn interpret(config: &Config) -> Result<()> {
    let mut results: IndexMap<String, Option<TaskResult>> = IndexMap::new();

    for (name, _) in &config.tasks {
        results.insert(name.clone(), None);
    }

    for (name, task) in &config.tasks {
        draw_box(format!("TASK: {name}"));
        println!();

        let result = execute_task(task)?;
        let result = if !result.success() && task.can_fail {
            TaskResult::Skipped
        } else if result.success() {
            TaskResult::Success
        } else {
            TaskResult::Failure
        };

        results.insert(name.clone(), Some(result));

        println!();

        if result == TaskResult::Failure {
            break;
        }
    }

    display_results(&results);

    for (name, result) in results {
        if let Some(result) = result {
            if result == TaskResult::Failure {
                bail!(Error::TaskFailed(name));
            }
        }
    }

    Ok(())
}
