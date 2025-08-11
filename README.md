# NullScript 🎭

**TypeScript with Attitude** - A fun parody programming language that transpiles to TypeScript/JavaScript.

[![npm version](https://badge.fury.io/js/nullscript.svg)](https://badge.fury.io/js/nullscript)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## ✨ Features

- 🎪 **Fun keyword aliases** (`pls` instead of `return`, `maybe` instead of `let`, etc.)
- 🔧 **Full TypeScript compatibility** - all TS features work exactly the same
- 📁 **`.ns` file extension** for that special feeling
- ⚡ **Zero runtime overhead** - compiles to identical JavaScript as TypeScript
- 🛠️ **CLI tooling** with build, run, and type-check commands

## 🚀 Quick Start

### Installation

```bash
npm install -g nullscript
```

### Your First NullScript Program

Create `hello.ns`:

```typescript
feels greet(name: string): string {
  pls `Hello, ${name}! Welcome to NullScript! 🎭`;
}

definitely message = greet("Developer");
console.log(message);
```

### Run it!

```bash
nsc run hello.ns
```

## 📝 Language Reference

### File Extension
- Use `.ns` extension for NullScript files
- Example: `main.ns`, `utils.ns`, `index.ns`

### Keyword Aliases

| NullScript | TypeScript | Usage |
|------------|------------|-------|
| `pls` | `return` | `pls myValue;` |
| `maybe` | `let` | `maybe x = 5;` |
| `definitely` | `const` | `definitely name = "Alice";` |
| `gimme` | `import` | `gimme fs from 'fs';` |
| `yeet` | `export` | `yeet { myFunction };` |
| `feels` | `function` | `feels add(a, b) { ... }` |
| `bigbrain` | `class` | `bigbrain MyClass { ... }` |
| `vibes` | `interface` | `vibes User { name: string; }` |
| `checkthis` | `if` | `checkthis (condition) { ... }` |
| `orelse` | `else` | `orelse { ... }` |
| `orsomething` | `else if` | `orsomething (condition) { ... }` |
| `loopin` | `for` | `loopin (item of items) { ... }` |
| `whilevibe` | `while` | `whilevibe (condition) { ... }` |
| `fr` | `true` | `definitely isValid = fr;` |
| `cap` | `false` | `definitely isValid = cap;` |
| `nocap` | `null` | `maybe value = nocap;` |
| `ghost` | `undefined` | `maybe value = ghost;` |
| `sus` | `any` | `feels process(data: sus) { ... }` |
| `fresh` | `new` | `definitely obj = fresh MyClass();` |
| `dis` | `this` | `dis.property` |
| `and` | `&&` | `checkthis (a and b) { ... }` |
| `or` | `||` | `maybe result = a or b;` |
| `not` | `!` | `checkthis (not condition) { ... }` |

## 🛠️ CLI Usage

```bash
# Transpile to TypeScript (default)
nsc build src/ --ts --outDir dist

# Transpile to JavaScript
nsc build src/ --js --outDir dist

# Skip TypeScript type checking
nsc build src/ --skip-type-check

# Run NullScript directly
nsc run hello.ns

# Type checking
nsc check src/

# Show all keywords
nsc keywords

# Show keywords by category
nsc keywords --category control-flow
```

### Build Options
- `--ts` - Output TypeScript (default)
- `--js` - Output JavaScript
- `--outDir <dir>` - Output directory (default: `dist`)
- `--skip-type-check` - Skip TypeScript type checking

## 💻 Example

```typescript
gimme { readFileSync } from 'fs';

vibes Person {
  name: string;
  age: number;
}

bigbrain Greeter {
  feels greet(person: Person): string {
    checkthis (person.age >= 18) {
      pls `Hello, ${person.name}! You're an adult.`;
    } orelse {
      pls `Hello, ${person.name}! You're young.`;
    }
  }
}

definitely greeter = fresh Greeter();
maybe person: Person = { name: "Alice", age: 25 };
console.log(greeter.greet(person));
```

## 🤝 TypeScript Compatibility

NullScript maintains **100% compatibility** with TypeScript:
- ✅ All TypeScript types work identically
- ✅ Same compilation output
- ✅ Compatible with existing TypeScript tooling
- ✅ Same runtime behavior
- ✅ Works with TypeScript compiler options
- ✅ Can import/export with TypeScript projects

## 🔗 Links

- **GitHub**: https://github.com/nullscript-lang/nullscript
- **npm**: https://www.npmjs.com/package/nullscript
- **Documentation**: https://github.com/nullscript-lang/nullscript#readme

## 📄 License

MIT License

---

*"NullScript: Because programming should be fun, even when it's serious."* 🎭
