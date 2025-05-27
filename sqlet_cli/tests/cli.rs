mod cli {
    use assert_cmd::prelude::*;
    use predicates::str::contains;
    use std::process::Command;

    // `sqlet` with no args should exit with a non-zero code.
    #[test]
    fn cli_no_args() {
        Command::cargo_bin("sqlet").unwrap().assert().failure();
    }

    #[test]
    fn cli_version() {
        Command::cargo_bin("sqlet")
            .unwrap()
            .args(["-V"])
            .assert()
            .stdout(contains(env!("CARGO_PKG_VERSION")));
    }

    // `sqlet migrate` with no args should exit with a non-zero code.
    #[test]
    fn migrate_no_args() {
        Command::cargo_bin("sqlet")
            .unwrap()
            .args(["migrate"])
            .assert()
            .failure();
    }

    // `sqlet generate` with no args should exit with a non-zero code.
    #[test]
    fn rollback_no_args() {
        Command::cargo_bin("sqlet")
            .unwrap()
            .args(["generate"])
            .assert()
            .failure();
    }
}
