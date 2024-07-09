use chrono::{DateTime as ChronoDateTime, Utc};
use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    pub id: ObjectId,
    pub title: String,
    pub done: bool,
}

impl Todo {
    pub fn new(title: String) -> Self {
        Todo {
            id: ObjectId::new(),
            title,
            done: false,
        }
    }
}

pub struct TodoList {
    pub todos: Vec<Todo>,
}

impl TodoList {
    pub fn new() -> Self {
        TodoList { todos: Vec::new() }
    }

    pub fn add_todo(&mut self, todo: Todo) {
        self.todos.push(todo)
    }

    pub fn remove_todo(&mut self, todo_id: ObjectId) -> Option<Todo> {
        match self.todos.iter().position(|t| t.id == todo_id) {
            Some(index) => Some(self.todos.remove(index)),
            None => None,
        }
    }
}
