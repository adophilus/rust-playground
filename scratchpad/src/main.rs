use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionPurposeOrder {
    pub order_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionPurposeOther;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum TransactionPurpose {
    #[serde(rename = "other")]
    Other(TransactionPurposeOther),
    #[serde(rename = "order")]
    Order(TransactionPurposeOrder),
}

fn main() {
    let purpose = TransactionPurpose::Other(TransactionPurposeOther);
    // let purpose = TransactionPurpose::Order(TransactionPurposeOrder {
    //     order_id: "order-id".to_owned(),
    // });
    let purpose_string = json!(purpose).to_string();

    println!("Serialized purpose: {}", &purpose_string);

    let purpose_string = Some(purpose_string);
    println!("Deserialized purpose: {:?}", purpose_string.map(serde_json::de::from_str));
}
