#[derive(Clone)]
struct Task {
    id: usize,
    description: String,
    completed: bool,
}


fn add_task(description: &str, todo_list: &mut Vec<Task>) -> Task {
    let id = todo_list.len() + 1; 
    let new_task = Task {
        id,
        description: description.to_string(),
        completed: false,
    };
    todo_list.push(new_task.clone()); 
    new_task
}

fn complete_task(id: usize, todo_list: &mut Vec<Task>) -> Option<&Task> {
    if let Some(task) = todo_list.iter_mut().find(|t| t.id == id) {
        task.completed = true;
        Some(task)
    } else {
        None
    }
}

fn list_tasks(todo_list: &Vec<Task>) {
    for task in todo_list {
        println!("ID: {}, Description: {}, Completed: {}", task.id, task.description, task.completed);
    }
}

fn main() {
    let mut todo_list: Vec<Task> = Vec::new();

    let task1 = add_task("Buy groceries", &mut todo_list);
    let _task2 = add_task("Read a book", &mut todo_list); 
    println!("Tasks before completion:");
    list_tasks(&todo_list);

    complete_task(task1.id, &mut todo_list);

    println!("Tasks after completion:");
    list_tasks(&todo_list);
}
