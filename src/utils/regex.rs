pub struct RegexUtils;

impl RegexUtils {
    pub fn extract_location(error_output: &str) -> (Option<u32>, Option<u32>) {
        let location_regex = regex::Regex::new(r"(\w+\.ts):(\d+):(\d+)\s*-\s*error|:(\d+):(\d+)").ok();

        if let Some(re) = location_regex {
            if let Some(captures) = re.captures(error_output) {
                let line = captures.get(2).or_else(|| captures.get(4))
                    .and_then(|m| m.as_str().parse().ok());
                let column = captures.get(3).or_else(|| captures.get(5))
                    .and_then(|m| m.as_str().parse().ok());
                return (line, column);
            }
        }

        (None, None)
    }

    pub fn extract_ts_error(error_output: &str) -> Option<String> {
        let error_regex = regex::Regex::new(r"error TS\d+: (.+)").ok()?;
        let lines: Vec<&str> = error_output.split('\n').collect();

        for line in lines {
            if line.contains("error TS") {
                if let Some(captures) = error_regex.captures(line) {
                    return captures.get(1).map(|m| m.as_str().to_string());
                }
            }
        }

        None
    }

    pub fn matches(pattern: &str, text: &str) -> bool {
        regex::Regex::new(pattern)
            .map(|re| re.is_match(text))
            .unwrap_or(false)
    }
}
