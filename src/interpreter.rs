use crate::config::Config;

pub fn interpret(config: &Config) {
    for (key, value) in &config.tasks {
        println!("{key}: {value:?}");
    }
}
