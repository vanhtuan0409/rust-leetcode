use regex::Regex;
use std::{
    env, fs,
    io::{Read, Write},
    path::Path,
};

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref LC_PATTERN: Regex =
        Regex::new(r"(?m)(?:https?://)?leetcode\.com/problems/([^/]+)/?").unwrap();
}

#[derive(Debug)]
struct ExitError(String);

fn parse_problem_name(problem: &str) -> Option<&str> {
    let captures = LC_PATTERN.captures(problem)?;
    captures.get(1).map(|m| m.as_str())
}

fn normalize_name(name: &str) -> String {
    name.replace("-", "_")
}

fn main() -> Result<(), ExitError> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err(ExitError("Must input leetcode url".to_string()));
    }

    // normalize input
    let input_url = args.get(1).unwrap().trim();
    let problem = parse_problem_name(input_url)
        .ok_or(ExitError("Unable to parse problem name".to_string()))?;
    let normalized_name = normalize_name(problem);

    // generate problem source code
    let template_str = fs::read_to_string("./templates/problem.rs")
        .map_err(|e| ExitError(format!("Cannot read problem template. Err: {}", e)))?;
    let source = template_str
        .replace("__PROBLEM_LINK__", input_url)
        .replace("__PROBLEM_NAME__", &normalized_name);
    let source_path = Path::new("./src/problems").join(format!("{}.rs", normalized_name));
    if source_path.exists() {
        return Err(ExitError("Source file already exists".to_string()));
    }
    let mut source_file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(source_path)
        .map_err(|e| ExitError(format!("Cannot open source path. Err: {}", e)))?;
    source_file
        .write_all(source.as_bytes())
        .map_err(|e| ExitError(format!("Cannot write into source file. Err: {}", e)))?;
    drop(source_file);

    let mut mod_file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .append(true)
        .open("./src/problems/mod.rs")
        .map_err(|e| ExitError(format!("Cannot open problems/mod.rs. Err: {}", e)))?;
    let mut mod_contents = String::new();
    mod_file
        .read_to_string(&mut mod_contents)
        .map_err(|e| ExitError(format!("Cannot read problems/mod.rs. Err: {}", e)))?;
    let problem_mod = format!("pub mod {};", normalized_name);
    if !mod_contents.contains(&problem_mod) {
        mod_file
            .write_fmt(format_args!("{}\n", problem_mod))
            .map_err(|e| ExitError(format!("Cannot write into problems/mod.rs. Err: {}", e)))?;
    }
    drop(mod_file);

    Ok(())
}
