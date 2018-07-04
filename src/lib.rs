extern crate reqwest;
#[macro_use] extern crate serde_derive;
use std::collections::HashMap;
use std::io::{self, Write};
extern crate rpassword;

struct LogInState {
    username: String,
    token: String,
}

impl LogInState {
    fn new() -> LogInState {
        LogInState {
            username: String::new(),
            token: String::new(),
        }
    }
}

#[derive(Deserialize, Debug)]
struct Todo {
    id: u32,
    done: bool,
    content: String,
}

#[derive(Deserialize)]
struct TodoResponse {
    todo: Todo,
}

#[derive(Deserialize, Debug)]
struct TodosResponse {
    todos: Vec<Todo>,
}

pub struct Client {
    login_state: LogInState,
}

impl Client {
    pub fn new() -> Client {
        Client {
            login_state: LogInState::new(),
        }
    }

    pub fn print_todos(&mut self, content_only: bool, show_done: bool) {
        let json: TodosResponse = reqwest::get("http://localhost:5170/todo").unwrap().json().unwrap();
        for todo in &json.todos {
            if !show_done && todo.done {
                continue;
            }
            let completed = completed_display(todo.done);
            if show_done && !content_only {
                println!("[{}] {} {}", completed, todo.id, todo.content);
            } else if show_done && content_only {
                println!("[{}] {}", completed, todo.content);
            } else if !show_done && !content_only {
                println!("{} {}", todo.id, todo.content);
            } else {
                println!("{}", todo.content);
            }
        }
    }

    pub fn todo_info(&mut self, id: String) {
        let json: TodoResponse = reqwest::get(
            &(String::from("http://localhost:5170/todo") + &id)).unwrap()
            .json().unwrap();
        let todo = json.todo;
        println!("[{}] {} {}", completed_display(todo.done), todo.id, todo.content);
    }

    pub fn add_todo(&mut self, content: String) {
        let mut todo = HashMap::new();
        todo.insert("content", content);
        let client = reqwest::Client::new();
        let _res = client.post("http://localhost:5170/todo")
            .json(&todo)
            .send()
            .unwrap();
    }

    pub fn do_todos(&mut self, ids: Vec<&str>) {
        let mut body = HashMap::new();
        body.insert("ids", ids);
        let client = reqwest::Client::new();
        let _res = client.post("http://localhost:5170/todo/do")
            .json(&body)
            .send()
            .unwrap();
    }

    pub fn login(&mut self) {
        let mut username = String::new();
        print!("username: ");
        let _ = io::stdout().flush();
        io::stdin().read_line(&mut username)
            .expect("Please put your username");
        username.pop();
        let password = rpassword::prompt_password_stdout("password: ")
            .expect("Please put your password");
        println!("Your username is {} password is {}", username, password);

        let mut credentials = HashMap::new();
        credentials.insert("username", username);
        credentials.insert("password", password);
        let client = reqwest::Client::new();
        let _res = client.post("http://localhost:5170/user/login")
            .json(&credentials)
            .send()
            .unwrap();
    }
}

fn completed_display(done: bool) -> String {
    if done {
        String::from("x")
    } else {
        String::from(" ")
    }
}
