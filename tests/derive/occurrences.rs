#![cfg(feature = "unstable-grouped")]
use clap::Parser;

#[test]
fn test_vec_of_vec() {
    #[derive(Parser, Debug, PartialEq)]
    struct Opt {
        #[arg(short = 'p', num_args = 2)]
        points: Vec<Vec<i32>>,
    }

    assert_eq!(
        Opt {
            points: vec![vec![1, 2], vec![0, 0]]
        },
        Opt::try_parse_from(&["test", "-p", "1", "2", "-p", "0", "0"]).unwrap()
    );
}

#[test]
fn test_vec_vec_empty() {
    #[derive(Parser, Debug, PartialEq)]
    struct Opt {
        #[arg(short = 'p', num_args = 2)]
        points: Vec<Vec<i32>>,
    }

    assert_eq!(
        Opt { points: vec![] },
        Opt::try_parse_from(&["test"]).unwrap()
    );
}

#[test]
fn test_option_vec_vec() {
    #[derive(Parser, Debug, PartialEq)]
    struct Opt {
        #[arg(short = 'p', num_args = 2)]
        points: Option<Vec<Vec<i32>>>,
    }

    assert_eq!(
        Opt {
            points: Some(vec![vec![1, 2], vec![3, 4]])
        },
        Opt::try_parse_from(&["test", "-p", "1", "2", "-p", "3", "4"]).unwrap()
    );
}

#[test]
fn test_option_vec_vec_empty() {
    #[derive(Parser, Debug, PartialEq)]
    struct Opt {
        #[arg(short = 'p', num_args = 2)]
        points: Option<Vec<Vec<i32>>>,
    }

    assert_eq!(
        Opt { points: None },
        Opt::try_parse_from(&["test"]).unwrap()
    );
}
