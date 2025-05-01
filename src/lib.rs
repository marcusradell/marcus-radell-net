#[derive(Debug, Clone)]
pub enum Status {
    Alive,
    Ready,
}
#[derive(Debug)]
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
}
