use serde::Deserialize;
use reqwest::{Client, Error, Response};

mod models;

#[derive(Deserialize)]
struct TelegramUpdate {
    message: Option<TelegramMessage>
}

#[derive(Deserialize)]
struct TelegramMessage {
    chat: TelegramChat,
    text: Option<String>
}

#[derive(Deserialize)]
struct TelegramChat {
    id: i64
}

#[tokio::main]
async fn main() {
    const TELEGRAM_BOT_TOKEN: &str = "7095767919:AAG9h7FA8-KnAvIrnBc6LdMWS-5S8yK6phU";
    let client = Client::new();

    let url = format!("https://api.telegram.org/bot{}/getUpdates", TELEGRAM_BOT_TOKEN);

    let res = client.get(&url).send().await;


    match res {
        Ok(res) => {
            println!("RESULT:\n {}", res.text().await.unwrap());
        },
        Err(err) => {
            println!("Error: {}", err);
        }
    }
    // loop {

    //     // let updates: Vec<TelegramUpdate> = res.json().await.unwrap();

    //     // for update in updates {
    //     //     if let Some(message) = update.message {
    //     //         if let Some(text) = message.text {

    //     //             let chat_id = message.chat.id;
    //     //             let response_text = format!("Echo: {}", text);

    //     //             let send_message_url = format!(
    //     //                 "https://api.telegram.org/bot{}/sendMessage?chat_id={}&text={}",
    //     //                 TELEGRAM_BOT_TOKEN, chat_id, response_text
    //     //             );

    //     //             let _ = client.get(&send_message_url).send().await;

    //     //         }
    //     //     }
    //     // }

    //     tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    // }


}
