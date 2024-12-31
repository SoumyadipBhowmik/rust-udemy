pub fn eat(s: String) -> bool { 
    s.starts_with("b") && s.contains("a")
}

pub fn bedazzle(s: &mut String) {
    *s = "sparkly".to_string();
}

pub fn inspect(s: &String) {
    if s.ends_with("s") {
        println!("Plural");
    } else {
        println!("Singular");
    }
}

pub fn change(s: &mut String) {
    if s.ends_with("s") {
        
    } else {
        s.push_str("s");
    }
}