use chrono::NaiveDate;

pub struct Transaction{
  pub id: u32,
  pub description: String,
  pub ammount: f64,
  pub date: NaiveDate,
  pub catagory: String,
}

impl Transaction{
  pub fn new(id: u32, description: String, ammount: f64, date: NaiveDate, catagory:String) -> Self{
    Self{
      id,
      description,
      ammount,
      date,
      catagory,
    }
  }
  pub fn print_transaction(self){
    println!("ID: {}", self.id);
    println!("Description: {}", self.description);
    println!("Ammount: {}", self.ammount);
    println!("Date: {}", self.date);
    println!("Catagory: {}", self.catagory);
  }
}