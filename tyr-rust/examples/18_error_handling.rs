use anyhow::{anyhow, Result};
use std::io;
use std::panic;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataStoreError {
    #[error("data store disconnected")]
    Disconnect(#[from] io::Error),
    #[error("the data for key `{0}` is not available")]
    Redaction(String),
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader { expected: String, found: String },
    #[error("unknown data store error")]
    Unknown,
    #[error(transparent)]
    Other(#[from] anyhow::Error), // source and Display delegate to anyhow::Error
}

fn test_error(i: i32) -> Result<i32> {
    match i {
        1 => {
            let e = DataStoreError::InvalidHeader {
                expected: "0".to_string(),
                found: "1".to_string(),
            };
            Err(e.into())
        }
        2 => Err(anyhow!("anyhow::Error {}", i)),
        _ => Ok(i),
    }
}

fn test_error2(s: &str) -> Result<i32> {
    let i: i32 = s.parse()?;
    test_error(i)
}

fn main() -> Result<()> {
    let result = panic::catch_unwind(|| {
        println!("hello!");
    });
    assert!(result.is_ok());
    let result = panic::catch_unwind(|| {
        panic!("oh no!");
    });
    assert!(result.is_err());
    println!("panic captured: {:#?}", result);

    assert_eq!(test_error(3)?, 3);
    assert_eq!(test_error2("3")?, 3);
    assert_eq!(test_error2("4")?, 4);

    let result = test_error2("a");
    println!("{:?}", result);
    let result = test_error2("2");
    println!("{:?}", result);
    let result = test_error2("1");
    println!("{:?}", result);

    Ok(())
}
