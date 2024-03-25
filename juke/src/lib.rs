
pub mod strings {
    pub fn remove_char_at(s:&str,postions:Vec<usize>)->String{
        let mut result = String::new();
        for (i,c) in s.chars().enumerate(){
            if !postions.contains(&i){
                result.push(c);
            }
        }
        result
    }
    pub fn spilt_string(s:&str)->Vec<String>{
        let mut result = Vec::new();
        let mut temp = String::new();
        for c in s.chars(){
            if c.is_alphabetic(){
                temp.push(c);
            }else{
                if !temp.is_empty(){
                    result.push(temp.clone());
                    temp.clear();
                }
            }
        }
        if !temp.is_empty(){
            result.push(temp);
        }
        result
    }
}


pub fn prime_factor_solver(n: u64) -> String {
    prime_factors(n, "".to_string())
}

fn prime_factors(n: u64, spaces: String) -> String {
    for i in 2..=n {
        if n % i == 0 {
            let quotient = n / i;
            if quotient == 1 {
                return format!("{}{} {}", spaces, i, quotient);
            } else {
                let factor_string = format!("{}{} {}", spaces, i, quotient);
                let next_spaces = format!("{}  ", spaces);
                let quotient_string = prime_factors(quotient, next_spaces);
                return format!("{}\n{}", factor_string, quotient_string);
            }
        }
    }
    format!("{}{}", spaces, n)
}


pub fn boom(){
    unsafe{std::arch::asm!("int3")}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_char_at(){
        assert_eq!(strings::remove_char_at2("hello",vec![1,2]),"ho");
        assert_eq!(strings::remove_char_at2("hello",vec![0,1,2,3,4]),"");
        assert_eq!(strings::remove_char_at2("hello",vec![0,4]),"ell");
    }
}
