pub struct StringUtils;

impl StringUtils {
    pub fn clean_error_message(message: &str) -> String {
        let clean_message = message
            .replace(regex::Regex::new(r"error TS\d+:\s*").unwrap().as_str(), "")
            .split('\n')
            .filter(|line| !line.trim().is_empty())
            .filter(|line| !line.starts_with("at "))
            .filter(|line| !line.contains("Command failed:"))
            .filter(|line| !line.contains("(node:"))
            .take(3)
            .collect::<Vec<&str>>()
            .join("\n")
            .trim()
            .to_string();

        clean_message
    }

    pub fn capitalize(s: &str) -> String {
        let mut chars = s.chars();
        match chars.next() {
            None => String::new(),
            Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        }
    }
}
