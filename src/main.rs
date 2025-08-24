// Copyright 2025 Baleine Jay - Phimmuno Engine
// Licensed under the Phicode Non-Commercial License (https://banes-lab.com/licensing)
// Commercial use requires a paid license. See link for details.
use aho_corasick::AhoCorasick;
use std::io::{self, Read};

const THREAT_PATTERNS: &[&str] = &[
    "eval(", "eval (", "exec(", "exec (",
    "subprocess.", "os.system(", "os.system (",
    "__import__", "getattr(__builtins__", "compile(",
    "globals(", "locals(", "open(", "input(",
    "pickle.", "marshal.", "ctypes.",
    "../", "rm -rf", "DELETE FROM", "DROP TABLE",
];

fn main() -> std::process::ExitCode {
    let mut content = String::new();
    if io::stdin().read_to_string(&mut content).is_err() {
        return std::process::ExitCode::FAILURE;
    }
    
    let detector = match AhoCorasick::new(THREAT_PATTERNS) {
        Ok(d) => d,
        Err(_) => return std::process::ExitCode::FAILURE,
    };
    
    let is_safe = !detector.is_match(&content);
    std::process::ExitCode::from(if is_safe { 0 } else { 1 })
}