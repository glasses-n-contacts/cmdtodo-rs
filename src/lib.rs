extern crate reqwest;
#[macro_use] extern crate serde_derive;

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

pub fn print_todos(content_only: bool, show_done: bool) -> Result<(), reqwest::Error> {
    let json: TodosResponse = reqwest::get("http://localhost:5170")?.json()?;
    for todo in &json.todos {
        if !show_done && todo.done {
            continue;
        }
        let completed = completed_display(todo.done);
        if show_done && !content_only {
            println!("- [{}] {} {}", completed, todo.id, todo.content);
        } else if show_done && content_only {
            println!("- [{}] {}", completed, todo.content);
        } else if !show_done && !content_only {
            println!("- {} {}", todo.id, todo.content);
        } else {
            println!("- {}", todo.content);
        }
    }
    Ok(())
}

pub fn todo_info(id: String) -> Result<(), reqwest::Error> {
    let json: TodoResponse = reqwest::get(
        &(String::from("http://localhost:5170/") + &id))?
        .json()?;
    let todo = json.todo;
    println!("[{}] {} {}", completed_display(todo.done), todo.id, todo.content);
    Ok(())
}

fn completed_display(done: bool) -> String {
    if done {
        String::from("x")
    } else {
        String::from(" ")
    }
}
