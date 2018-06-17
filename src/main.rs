extern crate clap;

use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("todo")
                        .version("1.0")
                        .author("Bill Yu <billyuhan@gmail.com>")
                        .about("Manage todo lists from the command line")
                        .arg(Arg::with_name("config")
                                    .short("c")
                                    .long("config")
                                    .value_name("FILE")
                                    .help("Sets a custom config file")
                                    .takes_value(true))
                        .arg(Arg::with_name("debug")
                                    .short("d")
                                    .help("Turn debugging information on"))
                        .subcommand(SubCommand::with_name("ls")
                                                .about("List all todos")
                                                .arg(Arg::with_name("long")
                                                    .short("l")
                                                    .long("long")
                                                    .help("List all information for each todo")))
                        .subcommand(SubCommand::with_name("info")
                                                .about("Get detailed info about a todo with its id")
                                                .arg(Arg::with_name("id")
                                                        .value_name("id")
                                                        .takes_value(true)
                                                        .help("id of the todo item to inspect")))
                        .subcommand(SubCommand::with_name("add")
                                                .about("Add a todo")
                                                .arg(Arg::with_name("content")
                                                        .value_name("content")
                                                        .takes_value(true)
                                                        .help("Content of the todo item to add")))
                        .subcommand(SubCommand::with_name("do")
                                                .about("Mark todo item(s) as completed")
                                                .arg(Arg::with_name("id")
                                                        .value_name("id")
                                                        .takes_value(true)
                                                        .multiple(true)
                                                        .help("id(s) of the todo item(s) to be marked")))
                        .get_matches();

    if let Some(o) = matches.value_of("output") {
        println!("Value for output: {}", o);
    }

    match matches.occurrences_of("debug") {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is on"),
        _ => panic!("The world is messed up"),
    }

    if let Some(matches) = matches.subcommand_matches("ls") {
        if matches.is_present("long") {
            println!("List long formatted todos");
        } else {
            println!("Listing todos");
        }
    }

    if let Some(matches) = matches.subcommand_matches("info") {
        if let Some(id) = matches.value_of("id") {
            println!("Inspecting todo with id: {}", id);
        }
    }

    if let Some(matches) = matches.subcommand_matches("add") {
        if let Some(content) = matches.value_of("content") {
            println!("Adding todo with content: {}", content);
        }
    }

    if let Some(matches) = matches.subcommand_matches("do") {
        if let Some(id_values) = matches.values_of("id") {
            let ids: Vec<&str> = id_values.collect();
            println!("Marking todo(s) completed with id(s): {:?}", ids);
        }
    }
}