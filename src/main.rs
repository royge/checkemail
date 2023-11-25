use regex::Regex;
use std::env;
use trust_dns_resolver::config::*;
use trust_dns_resolver::error::ResolveError;
use trust_dns_resolver::Resolver;

fn get(email_address: &mut String) {
    let mut args = env::args();
    args.next();

    if let Some(v) = args.next() {
        *email_address = v.to_string();
    }
}

fn is_valid_email(email_address: &str) -> bool {
    // Honestly, I'm not a fan of regex but ChatGPT recommended this.
    // Regular expression to match email syntax
    match Regex::new(r"^[^@\s]+@[^@\s]+\.[^@\s]+$") {
        Ok(email_regex) => {
            if !email_regex.is_match(email_address) {
                return false;
            }
        }
        Err(_) => { return false; },
    }
    true
}

fn validate_domain(email_address: &str) -> Result<(), ResolveError> {
    let domain = email_address.split('@').nth(1).unwrap();
    let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default())?;
    let mx_records = resolver.mx_lookup(domain);
    match mx_records {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut email_address = String::new();

    get(&mut email_address);

    if !is_valid_email(&email_address) {
        return Err("Invalid email format!".into());
    }

    match validate_domain(&email_address) {
        Ok(_) => {
            println!("Email address {} is valid.", email_address);
        }
        Err(_) => {
            eprintln!("Email address {} is not valid.", email_address);
        }
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_valid_email() {
        let input = String::from("user@example.com");
        let res = is_valid_email(&input);
        assert_eq!(true, res);

        let input = String::from("userexample.com");
        let res = is_valid_email(&input);
        assert_eq!(false, res);

        let input = String::from("user@examplecom");
        let res = is_valid_email(&input);
        assert_eq!(false, res);

        let input = String::from("userexamplecom");
        let res = is_valid_email(&input);
        assert_eq!(false, res);

        let input = String::from("");
        let res = is_valid_email(&input);
        assert_eq!(false, res);
    }
}
