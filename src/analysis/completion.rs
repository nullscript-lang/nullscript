use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::language::keywords::KEYWORDS;
use regex::Regex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionItem {
    pub label: String,
    pub kind: CompletionKind,
    pub detail: Option<String>,
    pub documentation: Option<String>,
    pub insert_text: Option<String>,
    pub filter_text: Option<String>,
    pub sort_text: Option<String>,
    pub snippet: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompletionKind {
    Keyword,
    Function,
    Variable,
    Class,
    Method,
    Property,
    Module,
    Snippet,
    Text,
}

#[derive(Debug, Clone)]
pub struct CompletionContext {
    pub text: String,
    pub position: usize,
}

#[derive(Debug, Clone)]
pub struct AutoCompletion {
    keyword_completions: Vec<CompletionItem>,
    context_completions: HashMap<String, Vec<CompletionItem>>,
    snippet_completions: Vec<CompletionItem>,
}

impl Default for AutoCompletion {
    fn default() -> Self {
        Self::new()
    }
}

impl AutoCompletion {
    pub fn new() -> Self {
        let mut completion = Self {
            keyword_completions: Vec::new(),
            context_completions: HashMap::new(),
            snippet_completions: Vec::new(),
        };

        completion.initialize_keywords();
        completion.initialize_context_completions();
        completion.initialize_snippets();

        completion
    }

    fn initialize_keywords(&mut self) {
        for (nullscript_keyword, js_keyword) in KEYWORDS.iter() {
            let completion = CompletionItem {
                label: nullscript_keyword.to_string(),
                kind: CompletionKind::Keyword,
                detail: Some(format!("NullScript keyword → {}", js_keyword)),
                documentation: Some(self.get_keyword_documentation(nullscript_keyword)),
                insert_text: Some(nullscript_keyword.to_string()),
                filter_text: Some(nullscript_keyword.to_string()),
                sort_text: Some(format!("0_{}", nullscript_keyword)),
                snippet: false,
            };
            self.keyword_completions.push(completion);
        }
    }

    fn initialize_context_completions(&mut self) {
        // Console/speak completions
        let speak_methods = vec![
            ("say", "log", "Log a message to the console"),
            ("yell", "warn", "Log a warning message"),
            ("scream", "error", "Log an error message"),
            ("whisper", "info", "Log an info message"),
            ("peek", "debug", "Log a debug message"),
            ("check", "assert", "Assert a condition"),
            ("wipe", "clear", "Clear the console"),
            ("tally", "count", "Count console calls"),
            ("resetcount", "countReset", "Reset console count"),
            ("show", "table", "Display data as a table"),
            ("time", "time", "Start a timer"),
            ("stoptimer", "timeEnd", "End a timer"),
            ("backtrace", "trace", "Show stack trace"),
        ];

        let speak_completions: Vec<CompletionItem> = speak_methods
            .into_iter()
            .map(|(ns_method, js_method, desc)| CompletionItem {
                label: ns_method.to_string(),
                kind: CompletionKind::Method,
                detail: Some(format!("speak.{} → console.{}", ns_method, js_method)),
                documentation: Some(desc.to_string()),
                insert_text: Some(format!("{}(${{1}})", ns_method)),
                filter_text: Some(ns_method.to_string()),
                sort_text: Some(format!("1_{}", ns_method)),
                snippet: true,
            })
            .collect();

        self.context_completions.insert("speak".to_string(), speak_completions);

        // Math completions
        let maths_methods = vec![
            ("abs", "Math.abs", "Absolute value"),
            ("ceil", "Math.ceil", "Round up to integer"),
            ("floor", "Math.floor", "Round down to integer"),
            ("round", "Math.round", "Round to nearest integer"),
            ("max", "Math.max", "Maximum value"),
            ("min", "Math.min", "Minimum value"),
            ("pow", "Math.pow", "Power/exponentiation"),
            ("sqrt", "Math.sqrt", "Square root"),
            ("random", "Math.random", "Random number 0-1"),
            ("sin", "Math.sin", "Sine function"),
            ("cos", "Math.cos", "Cosine function"),
            ("tan", "Math.tan", "Tangent function"),
        ];

        let maths_completions: Vec<CompletionItem> = maths_methods
            .into_iter()
            .map(|(method, js_equivalent, desc)| CompletionItem {
                label: method.to_string(),
                kind: CompletionKind::Method,
                detail: Some(format!("maths.{} → {}", method, js_equivalent)),
                documentation: Some(desc.to_string()),
                insert_text: Some(format!("{}(${{1}})", method)),
                filter_text: Some(method.to_string()),
                sort_text: Some(format!("1_{}", method)),
                snippet: true,
            })
            .collect();

        self.context_completions.insert("maths".to_string(), maths_completions);

        // Object/thing completions
        let thing_methods = vec![
            ("keys", "Object.keys", "Get object keys"),
            ("values", "Object.values", "Get object values"),
            ("entries", "Object.entries", "Get key-value pairs"),
            ("assign", "Object.assign", "Copy properties"),
            ("create", "Object.create", "Create object with prototype"),
            ("defineProperty", "Object.defineProperty", "Define property"),
            ("freeze", "Object.freeze", "Freeze object"),
            ("seal", "Object.seal", "Seal object"),
        ];

        let thing_completions: Vec<CompletionItem> = thing_methods
            .into_iter()
            .map(|(method, js_equivalent, desc)| CompletionItem {
                label: method.to_string(),
                kind: CompletionKind::Method,
                detail: Some(format!("thing.{} → {}", method, js_equivalent)),
                documentation: Some(desc.to_string()),
                insert_text: Some(format!("{}(${{1}})", method)),
                filter_text: Some(method.to_string()),
                sort_text: Some(format!("1_{}", method)),
                snippet: true,
            })
            .collect();

        self.context_completions.insert("thing".to_string(), thing_completions);
    }

    fn initialize_snippets(&mut self) {
        let snippets = vec![
            (
                "function",
                "Create a function",
                "run ${1:functionName}(${2:params}) {\n    ${3:// function body}\n    return ${4:value};\n}"
            ),
            (
                "class",
                "Create a class",
                "model ${1:ClassName} {\n    run __init__(${2:params}) {\n        ${3:// constructor}\n    }\n\n    run ${4:methodName}(${5:params}) {\n        ${6:// method body}\n    }\n}"
            ),
            (
                "if",
                "If statement",
                "whatever (${1:condition}) {\n    ${2:// if true}\n} otherwise {\n    ${3:// if false}\n}"
            ),
            (
                "for",
                "For loop",
                "since (let ${1:i} = 0; ${1:i} < ${2:length}; ${1:i}++) {\n    ${3:// loop body}\n}"
            ),
            (
                "while",
                "While loop",
                "when (${1:condition}) {\n    ${2:// loop body}\n}"
            ),
            (
                "try",
                "Try-catch block",
                "test {\n    ${1:// try code}\n} grab (${2:error}) {\n    ${3:// handle error}\n} atLast {\n    ${4:// finally}\n}"
            ),
            (
                "import",
                "Import statement",
                "use { ${1:imports} } from '${2:module}';"
            ),
            (
                "export",
                "Export statement",
                "share { ${1:exports} };"
            ),
            (
                "async",
                "Async function",
                "later run ${1:functionName}(${2:params}) {\n    ${3:// async function body}\n    return hold ${4:promise};\n}"
            ),
            (
                "promise",
                "Promise",
                "fresh promise((resolve, reject) => {\n    ${1:// promise body}\n    resolve(${2:value});\n})"
            ),
        ];

        for (label, detail, snippet) in snippets {
            let completion = CompletionItem {
                label: label.to_string(),
                kind: CompletionKind::Snippet,
                detail: Some(detail.to_string()),
                documentation: Some(format!("Code snippet: {}", detail)),
                insert_text: Some(snippet.to_string()),
                filter_text: Some(label.to_string()),
                sort_text: Some(format!("2_{}", label)),
                snippet: true,
            };
            self.snippet_completions.push(completion);
        }
    }

    fn get_keyword_documentation(&self, keyword: &str) -> String {
        match keyword {
            "run" => "Define a function. Usage: run functionName(params) { ... }".to_string(),
            "speak" => "Console object for logging. Usage: speak.say('message')".to_string(),
            "whatever" => "Conditional statement (if). Usage: whatever (condition) { ... }".to_string(),
            "otherwise" => "Else clause. Usage: whatever (condition) { ... } otherwise { ... }".to_string(),
            "since" => "For loop. Usage: since (let i = 0; i < 10; i++) { ... }".to_string(),
            "when" => "While loop. Usage: when (condition) { ... }".to_string(),
            "test" => "Try block for error handling. Usage: test { ... } grab (error) { ... }".to_string(),
            "grab" => "Catch block for error handling. Usage: test { ... } grab (error) { ... }".to_string(),
            "atLast" => "Finally block. Usage: test { ... } grab (error) { ... } atLast { ... }".to_string(),
            "fresh" => "Create new instance (new). Usage: fresh ClassName()".to_string(),
            "model" => "Define a class. Usage: model ClassName { ... }".to_string(),
            "fixed" => "Constant declaration (const). Usage: fixed name = value;".to_string(),
            "use" => "Import statement. Usage: use { item } from 'module';".to_string(),
            "share" => "Export statement. Usage: share { item };".to_string(),
            "later" => "Async function. Usage: later run functionName() { ... }".to_string(),
            "hold" => "Await expression. Usage: hold promise".to_string(),
            "maths" => "Math object for mathematical operations. Usage: maths.random()".to_string(),
            "yes" => "Boolean true value".to_string(),
            "no" => "Boolean false value".to_string(),
            "is" => "Strict equality operator (===)".to_string(),
            "isnt" => "Strict inequality operator (!==)".to_string(),
            "and" => "Logical AND operator (&&)".to_string(),
            "or" => "Logical OR operator (||)".to_string(),
            "not" => "Logical NOT operator (!)".to_string(),
            _ => "NullScript keyword that transpiles to JavaScript equivalent".to_string(),
        }
    }

    pub fn get_completions(&self, context: &CompletionContext) -> Vec<CompletionItem> {
        let mut completions = Vec::new();

        // Get the current word being typed
        let current_word = self.get_current_word(&context.text, context.position);
        let line_text = self.get_current_line(&context.text, context.position);

        // Check for context-aware completions (e.g., after "speak.")
        if let Some(context_key) = self.detect_context(&line_text, context.position) {
            if let Some(context_completions) = self.context_completions.get(&context_key) {
                completions.extend(
                    context_completions
                        .iter()
                        .filter(|item| self.matches_filter(&item.label, &current_word))
                        .cloned()
                );
            }
        } else {
            // Add keyword completions
            completions.extend(
                self.keyword_completions
                    .iter()
                    .filter(|item| self.matches_filter(&item.label, &current_word))
                    .cloned()
            );

            // Add snippet completions
            completions.extend(
                self.snippet_completions
                    .iter()
                    .filter(|item| self.matches_filter(&item.label, &current_word))
                    .cloned()
            );
        }

        // Sort completions by relevance
        completions.sort_by(|a, b| {
            // First by sort_text, then by label
            a.sort_text.cmp(&b.sort_text).then(a.label.cmp(&b.label))
        });

        completions
    }

    fn get_current_word(&self, text: &str, position: usize) -> String {
        let chars: Vec<char> = text.chars().collect();
        let mut start = position;
        let mut end = position;

        // Find start of word (go backwards)
        while start > 0 && chars.get(start - 1).is_some_and(|c| c.is_alphanumeric() || *c == '_') {
            start -= 1;
        }

        // Find end of word (go forwards)
        while end < chars.len() && chars.get(end).is_some_and(|c| c.is_alphanumeric() || *c == '_') {
            end += 1;
        }

        chars[start..end].iter().collect()
    }

    fn get_current_line(&self, text: &str, position: usize) -> String {
        let chars: Vec<char> = text.chars().collect();
        let mut start = position;

        // Find start of line
        while start > 0 && chars.get(start - 1) != Some(&'\n') {
            start -= 1;
        }

        // Find end of line
        let mut end = position;
        while end < chars.len() && chars.get(end) != Some(&'\n') {
            end += 1;
        }

        chars[start..end].iter().collect()
    }

    fn detect_context(&self, line_text: &str, _position: usize) -> Option<String> {
        // Check for object property access patterns
        let patterns = vec![
            (r"speak\.\s*$", "speak"),
            (r"maths\.\s*$", "maths"),
            (r"thing\.\s*$", "thing"),
        ];

        for (pattern, context) in patterns {
            if let Ok(regex) = Regex::new(pattern) {
                if regex.is_match(line_text) {
                    return Some(context.to_string());
                }
            }
        }

        None
    }

    fn matches_filter(&self, item_text: &str, filter: &str) -> bool {
        if filter.is_empty() {
            return true;
        }

        // Case-insensitive fuzzy matching
        let item_lower = item_text.to_lowercase();
        let filter_lower = filter.to_lowercase();

        // Exact prefix match gets highest priority
        if item_lower.starts_with(&filter_lower) {
            return true;
        }

        // Fuzzy match: all characters of filter must appear in order
        let mut item_chars = item_lower.chars();
        for filter_char in filter_lower.chars() {
            if !item_chars.any(|c| c == filter_char) {
                return false;
            }
        }

        true
    }

    pub fn get_signature_help(&self, context: &CompletionContext) -> Option<SignatureHelp> {
        let line_text = self.get_current_line(&context.text, context.position);

        // Look for function calls that need signature help
        if let Some(function_name) = self.extract_function_name(&line_text) {
            match function_name.as_str() {
                "speak.say" => Some(SignatureHelp {
                    signatures: vec![SignatureInformation {
                        label: "speak.say(message: any, ...optionalParams: any[]): void".to_string(),
                        documentation: Some("Outputs a message to the console.".to_string()),
                        parameters: vec![
                            ParameterInformation {
                                label: "message".to_string(),
                                documentation: Some("The message to log".to_string()),
                            }
                        ],
                    }],
                    active_signature: 0,
                    active_parameter: 0,
                }),
                "run" => Some(SignatureHelp {
                    signatures: vec![SignatureInformation {
                        label: "run functionName(parameters): returnType".to_string(),
                        documentation: Some("Define a function in NullScript.".to_string()),
                        parameters: vec![
                            ParameterInformation {
                                label: "functionName".to_string(),
                                documentation: Some("The name of the function".to_string()),
                            },
                            ParameterInformation {
                                label: "parameters".to_string(),
                                documentation: Some("Function parameters".to_string()),
                            }
                        ],
                    }],
                    active_signature: 0,
                    active_parameter: 0,
                }),
                _ => None,
            }
        } else {
            None
        }
    }

    fn extract_function_name(&self, line_text: &str) -> Option<String> {
        // Simple regex to extract function name from call pattern
        let patterns = vec![
            r"(speak\.\w+)\s*\(",
            r"(maths\.\w+)\s*\(",
            r"(run)\s+\w+\s*\(",
        ];

        for pattern in patterns {
            if let Ok(regex) = Regex::new(pattern) {
                if let Some(captures) = regex.captures(line_text) {
                    if let Some(match_) = captures.get(1) {
                        return Some(match_.as_str().to_string());
                    }
                }
            }
        }

        None
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureHelp {
    pub signatures: Vec<SignatureInformation>,
    pub active_signature: usize,
    pub active_parameter: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureInformation {
    pub label: String,
    pub documentation: Option<String>,
    pub parameters: Vec<ParameterInformation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterInformation {
    pub label: String,
    pub documentation: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diagnostic {
    pub range: Range,
    pub severity: DiagnosticSeverity,
    pub message: String,
    pub source: String,
    pub code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub line: u32,
    pub character: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiagnosticSeverity {
    Error = 1,
    Warning = 2,
    Information = 3,
    Hint = 4,
}

pub struct LanguageServer {
    auto_completion: AutoCompletion,
}

impl LanguageServer {
    pub fn new() -> Self {
        Self {
            auto_completion: AutoCompletion::new(),
        }
    }

    pub fn get_completions(&self, context: CompletionContext) -> Vec<CompletionItem> {
        self.auto_completion.get_completions(&context)
    }

    pub fn get_signature_help(&self, context: CompletionContext) -> Option<SignatureHelp> {
        self.auto_completion.get_signature_help(&context)
    }

    pub fn get_diagnostics(&self, text: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();

        // Check for forbidden TypeScript syntax
        let forbidden_patterns = vec![
            (r": (string|number|boolean|any)\b", "Type annotations are not allowed in NullScript"),
            (r"interface\s+\w+", "Interfaces are not supported in NullScript"),
            (r"enum\s+\w+", "Enums are not supported in NullScript"),
            (r"<T>", "Generic types are not supported in NullScript"),
            (r"implements\s+\w+", "Implements keyword is not supported in NullScript"),
        ];

        for (line_num, line) in text.lines().enumerate() {
            for (pattern, message) in &forbidden_patterns {
                if let Ok(regex) = Regex::new(pattern) {
                    for mat in regex.find_iter(line) {
                        diagnostics.push(Diagnostic {
                            range: Range {
                                start: Position {
                                    line: line_num as u32,
                                    character: mat.start() as u32,
                                },
                                end: Position {
                                    line: line_num as u32,
                                    character: mat.end() as u32,
                                },
                            },
                            severity: DiagnosticSeverity::Error,
                            message: message.to_string(),
                            source: "nullscript".to_string(),
                            code: Some("forbidden-syntax".to_string()),
                        });
                    }
                }
            }
        }

        diagnostics
    }
}
