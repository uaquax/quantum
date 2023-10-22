use crate::{
    models::{config::Config, dialog::Dialog, wait_message::WaitMessage},
    words::{NO, YES},
};
use grammers_client::{types::Chat, Client, Update};
use lazy_static::lazy_static;
use regex::Regex;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::Mutex;

type Res = std::result::Result<(), Box<dyn std::error::Error>>;
const TIMEOUT: i32 = 40;

lazy_static! {
    static ref CONFIG: Config = Config::new();
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

lazy_static! {
    static ref DIALOGS_COUNT: Arc<Mutex<u16>> = Arc::new(Mutex::new(0));
}

async fn get_channel(client: &Client) -> Option<Chat> {
    let mut dialogs = client.iter_dialogs();
    while let Some(dialog) = dialogs.next().await.unwrap_or_default() {
        if dialog.chat().id() == CONFIG.dialogs_channel {
            let ch = dialog.chat().clone();

            return Some(ch);
        }
    }

    None
}

async fn get_target(client: &Client) -> Option<Chat> {
    let mut dialogs = client.iter_dialogs();
    while let Some(dialog) = dialogs.next().await.unwrap_or_default() {
        if dialog.chat().id() == CONFIG.target_id {
            let ch = dialog.chat().clone();

            return Some(ch);
        }
    }

    None
}

async fn contains_yes() -> Regex {
    Regex::new(&format!("\\b(?:{})\\b", YES.lock().await.join("|"))).expect("Invalid regex pattern")
}

async fn contains_no() -> Regex {
    Regex::new(&format!("\\b(?:{})\\b", NO.lock().await.join("|"))).expect("Invalid regex pattern")
}

async fn new_message(text: String) {
    let mut dlg = DIALOG.lock().await;
    if dlg.is_some() {
        dlg.as_mut()
            .unwrap()
            .messages
            .as_mut()
            .unwrap()
            .push(text.clone());

        if text.contains('@') || text.contains('＠') {
            dlg.as_mut().unwrap().username = Some(text);
        } else {
            let re = Regex::new(r"\b\d+\b").unwrap();
            if let Some(captured) = re.captures(&text) {
                if let Ok(age) = captured[0].parse::<i32>() {
                    dlg.as_mut().unwrap().age = Some(age);
                }
            }
        }
    }
}

async fn reset() {
    let mut wait_message_lock = WAIT_MESSAGE.lock().await;
    *wait_message_lock = None;

    let mut wt = WAIT_TIME.lock().await;
    *wt = SystemTime::now();
}

async fn store_dialog(client: &Client) {
    if let Some(channel) = get_channel(&client).await {
        let mut dlg = DIALOG.lock().await;
        let dialog = dlg.clone();
        *dlg = None;

        if dialog.is_some() {
            if dialog.as_ref().unwrap().username.is_some()
                || dialog.as_ref().unwrap().name.is_some()
            {
                client
                    .send_message(
                        &channel,
                        format!(
                            "DIALOG {}\n\nName: {:?}\nAge: {:?}\nUsername: {:?}\nMessages: {:#?}",
                            DIALOGS_COUNT.lock().await,
                            dialog.as_ref().unwrap().name,
                            dialog.as_ref().unwrap().age,
                            dialog.as_ref().unwrap().username,
                            dialog.as_ref().unwrap().messages,
                        ),
                    )
                    .await
                    .unwrap();
            }
        }
    }
}

pub async fn handle_update(client: Client, update: Update) -> Res {
    match update {
        Update::NewMessage(msg) if !msg.outgoing() && msg.chat().id() == CONFIG.target_id => {
            new_message(msg.text().to_string()).await;

            for bmsg in &CONFIG.back_messages {
                if bmsg.target.contains("@yes") {
                    if contains_yes()
                        .await
                        .is_match(msg.text().to_lowercase().as_str())
                    {
                        client
                            .send_message(msg.chat(), bmsg.reply.clone())
                            .await
                            .unwrap();
                    }

                    if let Some(wait_msg) = &bmsg.wait_message {
                        let mut wait_message_lock = WAIT_MESSAGE.lock().await;
                        *wait_message_lock = Some(*wait_msg.clone());

                        let mut wt = WAIT_TIME.lock().await;
                        *wt = SystemTime::now();
                    } else {
                        reset().await;
                    }
                } else if bmsg.target.contains("@no") {
                    if contains_no()
                        .await
                        .is_match(msg.text().to_lowercase().as_str())
                    {
                        client
                            .send_message(msg.chat(), bmsg.reply.clone())
                            .await
                            .unwrap();
                    }

                    if let Some(wait_msg) = &bmsg.wait_message {
                        let mut wait_message_lock = WAIT_MESSAGE.lock().await;
                        *wait_message_lock = Some(*wait_msg.clone());

                        let mut wt = WAIT_TIME.lock().await;
                        *wt = SystemTime::now();
                    } else {
                        reset().await;
                    }
                } else if msg.text().contains(&bmsg.target.to_lowercase()) {
                    if bmsg.reply.contains("@") {
                        {
                            let mut wait_message_lock = WAIT_MESSAGE.lock().await;
                            *wait_message_lock = Some(bmsg.clone());

                            let mut wt = WAIT_TIME.lock().await;
                            *wt = SystemTime::now();
                        }

                        return Ok(());
                    } else {
                        client
                            .send_message(msg.chat(), bmsg.reply.clone())
                            .await
                            .unwrap();
                    }
                }
            }

            if msg
                .text()
                .to_lowercase()
                .contains(CONFIG.dialog_keyword.to_lowercase().as_str())
            {
                let mut dlg = DIALOG.lock().await;
                *dlg = Some(Dialog {
                    name: None,
                    age: None,
                    username: None,
                    messages: Some(vec![]),
                });
                let mut dc = DIALOGS_COUNT.lock().await;
                *dc += 1;
            }

            let wait_message = {
                let wait_message_lock = WAIT_MESSAGE.lock().await;
                wait_message_lock.clone()
            };

            if let Some(wait_msg) = wait_message {
                let mut dlg = DIALOG.lock().await;
                if dlg.is_some() {
                    if wait_msg.target.contains("@name") {
                        dlg.as_mut().unwrap().name = Some(msg.text().to_string());

                        if &wait_msg.reply == "@end" {
                            let mut wait_message_lock = WAIT_MESSAGE.lock().await;
                            *wait_message_lock = Some(wait_msg.clone());
                        } else if &wait_msg.reply.len() >= &1 {
                            client
                                .send_message(msg.chat(), wait_msg.reply.clone())
                                .await
                                .unwrap();
                        }

                        if let Some(wait_msg) = &wait_msg.wait_message {
                            let mut wait_message_lock = WAIT_MESSAGE.lock().await;
                            *wait_message_lock = Some(*wait_msg.clone());

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
                        reset().await;
                    }
                } else if wait_msg.target.contains("@yes") {
                    if contains_yes()
                        .await
                        .is_match(msg.text().to_lowercase().as_str())
                    {
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
                        reset().await;
                    }
                } else if wait_msg.target.contains("@no") {
                    if contains_no()
                        .await
                        .is_match(msg.text().to_lowercase().as_str())
                    {
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
                        reset().await;
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
                        reset().await;
                    }
                }
            }
        }
        _ => {}
    }

    Ok(())
}

pub async fn start_quantum(client: Client) {
    let wait_message_clone = WAIT_MESSAGE.clone();
    let mut messages = CONFIG.messages.clone().into_iter();

    if let Some(chat) = &get_target(&client).await {
        loop {
            let mut wait_message = wait_message_clone.lock().await;

            if wait_message.is_none() {
                let msg = match messages.next() {
                    Some(msg) => msg,
                    None => {
                        store_dialog(&client).await;

                        messages = CONFIG.messages.clone().into_iter();
                        let msg = messages.next().unwrap();
                        msg
                    }
                };

                if let Some(wait_msg) = msg.wait_message {
                    if wait_msg.reply == "@end" {
                        messages = CONFIG.messages.clone().into_iter();

                        store_dialog(&client).await;
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
                        messages = CONFIG.messages.clone().into_iter();
                        *wait_message = None;

                        store_dialog(&client).await;
                    }
                }

                /* --- Check for timeout ---- */
                if WAIT_TIME.lock().await.elapsed().unwrap().as_secs() as i32 >= TIMEOUT {
                    messages = CONFIG.messages.clone().into_iter();
                    *wait_message = None;
                }
            }

            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }
}
