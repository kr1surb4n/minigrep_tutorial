pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn parse_this() {
        let args: Vec<String> = vec![String::from("0"), String::from("a"), String::from("b")];

        let config = Config::new(&args);
        assert_eq!(config.query, "a");
        assert_eq!(config.file_path, "b");
    }

    #[test]
    fn build_this() {
        let args: Vec<String> = vec![String::from("0"), String::from("a"), String::from("b")];

        let config = Config::build(&args).unwrap_or_else(|err| {
            println!("Problem parsing arguments: {err}");
            process::exit(1);
        });

        assert_eq!(config.query, "a");
        assert_eq!(config.file_path, "b");
    }
}
