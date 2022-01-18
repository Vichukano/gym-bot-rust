use frankenstein::{Api, GetUpdatesParamsBuilder, SendMessageParamsBuilder, TelegramApi};
use super::handler::MessageHandler;
pub struct Bot {
    api: Api,
}

impl Bot {
    pub fn new(token: String) -> Self {
        let api = Api::new(token.as_str());
        return Bot { api };
    }

    pub fn start(&self, mut handler: MessageHandler) {
        let mut update_params_builder = GetUpdatesParamsBuilder::default();
        let mut update_params = update_params_builder
            .allowed_updates(vec!["message".to_string()])
            .build()
            .unwrap();
        loop {
            let result = self.api.get_updates(&update_params);
            log::debug!("Receive result: {:?}", result);
            match result {
                Ok(response) => {
                    for update in response.result {
                        if let Some(message) = update.message {
                            let answer = handler.handle_message(&message);
                            let send_message_params = SendMessageParamsBuilder::default()
                                .chat_id(message.chat.id)
                                .text(answer.as_str())
                                .reply_to_message_id(message.message_id)
                                .build()
                                .unwrap();
                            if let Err(err) = self.api.send_message(&send_message_params) {
                                log::error!("Failed to send message: {:?}", err);
                            }
                            update_params = update_params_builder
                                .offset(update.update_id + 1)
                                .build()
                                .unwrap();
                        }
                    }
                }
                Err(error) => {
                    log::error!("Failed to get updates: {:?}", error);
                }
            }
        }
    }
}
