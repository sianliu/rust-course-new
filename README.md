# Rust Course

## Project Structure

```
rust-course/
├── assignments/
│   ├── assignment1/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── main.rs          # User struct & interest calculation
│   ├── assignment2/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── main.rs          # [To be completed]
│   ├── assignment3/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── main.rs          # [To be completed]
│   └── shared/
│       ├── Cargo.toml
│       └── src/
│           └── lib.rs           # Common utilities & shared code
├── capstone/
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs              # Capstone main application
│   │   └── lib.rs               # Capstone modules
│   ├── tests/                   # Unit tests
│   └── docs/                    # Project documentation
├── Cargo.toml                   # Workspace configuration
├── README.md                    # This file
└── .gitignore                   # Git ignore rules
```

## Running the Projects

To run individual assignments:

```bash
cd assignments/assignment1
cargo run

cd ../assignment2
cargo run

cd ../assignment3
cargo run
```

To run the capstone project:

```bash
cd capstone
cargo run
```

To run all projects from the workspace root:

```bash
cargo run -p assignment1
cargo run -p assignment2
cargo run -p assignment3
cargo run -p capstone
```
