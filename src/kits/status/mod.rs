use std::fmt::Display;

use axum::{Router, routing::get};

#[derive(Debug, Clone)]
pub enum Status {
    Alive,
    Ready,
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status_string = match self {
            Status::Alive => "Alive",
            Status::Ready => "Ready",
        };

        write!(f, "{}", status_string)
    }
}

#[derive(Debug, Clone)]
pub struct Health {
    status: Status,
}

impl Health {
    pub fn new() -> Self {
        Health {
            status: Status::Alive,
        }
    }

    pub fn get_status(&self) -> Status {
        self.status.clone()
    }

    pub fn set_ready(&mut self) {
        self.status = Status::Ready;
    }

    pub fn set_alive(&mut self) {
        self.status = Status::Alive;
    }

    pub fn get_router(&self) -> Router {
        Router::new().route("/", {
            let this = self.clone();
            get(|| async move { this.get_status().to_string() })
        })
    }
}

impl Display for Health {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:}", self.status)
    }
}
