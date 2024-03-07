use std::env;

use teloxide::{
    macros::BotCommands,
    repls::CommandReplExt,
    requests::{Requester, ResponseResult},
    types::{ChatId, Message},
    Bot,
};
use tokio::process::Command as ProcessCommand;

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "BuildIt! supports the following commands:"
)]
pub enum Command {
    GetIp,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let bot = Bot::from_env();
    Command::repl(bot, answer).await;
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::GetIp => {
            let id = env::var("TELEGRAM_ID").unwrap();
            if msg.chat.id != ChatId(id.parse::<i64>().unwrap()) {
                return Ok(());
            }

            match run_cmd().await {
                Ok(s) => bot.send_message(msg.chat.id, s).await?,
                Err(e) => bot.send_message(msg.chat.id, e.to_string()).await?,
            };
        }
    }

    Ok(())
}

async fn run_cmd() -> anyhow::Result<String> {
    let res = ProcessCommand::new("ip").arg("a").output().await?;

    let mut s = String::new();
    s.push_str(&String::from_utf8_lossy(&res.stdout));
    s.push_str("\n\n");
    s.push_str(&String::from_utf8_lossy(&res.stderr));

    Ok(s)
}
