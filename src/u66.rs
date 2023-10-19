use std::{
    fs,
    time::{Duration, SystemTime},
};

use crate::models::{
    self,
    config::Config,
    dialog::Dialog,
    message::{JsonData, Message},
    wait_message::WaitMessage,
};
use grammers_client::{Client, Update};
use lazy_static::lazy_static;
use std::sync::Arc;
use tokio::sync::Mutex;

type Res = std::result::Result<(), Box<dyn std::error::Error>>;
const TIMEOUT: i32 = 90;
lazy_static! {
    static ref CONFIG: Config = Config {
        target_id: 660309226,
        target_username: "AnonRuBot".to_string()
    };
}
lazy_static! {
    static ref WAIT_MESSAGE: Arc<Mutex<Option<WaitMessage>>> = Arc::new(Mutex::new(None));
}
lazy_static! {
    static ref DIALOG: Arc<Mutex<Option<Dialog>>> = Arc::new(Mutex::new(None));
}
lazy_static! {
    static ref WAIT_TIME: Arc<Mutex<SystemTime>> = Arc::new(Mutex::new(SystemTime::now()));
}

fn get_messages() -> Vec<models::message::Message> {
    let parsed_json: JsonData =
        serde_json::from_str(fs::read_to_string("messages.json").unwrap().as_str()).unwrap();
    let messages: Vec<Message> = parsed_json.messages;

    messages
}

fn get_back_messages() -> Vec<models::wait_message::WaitMessage> {
    let parsed_json: JsonData =
        serde_json::from_str(fs::read_to_string("messages.json").unwrap().as_str()).unwrap();
    let back_messages: Vec<WaitMessage> = parsed_json.back_messages;

    back_messages
}

pub async fn handle_update(client: Client, update: Update) -> Res {
    match update {
        Update::NewMessage(msg) if !msg.outgoing() && msg.chat().id() == CONFIG.target_id => {
            // let mut dialog = DIALOG.lock().await;
            // match dialog.as_mut() {
            //     Some(dlg) => {
            //         dlg.messages
            //             .as_mut()
            //             .expect("Failed to unwrap messages")
            //             .push(msg.text().to_string());
            //         println!("{:?}", dlg);
            //     }
            //     None => {
            //         warn!("Empty dialog");
            //     }
            // }

            let back_messages = get_back_messages();

            for bmsg in back_messages {
                if msg.text().contains(&bmsg.target) {
                    if bmsg.reply.contains("@") {
                        {
                            let mut wait_message_lock = WAIT_MESSAGE.lock().await;
                            *wait_message_lock = Some(bmsg);

                            let mut wt = WAIT_TIME.lock().await;
                            *wt = SystemTime::now();
                        }

                        return Ok(());
                    } else {
                        client.send_message(msg.chat(), bmsg.reply).await.unwrap();
                    }
                }
            }

            let wait_message = {
                let wait_message_lock = WAIT_MESSAGE.lock().await;
                wait_message_lock.clone()
            };

            if let Some(wait_msg) = wait_message {
                if wait_msg.target == "@str" {
                    if wait_msg.reply == "@end" {
                        let mut wait_message_lock = WAIT_MESSAGE.lock().await;
                        *wait_message_lock = Some(wait_msg.clone());
                    } else if wait_msg.reply.len() >= 1 {
                        client
                            .send_message(msg.chat(), wait_msg.reply)
                            .await
                            .unwrap();
                    }

                    if let Some(wait_msg) = wait_msg.wait_message {
                        let mut wait_message_lock = WAIT_MESSAGE.lock().await;
                        *wait_message_lock = Some(*wait_msg);

                        let mut wt = WAIT_TIME.lock().await;
                        *wt = SystemTime::now();
                    } else {
                        let mut wait_message_lock = WAIT_MESSAGE.lock().await;
                        *wait_message_lock = None;

                        let mut wt = WAIT_TIME.lock().await;
                        *wt = SystemTime::now();
                    }
                } else if msg.text().contains(&wait_msg.target) {
                    if wait_msg.reply.len() >= 1 {
                        client
                            .send_message(msg.chat(), wait_msg.reply)
                            .await
                            .unwrap();
                    }

                    if let Some(wait_msg) = wait_msg.wait_message {
                        let mut wait_message_lock = WAIT_MESSAGE.lock().await;
                        *wait_message_lock = Some(*wait_msg);

                        let mut wt = WAIT_TIME.lock().await;
                        *wt = SystemTime::now();
                    } else {
                        let mut wait_message_lock = WAIT_MESSAGE.lock().await;
                        *wait_message_lock = None;

                        let mut wt = WAIT_TIME.lock().await;
                        *wt = SystemTime::now();
                    }
                }
            }
        }
        _ => {}
    }

    Ok(())
}

pub async fn start_u66(client: Client) {
    let wait_message_clone = WAIT_MESSAGE.clone();
    let mut messages = get_messages().into_iter();

    loop {
        let mut wait_message = wait_message_clone.lock().await;

        if wait_message.is_none() {
            let msg = match messages.next() {
                Some(msg) => msg,
                None => {
                    messages = get_messages().into_iter();
                    let msg = messages.next().unwrap();
                    msg
                }
            };

            let chat = client
                .resolve_username(&CONFIG.target_username)
                .await
                .unwrap()
                .unwrap();

            if let Some(wait_msg) = msg.wait_message {
                if wait_msg.reply == "@end" {
                    messages = get_messages().into_iter();
                } else {
                    client.send_message(chat, msg.content).await.unwrap();
                    *wait_message = Some(*wait_msg);

                    let mut wt = WAIT_TIME.lock().await;
                    *wt = SystemTime::now();
                }
            } else {
                client.send_message(chat, msg.content).await.unwrap();
            }
        } else {
            if let Some(wait_msg) = wait_message.as_ref() {
                if wait_msg.reply == "@end" {
                    messages = get_messages().into_iter();
                    *wait_message = None;
                }
            }

            /* --- Check for timeout ---- */
            if WAIT_TIME.lock().await.elapsed().unwrap().as_secs() as i32 >= TIMEOUT {
                messages = get_messages().into_iter();
                *wait_message = None;
            }
        }

        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}
