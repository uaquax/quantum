mod models;
mod quantum;
mod telegram;
mod words;

use std::env;
use telegram::async_main;
use tokio::runtime;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut args = env::args();

    if let Some(arg) = args.nth(1) {
        if arg.contains("help") {
            println!("USAGE:\nquantum\t[COMMAND]\n\nCOMMANDS:\n\thelp - print usage\n\trun - run");

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
