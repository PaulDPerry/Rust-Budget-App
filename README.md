# Budget App (Rust)

A personal budgeting tool built in Rust.

## Features
- Track transactions
- Plan fixed expenses
- Budget forecasting (planned)

## Tech
- Rust
- Serde (JSON storage)

## Run
cargo run

### Project Layout
budget-app/
├── README.md
├── .gitignore
├── src/
│   ├── main.rs
│   ├── app.rs              # App state & orchestration
│   │
│   ├── models/             # Data structures
│   │   ├── mod.rs
│   │   ├── transaction.rs
│   │   ├── budget.rs
│   │   └── category.rs
│   │
│   ├── storage/            # Persistence layer
│   │   ├── mod.rs
│   │   └── json.rs         # Start simple (serde)
│   │
│   ├── services/           # Business logic
│   │   ├── mod.rs
│   │   └── budget_service.rs
│   │
│   ├── ui/                 # CLI / TUI later
│   │   ├── mod.rs
│   │   └── cli.rs
│   │
│   └── utils/
│       ├── mod.rs
│       └── helpers.rs
│
└── data/
    └── budget.json         # Local storage file
