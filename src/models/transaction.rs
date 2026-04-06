use chrono::NativeDate;

pub struct Transaction{
  pub id: u32,
  pub description: String,
  pub ammount: f64,
  pub date: NativeDate,
  pub catagory: String,
}

impl Transaction{
  pub fn new(id: u32, description: String, ammount: f64, date: NativeDate, catagory:String) -> Self{
    Self{
      id,
      description,
      ammount,
      date,
      catagory,
    }
  }
}

