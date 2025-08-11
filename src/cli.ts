#!/usr/bin/env node
import chalk from "chalk";
import { Command } from "commander";
import path from "node:path";
import { nullScriptKeywords } from "./keywords.js";
import {
  buildDirectory,
  transpileFile,
  transpileToJs,
  type TranspileOptions,
} from "./transpiler.js";

const program = new Command();

program
  .name("nullc")
  .description("NullScript transpiler - TypeScript with attitude")
  .version("1.0.0")
  .helpOption("-h, --help", "Show help for command")
  .addHelpText(
    "after",
    `
Examples:
  $ nullc build src/                    # Transpile all .ns files in src/ to TypeScript
  $ nullc build src/ --js               # Transpile to JavaScript
  $ nullc run hello.ns                  # Run a NullScript file
  $ nullc check src/                    # Type-check NullScript files
  $ nullc keywords                       # Show all available keywords

Learn more at: https://github.com/kiron0/nullscript
    `,
  );

program.on("command:*", (cmd) => {
  console.error(chalk.red(`Unknown command: ${cmd}`));
  console.error("See --help for available commands.");
  process.exitCode = 1;
});

program
  .command("build")
  .argument("<path>", "Directory or file to transpile (.ns)")
  .option("-o, --outDir <dir>", "Output directory", "dist")
  .option("--js", "Compile directly to JavaScript", false)
  .option("--ts", "Transpile to TypeScript (default)", false)
  .option("--skip-type-check", "Skip TypeScript type checking", false)
  .action(async (inputPath: string, options: any) => {
    try {
      const transpileOptions: TranspileOptions = {
        outputFormat: options.js ? "js" : "ts",
        outDir: options.outDir,
        skipTypeCheck: options.skipTypeCheck,
      };

      const stats = await import("node:fs/promises").then((fs) =>
        fs.stat(inputPath),
      );

      if (stats.isDirectory()) {
        const outputs = await buildDirectory(
          inputPath,
          options.outDir,
          transpileOptions,
        );
        console.log(
          chalk.green(
            `✅ Transpiled ${outputs.length} file(s) to ${options.outDir}`,
          ),
        );
        outputs.forEach((file) => console.log(chalk.gray(`   → ${file}`)));
      } else {
        const outputExt = options.js ? ".js" : ".ts";
        const outputPath = path.join(
          options.outDir,
          path.basename(inputPath).replace(/\.ns$/, outputExt),
        );

        if (options.js) {
          await transpileToJs(inputPath, outputPath, transpileOptions);
        } else {
          await transpileFile(inputPath, outputPath, transpileOptions);
        }

        console.log(chalk.green(`✅ Transpiled ${inputPath} → ${outputPath}`));
      }
    } catch (err) {
      console.error(chalk.red("❌ Transpilation failed:"), err);
      process.exitCode = 1;
    }
  });

program
  .command("run")
  .argument("<file>", "NullScript file to run (.ns)")
  .option("--skip-type-check", "Skip TypeScript type checking", false)
  .action(async (file: string, options: any) => {
    try {
      console.log(chalk.cyan("🚀 Running NullScript..."));

      const tempJs = file.replace(/\.ns$/, ".temp.mjs");
      await transpileToJs(file, tempJs, {
        outputFormat: "js",
        skipTypeCheck: options.skipTypeCheck,
      });

      // Import and run the compiled JavaScript
      const { pathToFileURL } = await import("node:url");
      await import(pathToFileURL(path.resolve(tempJs)).href);

      // Clean up temp file
      await import("node:fs/promises").then((fs) =>
        fs.unlink(tempJs).catch(() => {}),
      );
    } catch (err) {
      console.error(chalk.red("❌ Runtime error:"), err);
      process.exitCode = 1;
    }
  });

program
  .command("check")
  .argument("<path>", "File or directory to type-check")
  .description("Type-check NullScript files using TypeScript")
  .action(async (inputPath: string) => {
    try {
      console.log(chalk.cyan("🔍 Type-checking NullScript..."));

      // Transpile to .ts and run tsc --noEmit
      const { exec } = await import("node:child_process");
      const { promisify } = await import("node:util");
      const execAsync = promisify(exec);

      const tempDir = ".nullscript-check";
      await buildDirectory(inputPath, tempDir, { outputFormat: "ts" });

      try {
        await execAsync(`npx tsc --noEmit --project ${tempDir}`);
        console.log(chalk.green("✅ Type checking passed!"));
      } catch (error) {
        console.error(chalk.red("❌ Type checking failed:"));
        console.error(error);
        process.exitCode = 1;
      }

      // Clean up
      await import("node:fs/promises").then((fs) =>
        fs.rm(tempDir, { recursive: true, force: true }).catch(() => {}),
      );
    } catch (err) {
      console.error(chalk.red("❌ Type-check failed:"), err);
      process.exitCode = 1;
    }
  });

program
  .command("keywords")
  .description("Show all available NullScript keywords")
  .option("-c, --category <category>", "Show aliases for specific category", "")
  .action((options: any) => {
    // Use the structured aliases directly
    const categories = nullScriptKeywords;

    if (options.category) {
      const category = categories[options.category as keyof typeof categories];
      if (category) {
        console.log(chalk.cyan(`\n📋 ${category.title} Keywords:`));
        console.log(chalk.gray("─".repeat(50)));
        Object.entries(category.keywords).forEach(([alias, keyword]) => {
          console.log(
            chalk.yellow(`  ${alias.padEnd(15)}`) + chalk.white(`→ ${keyword}`),
          );
        });
      } else {
        console.error(chalk.red(`❌ Unknown category: ${options.category}`));
        console.log(chalk.gray("Available categories:"));
        Object.keys(categories).forEach((cat) => {
          console.log(chalk.gray(`  - ${cat}`));
        });
        process.exitCode = 1;
      }
    } else {
      console.log(chalk.cyan("\n🎭 NullScript Keywords"));
      console.log(chalk.gray("=".repeat(50)));

      Object.entries(categories).forEach(([key, category]) => {
        console.log(chalk.cyan(`\n📋 ${category.title}:`));
        console.log(chalk.gray("─".repeat(30)));
        Object.entries(category.keywords).forEach(([alias, keyword]) => {
          console.log(
            chalk.yellow(`  ${alias.padEnd(15)}`) + chalk.white(`→ ${keyword}`),
          );
        });
      });

      console.log(
        chalk.gray(
          "\n💡 Tip: Use 'nullc keywords --category <name>' to see specific categories",
        ),
      );
      console.log(
        chalk.gray(
          "   Available categories: " + Object.keys(categories).join(", "),
        ),
      );
    }
  });

program.parseAsync().catch((err) => {
  console.error(chalk.red("❌ CLI error:"), err);
  process.exitCode = 1;
});
