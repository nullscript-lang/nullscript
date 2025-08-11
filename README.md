# NullScript 🎭

**TypeScript with Attitude** - A fun parody programming language that looks different but behaves identically to TypeScript.

NullScript is a transpiler that converts `.ns` files (with playful keyword aliases) into standard TypeScript/JavaScript while preserving **100% compatibility** with TypeScript's type system, tooling, and runtime behavior.

[![npm version](https://badge.fury.io/js/nullscript.svg)](https://badge.fury.io/js/nullscript)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Node.js Version](https://img.shields.io/badge/node-%3E%3D18.0.0-brightgreen.svg)](https://nodejs.org/)

## ✨ Features

- 🎪 **Fun keyword aliases** (`pls` instead of `return`, `maybe` instead of `let`, etc.)
- 🔧 **Full TypeScript compatibility** - all TS features work exactly the same
- 📁 **`.ns` file extension** for that special feeling
- ⚡ **Zero runtime overhead** - compiles to identical JavaScript as TypeScript
- 🛠️ **CLI tooling** with build, run, and type-check commands
- 🎨 **VS Code syntax highlighting** support
- 📦 **Ready for production** - stable, tested, and well-documented

## 🚀 Quick Start

### Installation

```bash
# Global installation (recommended)
npm install -g nullscript

# Local installation
npm install nullscript
```

### Your First NullScript Program

Create a file called `hello.ns`:

```typescript
feels greet(name: string): string {
  pls `Hello, ${name}! Welcome to NullScript! 🎭`;
}

definitely message = greet("Developer");
console.log(message);
```

### Run it!

```bash
# Transpile and run
nullc run hello.ns

# Or build to TypeScript first
nullc build hello.ns --ts --outDir dist
```

## 📝 Language Reference

### Keyword Aliases

| NullScript | TypeScript | Usage |
|------------|------------|-------|
| `pls` | `return` | `pls myValue;` |
| `maybe` | `let` | `maybe x = 5;` |
| `definitely` | `const` | `definitely name = "Alice";` |
| `mayhap` | `var` | `mayhap oldStyle = true;` |
| `gimme` | `import` | `gimme fs from 'fs';` |
| `yeet` | `export` | `yeet { myFunction };` |
| `feels` | `function` | `feels add(a, b) { ... }` |
| `bigbrain` | `class` | `bigbrain MyClass { ... }` |
| `vibes` | `interface` | `vibes User { name: string; }` |
| `vibe` | `type` | `vibe ID = string \| number;` |
| `mood` | `enum` | `mood Colors { Red, Blue }` |
| `checkthis` | `if` | `checkthis (condition) { ... }` |
| `orelse` | `else` | `orelse { ... }` |
| `orsomething` | `else if` | `orsomething (other) { ... }` |
| `loopin` | `for` | `loopin (item of items) { ... }` |
| `whilevibe` | `while` | `whilevibe (condition) { ... }` |
| `switchup` | `switch` | `switchup (value) { ... }` |
| `whenits` | `case` | `whenits 'value': ...` |
| `otherwise` | `default` | `otherwise: ...` |
| `keepgoing` | `continue` | `keepgoing;` |
| `stopit` | `break` | `stopit;` |
| `oop` | `try` | `oop { ... }` |
| `mybad` | `catch` | `mybad (error) { ... }` |
| `anyway` | `finally` | `anyway { ... }` |
| `fr` | `true` | `definitely isValid = fr;` |
| `cap` | `false` | `definitely isValid = cap;` |
| `nocap` | `null` | `maybe value = nocap;` |
| `ghost` | `undefined` | `maybe value = ghost;` |
| `sus` | `any` | `feels process(data: sus) { ... }` |

## 🛠️ CLI Usage

### Commands

```bash
# Transpile to TypeScript
nullc build <input> [options]

# Run NullScript directly
nullc run <file>

# Type checking
nullc check <input>
```

### Options

```bash
# Build options
nullc build src/ --ts --outDir dist     # Output TypeScript
nullc build src/ --js --outDir dist     # Output JavaScript
nullc build src/ --watch                # Watch mode
nullc build src/ --verbose              # Verbose output

# Run options
nullc run hello.ns --args "arg1 arg2"   # Pass arguments
```

### Examples

```bash
# Single file
nullc build hello.ns --ts --outDir dist

# Directory
nullc build src/ --ts --outDir dist

# Direct to JavaScript
nullc build src/ --js --outDir dist

# Run with arguments
nullc run examples/hello-world.ns --args "Alice Bob"
```

## 💻 Example Code

### Simple Example (`examples/simple.ns`)

```typescript
feels add(a: number, b: number): number {
  pls a + b;
}

definitely result = add(5, 3);
console.log(`5 + 3 = ${result}`);
```

### Advanced Example (`examples/advanced-features.ns`)

```typescript
gimme { readFileSync } from 'fs';

vibes Person {
  name: string;
  age: number;
  isStudent?: boolean;
}

bigbrain Greeter {
  private message: string;

  constructor(greeting: string) {
    this.message = greeting;
  }

  feels greet(person: Person): string {
    checkthis (person.isStudent) {
      pls `${this.message}, ${person.name}! Hope your studies are going well.`;
    } orelse {
      pls `${this.message}, ${person.name}!`;
    }
  }
}

definitely greeter = new Greeter("Hey there");
maybe people: Person[] = [
  { name: "Alice", age: 25, isStudent: fr },
  { name: "Bob", age: 30, isStudent: cap }
];

loopin (definitely person of people) {
  console.log(greeter.greet(person));
}

yeet { Greeter };
```

## 🎨 VS Code Setup

1. Install the NullScript extension (if available) or create a custom syntax highlighting file
2. Add to your VS Code `settings.json`:
```json
{
  "files.associations": {
    "*.ns": "typescript"
  }
}
```

## 🏗️ Development

### Building from Source

```bash
git clone https://github.com/your-org/nullscript
cd nullscript
npm install
npm run build
```

### Running Tests

```bash
npm run test:examples  # Transpile examples
npm run test:run      # Run hello-world example
```

### Development Scripts

```bash
npm run build         # Build the project
npm run dev           # Watch mode for development
npm run clean         # Clean build artifacts
npm run test:examples # Test example transpilation
```

## 🤝 TypeScript Compatibility

NullScript maintains **100% compatibility** with TypeScript:

- ✅ All TypeScript types work identically
- ✅ Same type checking rules
- ✅ Same compilation output
- ✅ Compatible with existing TypeScript tooling
- ✅ Same runtime behavior
- ✅ Can import/export with TypeScript projects
- ✅ Works with TypeScript compiler options
- ✅ Supports all TypeScript language features

## 📦 Production Ready

NullScript is designed for production use:

- **Stable API**: Consistent behavior across versions
- **Performance**: Zero runtime overhead
- **Compatibility**: Full TypeScript ecosystem support
- **Documentation**: Comprehensive guides and examples
- **Testing**: Thoroughly tested with real-world examples
- **Maintenance**: Active development and bug fixes

## 🤔 Why NullScript?

- **Educational**: Understand how transpilers work
- **Fun**: Code with personality and humor
- **Compatibility**: Drop-in replacement for TypeScript
- **Experimental**: Play with language design without breaking things
- **Production**: Actually usable in real projects

## 📄 License

MIT License - Feel free to fork, modify, and have fun!

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

## 📚 Documentation

- [Language Reference](docs/language-reference.md)
- [CLI Reference](docs/cli-reference.md)
- [Examples](examples/)
- [Migration Guide](docs/migration.md)

---

*"NullScript: Because programming should be fun, even when it's serious."* 🎭

**Made with ❤️ by the NullScript Language Team**
