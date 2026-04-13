use crate::models::Transaction;
use serde_json;

//Reading from json
pub fn load() -> Option<Vec<Transaction>>{
  let json_string = std::fs::read_to_string("data/ledger.json").ok()?;
  let transactions: Vec<Transaction> = serde_json::from_str(&json_string).ok()?;
  Some(transactions)
}

//Write to json
pub fn save(transactions :&[Transaction]){
  let json_string = serde_json::to_string_pretty(transactions).unwrap();
  std::fs::write("data/ledger.json", json_string).unwrap();
}