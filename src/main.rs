
#[derive(Clone, Debug)]
struct Task {
    id: i32,
    description: String,
    completed: bool,
}

struct TodoList {
    tasks: Vec<Task>,
}

impl TodoList {
    fn new() -> TodoList {
        TodoList { tasks: Vec::new() }
    }

    fn add_task(&mut self, description: &str) -> Task {
        let task = Task {
            id: self.tasks.len() as i32,
            description: description.to_string(),
            completed: false,
        };
        self.tasks.push(task.clone());
        task
    }

    fn complete_task(&mut self, id: i32) -> Option<&Task> {
        let task = self.tasks.iter_mut().find(|task| task.id == id);
        match task {
            Some(task) => {
                task.completed = true;
                Some(task)
            }
            None => None,
        }
    }

    fn list_tasks(&self) {
        for task in &self.tasks {
            println!("Task id: {}", task.id);
            println!("Task description: {}", task.description);
            println!("Task completed: {}\n", task.completed);
        }
    }
}

fn main() {
    let mut todo_list = TodoList::new();

    let first_task = todo_list.add_task("Learn Rust");
    todo_list.add_task("Learn Python");
    todo_list.add_task("Learn C++");
    todo_list.add_task("Give up on C++");

    todo_list.complete_task(first_task.id);

    todo_list.list_tasks();
}
