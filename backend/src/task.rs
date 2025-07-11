use csv::{Reader, Writer};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, BufWriter};

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub completed: bool,
}

impl Task {
    pub fn new(id: u32, title: String, description: String) -> Self {
        Task {
            id,
            title,
            description,
            completed: false,
        }
    }
}

pub fn load_tasks() -> Vec<Task> {
    let file = match File::open("tasks.csv") {
        Ok(file) => file,
        Err(_) => return Vec::new(), // Return empty vec if file doesn't exist
    };

    let mut reader = Reader::from_reader(BufReader::new(file));
    let mut tasks = Vec::new();

    for result in reader.deserialize() {
        match result {
            Ok(task) => tasks.push(task),
            Err(_) => continue, // Skip invalid rows
        }
    }

    tasks
}

pub fn save_tasks(tasks: &[Task]) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create("tasks.csv")?;
    let mut writer = Writer::from_writer(BufWriter::new(file));

    for task in tasks {
        writer.serialize(task)?;
    }

    writer.flush()?;
    Ok(())
}
