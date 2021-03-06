extern crate clap;
extern crate todo;

use clap::{App, Arg, SubCommand};

fn main() {
    let mut client = todo::Client::new();
    let matches = App::new("todo")
                        .version("1.0")
                        .author("Bill Yu <billyuhan@gmail.com>")
                        .about("Manage todo lists from the command line")
                        .subcommand(SubCommand::with_name("ls")
                                                .about("List all todos")
                                                .arg(Arg::with_name("content-only")
                                                    .short("c")
                                                    .long("content-only")
                                                    .help("List only the content for earch todo"))
                                                .arg(Arg::with_name("done")
                                                    .short("d")
                                                    .long("done")
                                                    .help("Show completed todos as well")))
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
                        .subcommand(SubCommand::with_name("login")
                                                .about("Log in with credentials"))
                        .get_matches();

    if let Some(matches) = matches.subcommand_matches("ls") {
        client.print_todos(
            matches.is_present("content-only"),
            matches.is_present("done"));
    }

    if let Some(matches) = matches.subcommand_matches("info") {
        if let Some(id) = matches.value_of("id") {
            client.todo_info(id.to_string());
        }
    }

    if let Some(matches) = matches.subcommand_matches("add") {
        if let Some(content) = matches.value_of("content") {
            client.add_todo(content.to_string());
        }
    }

    if let Some(matches) = matches.subcommand_matches("do") {
        if let Some(id_values) = matches.values_of("id") {
            let ids: Vec<&str> = id_values.collect();
            client.do_todos(ids);
        }
    }

    if let Some(_matches) = matches.subcommand_matches("login") {
        client.login();
    }
}