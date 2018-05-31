use std;
use std::path::PathBuf;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Write, BufWriter, BufReader, BufRead};
use std::process::Command;

pub fn default_path() -> PathBuf {
    match std::env::home_dir(){
        Some(path) => path.join(".quick_aliases.sh"),
        None => panic!("no home directory in env"),
    }
}

pub fn default_history_path() -> PathBuf {
    match std::env::home_dir(){
        Some(path) => path.join(".bash_history"),
        None => panic!("no home directory in env"),
    }
}
pub struct AliasConfig {
    config_location: PathBuf,
    aliases: HashMap<String, String>
    
}

impl AliasConfig {
    pub fn new(config_location: PathBuf) -> AliasConfig{
        AliasConfig{config_location: config_location,
                    aliases: HashMap::new()}
    }
    pub fn load(&mut self){
        if self.config_location.exists() {
            let file = File::open(&self.config_location).unwrap();
            let reader = BufReader::new(file);
            for line in reader.lines() {
                let l = line.unwrap();
                self.handle_line(l);
            }

        }
        else {
            println!("{:?} doesnt exist, will create" , self.config_location);
            self.create_file_if_doesnt_exist(&self.config_location);
        }
    }

    pub fn debug(&self){
        println!("state of alias mapping: ");
        println!("{:?}", self.aliases);
    }

    fn handle_line(&mut self, line: String){
        let mut split = line.split(" ");
        if split.next() == Some("alias"){
            let rest = split.collect::<Vec<_>>().join(" ");    
            // do some stuff with the rest of the string
            let mut split_eq = rest.split("=");
            let alias = split_eq.next().unwrap();
            let command = split_eq.collect::<Vec<_>>().join("=");    
            let l = command.len();
            let command_uq = &command[1..l-1];
            // println!("alias: {}", alias);
            // println!("command unquoted: {}", command_uq);
            self.add_alias(alias.to_string(), command_uq.to_string());

        }

    }
    pub fn add_alias(&mut self, alias: String, command: String){
        self.aliases.insert(alias, command);
    }
    pub fn remove_alias(&mut self, alias: String) -> Option<String>{
        self.aliases.remove(&alias)
    }
    pub fn create_file_if_doesnt_exist(&self, alias_path: &PathBuf){
        if !alias_path.exists() {
            File::create(alias_path);
        }
    }
    pub fn dump_aliases_to_alias_file(self) -> std::io::Result<()>{
        let file = File::create(&self.config_location)?;
        println!("Writing to {:?}", self.config_location);
        let mut writer = BufWriter::new(file);
        println!("{:?}", writer);
        for (alias, command) in self.aliases {
            let line = format!("alias {}=\"{}\"\n", alias, command);
            println!("line: {}", line);
            writer.write(line.as_bytes()).unwrap();
        }
        writer.flush()?;
        Ok(())
    }

    pub fn scan_history(&self) -> String {
        let history_loc = default_history_path();
        println!("{:?}", history_loc);
        let output = Command::new("cat")
            .arg(history_loc)
            .output()
			.expect("Couldn't run 'history' command").stdout;
        // std::str::from_utf8(&output).expect("output was non parsable utf8").to_string()
			format!("{}", std::str::from_utf8(&output).expect("output contained badly formatted utf8"))
    }

    pub fn parse_history_string(&self, history: String) -> HashMap<String, u32> {
        let mut counts = HashMap::new();
        for command in history.split('\n') {
            let copied_command = command.to_string();
            *counts.entry(copied_command).or_insert(0) += 1; 
        }
        counts
    }

    pub fn test_ls(&self) {
	let opt = Command::new("ls")
			.arg("-l")
			.arg("-a")
			.output()
			.expect("ls command failed to start");
	println!("{:?}", opt);

    }


}
