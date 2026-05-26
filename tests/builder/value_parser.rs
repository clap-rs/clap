use std::ffi::OsString;
use std::path::PathBuf;

use clap::error::ErrorKind;
use clap::{arg, value_parser, Command};

#[test]
fn path_buf_value_parser_exists() {
    let dir = std::env::current_dir().unwrap();
    let matches = path_cmd(value_parser!(PathBuf).exists())
        .try_get_matches_from(path_args(&dir))
        .unwrap();

    assert_eq!(matches.get_one::<PathBuf>("path"), Some(&dir));
}

#[test]
fn path_buf_value_parser_rejects_missing_path() {
    let missing = missing_path();
    let err = path_cmd(value_parser!(PathBuf).exists())
        .try_get_matches_from(path_args(&missing))
        .unwrap_err();

    assert_eq!(err.kind(), ErrorKind::ValueValidation);
}

#[test]
fn path_buf_value_parser_is_file() {
    let file = std::env::current_exe().unwrap();
    let matches = path_cmd(value_parser!(PathBuf).is_file())
        .try_get_matches_from(path_args(&file))
        .unwrap();

    assert_eq!(matches.get_one::<PathBuf>("path"), Some(&file));
}

#[test]
fn path_buf_value_parser_rejects_directory_as_file() {
    let dir = std::env::current_dir().unwrap();
    let err = path_cmd(value_parser!(PathBuf).is_file())
        .try_get_matches_from(path_args(&dir))
        .unwrap_err();

    assert_eq!(err.kind(), ErrorKind::ValueValidation);
}

#[test]
fn path_buf_value_parser_is_dir() {
    let dir = std::env::current_dir().unwrap();
    let matches = path_cmd(value_parser!(PathBuf).is_dir())
        .try_get_matches_from(path_args(&dir))
        .unwrap();

    assert_eq!(matches.get_one::<PathBuf>("path"), Some(&dir));
}

#[test]
fn path_buf_value_parser_rejects_file_as_directory() {
    let file = std::env::current_exe().unwrap();
    let err = path_cmd(value_parser!(PathBuf).is_dir())
        .try_get_matches_from(path_args(&file))
        .unwrap_err();

    assert_eq!(err.kind(), ErrorKind::ValueValidation);
}

fn path_cmd(parser: impl Into<clap::builder::ValueParser>) -> Command {
    Command::new("test").arg(arg!(<path>).value_parser(parser))
}

fn path_args(path: &std::path::Path) -> [OsString; 2] {
    [OsString::from("test"), path.as_os_str().to_owned()]
}

fn missing_path() -> PathBuf {
    let id = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir().join(format!("clap-missing-{id}"))
}
