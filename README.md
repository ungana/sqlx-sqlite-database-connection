SQLx with SQLite Example
========================

This is an example of using [SQLx](https://github.com/launchbadge/sqlx) with [SQLite](https://sqlite.org/). It doesn't do too much, but it you can see a simple example of how to setup your project to use the two together.

## Getting Started

You will need [Rust](https://rust-lang.org), [SQLite](https://sqlite.org/)(if your operating system does not already have it) and the [SQLx CLI](https://crates.io/crates/sqlx-cli). Once installed, navigate to this cloned project in the terminal and run:

```bash
cargo run
```

This project simply adds "John Doe" to an `example.db` file. You can see the additions using the [SQLite](https://sqlite.org/) app of your choice and checking the `Users` table.

```sql
SELECT * FROM Users;
```

