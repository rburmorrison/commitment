use std::collections::HashMap;

use config::CommitmentFile;

use crate::config::Task;

mod config;

fn main() {
    let mut tasks = HashMap::new();
    tasks.insert(
        "cargo-clippy".to_owned(),
        Task {
            can_fail: None,
            execute: vec!["cargo clippy -- -D warnings".to_owned()],
        },
    );
    tasks.insert(
        "cargo-fmt".to_owned(),
        Task {
            can_fail: None,
            execute: vec!["cd subdirectory".to_owned(), "cargo fmt --check".to_owned()],
        },
    );

    let commitment_file = CommitmentFile {
        tasks,
        ..Default::default()
    };
    println!("{commitment_file:#?}");
}
