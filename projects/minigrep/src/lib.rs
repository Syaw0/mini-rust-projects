use std::{ error::Error, fs, vec };

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filepath)?;

    search(&config.query, &contents)
        .iter()
        .for_each(|line| { println!("{line}") });

    Ok(())
}

#[derive(Debug)]
pub struct Config {
    query: String,
    filepath: String,
}

impl Config {
    pub fn build(mut args: Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not Enough Arguments");
        }
        let query = args.remove(1);
        let filepath = args.remove(1);

        Ok(Config {
            query,
            filepath,
        })
    }
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = vec![];

    contents.lines().for_each(|line| {
        if line.contains(query) {
            results.push(line);
        }
    });

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn none_result() {
        let query = "threee";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec![] as Vec<&str>, search(query, contents))
    }

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }

    #[test]
    fn two_result() {
        let query = "i";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive.", "Pick three."], search(query, contents))
    }
}
