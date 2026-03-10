use std::process::{Command, Stdio};
use std::io::Write;

#[test]
fn test_fib_program() {
    let mut child = Command::new("./target/debug/Whitespace")
        .arg("tests/fib.ws")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let stdin = child.stdin.as_mut().unwrap();
    stdin.write_all(b"3\n").unwrap();

    let output = child.wait_with_output().unwrap();
    let out = String::from_utf8(output.stdout).unwrap();

    assert!(out.contains("1"));
}

#[test]
fn test_hello_program() {
    let child = Command::new("./target/debug/Whitespace")
        .arg("tests/hello.ws")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let output = child.wait_with_output().unwrap();
    let out = String::from_utf8(output.stdout).unwrap();

    assert_eq!(out, "Hello, world!");
}

#[test]
fn test_collatz() {
    let mut child = Command::new("./target/debug/Whitespace")
        .arg("tests/collaz.ws")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let stdin = child.stdin.as_mut().unwrap();
    stdin.write_all(b"15\n").unwrap();

    let output = child.wait_with_output().unwrap();
    let out = String::from_utf8(output.stdout).unwrap();

    assert_eq!(out, "Enter a number: 15\n46\n23\n70\n35\n106\n53\n160\n80\n40\n20\n10\n5\n16\n8\n4\n2\n1\n");
}
