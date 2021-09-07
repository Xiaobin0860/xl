use sqlparser::dialect::Dialect;

#[derive(Debug, Default)]
pub struct XlDialect;

// 创建自己的方言。XlDialect 支持 identifier 可以是简单的 url
impl Dialect for XlDialect {
    fn is_identifier_start(&self, ch: char) -> bool {
        ('a'..='z').contains(&ch) || ('A'..='Z').contains(&ch) || ch == '_'
    }

    fn is_identifier_part(&self, ch: char) -> bool {
        ('a'..='z').contains(&ch)
            || ('A'..='Z').contains(&ch)
            || ('0'..='9').contains(&ch)
            || [':', '/', '?', '&', '=', '-', '_', '.'].contains(&ch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlparser::parser::Parser;

    fn exmample_sql() -> String {
        let url = "https://raw.githubusercontent.com/owid/covid-19-data/master/public/data/latest/owid-covid-latest.csv";
        format!(
            "SELECT location name, total_cases, new_cases, total_deaths, new_deaths \
        FROM {} where new_deaths>=500 ORDER BY new_cases DESC LIMIT 6 OFFSET 5",
            url
        )
    }

    #[test]
    fn it_works() {
        assert!(Parser::parse_sql(&XlDialect::default(), &exmample_sql()).is_ok());
        let ast = Parser::parse_sql(&XlDialect::default(), &exmample_sql()).unwrap();
        println!("{:#?}", ast);
    }
}
