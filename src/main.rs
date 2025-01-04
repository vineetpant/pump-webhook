#[macro_use]
extern crate rocket;

use dotenv::dotenv;
use pump_webhook::{
    Payload, TelegramMessage, TokenInfoApiResponse, TokenPriceApiResponse, WebHookError, ENV_VARS,
};
use rocket::serde::json::Json;

#[get("/")]
fn index() -> &'static str {
    "Hello, I am just a bot!"
}

#[post("/webhook", format = "json", data = "<payload>")]
async fn process(payload: Json<Payload>) -> Result<String, WebHookError> {
    // Process the payload here
    // println!("{:?}?",payload);
    let env_vars = ENV_VARS.clone();

    if let Some(transactions) = payload.0.matched_transactions {
        for transaction in transactions {
            let mut price = String::new();
            let mut token_info = TokenInfoApiResponse::new();

            // Get token current price
            let url = format!(
                "{}{}={}",
                env_vars.radium_api, env_vars.mint_price_get, transaction.accounts.mint
            );
            let response_result = reqwest::get(&url)
                .await
                .map_err(|_| WebHookError::FailedToFetchPrice);

            match response_result {
                Ok(response) => match response.json::<TokenPriceApiResponse>().await {
                    Ok(result) => {
                        price = result
                            .data
                            .get(&transaction.accounts.mint)
                            .unwrap_or_else(|| &price)
                            .to_owned()
                    }
                    Err(err) => {
                        println!("{}", err.to_string())
                    }
                },
                Err(err) => println!("{}", err.to_string()),
            };

            // Get token information
            let url = format!(
                "{}{}={}",
                env_vars.radium_api, env_vars.mint_info_get, transaction.accounts.mint
            );
            let response_result = reqwest::get(&url)
                .await
                .map_err(|_| WebHookError::FailedToFetchPrice);

            match response_result {
                Ok(response) => match response.json::<TokenInfoApiResponse>().await {
                    Ok(result) => token_info = result,
                    Err(err) => {
                        println!("{}", err.to_string())
                    }
                },
                Err(err) => println!("{}", err.to_string()),
            };

            // telegram bot url to send post request
            let telegram_url = format!(
                "{}{}/{}",
                env_vars.telegram_url, env_vars.telegram_token, env_vars.telegram_send_function
            );

            let mut message = String::new();

            if token_info.success {
                message = format!(
                    "New Graduated Token {} {} {} {:?}",
                    &transaction.accounts.mint,
                    token_info.data[0].symbol,
                    token_info.data[0].name,
                    price
                );
            } else {
                message = format!(
                    "New Graduated Token {} failed to fetch price and other metadata",
                    &transaction.accounts.mint
                );
            }

            // Print and Send message to telegram bot

            println!("{}", message);

            let client = reqwest::Client::new();
            let my_data = TelegramMessage {
                chat_id: env_vars.telegram_chat_id.clone(),
                parse_mode: "Markdown".to_string(),
                text: message,
            };
            match client
                .post(telegram_url)
                .json(&my_data)
                .send()
                .await
                .map_err(|_| WebHookError::ErrorInSendingTelegramBotMessage)
            {
                Ok(response) => {
                    match response
                        .text()
                        .await
                        .map_err(|_| WebHookError::ErrorInSendingTelegramBotMessage)
                    {
                        Ok(response_text) => println!("{}", response_text),
                        Err(err) => println!("{}", err.to_string()),
                    }
                }
                Err(err) => println!("{}", err.to_string()),
            };
        }
    }
    Ok("Transaction Procesed".to_string())
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    rocket::build().mount("/", routes![index, process])
}
