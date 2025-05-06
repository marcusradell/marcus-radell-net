use std::fmt::Display;

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

    pub fn get_status_route(&self) -> String {
        self.status.to_string()
    }
}

impl Display for Health {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:}", self.status)
    }
}
