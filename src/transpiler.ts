import { exec } from "node:child_process";
import fs from "node:fs/promises";
import path from "node:path";
import { promisify } from "node:util";
import {
  getAllKeywords,
  getFunctionKeywords,
  getMultiWordKeywords,
} from "./keywords.js";

export class NullScriptTranspileError extends Error {
  constructor(
    message: string,
    public filePath?: string,
    public line?: number,
    public column?: number,
  ) {
    super(message);
    this.name = "NullScriptTranspileError";
  }
}

export class NullScriptSyntaxError extends NullScriptTranspileError {
  constructor(
    message: string,
    filePath?: string,
    line?: number,
    column?: number,
  ) {
    super(message, filePath, line, column);
    this.name = "NullScriptSyntaxError";
  }
}

export class NullScriptTypeError extends NullScriptTranspileError {
  constructor(
    message: string,
    filePath?: string,
    line?: number,
    column?: number,
  ) {
    super(message, filePath, line, column);
    this.name = "NullScriptTypeError";
  }
}

const execAsync = promisify(exec);

// Validation functions
function validateNullScriptSyntax(source: string, filePath?: string): void {
  const lines = source.split("\n");

  // Check for common NullScript syntax issues
  for (let i = 0; i < lines.length; i++) {
    const line = lines[i].trim();
    const lineNumber = i + 1;

    // Skip empty lines and comments
    if (!line || line.startsWith("//") || line.startsWith("/*")) continue;

    // Check for invalid keywords (not in our keyword list but look like they should be)
    const invalidKeywordPatterns = [
      /\b(function\s+\w+\s*\()/, // using 'function' instead of 'feels'
      /\b(const\s+\w+)/, // using 'const' instead of 'definitely'
      /\b(let\s+\w+)/, // using 'let' instead of 'maybe'
      /\b(var\s+\w+)/, // using 'var' instead of 'mayhap'
      /\b(if\s*\()/, // using 'if' instead of 'checkthis'
      /\b(else\s+)/, // using 'else' instead of 'orelse'
      /\b(return\s+)/, // using 'return' instead of 'pls'
      /\b(true)\b/, // using 'true' instead of 'fr'
      /\b(false)\b/, // using 'false' instead of 'cap'
      /\b(null)\b/, // using 'null' instead of 'nocap'
      /\b(undefined)\b/, // using 'undefined' instead of 'ghost'
      /\b(interface\s+\w+)/, // using 'interface' instead of 'vibes'
      /\b(type\s+\w+)/, // using 'type' instead of 'vibe'
      /\b(class\s+\w+)/, // using 'class' instead of 'bigbrain'
      /\b(try\s*\{)/, // using 'try' instead of 'oops' or 'oop'
      /\b(catch\s*\()/, // using 'catch' instead of 'mybad'
      /\b(finally\s*\{)/, // using 'finally' instead of 'anyway'
    ];

    for (const pattern of invalidKeywordPatterns) {
      if (pattern.test(line)) {
        throw new NullScriptSyntaxError(
          `Invalid syntax on line ${lineNumber}: You're using standard TypeScript/JavaScript syntax instead of NullScript keywords.\n💡 Run 'nullc keywords' to see the correct NullScript syntax.`,
          filePath,
          lineNumber,
        );
      }
    }

    // Check for completely invalid keywords
    const unknownKeywordPattern = /^(\w+)\s+\w+\s*=/;
    const match = line.match(unknownKeywordPattern);
    if (match) {
      const keyword = match[1];
      // Check if this keyword is not in our valid NullScript keywords
      const allKeywords = Object.values(getAllKeywords());
      const allKeys = Object.keys(getAllKeywords());

      if (
        !allKeys.includes(keyword) &&
        !["export", "import", "from", "as"].includes(keyword)
      ) {
        throw new NullScriptSyntaxError(
          `Unknown keyword '${keyword}' on line ${lineNumber}.\n💡 Use valid NullScript keywords. Run 'nullc keywords' to see all available options.`,
          filePath,
          lineNumber,
        );
      }
    }
  }
}

// TypeScript error patterns and their NullScript equivalents
const ERROR_MAPPINGS = {
  // Common syntax errors
  "Cannot find name 'feels'": {
    type: "syntax",
    message:
      "Invalid function declaration. Use 'feels' followed by a function name.",
    suggestion: "Example: feels myFunction() { ... }",
  },
  "Cannot find name 'definitely'": {
    type: "syntax",
    message: "Invalid variable declaration. Use 'definitely' for constants.",
    suggestion: "Example: definitely myVar = 'value'",
  },
  "Cannot find name 'maybe'": {
    type: "syntax",
    message:
      "Invalid variable declaration. Use 'maybe' for variables that can change.",
    suggestion: "Example: maybe myVar = 'value'",
  },
  "Cannot find name 'checkthis'": {
    type: "syntax",
    message:
      "Invalid conditional statement. Use 'checkthis' for if statements.",
    suggestion: "Example: checkthis (condition) { ... }",
  },
  "Cannot find name 'orelse'": {
    type: "syntax",
    message: "Invalid else statement. Use 'orelse' for else clauses.",
    suggestion: "Example: checkthis (condition) { ... } orelse { ... }",
  },
  "Cannot find name 'pls'": {
    type: "syntax",
    message: "Invalid return statement. Use 'pls' to return values.",
    suggestion: "Example: pls myValue",
  },
  "Cannot find name 'fr'": {
    type: "syntax",
    message: "Invalid boolean value. Use 'fr' for true.",
    suggestion: "Example: definitely isValid = fr",
  },
  "Cannot find name 'cap'": {
    type: "syntax",
    message: "Invalid boolean value. Use 'cap' for false.",
    suggestion: "Example: definitely isValid = cap",
  },
  "Cannot find name 'nocap'": {
    type: "syntax",
    message: "Invalid null value. Use 'nocap' for null.",
    suggestion: "Example: definitely value = nocap",
  },
  "Cannot find name 'ghost'": {
    type: "syntax",
    message: "Invalid undefined value. Use 'ghost' for undefined.",
    suggestion: "Example: definitely value = ghost",
  },
  "Cannot find name 'vibes'": {
    type: "syntax",
    message: "Invalid interface declaration. Use 'vibes' to define interfaces.",
    suggestion: "Example: vibes MyInterface { ... }",
  },
  "Cannot find name 'vibe'": {
    type: "syntax",
    message: "Invalid type alias. Use 'vibe' to define type aliases.",
    suggestion: "Example: vibe MyType = string | number",
  },
  "Cannot find name 'bigbrain'": {
    type: "syntax",
    message: "Invalid class declaration. Use 'bigbrain' to define classes.",
    suggestion: "Example: bigbrain MyClass { ... }",
  },
  "Unexpected token": {
    type: "syntax",
    message:
      "Syntax error in NullScript code. Check for missing keywords or incorrect syntax.",
    suggestion:
      "Make sure you're using NullScript keywords correctly. Run 'nullc keywords' to see all available keywords.",
  },
  "Declaration or statement expected": {
    type: "syntax",
    message: "Invalid statement. Check your NullScript syntax.",
    suggestion: "Make sure you're using proper NullScript keywords and syntax.",
  },
  "Function implementation is missing": {
    type: "syntax",
    message:
      "Function body is missing. Add implementation after your function declaration.",
    suggestion: "Example: feels myFunction() { /* your code here */ }",
  },
  "Unexpected keyword or identifier": {
    type: "syntax",
    message:
      "Invalid NullScript syntax. You're using an undefined keyword or incorrect syntax.",
    suggestion:
      "Check that you're using valid NullScript keywords. Run 'nullc keywords' to see all available options.",
  },
};

export function parseTypeScriptError(
  error: string,
  filePath?: string,
): NullScriptTranspileError {
  const lines = error.split("\n");
  let line: number | undefined;
  let column: number | undefined;
  let errorMessage = error;

  // Extract line and column information from TypeScript error
  // Handle format like "file.ts:12:1 - error TS1434:"
  const locationMatch = error.match(
    /(\w+\.ts):(\d+):(\d+)\s*-\s*error|:(\d+):(\d+)/,
  );
  if (locationMatch) {
    line = parseInt(locationMatch[2] || locationMatch[4]);
    column = parseInt(locationMatch[3] || locationMatch[5]);
  }

  // Find the core error message(s) - handle multiple TypeScript errors
  const errorLines = lines.filter((line) => line.includes("error TS"));
  if (errorLines.length > 0) {
    // Take the first meaningful error
    const firstError = errorLines[0];
    const errorMatch = firstError.match(/error TS\d+: (.+)/);
    if (errorMatch) {
      errorMessage = errorMatch[1];
    }
  }

  // If no TypeScript errors found, look for other common patterns
  if (!errorLines.length) {
    // Look for compilation failure patterns
    const compilationError = lines.find(
      (line) =>
        line.includes("Cannot find name") ||
        line.includes("Unexpected token") ||
        line.includes("Declaration or statement expected"),
    );
    if (compilationError) {
      errorMessage = compilationError.trim();
    }
  }

  // Check for known error patterns
  for (const [pattern, mapping] of Object.entries(ERROR_MAPPINGS)) {
    if (errorMessage.includes(pattern)) {
      const customMessage = `${mapping.message}\n💡 ${mapping.suggestion}`;

      if (mapping.type === "syntax") {
        return new NullScriptSyntaxError(customMessage, filePath, line, column);
      } else {
        return new NullScriptTypeError(customMessage, filePath, line, column);
      }
    }
  }

  // Fallback: clean up the TypeScript error message
  const cleanMessage = errorMessage
    .replace(/error TS\d+:\s*/, "")
    .replace(/\s+/g, " ")
    .replace(/^\s*at\s+.*$/gm, "") // Remove stack trace lines
    .replace(/Command failed:.*$/m, "") // Remove command failure messages
    .replace(/\(node:\d+\).*$/gm, "") // Remove node warnings
    .split("\n")
    .filter((line) => line.trim().length > 0)
    .slice(0, 3) // Take first 3 meaningful lines
    .join("\n")
    .trim();

  return new NullScriptTranspileError(
    `Transpilation error: ${cleanMessage}\n💡 This might be due to incorrect NullScript syntax. Run 'nullc keywords' to see available keywords.`,
    filePath,
    line,
    column,
  );
}

export function formatNullScriptError(error: NullScriptTranspileError): string {
  let output = `❌ ${error.name}`;

  if (error.filePath) {
    output += ` in ${path.basename(error.filePath)}`;
  }

  if (error.line !== undefined) {
    output += `:${error.line}`;
    if (error.column !== undefined) {
      output += `:${error.column}`;
    }
  }

  output += `\n\n${error.message}`;

  return output;
}

export type TranspileOptions = {
  outputFormat?: "ts" | "js";
  outDir?: string;
  preserveComments?: boolean;
  skipTypeCheck?: boolean;
};

export function transpileNullScript(
  source: string,
  options: TranspileOptions = {},
): string {
  let output = source;

  for (const [alias, tsKeyword] of Object.entries(getFunctionKeywords())) {
    if (alias.includes("async")) {
      output = output.replace(
        new RegExp(`\\b${alias}\\s+([a-zA-Z_$][\\w$]*)`, "g"),
        `${tsKeyword} $1`,
      );
    } else {
      output = output.replace(
        new RegExp(
          `\\b${alias}\\s+([a-zA-Z_$][\\w$]*)\\s*(?:<[^>]*>)?\\s*\\(`,
          "g",
        ),
        (match, functionName) => {
          const lines = output.split("\n");
          const currentLineIndex =
            output.substring(0, output.indexOf(match)).split("\n").length - 1;
          const currentLine = lines[currentLineIndex];
          const indent = currentLine.match(/^(\s*)/)?.[1] || "";

          const restOfMatch = match.substring(
            alias.length + 1 + functionName.length,
          );

          if (indent.length > 0) {
            return `${functionName}${restOfMatch}`;
          } else {
            return `${tsKeyword} ${functionName}${restOfMatch}`;
          }
        },
      );

      output = output.replace(
        new RegExp(`\\b${alias}\\s*\\(`, "g"),
        `${tsKeyword}(`,
      );
    }
  }

  for (const [alias, tsKeyword] of Object.entries(getAllKeywords())) {
    if (alias === "feels" || alias === "feels async") continue;

    if (alias === "remove") {
      output = output.replace(
        new RegExp(
          `\\bremove\\s+([a-zA-Z_$][\\w$]*(?:\\.[a-zA-Z_$][\\w$]*)*(?:\\[[^\\]]+\\])?)\\b`,
          "g",
        ),
        `delete $1`,
      );
    } else {
      const regex = new RegExp(`\\b${alias}\\b`, "g");
      output = output.replace(regex, tsKeyword);
    }
  }

  for (const [alias, tsKeyword] of Object.entries(getMultiWordKeywords())) {
    output = output.replace(
      new RegExp(`\\b${alias}\\s+`, "g"),
      `${tsKeyword} `,
    );
  }

  return output;
}

export async function transpileFile(
  inputPath: string,
  outputPath: string,
  options: TranspileOptions = {},
): Promise<string> {
  const source = await fs.readFile(inputPath, "utf8");

  // Validate NullScript syntax before transpiling
  validateNullScriptSyntax(source, inputPath);

  const transpiled = transpileNullScript(source, options);

  await fs.mkdir(path.dirname(outputPath), { recursive: true });
  await fs.writeFile(outputPath, transpiled, "utf8");

  return transpiled;
}

export async function transpileToJs(
  nsPath: string,
  jsPath: string,
  options: TranspileOptions = {},
): Promise<void> {
  const tsPath = nsPath.replace(/\.ns$/, ".ts");

  try {
    await transpileFile(nsPath, tsPath, { ...options, outputFormat: "ts" });
  } catch (error) {
    // If transpilation to .ts fails, make sure no .ts file is left behind
    await fs.unlink(tsPath).catch(() => {}); // ignore errors if file doesn't exist
    throw error;
  }

  const tempDir = path.dirname(tsPath);
  const tsConfigPath = path.join(tempDir, "tsconfig.json");
  const tsConfig = {
    compilerOptions: {
      target: "ES2022",
      module: "ES2022",
      moduleResolution: "node",
      outDir: path.dirname(jsPath),
      rootDir: tempDir,
      esModuleInterop: true,
      allowSyntheticDefaultImports: true,
      skipLibCheck: true,
      noEmit: false,
    },
    include: [path.basename(tsPath)],
  };

  let configCreated = false;

  try {
    await fs.writeFile(tsConfigPath, JSON.stringify(tsConfig, null, 2));
    configCreated = true;

    const tscCommand = options.skipTypeCheck
      ? `npx tsc --noCheck --project "${tsConfigPath}"`
      : `npx tsc --project "${tsConfigPath}"`;

    await execAsync(tscCommand, { encoding: "utf8" });

    if (options.outputFormat === "js") {
      await fs.unlink(tsPath).catch(() => {});
    }
  } catch (error) {
    // Extract the actual error output from the exec error
    let errorOutput = "";

    if (error && typeof error === "object") {
      // TypeScript errors are typically in stdout, not stderr
      // Check stdout first for TypeScript error patterns, then stderr
      const stdout = String((error as any).stdout || "");
      const stderr = String((error as any).stderr || "");

      if (
        stdout &&
        (stdout.includes("error TS") ||
          (stdout.includes("Found") && stdout.includes("error")))
      ) {
        errorOutput = stdout;
      } else if (stderr && stderr.includes("error")) {
        errorOutput = stderr;
      } else if ("message" in error && error.message) {
        errorOutput = String(error.message);
      } else {
        errorOutput = stdout || stderr || String(error);
      }
    } else {
      errorOutput = String(error);
    }

    const customError = parseTypeScriptError(errorOutput, nsPath);
    throw customError;
  } finally {
    if (configCreated) {
      await fs.unlink(tsConfigPath).catch(() => {});
    }
  }
}

export async function buildDirectory(
  inputDir: string,
  outputDir: string,
  options: TranspileOptions = {},
): Promise<string[]> {
  const { glob } = await import("glob");
  const nsFiles = await glob("**/*.ns", { cwd: inputDir, absolute: true });

  const outputs: string[] = [];

  for (const nsFile of nsFiles) {
    const relativePath = path.relative(inputDir, nsFile);
    const outputExt = options.outputFormat === "js" ? ".js" : ".ts";
    const outputPath = path.join(
      outputDir,
      relativePath.replace(/\.ns$/, outputExt),
    );

    if (options.outputFormat === "js") {
      await transpileToJs(nsFile, outputPath, options);
    } else {
      await transpileFile(nsFile, outputPath, options);
    }

    outputs.push(outputPath);
  }

  return outputs;
}
