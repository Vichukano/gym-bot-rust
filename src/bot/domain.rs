use std::time::Instant;

#[derive(Debug, Clone)]
pub struct User {
    id: u64,
    name: String,
    state: State,
    training: Training,
}

impl User {
    pub fn new(id: u64, name: String, state: State) -> Self {
        User {
            id: id,
            name: name,
            state: state,
            training: Training::new(),
        }
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

    pub fn training(&mut self) -> &mut Training {
        &mut self.training
    }

    pub fn set_state(&mut self, state: State) -> Self {
        self.state = state;
        self.to_owned()
    }
}

#[derive(Debug, Clone)]
pub struct Training {
    start_time: Instant,
    exercises: Vec<Exercise>,
}

impl Training {
    pub fn new() -> Self {
        Training {
            start_time: Instant::now(),
            exercises: Vec::new(),
        }
    }

    pub fn add_exercise(&mut self, exercise: Exercise) {
        self.exercises.push(exercise)
    }

    pub fn get_exercises(&mut self) -> &mut Vec<Exercise> {
        &mut self.exercises
    }

    pub fn get_start_time(&self) -> Instant {
        self.start_time
    }
}

#[derive(Debug, Clone)]
pub struct Exercise {
    name: String,
    weight: Vec<u32>,
    reps: Vec<u32>,
}

impl Exercise {
    pub fn new(name: String) -> Self {
        Exercise {
            name,
            weight: vec![],
            reps: vec![],
        }
    }

    pub fn set_weight(&mut self, weight: u32) {
        self.weight.push(weight);
    }

    pub fn set_reps(&mut self, reps: u32) {
        self.reps.push(reps);
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
