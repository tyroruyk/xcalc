pub fn calculate(expression: &str) -> String {
    match meval::eval_str(expression) {
        Ok(result) => result.to_string(),
        Err(_) => "Error".to_string(),
    }
}
