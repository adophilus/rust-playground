use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum Error {
    DeserializationFailed,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata {
    pub order_id: String,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "event", content = "data")]
pub enum PaystackEvent {
    #[serde(rename = "charge.success")]
    TransactionSuccessful {
        amount: BigDecimal,
        metadata: Metadata,
    },
}

pub fn parse(data: &str) -> Result<PaystackEvent, Error> {
    serde_json::from_str::<PaystackEvent>(data).map_err(|err| {
        println!(
            "Error occurred while trying to deserialize paystack data string: {}",
            err
        );
        Error::DeserializationFailed
    })
}
