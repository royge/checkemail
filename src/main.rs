use std::env;
use regex::{Regex, Error};
use trust_dns_resolver::config::*;
use trust_dns_resolver::Resolver;
use trust_dns_resolver::error::ResolveError;

fn get(email_address: &mut String) {
    let mut args = env::args();
    args.next();

    match args.next() {
        Some(v) => { *email_address = v.to_string() },
        None => {},
    }
}

fn validate_format(email_address: &String) -> Result<(), Error> {
    // Honestly, I'm not a fan of regex but ChatGPT recommended this.
    // Regular expression to match email syntax
    let email_regex = Regex::new(r"^[^@\s]+@[^@\s]+\.[^@\s]+$")?;
    if !email_regex.is_match(email_address) {
        return Err(Error::Syntax(String::from("Invalid email format")));
    }
    Ok(())
}

fn validate_domain(email_address: &String) -> Result<(), ResolveError> {
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

    if let Err(error) = validate_format(&email_address) {
        eprintln!("{}", error);
        return Err(Box::new(error));
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
    fn test_validate_email() {
        let input = String::from("user@example.com");
        let res = validate_format(&input);
        assert_eq!(Ok(()), res);

        let input = String::from("userexample.com");
        let res = validate_format(&input);
        assert_ne!(Ok(()), res);

        let input = String::from("user@examplecom");
        let res = validate_format(&input);
        assert_ne!(Ok(()), res);

        let input = String::from("userexamplecom");
        let res = validate_format(&input);
        assert_ne!(Ok(()), res);

        let input = String::from("");
        let res = validate_format(&input);
        assert_ne!(Ok(()), res);
    }
}
