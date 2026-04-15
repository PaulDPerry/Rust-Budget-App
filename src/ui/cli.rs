use std::io;

pub enum Command{
  Add,
  Remove,
  List,
  Quit,
}

impl Command{
  pub fn start(){
    let mut input = String::new();
    while input.trim() != "quit"{
      input.clear();
      println!("App Running");
      io::stdin().read_line(&mut input);
      // match String::from(input){
      //   "add" => Command::add(),
      //   "remove" => Command::remove(),
      //   "list" => Command::list(),
      //   "quit" => Command::quit(),
      // }
    }
  }
  fn from_input(input:String){
    //Pasrse command and data
    //Call execute with command and data
  }

  pub fn execute(opperation:Command, data:String){
    //Take in opperation and data executes
  }
}