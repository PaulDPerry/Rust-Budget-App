mod models;
use models::Transaction;
use chrono::NaiveDate;
use models::Category;
use models::Direction;
use models::Budget;
use models::Period;

use crate::models::transaction;
use crate::storage::load;
use crate::storage::save;

// mod services;
mod storage;
// mod ui;
// mod utils;
mod app;
fn main() {
    //let app = app::App::new();
    //app.run();

    /*
    let now = NaiveDate::from_ymd_opt(2026, 4, 6).unwrap();
    let test = Transaction::new(
        1,
        String::from("test"),
        50.00,
        now,
        Category::EatingOut,
        Direction::Expense
    );
    let transactions = vec![test];
    save(&transactions);
    */

    load();





    //Sandbox
    /*
    let now = NaiveDate::from_ymd_opt(2026, 4, 6).unwrap();
    let test = Transaction::new(
        1,
        String::from("test"),
        50.00,
        now,
        Category::EatingOut,
        Direction::Expense
    );

    let test_budget:Budget = Budget::new(
        Category::EatingOut,
        400,
        Period::Monthly
    );

    test.display();
    println!();
    test_budget.display();
   */
}
