

use regex::Regex;
use std::collections::HashMap;
use crate::language::keywords::KEYWORDS;

pub struct ReverseTranspiler {
    js_to_ns_map: HashMap<String, String>,
}

impl ReverseTranspiler {
    pub fn new() -> Self {
        let mut js_to_ns_map = HashMap::new();

        // Build reverse mapping from JavaScript to NullScript
        for (ns_keyword, js_keyword) in KEYWORDS.iter() {
            js_to_ns_map.insert(js_keyword.to_string(), ns_keyword.to_string());
        }

        Self { js_to_ns_map }
    }

    pub fn reverse_transpile(&self, js_content: &str) -> Result<String, Box<dyn std::error::Error>> {
        let mut output = js_content.to_string();

        // Remove source map comments
        let source_map_regex = Regex::new(r"//# sourceMappingURL=.*")?;
        output = source_map_regex.replace_all(&output, "").to_string();

        // Convert function declarations
        let function_regex = Regex::new(r"function\s+([a-zA-Z_$][\w$]*)\s*\(([^)]*)\)\s*\{")?;
        output = function_regex.replace_all(&output, "run $1($2) {").to_string();

        // Convert async function declarations
        let async_function_regex = Regex::new(r"async\s+function\s+([a-zA-Z_$][\w$]*)\s*\(([^)]*)\)\s*\{")?;
        output = async_function_regex.replace_all(&output, "later run $1($2) {").to_string();

        // Convert static method declarations
        let static_method_regex = Regex::new(r"static\s+([a-zA-Z_$][\w$]*)\s*\(([^)]*)\)\s*\{")?;
        output = static_method_regex.replace_all(&output, "run forever $1($2) {").to_string();

        // Convert class declarations
        let class_regex = Regex::new(r"class\s+([a-zA-Z_$][\w$]*)\s*\{")?;
        output = class_regex.replace_all(&output, "model $1 {").to_string();

        // Convert extends
        let extends_regex = Regex::new(r"class\s+([a-zA-Z_$][\w$]*)\s+extends\s+([a-zA-Z_$][\w$]*)\s*\{")?;
        output = extends_regex.replace_all(&output, "model $1 inherits $2 {").to_string();

        // Convert constructor
        let constructor_regex = Regex::new(r"constructor\s*\(([^)]*)\)\s*\{")?;
        output = constructor_regex.replace_all(&output, "__init__($1) {").to_string();

        // Convert method declarations in classes
        let method_regex = Regex::new(r"(\s+)([a-zA-Z_$][\w$]*)\s*\(([^)]*)\)\s*\{")?;
        output = method_regex.replace_all(&output, "$1run $2($3) {").to_string();

        // Convert variable declarations
        let const_regex = Regex::new(r"\bconst\b")?;
        output = const_regex.replace_all(&output, "fixed").to_string();

        let let_regex = Regex::new(r"\blet\b")?;
        output = let_regex.replace_all(&output, "let").to_string();

        let var_regex = Regex::new(r"\bvar\b")?;
        output = var_regex.replace_all(&output, "var").to_string();

        // Convert import/export statements
        let import_regex = Regex::new(r"import\s+")?;
        output = import_regex.replace_all(&output, "use ").to_string();

        let export_regex = Regex::new(r"export\s+")?;
        output = export_regex.replace_all(&output, "share ").to_string();

        // Convert control flow
        let if_regex = Regex::new(r"\bif\b")?;
        output = if_regex.replace_all(&output, "whatever").to_string();

        let else_regex = Regex::new(r"\belse\b")?;
        output = else_regex.replace_all(&output, "otherwise").to_string();

        let for_regex = Regex::new(r"\bfor\b")?;
        output = for_regex.replace_all(&output, "since").to_string();

        let while_regex = Regex::new(r"\bwhile\b")?;
        output = while_regex.replace_all(&output, "when").to_string();

        // Convert try-catch-finally
        let try_regex = Regex::new(r"\btry\b")?;
        output = try_regex.replace_all(&output, "test").to_string();

        let catch_regex = Regex::new(r"\bcatch\b")?;
        output = catch_regex.replace_all(&output, "grab").to_string();

        let finally_regex = Regex::new(r"\bfinally\b")?;
        output = finally_regex.replace_all(&output, "atLast").to_string();

        // Convert keywords using the mapping
        for (js_keyword, ns_keyword) in &self.js_to_ns_map {
            // Skip keywords we've already handled above
            if matches!(js_keyword.as_str(), "function" | "const" | "let" | "var" | "if" | "else" | "for" | "while" | "try" | "catch" | "finally" | "class" | "import" | "export") {
                continue;
            }

            let pattern = format!(r"\b{}\b", regex::escape(js_keyword));
            if let Ok(regex) = Regex::new(&pattern) {
                output = regex.replace_all(&output, ns_keyword).to_string();
            }
        }

        // Convert operators (fix potential capacity overflow by processing in order)
        let strict_equal_regex = Regex::new(r"===")?;
        output = strict_equal_regex.replace_all(&output, " is ").to_string();

        let strict_not_equal_regex = Regex::new(r"!==")?;
        output = strict_not_equal_regex.replace_all(&output, " isnt ").to_string();

        let and_regex = Regex::new(r"&&")?;
        output = and_regex.replace_all(&output, " and ").to_string();

        let or_regex = Regex::new(r"\|\|")?;
        output = or_regex.replace_all(&output, " or ").to_string();

        // Be more careful with ! replacement to avoid breaking !==
        let not_regex = Regex::new(r"!(\w+)")?;
        output = not_regex.replace_all(&output, "not $1").to_string();

        // Convert new keyword
        let new_regex = Regex::new(r"\bnew\s+")?;
        output = new_regex.replace_all(&output, "fresh ").to_string();

        // Convert this/super
        let this_regex = Regex::new(r"\bthis\b")?;
        output = this_regex.replace_all(&output, "self").to_string();

        let super_regex = Regex::new(r"\bsuper\b")?;
        output = super_regex.replace_all(&output, "parent").to_string();

        // Convert delete
        let delete_regex = Regex::new(r"\bdelete\s+")?;
        output = delete_regex.replace_all(&output, "remove ").to_string();

        // Convert async/await
        let await_regex = Regex::new(r"\bawait\s+")?;
        output = await_regex.replace_all(&output, "hold ").to_string();

        let async_regex = Regex::new(r"\basync\s+")?;
        output = async_regex.replace_all(&output, "later ").to_string();

        // Convert break/continue
        let break_regex = Regex::new(r"\bbreak\b")?;
        output = break_regex.replace_all(&output, "stop").to_string();

        let continue_regex = Regex::new(r"\bcontinue\b")?;
        output = continue_regex.replace_all(&output, "keepgoing").to_string();

        // Convert switch/case/default
        let switch_regex = Regex::new(r"\bswitch\b")?;
        output = switch_regex.replace_all(&output, "switch").to_string();

        let case_regex = Regex::new(r"\bcase\b")?;
        output = case_regex.replace_all(&output, "case").to_string();

        let default_regex = Regex::new(r"\bdefault\b")?;
        output = default_regex.replace_all(&output, "done").to_string();

        // Convert boolean literals
        let true_regex = Regex::new(r"\btrue\b")?;
        output = true_regex.replace_all(&output, "yes").to_string();

        let false_regex = Regex::new(r"\bfalse\b")?;
        output = false_regex.replace_all(&output, "no").to_string();

        // Convert typeof
        let typeof_regex = Regex::new(r"\btypeof\b")?;
        output = typeof_regex.replace_all(&output, "what").to_string();

        // Convert instanceof
        let instanceof_regex = Regex::new(r"\binstanceof\b")?;
        output = instanceof_regex.replace_all(&output, "kind").to_string();

        // Convert in operator
        let in_regex = Regex::new(r"\bin\b")?;
        output = in_regex.replace_all(&output, "inside").to_string();

        // Convert comparison operators (skip these for now to avoid issues)
        // These could interfere with other patterns, so we'll keep them as JS for now

        // Clean up multiple spaces and fix formatting
        let multiple_spaces = Regex::new(r"  +")?;
        output = multiple_spaces.replace_all(&output, " ").to_string();

        Ok(output)
    }

    pub fn suggest_improvements(&self, js_content: &str) -> Vec<String> {
        let mut suggestions = Vec::new();

        // Check for TypeScript-specific patterns that can't be converted
        if js_content.contains("interface ") {
            suggestions.push("Remove TypeScript interfaces - NullScript doesn't support them".to_string());
        }

        if js_content.contains("enum ") {
            suggestions.push("Replace TypeScript enums with objects or constants".to_string());
        }

        if js_content.contains(": string") || js_content.contains(": number") {
            suggestions.push("Remove type annotations - NullScript infers types automatically".to_string());
        }

        if js_content.contains("<T>") || js_content.contains("extends T") {
            suggestions.push("Remove generic types - NullScript doesn't support generics".to_string());
        }

        // Check for complex patterns that might need manual conversion
        if js_content.contains("Promise.all") {
            suggestions.push("Consider using NullScript's promise handling patterns".to_string());
        }

        if js_content.contains("Array.from") {
            suggestions.push("Use list construction patterns instead of Array.from".to_string());
        }

        if js_content.contains("Object.assign") {
            suggestions.push("Use thing.assign for object merging".to_string());
        }

        // Performance suggestions
        if js_content.contains("console.log") && js_content.matches("console.log").count() > 10 {
            suggestions.push("Consider reducing console output in production - use speak.say sparingly".to_string());
        }

        if js_content.len() > 10000 {
            suggestions.push("Large file detected - consider splitting into smaller modules".to_string());
        }

        suggestions
    }

    pub fn analyze_conversion_quality(&self, original_js: &str, converted_ns: &str) -> ConversionReport {
        let js_lines = original_js.lines().count();
        let ns_lines = converted_ns.lines().count();

        let mut issues = Vec::new();
        let mut warnings = Vec::new();

        // Check for potential issues
        if converted_ns.contains("undefined") {
            issues.push("Contains 'undefined' - verify this is intentional in NullScript".to_string());
        }

        if converted_ns.contains("==") || converted_ns.contains("!=") {
            warnings.push("Uses loose equality operators - consider using 'is'/'isnt' for strict comparison".to_string());
        }

        if converted_ns.contains("var ") {
            warnings.push("Uses 'var' declarations - consider using 'let' or 'fixed' instead".to_string());
        }

        // Calculate conversion confidence
        let confidence = if issues.is_empty() && warnings.len() <= 2 {
            if warnings.is_empty() { 95.0 } else { 85.0 }
        } else if issues.len() <= 2 {
            70.0
        } else {
            50.0
        };

        ConversionReport {
            original_lines: js_lines,
            converted_lines: ns_lines,
            conversion_confidence: confidence,
            issues,
            warnings,
            suggestions: self.suggest_improvements(original_js),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ConversionReport {
    pub original_lines: usize,
    pub converted_lines: usize,
    pub conversion_confidence: f64,
    pub issues: Vec<String>,
    pub warnings: Vec<String>,
    pub suggestions: Vec<String>,
}

impl ConversionReport {
    pub fn print_report(&self) {
        println!("\n📊 Conversion Report");
        println!("==================");
        println!("📏 Lines: {} → {}", self.original_lines, self.converted_lines);
        println!("🎯 Confidence: {:.1}%", self.conversion_confidence);

        if !self.issues.is_empty() {
            println!("\n❌ Issues:");
            for issue in &self.issues {
                println!("  • {}", issue);
            }
        }

        if !self.warnings.is_empty() {
            println!("\n⚠️  Warnings:");
            for warning in &self.warnings {
                println!("  • {}", warning);
            }
        }

        if !self.suggestions.is_empty() {
            println!("\n💡 Suggestions:");
            for suggestion in &self.suggestions {
                println!("  • {}", suggestion);
            }
        }

        // Overall assessment
        if self.conversion_confidence >= 90.0 {
            println!("\n✅ Conversion looks excellent! Ready to use.");
        } else if self.conversion_confidence >= 75.0 {
            println!("\n🟡 Conversion looks good with minor adjustments needed.");
        } else {
            println!("\n🟠 Conversion needs review - manual adjustments recommended.");
        }
    }
}
