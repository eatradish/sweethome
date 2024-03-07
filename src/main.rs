use std::env;

use local_ip_address::list_afinet_netifas;
use teloxide::{
    macros::BotCommands,
    repls::CommandReplExt,
    requests::{Requester, ResponseResult},
    types::{ChatId, Message},
    Bot,
};

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

            match get_ip_list().await {
                Ok(s) => bot.send_message(msg.chat.id, s).await?,
                Err(e) => bot.send_message(msg.chat.id, e.to_string()).await?,
            };
        }
    }

    Ok(())
}

async fn get_ip_list() -> anyhow::Result<String> {
    let mut s = String::new();
    let network_interfaces = list_afinet_netifas()?;

    for (name, ip) in network_interfaces.iter() {
        s.push_str(&format!("{}:\t{:?}\n", name, ip));
    }

    Ok(s)
}
