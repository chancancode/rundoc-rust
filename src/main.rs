fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use lalrpop_util::lalrpop_mod;

    lalrpop_mod!(parser);

    #[test]
    fn parser_can_parse() {
        assert!(parser::TermParser::new().parse("22").is_ok());
        assert!(parser::TermParser::new().parse("(22)").is_ok());
        assert!(parser::TermParser::new().parse("((((22))))").is_ok());
        assert!(parser::TermParser::new().parse("((22)").is_err());
    }
}
