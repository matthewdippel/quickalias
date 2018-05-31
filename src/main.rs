extern crate clap;
use clap::{Arg, App, SubCommand, ArgMatches};
mod aliases;
use aliases::AliasConfig;
use std::io::{Error, ErrorKind};

fn run(matches: ArgMatches) -> std::io::Result<()> {
    match matches.subcommand() {
        ("add", Some(m)) => run_add(m),
        ("remove", Some(m)) => run_remove(m),
        ("show", Some(m)) => run_show(m),
        ("scan", Some(m)) => run_scan(m),
        _ => Ok(())
    }
}

fn run_add(matches: &ArgMatches) -> std::io::Result<()>{
    let alias = matches.value_of("alias").unwrap().to_string();
    let command = matches.value_of("command").unwrap().to_string();
    println!("alias:{}", alias);
    println!("command:{}", command);
    let alias_path = aliases::default_path();
    let mut aliasconfig = AliasConfig::new(alias_path);
    aliasconfig.load();
    aliasconfig.add_alias(alias, command);
    aliasconfig.debug();
    aliasconfig.dump_aliases_to_alias_file()
}

fn run_remove(matches: &ArgMatches) -> std::io::Result<()>{
    let alias = matches.value_of("alias").unwrap().to_string();
    println!("alias:{}", alias);
    let alias_path = aliases::default_path();
    let mut aliasconfig = AliasConfig::new(alias_path.clone());
    aliasconfig.load();
    match aliasconfig.remove_alias(alias.clone()){
        Some(s) => {aliasconfig.debug();
                    println!("will remove alias: {}", alias);
                    println!("            command: {}", s);
                    aliasconfig.dump_aliases_to_alias_file()},
        None => Err(Error::new(ErrorKind::Other, format!("No such alias {} in config file {:?}", alias, alias_path)))
    }
}
fn run_show(matches: &ArgMatches) -> std::io::Result<()>{
    let alias_path = aliases::default_path();
    let mut aliasconfig = AliasConfig::new(alias_path);
    aliasconfig.load();
    aliasconfig.debug();
    Ok(())
}
fn run_scan(matches: &ArgMatches) -> std::io::Result<()>{
    let alias_path = aliases::default_path();
    let aliasconfig = AliasConfig::new(alias_path);
    // aliasconfig.load();
    // aliasconfig.debug();
    // aliasconfig.test_ls();
    let history_string = aliasconfig.scan_history();
    let history_counts = aliasconfig.parse_history_string(history_string);
    let mut count_vec: Vec<_> = history_counts.iter().collect();
    count_vec.sort_by(|a, b| b.1.cmp(a.1));
    let n = matches.value_of("count").unwrap_or("10").parse::<usize>().unwrap();
    for tup in count_vec.iter().take(n) {
        println!("{} :: {}", tup.0, tup.1);
    }
    Ok(())
}
fn main() {
    let matches = App::new("QuickAlias")
        .version("0.1.0")
        .author("Matthew Dippel <mattdippel@gmail.com>")
        .about("Tool for quickly adding aliases to your environment, searching history for likely candidates for aliasing, and other useful utilities")
        .subcommand(SubCommand::with_name("add")
                    .about("Add an alias")
                    .arg(Arg::with_name("alias")
                         .short("a")
                         .long("alias")
                         .value_name("alias")
                         .help("the desired alias to add"))
                    .arg(Arg::with_name("command")
                         .short("c")
                         .long("command")
                         .value_name("command")
                         .help("the command to alias")))
        .subcommand(SubCommand::with_name("remove")
                    .about("Remove an alias")
                    .arg(Arg::with_name("alias")
                         .short("a")
                         .long("alias")
                         .value_name("alias")
                         .help("the desired alias to remove")))
        .subcommand(SubCommand::with_name("show")
                    .about("Show the current set of quick aliases"))
        .subcommand(SubCommand::with_name("scan")
                    .about("Scan the current history for candidate aliases")
                    .arg(Arg::with_name("count")
                         .short("c")
                         .long("count")
                         .value_name("count")
                         .help("Limit on how many commands to show")))
        .get_matches();
    println!("{:?}",run(matches));
}
