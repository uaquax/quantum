mod models;
mod telegram;
mod u66;

use std::{env, fs};

use models::{
    message::{JsonData, Message},
    wait_message::WaitMessage,
};
use serde_json::json;
use telegram::async_main;
use tokio::runtime;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut args = env::args();

    if let Some(arg) = args.nth(1) {
        if arg.contains("help") {
            println!("USAGE:\n\tu66\t[COMMAND]\n\nCOMMANDS:\n\thelp - print usage\n\trun - run");

            Ok(())
        } else if arg.contains("export") {
            let msgs = json!({"messages": [],"back_messages": []});
            let result = serde_json::to_string(&msgs).unwrap();
            fs::write("messages.json", result).unwrap();

            Ok(())
        } else if arg.contains("import") {
            let parsed_json: JsonData =
                serde_json::from_str(fs::read_to_string("messages.json").unwrap().as_str())
                    .unwrap();

            let messages: Vec<Message> = parsed_json.messages;
            let back_messages: Vec<WaitMessage> = parsed_json.back_messages;

            Ok(())
        } else {
            runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(async_main())
        }
    } else {
        runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async_main())
    }
}
