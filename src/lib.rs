pub mod util;

use dotenv::dotenv;
use lazy_static::lazy_static;

use std::{collections::HashMap, env, sync::Arc};

use rocket::{
    http::Status,
    response,
    serde::{Deserialize, Serialize},
    Request, Response,
};

#[derive(Serialize, Deserialize)]
pub struct TelegramMessage{
    pub chat_id: String,
    pub parse_mode: String,
    pub text: String
}

#[derive(Debug)]
pub enum WebHookError {
    EnrironmentVariableRead,
    FailedToFetchPrice,
    FailedToParsePriceApiResponse,
    FailedToParseInfoApiResponse,
    ErrorInSendingTelegramBotMessage,
}

impl std::fmt::Display for WebHookError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            WebHookError::EnrironmentVariableRead => {
                write!(f, "Error in reading environment variables")
            }
            WebHookError::FailedToFetchPrice => write!(f, "Error in fetching mint price"),
            WebHookError::FailedToParsePriceApiResponse => {
                write!(f, "Error in parsing Radium price api response")
            }
            WebHookError::FailedToParseInfoApiResponse => {
                write!(f, "Error in parsing Radium token info api response")
            }
            WebHookError::ErrorInSendingTelegramBotMessage => {
                write!(f, "Error in sending telegram bot message")
            },
        }
    }
}

impl std::error::Error for WebHookError {}

impl<'r> response::Responder<'r, 'static> for WebHookError {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        let (status, message) = match self {
            WebHookError::EnrironmentVariableRead => {
                (Status::InternalServerError, self.to_string())
            }
            WebHookError::FailedToFetchPrice => (Status::InternalServerError, self.to_string()),
            WebHookError::FailedToParsePriceApiResponse => {
                (Status::InternalServerError, self.to_string())
            }
            WebHookError::FailedToParseInfoApiResponse => {
                (Status::InternalServerError, self.to_string())
            }
            WebHookError::ErrorInSendingTelegramBotMessage => {
                (Status::InternalServerError, self.to_string())
            },
        };

        Response::build()
            .status(status)
            .header(rocket::http::ContentType::JSON)
            .sized_body(message.len(), std::io::Cursor::new(message))
            .ok()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Accounts {
    pub mint: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenInfoApiResponse {
    pub id: String,
    pub success: bool,
    pub data: Vec<Token>,
}

impl TokenInfoApiResponse {
    pub fn new() -> Self {
        Self {
            id: "".to_string(),
            success: false,
            data: vec![],
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    pub chain_id: u32,
    pub address: String,
    pub program_id: String,
    #[serde(rename = "logoURI")]
    pub logo_uri: String,
    pub symbol: String,
    pub name: String,
    pub decimals: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TokenPriceApiResponse {
    pub id: String,
    pub success: bool,
    pub data: HashMap<String, String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchedTransaction {
    pub accounts: Accounts,
    pub block_time: i64,
    pub signature: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Payload {
    pub matched_transactions: Option<Vec<MatchedTransaction>>,
}

lazy_static! {
    pub static ref ENV_VARS: Arc<EnvVars> = Arc::new(EnvVars::new());
}
pub struct EnvVars {
    pub radium_api: String,
    pub mint_price_get: String,
    pub mint_info_get: String,
    pub telegram_token: String,
    pub telegram_chat_id: String,
    pub telegram_url: String,
    pub telegram_send_function: String,
}
impl EnvVars {
    pub fn new() -> Self {
        dotenv().ok();
        EnvVars {
            radium_api: env::var("RADIUM_API_URL").expect("RADIUM_API_URL must be set"),
            mint_price_get: env::var("MINT_PRICE_METHOD").expect("MINT_PRICE_METHOD must be set"),
            mint_info_get: env::var("MINT_INFO_METHOD").expect("MINT_INFO_METHOD must be set"),
            telegram_token: env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN must be set"),
            telegram_chat_id: env::var("TELEGRAM_BOT_CHAT_ID").expect("TELEGRAM_BOT_CHAT_ID must be set"),
            telegram_url: env::var("TELEGRAM_BOT_URL").expect("TELEGRAM_BOT_URL must be set"),
            telegram_send_function: env::var("TELEGRAM_BOT_SEND_FUNCTION").expect("TELEGRAM_BOT_SEND_FUNCTION must be set"),
        }
    }
}
