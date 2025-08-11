import { exec } from "node:child_process";
import fs from "node:fs/promises";
import path from "node:path";
import { promisify } from "node:util";
import {
  getAllKeywords,
  getFunctionKeywords,
  getMultiWordKeywords,
} from "./keywords";

const execAsync = promisify(exec);

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

  // Handle function declarations specially
  for (const [alias, tsKeyword] of Object.entries(getFunctionKeywords())) {
    if (alias.includes("async")) {
      output = output.replace(
        new RegExp(`\\b${alias}\\s+([a-zA-Z_$][\\w$]*)`, "g"),
        `${tsKeyword} $1`,
      );
    } else {
      output = output.replace(
        new RegExp(`\\b${alias}\\s+([a-zA-Z_$][\\w$]*)`, "g"),
        `${tsKeyword} $1`,
      );
    }
  }

  // Handle other keywords with better context awareness
  for (const [alias, tsKeyword] of Object.entries(getAllKeywords())) {
    // Skip function-related aliases as they're handled above
    if (alias === "feels" || alias === "feels async") continue;

    // Use word boundaries but be more careful about context
    const regex = new RegExp(`\\b${alias}\\b`, "g");
    output = output.replace(regex, tsKeyword);
  }

  // Handle special multi-word aliases
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
  // First transpile .ns to .ts
  const tsPath = nsPath.replace(/\.ns$/, ".ts");
  await transpileFile(nsPath, tsPath, { ...options, outputFormat: "ts" });

  try {
    // Create a minimal tsconfig for compilation
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

    await fs.writeFile(tsConfigPath, JSON.stringify(tsConfig, null, 2));

    // Use TypeScript compiler to compile .ts to .js
    const tscCommand = options.skipTypeCheck
      ? `npx tsc --noCheck --project "${tsConfigPath}"`
      : `npx tsc --project "${tsConfigPath}"`;

    await execAsync(tscCommand);

    // Clean up intermediate files if we only want .js
    if (options.outputFormat === "js") {
      await fs.unlink(tsPath).catch(() => {}); // ignore errors
      await fs.unlink(tsConfigPath).catch(() => {}); // ignore errors
    }
  } catch (error) {
    throw new Error(`TypeScript compilation failed: ${error}`);
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
