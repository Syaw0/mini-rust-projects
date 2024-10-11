use std::{ env, error::Error, fs };

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filepath)?;

    let result = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    result.iter().for_each(|line| { println!("{line}") });

    Ok(())
}

#[derive(Debug)]
pub struct Config {
    query: String,
    filepath: String,
    ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(q) => q,
            None => {
                return Err("Didn't get the query string");
            }
        };

        let filepath = match args.next() {
            Some(q) => q,
            None => {
                return Err("Didn't get the filepath");
            }
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            filepath,
            ignore_case,
        })
    }
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive_none_result() {
        let query = "threee";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec![] as Vec<&str>, search(query, contents))
    }

    #[test]
    fn case_sensitive_one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }

    #[test]
    fn case_sensitive_two_result() {
        let query = "i";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive.", "Pick three."], search(query, contents))
    }

    // =====================================================================
    #[test]
    fn case_insensitive_one_result() {
        let query = "rUst";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."] as Vec<&str>,
            search_case_insensitive(query, contents)
        )
    }
}
