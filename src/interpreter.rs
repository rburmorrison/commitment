use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    io::{stderr, stdout, BufRead, BufReader, Write},
    path::Path,
    process::{Command, Stdio},
    sync::mpsc,
    thread,
};

use anyhow::Result;
use crossterm::style::{Color, Print, ResetColor, SetForegroundColor};

use crate::{
    config::{Config, Task},
    defs::APP_DATA_DIR,
    scriptgen::generate_script,
    temp::TFile,
};

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

    const fn data(&self) -> &String {
        match self {
            Self::Stdout(data) | Self::Stderr(data) => data,
        }
    }
}

fn run_script<P: AsRef<Path>>(path: P) -> Result<()> {
    let path = path.as_ref();

    let mut process = Command::new("bash")
        .arg(path.to_str().unwrap())
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

    Ok(())
}

/// Create a script on the file system.
fn create_script<S: AsRef<str>>(name: S, task: &Task) -> Result<TFile> {
    let name = name.as_ref();

    let mut hasher = DefaultHasher::new();
    name.hash(&mut hasher);
    let name_hash = hasher.finish();

    let path = format!("commitment-{name_hash}.tmp");
    let path = APP_DATA_DIR.join(path);

    let mut tfile = TFile::new(path)?;
    let script = generate_script(task)?;

    tfile.file.write_all(script.as_bytes())?;

    Ok(tfile)
}

/// Execute the commitment file.
pub fn interpret(config: &Config) -> Result<()> {
    for (name, task) in &config.tasks {
        draw_box(format!("TASK: {name}"));
        println!();

        let script = create_script(name, task)?;
        run_script(&script.path)?;
    }

    Ok(())
}
