mod lib;

use crate::lib::{parser, paystack, custom};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Wrapper(#[serde(deserialize_with = "ast_deserializer")] parser::Ast);

fn ast_deserializer<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<parser::Ast, D::Error> {
    Ok(parser::parse(String::deserialize(deserializer)?))
}

fn custom_deserialization() {
    let source_code = "(This is a _special_ string)";

    println!(
        "Deserialized with the parser: {:?}",
        parser::parse(source_code.to_string())
    );
    println!(
        "Deserialized with serde: {:?}",
        serde_plain::from_str::<parser::Ast>(source_code).unwrap()
    );
    println!(
        "Deserialized with serde derive macro: {:?}",
        serde_plain::from_str::<Wrapper>(source_code).unwrap().0
    );
}

fn json_value_introspection() {
    let x: Option<String> = None;

    println!(
        "Here's x in string form: '{}'",
        serde_json::json!(x).to_string()
    );
}

fn paystack_parse() {
    let data = r#"{"event":"charge.success","data":{"id":4038018189,"domain":"test","status":"success","reference":"72rx4il2t6","amount":1500000,"message":null,"gateway_response":"Successful","paid_at":"2024-08-02T08:03:51.000Z","created_at":"2024-08-02T08:02:35.000Z","channel":"card","currency":"NGN","ip_address":"169.150.209.163","metadata": {"order_id": "01J42YKJ4AT2XEQE4V3ZWR7QJS"},"fees_breakdown":null,"log":null,"fees":32500,"fees_split":null,"authorization":{"authorization_code":"AUTH_tgez0q1oxo","bin":"408408","last4":"4081","exp_month":"12","exp_year":"2030","channel":"card","card_type":"visa ","bank":"TEST BANK","country_code":"NG","brand":"visa","reusable":true,"signature":"SIG_WCkWUdR4ON25Ue3Q9KOS","account_name":null},"customer":{"id":176868034,"first_name":null,"last_name":null,"email":"john.doe@gmail.com","customer_code":"CUS_cvxd4izaq5yu2kf","phone":null,"metadata":null,"risk_action":"default","international_format_phone":null},"plan":{},"subaccount":{},"split":{},"order_id":null,"paidAt":"2024-08-02T08:03:51.000Z","requested_amount":1500000,"pos_transaction_data":null,"source":{"type":"api","source":"merchant_api","entry_point":"transaction_initialize","identifier":null}}}"#;
    dbg!("{:?}", paystack::parse(data));
}

fn custom_parse () {
    let s: &str = r#"{"key":"Qj0vJgdkrZH0rUePP8iLGaJ+5ueCGtejMFuG00bBHjs="}"#;
    let cfg: custom::Config = serde_json::from_str(&s).unwrap();

    let j = serde_json::to_string(&cfg).unwrap();
    println!("{:#?}", cfg);
    println!("str: {}", j);
}

fn main() {
    // custom_deserialization();
    // json_value_introspection();
    // paystack_parse();
    custom_parse();
}
