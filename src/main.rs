use std::io::Write;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    id: u32,
    task: String,
    completed: bool,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    //Basic Main Menu with one Question
    loop {
        println!("What do you want to do?\n");
        println!("1. Add Todo");
        println!("2. Remove Todo");
        println!("3. Edit Todo");
        println!("4. Exit");

        //Create String
        let mut raw_answer = String::new();
        //Read raw_answer Line
        std::io::stdin()
            .read_line(&mut raw_answer)
            .ok()
            .expect("Err cannot read the line");

        //Match & Trim the Answer and running the selected Fn
        match raw_answer.trim() {
            "1" => {
                println!("Enter Todo: ");
                let mut add_todo_answer = String::new();
                std::io::stdin()
                    .read_line(&mut add_todo_answer)
                    .ok()
                    .expect("Err cannot read line add_todo_answer");

                add_todo(add_todo_answer).await?
            }
            "2" => remove_todo().await?,
            "3" => edit_todo().await?,
            "4" => {
                exit();
                break;
            }
            _ => println!("No Option"),
        }
    }

    Ok(())
}

async fn add_todo(task_name: String) -> std::io::Result<()> {
    // 1. Load List
    let mut todos = load_todos().await?;

    // 2. Find the Highest
    // We Look at all ID's and pick the highest one and add 1. (If there's no ID's We'll pick 0)
    let next_id = todos.iter().map(|t| t.id).max().unwrap_or(0) + 1;

    // 3. Create new Todo
    let new_todo = Todo {
        id: next_id,
        task: task_name,
        completed: false,
    };

    // 4. push in the List and save it
    todos.push(new_todo);
    save_json(&todos).await?;

    Ok(())
}

async fn load_todos() -> std::io::Result<Vec<Todo>> {
    let path = "todo.json";
    if !std::path::Path::new(path).exists() {
        return Ok(vec![]); // If File is empty = return nothing
    }

    let content = tokio::fs::read_to_string(path).await?;
    let todos: Vec<Todo> = serde_json::from_str(&content).unwrap_or_else(|_| vec![]);
    Ok(todos)
}

async fn save_json(todos: &Vec<Todo>) -> std::io::Result<()> {
    //convert in Json
    let json_string = serde_json::to_string_pretty(todos).unwrap();
    tokio::fs::write("todo.json", json_string).await?;
    Ok(())
}

async fn remove_todo() -> std::io::Result<()> {
    let file_path = "todo.json";
    let data = std::fs::read_to_string(file_path)?;
    let mut json_data: Value = serde_json::from_str(&data)?;
    let mut id_to_delete = String::new();

    let todos = load_todos().await?;

    if todos.is_empty() {
        println!("\nYour Todo list is empty!");
    } else {
        println!("\n--- YOUR TODOS ---");
        for item in todos {
            // [x] completed, [ ] not completed
            let status = if item.completed { "x" } else { " " };
            println!("{}. [{}] {}", item.id, status, item.task);
        }
        println!("-------------------\n");
    }
    print!("Todo Id for removing: ");
    std::io::stdout().flush()?;
    std::io::stdin().read_line(&mut id_to_delete)?;

    let id_to_delete: i64 = id_to_delete
        .trim()
        .parse()
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidInput, "Not an ID"))?;

    if let Some(array) = json_data.as_array_mut() {
        array.retain(|item| item.get("id").and_then(Value::as_i64) != Some(id_to_delete));
    }

    let modified_data = serde_json::to_string_pretty(&json_data)?;
    std::fs::write(file_path, modified_data)?;

    println!("ID: {} deleted", id_to_delete);
    Ok(())
}

async fn edit_todo() -> std::io::Result<()> {
    let file_path = "todo.json";
    let data = std::fs::read_to_string(file_path)?;
    let mut json_data: Value = serde_json::from_str(&data)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

    // 1. Todos anzeigen
    if let Some(array) = json_data.as_array() {
        if array.is_empty() {
            println!("\nListe is empty.");
            return Ok(());
        }
        println!("\n--- YOUR TODOS ---");
        for item in array {
            let id = item["id"].as_i64().unwrap_or(0);
            let status = if item["completed"].as_bool().unwrap_or(false) {
                "x"
            } else {
                " "
            };
            println!("{}. [{}] {}", id, status, item["task"]);
        }
    }

    // 2. select ID
    print!("\nWhich ID do you want to edit? ");
    std::io::stdout().flush()?;
    let mut id_input = String::new();
    std::io::stdin().read_line(&mut id_input)?;
    let target_id: i64 = id_input.trim().parse().unwrap_or(-1);

    // 3. Ask for new Text
    print!("New Text for the task: ");
    std::io::stdout().flush()?;
    let mut new_task_text = String::new();
    std::io::stdin().read_line(&mut new_task_text)?;
    let new_task_text = new_task_text.trim();

    // 4. Search for the Text and change it
    let mut found = false;
    if let Some(array) = json_data.as_array_mut() {
        for item in array {
            if item.get("id").and_then(Value::as_i64) == Some(target_id) {
                // changed text to new Text:
                item["task"] = Value::String(new_task_text.to_string());
                found = true;
                break;
            }
        }
    }

    if found {
        // 5. Save
        let modified_data = serde_json::to_string_pretty(&json_data)?;
        std::fs::write(file_path, modified_data)?;
        println!("Task changed!");
    } else {
        println!("ID {} not found.", target_id);
    }

    Ok(())
}

fn exit() {
    //Exit the Program Successfull with Err code 0
    std::process::exit(0)
}
