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

    fn handle_text(&self, text: &String, mut user: User) -> String {
        let store = &self.store;
        let user_info;
        let command = text_to_commant(text);
        let bot_answer = match command {
            MessageType::BotCommand(text) => {
                let answer = match text.as_str() {
                    "/help" => "Type /start for start training",
                    "/start" => {
                        user.set_state(State::SelectExercise);
                        store.insert(user.id(), user);
                        "Start training, select exercise from:\n
                        /bench_press\n
                        /squat\n
                        /dead_lift\n
                        "
                    }
                    "/bench_press" => {
                        let ans = match user.state() {
                            State::SelectExercise => {
                                user.set_state(State::SelectWeight);
                                let exercise = Exercise::new(text.clone());
                                user.training().add_exercise(exercise);
                                store.insert(user.id(), user);
                                "Select bench_press"
                            }
                            _ => "Can not select exersice now",
                        };
                        ans
                    }
                    "/squat" => {
                        let ans = match user.state() {
                            State::SelectExercise => {
                                user.set_state(State::SelectWeight);
                                let exercise = Exercise::new(text.clone());
                                user.training().add_exercise(exercise);
                                store.insert(user.id(), user);
                                "Select squat"
                            }
                            _ => "Can not select exersice now",
                        };
                        ans
                    }
                    "/dead_lift" => {
                        let ans = match user.state() {
                            State::SelectExercise => {
                                user.set_state(State::SelectWeight);
                                let exercise = Exercise::new(text.clone());
                                user.training().add_exercise(exercise);
                                store.insert(user.id(), user);
                                "Select dead_lift"
                            }
                            _ => "Can not select exersice now",
                        };
                        ans
                    }
                    "/exercise" => {
                        let ans = match user.state() {
                            State::SelectWeight => {
                                user.set_state(State::SelectExercise);
                                store.insert(user.id(), user);
                                "Start training, select exercise from:\n
                                /bench_press\n
                                /squat\n
                                /dead_lift"
                            }
                            _ => "can not select exersice now",
                        };
                        ans
                    }
                    "/show" => {
                        user_info = format!("User: {:?}", user);
                        user_info.as_str()
                    }
                    "/stop" => {
                        let training_time = user.training().get_start_time().elapsed();
                        user_info = format!(
                            "Finish trainig\nTraining duration: {:?}\nTraining info: {:?}",
                            training_time.clone(),
                            user.training().clone()
                        );
                        store.remove(&user.id());
                        user_info.as_str()
                    }
                    _ => "Unknown command, try /help",
                };
                answer.to_string()
            }
            MessageType::Number(num) => {
                let ans = match user.state() {
                    State::SelectWeight => {
                        user.set_state(State::SelectReps);
                        match user.training().get_exercises().last_mut() {
                            Some(ex) => {
                                log::info!("Add weight: {}", num);
                                ex.set_weight(num);
                                store.insert(user.id(), user);
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
                                store.insert(user.id(), user);
                                format!("Reps selected: {}, now select weight for next turn, or select /exercise for select another one", num)
                            }
                            None => "Exercise not found".to_string(),
                        }
                    }
                    _ => "can not process request".to_string(),
                };
                ans
            }
            MessageType::Other(text) => {
                format!("Unknown command: {} start command with /", text)
            }
        };
        String::from(bot_answer)
    }
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
