use std::fs;
use std::error::Error;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {

        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        let case_sensitive: bool;
        if args.len() > 3 {
            case_sensitive = get_case_sensitivities_from_arg(&args[3]);
        } else {
            case_sensitive = false
        }

        Ok(Config {
            query,
            file_path,
            case_sensitive
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    let search_results: Vec<&str> = if config.case_sensitive {
                                        search_case_sensitive(&config.query, &contents)
                                    } else {
                                        search_case_insensitive(&config.query, &contents)
                                    };

    if search_results.len() == 0 {
        println!("SYSTEM MESSAGE: No matches found")
    } else {
        println!("SYSTEM MESSAGE: {} matches found", search_results.len());
        print_vec(search_results);
    }


    Ok(())
}

pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = vec![];

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = vec![];
    let query = query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

fn get_case_sensitivities_from_arg(arg: &str) -> bool {
    match arg {
        "case_sensitive" => true,
        "-c_s" => true,
        "-cs" => true,
        _ => false
    }
}

fn print_vec(vec: Vec<&str>) {
    for line in vec {
        println!("{line}");
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_work() {
        let query = "duct";
        let contents = "\n
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search_case_sensitive(query, contents));
    }

    #[test]
    fn should_not_work() {
        let query = "asdfvbg";
        let contents = "\n
Rust:
safe, fast, productive.
Pick three.";

        assert_ne!(vec!["safe, fast, productive."], search_case_sensitive(query, contents));
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search_case_sensitive(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }

    #[test]
    fn get_positive_case_sensitivities_cs() {
        assert!(get_case_sensitivities_from_arg("-cs"))
    }

    #[test]
    fn get_positive_case_sensitivities_c_s() {
        assert!(get_case_sensitivities_from_arg("-c_s"))
    }

    #[test]
    fn get_positive_case_sensitivities() {
        assert!(get_case_sensitivities_from_arg("case_sensitive"))
    }

    #[test]
    fn get_negative_case_sensitivities() {
        assert!(!get_case_sensitivities_from_arg("random_value"))
    }
}