export const nullScriptKeywords = {
  "control-flow": {
    title: "Control Flow",
    keywords: {
      checkthis: "if", // Check a condition and run code if true
      orelse: "else", // If not true, do this instead
      loopin: "for", // Loop a fixed number of times or over items
      whilevibe: "while", // Keep looping while condition is true
      switchup: "switch", // Choose what to run based on a value
      whenits: "case", // Run if this specific value matches
      otherwise: "default", // Fallback if no case matches
      keepgoing: "continue", // Skip to the next loop iteration
      stopit: "break", // Exit loop or switch immediately
    },
  },
  "error-handling": {
    title: "Error Handling",
    keywords: {
      oops: "try", // Attempt code that might throw an error
      mybad: "catch", // Handle an error if it happens
      anyway: "finally", // Always run after try/catch
    },
  },
  variables: {
    title: "Variable Declarations",
    keywords: {
      maybe: "let", // Variable that can change
      definitely: "const", // Value that cannot change
      mayhap: "var", // Old variable declaration style
    },
  },
  imports: {
    title: "Import/Export",
    keywords: {
      gimme: "import", // Get code from another file/module
      yeet: "export", // Send code so other files can use it
    },
  },
  types: {
    title: "Type Declarations",
    keywords: {
      vibes: "interface", // TypeScript object shape
      vibe: "type", // Type alias
      mood: "enum", // Named set of constant values
      bigbrain: "class", // Blueprint for creating objects
    },
  },
  values: {
    title: "Values",
    keywords: {
      fr: "true", // Yes/true
      cap: "false", // No/false
      nocap: "null", // Intentional no value
      ghost: "undefined", // No value assigned
      sus: "any", // Any type allowed
    },
  },
  objects: {
    title: "Object and Context",
    keywords: {
      dis: "this", // Current object or instance
      parent: "super", // Call or reference parent class
      fresh: "new", // Create a new instance
      remove: "delete", // Remove a property from an object
    },
  },
  operators: {
    title: "Operators and Expressions",
    keywords: {
      and: "&&", // Both conditions must be true
      or: "||", // Either condition can be true
      not: "!", // Reverse a boolean value
      is: "===", // Strict comparison (value and type equal)
      aint: "!==", // Strict inequality
      bigger: ">", // Greater than
      smaller: "<", // Less than
      biggereq: ">=", // Greater than or equal to
      smallereq: "<=", // Less than or equal to
    },
  },
  functions: {
    title: "Functions",
    keywords: {
      pls: "return", // Give back a value from a function
    },
  },
  keywords: {
    title: "Other Keywords",
    keywords: {
      with: "with", // Use an object as scope
      in: "in", // Check if property exists in object
      of: "of", // Loop over iterable values
      as: "as", // Rename import or specify type
      from: "from", // Import source module
    },
  },
  "multi-word": {
    title: "Multi-word Aliases",
    keywords: {
      orsomething: "else if", // Another condition if first fails
    },
  },
  "function-declarations": {
    title: "Function Declarations",
    keywords: {
      "feels async": "async function", // Async function
      feels: "function", // Regular function
    },
  },
} as const;

// Helper functions to extract data for the transpiler
export const getAllKeywords = () => {
  const allKeywords: Record<string, string> = {};
  Object.values(nullScriptKeywords).forEach((category) => {
    Object.assign(allKeywords, category.keywords);
  });
  return allKeywords;
};

export const getMultiWordKeywords = () => {
  return nullScriptKeywords["multi-word"].keywords;
};

export const getFunctionKeywords = () => {
  return nullScriptKeywords["function-declarations"].keywords;
};

// Legacy exports for backward compatibility
export const multiWordKeywords = getMultiWordKeywords();
export const functionKeywords = getFunctionKeywords();

// Type definitions
export type KeywordCategory = keyof typeof nullScriptKeywords;
export type KeywordMap = ReturnType<typeof getAllKeywords>;
export type MultiWordKeywordMap = ReturnType<typeof getMultiWordKeywords>;
export type FunctionKeywordMap = ReturnType<typeof getFunctionKeywords>;
