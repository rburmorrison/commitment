use anyhow::Result;

use crate::{config::Config, parsing::parse_command};

fn draw_box<S: AsRef<str>>(input: S) {
    let input = input.as_ref();

    println!("┌{}┐", "─".repeat(input.len() + 10));
    println!("│     {input}     │");
    println!("└{}┘", "─".repeat(input.len() + 10));
}

/// Execute the commitment file.
pub fn interpret(config: &Config) -> Result<()> {
    for (key, value) in &config.tasks {
        draw_box(key);
        println!();

        for command in &value.execute {
            println!(">>> {command} <<<");
            println!("{}", "─".repeat(command.len() + 8));

            let result = parse_command(command)?;
            println!("{result:?}");
        }
        println!();
    }

    Ok(())
}
