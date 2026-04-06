use crate::models::transaction::Transaction;
use crate::storage::json;
pub struct App{
  pub transactions: Vec<Transaction>,
}

impl App {
    pub fn new() -> Self{
      let transactions = json::load().unwrap_or_default();
      Self { transactions }
    }
}