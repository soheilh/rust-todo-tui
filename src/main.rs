struct Task {
    description: String,
    status: bool,
}

impl Task {
    fn new(description: String) -> Task {
        Task {
            description,
            status: false,
        }
    }
}

fn main() {
    let task = Task::new(String::from("First Task"));
    println!("Task: {}, status: {}", task.description, task.status);
}
