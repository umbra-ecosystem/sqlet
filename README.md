# Sqlet

[![Crates.io][crates-badge]][crates-url]
[![docs.rs][docs-badge]][docs-url]
[![MIT licensed][mit-badge]][mit-url]
[![Build Status][circleci-badge]][circleci-url]

[crates-badge]: https://img.shields.io/crates/v/sqlet.svg
[crates-url]: https://crates.io/crates/sqlet
[docs-badge]: https://docs.rs/sqlet/badge.svg
[docs-url]: https://docs.rs/sqlet/
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: LICENSE
[circleci-badge]: https://img.shields.io/circleci/build/github/rust-db/sqlet
[circleci-url]: https://circleci.com/gh/rust-db/sqlet/tree/master

Powerful SQL migration toolkit for Rust.

This project, **sqlet**, is a **fork of the original refinery crate** ([https://github.com/rust-db/refinery](https://github.com/rust-db/refinery)). While it builds upon the robust foundation of refinery, we have introduced significant changes that deviate from its original philosophy to offer enhanced flexibility and control over your database migrations.

sqlet strives to make running migrations for different databases as easy as possible.
It works by running your migrations on a provided database connection, either by embedding them on your Rust code, or via the `sqlet_cli`.
Currently postgres, `tokio-postgres` , `mysql`, `mysql_async`, `rusqlite` and `tiberius` are supported.
If you are using a driver that is not yet supported, namely SQLx you can run migrations providing a Config instead of the connection type, as Config impl's Migrate. You will still need to provide the `postgres`/`mysql`/`rusqlite`/`tiberius` driver as a feature for Runner::run and `tokio-postgres/mysql_async` for `Runner::run_async`.

You can use sqlet to run migrations in your Rust code, or via the `sqlet_cli` command line tool.

## **Key Deviations from Original Refinery**

We have significantly altered the migration process to provide more control and flexibility:

* **Rollback Support:** We've added explicit **rollback support**, allowing you to revert applied migrations.
* **Timestamp-Based Versioning:** Migration versions are now based on **timestamps** in the format `{YYYYMMDD}_{HHMMSS}_{name}`. This ensures unique and chronologically ordered migrations, even across different development environments.
* **Flexible Migration Formats:** Migrations can now be defined in two distinct ways:
  * **Directory-based:** Migrations are placed in a directory containing up.sql and down.sql files for applying and reverting changes, respectively.
  * **File-based Rust Modules:** Migrations can be defined in a single Rust file ({filename}.rs) which contains both up and down functions.

These changes represent a significant departure from the original refinery crate's philosophy, offering a more robust and adaptable migration workflow.

## **Usage**

* Add **sqlet** to your Cargo.toml dependencies with the selected driver as feature eg: `sqlet = { version = "0.8", features = ["rusqlite"]}`
* Migrations are now named in the format `{YYYYMMDD}_{HHMMSS}_{name}` (for directories) or `{YYYYMMDD}_{HHMMSS}_{name}.rs` (for Rust modules).
* For directory-based SQL migrations, create a directory with the timestamp-based name, containing up.sql and down.sql.
* Migrations can be run either by embedding them in your Rust code with `embed_migrations!` macro, or via [sqlet_cli](https://crates.io/crates/sqlet_cli).

### **Example: Library**

```rust
use rusqlite::Connection;

mod embedded {
    use sqlet::embed_migrations; // Updated to sqlet
    embed_migrations!("./tests/sql_migrations");
}

fn main() {
    let mut conn = Connection::open_in_memory().unwrap();
    embedded::migrations::runner().run(&mut conn).unwrap();
}
```

For more library examples, refer to the [examples](https://github.com/rust-db/refinery/tree/main/examples).

### **Example: CLI**

```rust
export DATABASE_URL="postgres://postgres:secret@localhost:5432/your-db"
pushd migrations
    # To run migrations
    sqlet migrate -e DATABASE_URL -p ./src
    # To rollback the last applied migration (example for timestamp-based version)
    sqlet rollback -e DATABASE_URL -p ./src --version 20230526_100000_create_users_table
popd
```

### **Example: Deadpool**

```rust
let mut conn = pool.get().await?;
let client = conn.deref_mut().deref_mut();
let report = embedded::migrations::runner().run_async(client).await?;
```

### **Example: bb8**

```rust
let mut client = pool.dedicated_connection().await?;
let report = embedded::migrations::runner().run_async(&mut client).await?;
```

## **Implementation Details**

**sqlet** works by creating a table that keeps all the applied migrations' versions and their metadata. When you [run](https://docs.rs/refinery/latest/refinery/struct.Runner.html#method.run) the migrations Runner, **sqlet** compares the applied migrations with the ones to be applied, checking for [divergent](https://docs.rs/refinery/latest/refinery/struct.Runner.html#method.set_abort_divergent) and [missing](https://docs.rs/refinery/latest/refinery/struct.Runner.html#method.set_abort_missing) and executing unapplied migrations.

By default, sqlet runs each migration in a single transaction. Alternatively, you can also configure sqlet to wrap the entire execution of all migrations in a single transaction by setting `set_grouped` to true.
The rust crate intentionally ignores new migration files until your sourcecode is rebuild. This prevents accidental migrations and altering the database schema without any code changes. We can also bake the migrations into the binary, so no additional files are needed when deployed.

### **Rollback**

Unlike the original refinery and its earlier philosophy on undo/rollback migrations (which was based on Flyway), **this fork explicitly supports rollbacks**. You can define down.sql files for SQL-based migrations or down functions in Rust modules to revert changes introduced by their corresponding up counterparts. This provides a clear and direct mechanism for undoing migrations.

## **Support for Additional Database Drivers**

While initially it seemed beneficial to support as many aditional drivers as possible in this repo, with the current bandwidth available by the maintainers it's preferable to create them and maintain them on external repositories (see [here](https://github.com/rust-db/refinery/pull/264#issuecomment-1419198667) for context).

Notable external database drivers:

* [Klickhouse](https://github.com/Protryon/klickhouse) ([Clickhouse](https://clickhouse.tech/docs/en/) database driver with **sqlet** support)

## **MSRV**

**sqlet** aims to support stable Rust, the previous Rust version, and nightly.

## **Contributing**

:balloon: Thanks for your help to improve the project!
No contribution is too small and all contributions are valued, feel free to open Issues and submit Pull Requests.

## **License**

This project is licensed under the [MIT license](http://docs.google.com/LICENSE).

### **Contribution**

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in sqlet by you, shall be licensed as MIT, without any additional
terms or conditions.
