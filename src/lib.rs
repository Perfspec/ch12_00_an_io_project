use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
	
	for line in search(&config.query, &contents) {
        println!("{}", line);
    }
	
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	let mut results = Vec::new();
	
	for line in contents.lines() {
		if line.contains(query) {
			results.push(line);
		}
	}
    results
}

#[cfg(test)]
mod tests {
	use super::*;
	
    #[test]
	#[should_panic(expected = "not enough arguments")]
    fn when_less_than_three_arguments_then_return_error() {
		let args = vec!["first".to_string(), "second".to_string()];
        Config::new(&args).unwrap();
    }
	
	#[test]
	fn when_at_least_three_arguments_then_create_config() {
		let args = vec!["first".to_string(), "second".to_string(), "third".to_string()];
        let conf = Config::new(&args).unwrap();
		let mut sum = String::new();
		sum.push_str(&conf.query);
		sum.push_str(&conf.filename);
		assert_eq!(&sum, "secondthird");
    }
	
	#[test]
	fn when_given_appropriate_config_then_can_run() -> Result<(), &'static str> {
		let args = vec!["first".to_string(), "query".to_string(), "poem.txt".to_string()];
        match Config::new(&args) {
			Ok(config) => match run(config) {
				Ok(()) => Ok(()),
				Err(_) => Err("could not run config")
			},
			Err(e) => return Err(e),
		}
	}
	
	#[test]
    fn when_one_line_contains_search_query_then_return_that_line_in_a_vec() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}