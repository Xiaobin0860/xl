mod error;
mod pb;
mod storage;

pub use error::KvError;
pub use pb::abi::*;
pub use storage::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
