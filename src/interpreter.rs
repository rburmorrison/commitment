use std::{
    io::{stderr, stdout, BufRead, BufReader},
    process::{Command, Stdio},
    sync::mpsc,
    thread,
};

use anyhow::Result;
use crossterm::style::{Color, Print, ResetColor, SetForegroundColor};

use crate::{config::Config, parsing::parse_command};

fn draw_box<S: AsRef<str>>(input: S) {
    let input = input.as_ref();

    println!("┌{}┐", "─".repeat(input.len() + 10));
    println!("│     {input}     │");
    println!("└{}┘", "─".repeat(input.len() + 10));
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
}

fn run_command<S: AsRef<str>>(command: S) -> Result<()> {
    let (exe, args) = parse_command(command.as_ref())?;
    let mut process = Command::new(exe)
        .args(args)
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

    let mut count = 1;
    while let Ok(output) = rx.recv() {
        output.print(count)?;
        count += 1;
    }

    stdout_thread.join().unwrap()?;
    stderr_thread.join().unwrap()?;

    let result = process.wait()?;
    println!("Status Code: {}", result.code().unwrap());

    Ok(())
}

/// Execute the commitment file.
pub fn interpret(config: &Config) -> Result<()> {
    for (key, value) in &config.tasks {
        draw_box(format!("TASK: {key}"));
        println!();

        for command in &value.execute {
            println!(">>> {command} <<<");
            println!("{}", "─".repeat(command.len() + 8));

            run_command(command)?;

            println!();
        }

        println!();
    }

    Ok(())
}
