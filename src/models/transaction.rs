use chrono::NaiveDate;
use crate::models::category::{Category, Direction};
use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize)]
pub struct Transaction{
  pub id: u32,
  pub description: String,
  pub ammount: f64,
  pub date: NaiveDate,
  pub catagory: Category,
  pub direction: Direction,
}

impl Transaction{
  pub fn new(id: u32, description: String, ammount: f64, date: NaiveDate, catagory:Category, direction:Direction) -> Self{
    Self{
      id,
      description,
      ammount,
      date,
      catagory,
      direction,
    }
  }
  pub fn display(&self){
    println!("ID: {}", self.id);
    println!("Description: {}", self.description);
    println!("Ammount: {}", self.ammount);
    println!("Date: {}", self.date);
    println!("Catagory: {}", self.catagory);
    println!("Direction: {}", self.direction);
  }
}