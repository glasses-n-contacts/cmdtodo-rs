extern crate reqwest;
extern crate rpassword;
#[macro_use] extern crate serde_derive;

mod login_state;

use std::collections::HashMap;
use std::io::{self, Write};
use self::login_state::LogInState;
use reqwest::header::{Bearer, Authorization, Headers};

#[derive(Deserialize, Debug)]
struct Todo {
    id: String,
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

#[derive(Deserialize, Debug)]
struct LoginResponse {
    username: String,
    token: String,
}

pub struct Client {
    login_state: LogInState,
    client: reqwest::Client,
}

impl Client {
    pub fn new() -> Client {
        Client {
            login_state: LogInState::new(),
            client: reqwest::Client::new(),
        }
    }

    fn build_headers(&mut self) -> Headers {
        let mut headers = Headers::new();
        headers.set(
            Authorization(
                Bearer {
                    token: self.login_state.get_token().clone(),
                }
            )
        );
        headers
    }

    pub fn print_todos(&mut self, content_only: bool, show_done: bool) {
        let json: TodosResponse = self.client.get("http://localhost:5170/todo")
            .headers(self.build_headers())
            .send()
            .unwrap()
            .json()
            .unwrap();
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
        let json: TodoResponse = self.client.get(
            &(String::from("http://localhost:5170/todo/") + &id))
            .headers(self.build_headers())
            .send()
            .unwrap()
            .json()
            .unwrap();
        let todo = json.todo;
        println!("[{}] {} {}", completed_display(todo.done), todo.id, todo.content);
    }

    pub fn add_todo(&mut self, content: String) {
        let mut todo = HashMap::new();
        todo.insert("content", content.clone());
        let _res = self.client.post("http://localhost:5170/todo")
            .headers(self.build_headers())
            .json(&todo)
            .send()
            .unwrap();
        println!("Added todo item: {}", content);
    }

    pub fn do_todos(&mut self, ids: Vec<&str>) {
        let mut body = HashMap::new();
        body.insert("ids", ids);
        let _res = self.client.post("http://localhost:5170/todo/do")
            .headers(self.build_headers())
            .json(&body)
            .send()
            .unwrap();
        println!("Did");
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

        let mut credentials = HashMap::new();
        credentials.insert("username", username);
        credentials.insert("password", password);
        let res: LoginResponse = self.client.post("http://localhost:5170/user/login")
            .json(&credentials)
            .send()
            .unwrap()
            .json()
            .unwrap();
        self.login_state.set_token(res.token);
        println!("Logged in as {}", res.username);
    }
}

fn completed_display(done: bool) -> String {
    if done {
        String::from("x")
    } else {
        String::from(" ")
    }
}
