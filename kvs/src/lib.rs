mod error;
mod pb;
mod service;
mod storage;

pub use error::KvError;
pub use pb::*;
pub use service::*;
pub use storage::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
