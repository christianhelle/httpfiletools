use std::{process::Command, str};

struct Golden<'a> {
    args: &'a [&'a str],
    exit_code: i32,
    stdout: &'a str,
    stderr: &'a str,
}

impl Golden<'_> {
    fn assert_matches(&self) {
        let output = Command::new(env!("CARGO_BIN_EXE_httpfiletools"))
            .args(self.args)
            .output()
            .expect("CLI should execute");

        assert_eq!(output.status.code(), Some(self.exit_code));
        assert_eq!(
            normalize_newlines(str::from_utf8(&output.stdout).unwrap()),
            self.stdout
        );
        assert_eq!(
            normalize_newlines(str::from_utf8(&output.stderr).unwrap()),
            self.stderr
        );
    }
}

fn normalize_newlines(value: &str) -> String {
    value.replace("\r\n", "\n")
}

#[test]
#[ignore = "pending Linus CLI implementation; run explicitly to lock stdout/stderr/exit-code compatibility"]
fn help_output_matches_golden_fixture() {
    Golden {
        args: &["--help"],
        exit_code: 0,
        stdout: include_str!("fixtures/golden/help.stdout"),
        stderr: include_str!("fixtures/golden/empty.stderr"),
    }
    .assert_matches();
}
