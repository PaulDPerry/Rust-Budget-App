use std::fmt;
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
      Category::Groceries => write!(f, "Groceris"),
      Category::EatingOut => write!(f, "Eating Out"),
      Category::Housing => write!(f, "Housing"),
      Category::Auto => write!(f, "Auto"),
      Category::Savings => write!(f, "Savings"),
    }
  }
}

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