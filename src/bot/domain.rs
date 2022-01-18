#[derive(Debug, Clone)]
pub struct User {
    id: u64,
    name: String,
    state: State,
}

impl User {
    pub fn new(id: u64, name: String, state: State) -> Self {
        User { id, name, state }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn state(&self) -> State {
        self.state.clone()
    }

    pub fn set_state(&mut self, state: State) -> Self {
        self.state = state;
        self.to_owned()
    }
}

#[derive(Debug, Clone)]
pub enum State {
    StartTraining,
    SelectExercise,
    SelectWeight,
    SelectReps,
}

#[derive(Debug, Clone)]
pub enum MessageType {
    BotCommand(String),
    Number(u32),
    Other(String),
}
