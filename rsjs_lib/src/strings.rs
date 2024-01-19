
//vec!["a", "b", "c"] -> "a,b,c"
pub fn add_delimter(vec: Vec<&str>,delimter:char) -> String {
    let mut string = String::new();
    for i in vec {
        string.push_str(i);
        string.push_str(delimter.to_string().as_str());
    }
    string.remove(string.len() - 1);
    string
}
pub fn remove_delimter(string: String,delimter:char) -> Vec<String> {
    let mut vec: Vec<String> = Vec::new();
    let string: String = string;
    let string: &str = string.as_str();
    let string: std::str::Split<'_, char> = string.split(delimter);
    for i in string {
        vec.push(i.to_string());
    }
    vec
}
//turn String to static str






