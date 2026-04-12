use std::fmt;
use serde::{Serialize,Deserialize};



#[derive(Clone, Copy,Serialize,Deserialize)]
pub enum Category{
  Groceries,
  EatingOut,
  Housing,
  Auto,
  Savings,
}

impl fmt::Display for Category {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
    match self{
      Category::Groceries => write!(f, "Groceries"),
      Category::EatingOut => write!(f, "Eating Out"),
      Category::Housing => write!(f, "Housing"),
      Category::Auto => write!(f, "Auto"),
      Category::Savings => write!(f, "Savings"),
    }
  }
}
#[derive(Clone, Copy,Serialize,Deserialize)]
pub enum Direction {
  Income,
  Expense
}

impl fmt::Display for Direction {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
    match self{
      Direction::Income => write!(f, "Income"),
      Direction::Expense => write!(f, "Expense"),
    }
  }
}