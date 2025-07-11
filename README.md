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
│   │   ├── main.rs              # Transaction data analysis application
│   │   ├── lib.rs               # Library documentation
│   │   ├── location.rs          # Country and continent enums
│   │   └── transaction.rs       # Transaction processing logic
│   ├── tests/                   # Unit tests
│   ├── docs/                    # Project documentation
│   └── transaction.csv          # Sample transaction data
├── Cargo.toml                   # Workspace configuration
├── README.md                    # This file
└── .gitignore                   # Git ignore rules
```

## Running the Projects

### Individual Assignments

To run individual assignments:

```bash
cd assignments/assignment1
cargo run

cd ../assignment2
cargo run

cd ../assignment3
cargo run
```

### Capstone Project

**Primary Method (Navigate to capstone folder):**

```bash
cd capstone
cargo run
```

**Alternative Method (From workspace root):**

```bash
cargo run -p capstone
```

### Running All Projects

To run all projects from the workspace root:

```bash
cargo run -p assignment1
cargo run -p assignment2
cargo run -p assignment3
cargo run -p capstone
```

## Documentation

### Generate and View Documentation

**For Capstone Project:**

```bash
# Navigate to capstone folder
cd capstone

# Generate and open documentation in browser
cargo doc --open

# Generate documentation without opening
cargo doc

# Generate with dependencies
cargo doc --document-private-items
```

**Alternative Syntax:**

```bash
# From workspace root
cargo doc -p capstone --open

# Generate documentation for all workspace members
cargo doc --workspace --open
```

**Manual Documentation Access:**
After running `cargo doc`, documentation is available at:

```
target/doc/capstone/index.html
```

## Capstone Features

The capstone project demonstrates:

- ✅ **Data Processing**: CSV parsing with error handling
- ✅ **Missing Value Detection**: Excludes incomplete records
- ✅ **Data Transformation**: Asset name capitalization
- ✅ **Geographic Analysis**: Country-to-continent mapping
- ✅ **Time Calculations**: Days under management
- ✅ **Statistical Analysis**: Investment totals by continent
- ✅ **Error Handling**: Comprehensive `Result<T>` usage
- ✅ **Data Structures**: HashMap, Vec, Enums, Structs
- ✅ **Documentation**: Full rustdoc with examples
