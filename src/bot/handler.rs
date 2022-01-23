use super::domain::*;
use dashmap::DashMap;
use frankenstein::Message;

pub struct MessageHandler {
    store: DashMap<u64, User>,
}

impl MessageHandler {
    pub fn new() -> Self {
        MessageHandler {
            store: DashMap::new(),
        }
    }

    pub fn handle_message(&mut self, message: &Message) -> String {
        let from = &message.from.as_ref().unwrap();
        let store = &self.store;
        let user = store.get(&from.id);
        let user = match user {
            Some(u) => {
                let u = u.value().to_owned();
                log::info!("user found: {:?}", u);
                u
            }
            None => {
                let user = User::new(from.id, from.first_name.clone(), State::StartTraining);
                store.insert(user.id(), user.clone());
                log::info!("new user created: {:?}", user);
                user
            }
        };
        let text = message.text.as_ref().unwrap();
        log::info!("Receive message: {}", text);
        let answer = self.handle_text(text, user);
        answer
    }

    fn handle_text(&self, text: &String, user: User) -> String {
        let command = text_to_commant(text);
        let bot_answer = match command {
            MessageType::BotCommand(text) => {
                let answer = self.handle_bot_command(&text, user);
                answer
            }
            MessageType::Number(num) => {
                let answer = self.handle_bot_digit_command(num, user);
                answer
            }
            MessageType::Other(text) => {
                format!("Unknown command: {} start command with /", text)
            }
        };
        String::from(bot_answer)
    }

    fn handle_bot_command(&self, text: &String, mut user: User) -> String {
        let store = &self.store;
        let answer = match text.as_str() {
            "/help" => "Type /start for start training".to_string(),
            "/start" => {
                user.set_state(State::SelectExercise);
                store.insert(user.id(), user);
                "Start training, select exercise from:\n
                /bench_press\n
                /squat\n
                /dead_lift\n
                "
                .to_string()
            }
            "/bench_press" | "/squat" | "/dead_lift" => handle_exersice(&mut user, text, store),
            "/exercise" => {
                let ans = match user.state() {
                    State::SelectWeight => {
                        user.set_state(State::SelectExercise);
                        store.insert(user.id(), user);
                        "Start training, select exercise from:\n
                        /bench_press\n
                        /squat\n
                        /dead_lift"
                            .to_string()
                    }
                    _ => "can not select exersice now".to_string(),
                };
                ans
            }
            "/cancel" => {
                let ans = match user.state() {
                    State::SelectWeight => {
                        log::info!("Start to cancel exercise or reps");
                        let reps = user.training().get_exercises().last_mut().unwrap().reps();
                        if reps.is_empty() {
                            let exercises = user.training().get_exercises();
                            let removed =                            exercises.pop();
                            log::info!("Deubug trainings: {:?}", exercises);
                            user.set_state(State::SelectExercise);
                            store.insert(user.id(), user);
                            format!("Remove last exercise: {:?}, select exercise again", removed)
                        } else {
                            let removed = reps.pop();
                            let exercises = user.training().get_exercises();
                            log::info!("Deubug trainings: {:?}", exercises);
                            user.set_state(State::SelectReps);
                            store.insert(user.id(), user);
                            format!("Remove last reps: {:?}, select reps again", removed)
                        }
                    }
                    State::SelectReps => {
                        log::info!("Start to cancel exercise or weight");
                        let weight = user.training().get_exercises().last_mut().unwrap().weight();
                        if weight.is_empty() {
                            let exercises = user.training().get_exercises();
                            let removed = exercises.pop();
                            log::info!("Deubug trainings: {:?}", exercises);
                            user.set_state(State::SelectExercise);
                            store.insert(user.id(), user);
                            format!("Remove last exercise: {:?}, select exercise again", removed)
                        } else {
                            let removed = weight.pop();
                            let exercises = user.training().get_exercises();
                            log::info!("Deubug trainings: {:?}", exercises);
                            user.set_state(State::SelectWeight);
                            store.insert(user.id(), user);
                            format!("Remove last weight: {:?}, select weight again", removed)
                        }
                    }
                    _ => "nothing to cancel".to_string()
                };
                ans
            }
            "/show" => {
                format!("User: {:?}", user)
            }
            "/stop" => {
                let training_time = user.training().get_start_time().elapsed();
                let info = format!(
                    "Finish trainig\nTraining duration: {:?}\nTraining info: {:?}",
                    training_time.clone(),
                    user.training().clone()
                );
                store.remove(&user.id());
                info
            }
            _ => "Unknown command, try /help".to_string(),
        };
        answer
    }

    fn handle_bot_digit_command(&self, num: u32, mut user: User) -> String {
        let ans = match user.state() {
            State::SelectWeight => {
                user.set_state(State::SelectReps);
                match user.training().get_exercises().last_mut() {
                    Some(ex) => {
                        log::info!("Add weight: {}", num);
                        ex.set_weight(num);
                        self.store.insert(user.id(), user);
                        format!("Weight selected: {}, now select reps", num)
                    }
                    None => "Exercise not found".to_string(),
                }
            }
            State::SelectReps => {
                user.set_state(State::SelectWeight);
                match user.training().get_exercises().last_mut() {
                    Some(ex) => {
                        log::info!("Add reps: {}", num);
                        ex.set_reps(num);
                        self.store.insert(user.id(), user);
                        format!("Reps selected: {}, now select weight for next turn, or select /exercise for select another one", num)
                    }
                    None => "Exercise not found".to_string(),
                }
            }
            _ => "can not process request".to_string(),
        };
        ans
    }
}

fn handle_exersice(user: &mut User, text: &String, store: &DashMap<u64, User>) -> String {
    let ans = match user.state() {
        State::SelectExercise => {
            user.set_state(State::SelectWeight);
            let exercise = Exercise::new(text.clone());
            user.training().add_exercise(exercise);
            store.insert(user.id(), user.to_owned());
            format!("Select {}, now select weright", text)
        }
        _ => "Can not select exersice now".to_string(),
    };
    ans
}

fn text_to_commant(text: &String) -> MessageType {
    if let Ok(num) = text.parse::<u32>() {
        return MessageType::Number(num);
    };
    if text.starts_with("/") {
        return MessageType::BotCommand(text.clone());
    };
    MessageType::Other(text.clone())
}
