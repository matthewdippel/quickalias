use std;
use std::path::PathBuf;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::process::Command;

/// Get default path to the file we will store aliases.
pub fn default_path() -> PathBuf {
    match std::env::home_dir() {
        Some(path) => path.join(".quick_aliases.sh"),
        None => panic!("no home directory in env"),
    }
}

/// Get the default path to the user's bash history file.
pub fn default_history_path() -> PathBuf {
    match std::env::home_dir() {
        Some(path) => path.join(".bash_history"),
        None => panic!("no home directory in env"),
    }
}

pub struct AliasConfig {
    config_location: PathBuf,
    aliases: HashMap<String, String>,
}

impl AliasConfig {
    pub fn new(config_location: PathBuf) -> AliasConfig {
        AliasConfig {
            config_location: config_location,
            aliases: HashMap::new(),
        }
    }

    /// Load the aliases from the config file into memory.
    pub fn load(&mut self) {
        if self.config_location.exists() {
            let file = File::open(&self.config_location).unwrap();
            let reader = BufReader::new(file);
            for line in reader.lines() {
                let l = line.unwrap();
                self.handle_line(l);
            }
        } else {
            // do nothing
        }
    }

    /// Output a debug representation of the aliases mapping to stdout.
    pub fn debug(&self) {
        println!("state of alias mapping: ");
        println!("{:?}", self.aliases);
    }

    /// Parse a line from a file, looking for aliases.
    fn handle_line(&mut self, line: String) {
        let mut split = line.split(" ");
        if split.next() == Some("alias") {
            let rest = split.collect::<Vec<_>>().join(" ");
            // do some stuff with the rest of the string
            let mut split_eq = rest.split("=");
            let alias = split_eq.next().unwrap();
            let command = split_eq.collect::<Vec<_>>().join("=");
            let l = command.len();
            let command_uq = &command[1..l - 1];
            self.add_alias(alias.to_string(), command_uq.to_string());
        }
    }

    /// Add an alias and associated command to the alias mapping.
    pub fn add_alias(&mut self, alias: String, command: String) {
        self.aliases.insert(alias, command);
    }

    /// Remove an alias from the alias mapping if it exists.
    /// Returns Some(command) if there was a command associated with this alias.
    /// else returns None.
    pub fn remove_alias(&mut self, alias: String) -> Option<String> {
        self.aliases.remove(&alias)
    }

    /// Write the state of the alias mapping to the alias file.
    /// The target file will be completely overwritten. It is assumed
    /// in usage of this method that the aliases in the file were
    /// previously laoded into memory.
    pub fn dump_aliases_to_alias_file(self) -> std::io::Result<()> {
        let file = File::create(&self.config_location)?;
        println!("Writing to {:?}", self.config_location);
        let mut writer = BufWriter::new(file);
        for (alias, command) in self.aliases {
            let line = format!("alias {}=\"{}\"\n", alias, command);
            writer.write(line.as_bytes()).unwrap();
        }
        writer.flush()?;
        Ok(())
    }

    /// Read the user's history from the default histoy file.
    pub fn scan_history(&self) -> String {
        let history_loc = default_history_path();
        println!("{:?}", history_loc);
        let output = Command::new("cat")
            .arg(history_loc)
            .output()
            .expect("Couldn't run 'history' command")
            .stdout;
        format!(
            "{}",
            std::str::from_utf8(&output).expect("output contained badly formatted utf8")
        )
    }

    /// Parse the user's history by counting occurances of commands.
    pub fn parse_history_string(&self, history: String) -> HashMap<String, u32> {
        let mut counts = HashMap::new();
        for command in history.split('\n') {
            let copied_command = command.to_string();
            *counts.entry(copied_command).or_insert(0) += 1;
        }
        counts
    }
}
