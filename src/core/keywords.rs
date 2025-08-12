use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeywordCategory {
    pub title: String,
    pub keywords: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NullScriptKeywords {
    pub control_flow: KeywordCategory,
    pub error_handling: KeywordCategory,
    pub variables: KeywordCategory,
    pub imports: KeywordCategory,
    pub types: KeywordCategory,
    pub values: KeywordCategory,
    pub objects: KeywordCategory,
    pub operators: KeywordCategory,
    pub functions: KeywordCategory,
    pub keywords: KeywordCategory,
    pub multi_word: KeywordCategory,
    pub function_declarations: KeywordCategory,
}

impl Default for NullScriptKeywords {
    fn default() -> Self {
        Self::new()
    }
}

impl NullScriptKeywords {
    pub fn new() -> Self {
        let mut control_flow_keywords = HashMap::new();
        control_flow_keywords.insert("checkthis".to_string(), "if".to_string());
        control_flow_keywords.insert("orelse".to_string(), "else".to_string());
        control_flow_keywords.insert("loopin".to_string(), "for".to_string());
        control_flow_keywords.insert("whilevibe".to_string(), "while".to_string());
        control_flow_keywords.insert("switchup".to_string(), "switch".to_string());
        control_flow_keywords.insert("whenits".to_string(), "case".to_string());
        control_flow_keywords.insert("otherwise".to_string(), "default".to_string());
        control_flow_keywords.insert("keepgoing".to_string(), "continue".to_string());
        control_flow_keywords.insert("stopit".to_string(), "break".to_string());

        let mut error_handling_keywords = HashMap::new();
        error_handling_keywords.insert("oops".to_string(), "try".to_string());
        error_handling_keywords.insert("oop".to_string(), "try".to_string());
        error_handling_keywords.insert("mybad".to_string(), "catch".to_string());
        error_handling_keywords.insert("anyway".to_string(), "finally".to_string());

        let mut variable_keywords = HashMap::new();
        variable_keywords.insert("maybe".to_string(), "let".to_string());
        variable_keywords.insert("definitely".to_string(), "const".to_string());
        variable_keywords.insert("mayhap".to_string(), "var".to_string());

        let mut import_keywords = HashMap::new();
        import_keywords.insert("gimme".to_string(), "import".to_string());
        import_keywords.insert("yeet".to_string(), "export".to_string());

        let mut type_keywords = HashMap::new();
        type_keywords.insert("vibes".to_string(), "interface".to_string());
        type_keywords.insert("vibe".to_string(), "type".to_string());
        type_keywords.insert("mood".to_string(), "enum".to_string());
        type_keywords.insert("bigbrain".to_string(), "class".to_string());

        let mut value_keywords = HashMap::new();
        value_keywords.insert("fr".to_string(), "true".to_string());
        value_keywords.insert("cap".to_string(), "false".to_string());
        value_keywords.insert("nocap".to_string(), "null".to_string());
        value_keywords.insert("ghost".to_string(), "undefined".to_string());
        value_keywords.insert("sus".to_string(), "any".to_string());

        let mut object_keywords = HashMap::new();
        object_keywords.insert("dis".to_string(), "this".to_string());
        object_keywords.insert("parent".to_string(), "super".to_string());
        object_keywords.insert("fresh".to_string(), "new".to_string());
        object_keywords.insert("remove".to_string(), "delete".to_string());

        let mut operator_keywords = HashMap::new();
        operator_keywords.insert("and".to_string(), "&&".to_string());
        operator_keywords.insert("or".to_string(), "||".to_string());
        operator_keywords.insert("not".to_string(), "!".to_string());
        operator_keywords.insert("is".to_string(), "===".to_string());
        operator_keywords.insert("aint".to_string(), "!==".to_string());
        operator_keywords.insert("bigger".to_string(), ">".to_string());
        operator_keywords.insert("smaller".to_string(), "<".to_string());
        operator_keywords.insert("biggereq".to_string(), ">=".to_string());
        operator_keywords.insert("smallereq".to_string(), "<=".to_string());

        let mut function_keywords = HashMap::new();
        function_keywords.insert("pls".to_string(), "return".to_string());

        let mut other_keywords = HashMap::new();
        other_keywords.insert("with".to_string(), "with".to_string());
        other_keywords.insert("in".to_string(), "in".to_string());
        other_keywords.insert("of".to_string(), "of".to_string());
        other_keywords.insert("as".to_string(), "as".to_string());
        other_keywords.insert("from".to_string(), "from".to_string());

        let mut multi_word_keywords = HashMap::new();
        multi_word_keywords.insert("orsomething".to_string(), "else if".to_string());

        let mut function_declaration_keywords = HashMap::new();
        function_declaration_keywords.insert("feels async".to_string(), "async function".to_string());
        function_declaration_keywords.insert("feels".to_string(), "function".to_string());

        Self {
            control_flow: KeywordCategory {
                title: "Control Flow".to_string(),
                keywords: control_flow_keywords,
            },
            error_handling: KeywordCategory {
                title: "Error Handling".to_string(),
                keywords: error_handling_keywords,
            },
            variables: KeywordCategory {
                title: "Variable Declarations".to_string(),
                keywords: variable_keywords,
            },
            imports: KeywordCategory {
                title: "Import/Export".to_string(),
                keywords: import_keywords,
            },
            types: KeywordCategory {
                title: "Type Declarations".to_string(),
                keywords: type_keywords,
            },
            values: KeywordCategory {
                title: "Values".to_string(),
                keywords: value_keywords,
            },
            objects: KeywordCategory {
                title: "Object and Context".to_string(),
                keywords: object_keywords,
            },
            operators: KeywordCategory {
                title: "Operators and Expressions".to_string(),
                keywords: operator_keywords,
            },
            functions: KeywordCategory {
                title: "Functions".to_string(),
                keywords: function_keywords,
            },
            keywords: KeywordCategory {
                title: "Other Keywords".to_string(),
                keywords: other_keywords,
            },
            multi_word: KeywordCategory {
                title: "Multi-word Aliases".to_string(),
                keywords: multi_word_keywords,
            },
            function_declarations: KeywordCategory {
                title: "Function Declarations".to_string(),
                keywords: function_declaration_keywords,
            },
        }
    }

    pub fn get_all_keywords(&self) -> HashMap<String, String> {
        let mut all_keywords = HashMap::new();

        for category in self.all_categories() {
            all_keywords.extend(category.keywords.clone());
        }

        all_keywords
    }

    pub fn get_multi_word_keywords(&self) -> &HashMap<String, String> {
        &self.multi_word.keywords
    }

    pub fn get_function_keywords(&self) -> &HashMap<String, String> {
        &self.function_declarations.keywords
    }

    pub fn all_categories(&self) -> Vec<&KeywordCategory> {
        vec![
            &self.control_flow,
            &self.error_handling,
            &self.variables,
            &self.imports,
            &self.types,
            &self.values,
            &self.objects,
            &self.operators,
            &self.functions,
            &self.keywords,
            &self.multi_word,
            &self.function_declarations,
        ]
    }

    pub fn get_category(&self, name: &str) -> Option<&KeywordCategory> {
        match name {
            "control-flow" | "control_flow" => Some(&self.control_flow),
            "error-handling" | "error_handling" => Some(&self.error_handling),
            "variables" => Some(&self.variables),
            "imports" => Some(&self.imports),
            "types" => Some(&self.types),
            "values" => Some(&self.values),
            "objects" => Some(&self.objects),
            "operators" => Some(&self.operators),
            "functions" => Some(&self.functions),
            "keywords" => Some(&self.keywords),
            "multi-word" | "multi_word" => Some(&self.multi_word),
            "function-declarations" | "function_declarations" => Some(&self.function_declarations),
            _ => None,
        }
    }

    pub fn get_category_names(&self) -> Vec<&str> {
        vec![
            "control-flow",
            "error-handling",
            "variables",
            "imports",
            "types",
            "values",
            "objects",
            "operators",
            "functions",
            "keywords",
            "multi-word",
            "function-declarations",
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyword_creation() {
        let keywords = NullScriptKeywords::new();
        let all = keywords.get_all_keywords();

        assert_eq!(all.get("checkthis"), Some(&"if".to_string()));
        assert_eq!(all.get("definitely"), Some(&"const".to_string()));
        assert_eq!(all.get("fr"), Some(&"true".to_string()));
        assert_eq!(all.get("feels"), Some(&"function".to_string()));
    }

    #[test]
    fn test_function_keywords() {
        let keywords = NullScriptKeywords::new();
        let function_keywords = keywords.get_function_keywords();

        assert_eq!(function_keywords.get("feels"), Some(&"function".to_string()));
        assert_eq!(function_keywords.get("feels async"), Some(&"async function".to_string()));
    }

    #[test]
    fn test_multi_word_keywords() {
        let keywords = NullScriptKeywords::new();
        let multi_word = keywords.get_multi_word_keywords();

        assert_eq!(multi_word.get("orsomething"), Some(&"else if".to_string()));
    }

    #[test]
    fn test_category_access() {
        let keywords = NullScriptKeywords::new();

        assert!(keywords.get_category("control-flow").is_some());
        assert!(keywords.get_category("variables").is_some());
        assert!(keywords.get_category("nonexistent").is_none());
    }
}
