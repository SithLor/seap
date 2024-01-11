#[derive(Debug)]
pub struct EmailSeparated {
    username: String,
    domain: String,
    tld: String,
}
pub fn email_to_struct(email: &str) -> EmailSeparated {
    let parts: Vec<&str> = email.split('@').collect();
    let username = parts[0];
    let domain_parts: Vec<&str> = parts[1].split('.').collect();
    let domain = domain_parts[0];
    let tld = domain_parts[1];
    EmailSeparated {
        username: username.to_string(),
        domain: domain.to_string(),
        tld: tld.to_string(),
    }
}
pub fn email_struct_to_string(email: &EmailSeparated) -> String {
    let mut email = String::new();
    email.push_str(&email.username);
    email.push_str("@");
    email.push_str(&email.domain);
    email.push_str(".");
    email.push_str(&email.tld);
    email
}

#[macro_export]
macro_rules! email_to_struct {
    ($e:expr) => {
        {
            let parts: Vec<&str> = $e.split('@').collect();
            let username = parts[0];
            let domain_parts: Vec<&str> = parts[1].split('.').collect();
            let domain = domain_parts[0];
            let tld = domain_parts[1];
            EmailSeparated {
                username: username.to_string(),
                domain: domain.to_string(),
                tld: tld.to_string(),
            }
        }
    };
}
#[macro_export]
macro_rules! email_struct_to_string {
    ($e:expr) => {
        {
            let mut email = String::new();
            email.push_str(&$e.username);
            email.push_str("@");
            email.push_str(&$e.domain);
            email.push_str(".");
            email.push_str(&$e.tld);
            email
        }
    };
}




#[cfg(test)]
mod tests {
    use super::*;

}
