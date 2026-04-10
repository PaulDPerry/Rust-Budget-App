mod models;
use models::Transaction;
use chrono::NaiveDate;
// mod services;
// mod storage;
// mod ui;
// mod utils;
// mod app;
fn main() {
    //let mut app = app::App::new();
    //app.run();
    let now = NaiveDate::from_ymd_opt(2026, 4, 6).unwrap();
    let test = Transaction::new(
        1,
        String::from("test"),
        50.00,
        now,
        String::from("Food")
    );

    test.print_transaction();
}
