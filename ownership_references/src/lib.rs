pub fn eat(s: String) -> bool { 
    if s.starts_with("b") && s.contains("a") {
        return true;
    } else {
        return false;
    }
}

pub fn bedazzle(s: &mut String) {
    *s = "sparkly".to_string();
}