mod convert;
mod dialect;

pub use dialect::XlDialect;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
