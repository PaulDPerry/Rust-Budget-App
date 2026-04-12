use std::fmt;
use crate::models::category::Category;
use serde::{Serialize,Deserialize};


#[derive(Serialize,Deserialize)]
pub struct Budget{
 pub category : Category,
 pub limit : i32,
 pub period: Period

}
impl Budget {
  pub fn new(category:Category, limit:i32,period:Period) ->Self{
    Self{
      category,
      limit,
      period,
    }
  }

  pub fn display(&self){
    println!("Category: {}", self.category);
    println!("Category: {}", self.limit);
    println!("Category: {}", self.period);
  }
}

#[derive(Serialize,Deserialize)]
pub enum Period {
  Weekly,
  Monthly
}

impl fmt::Display for Period{
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
    match self{
      Period::Weekly => write!(f, "Weekly"),
      Period::Monthly => write!(f, "Monthly"),
    }
  }
}