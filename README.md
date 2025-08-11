# NullScript 🎭

**TypeScript with Attitude** - A fun parody programming language that looks different but behaves identically to TypeScript.

NullScript is a transpiler that converts `.ns` files (with playful keyword aliases) into standard TypeScript/JavaScript while preserving **100% compatibility** with TypeScript's type system, tooling, and runtime behavior.

## ✨ Features

- 🎪 **Fun keyword aliases** (`pls` instead of `return`, `maybe` instead of `let`, etc.)
- 🔧 **Full TypeScript compatibility** - all TS features work exactly the same
- 📁 **`.ns` file extension** for that special feeling
- ⚡ **Zero runtime overhead** - compiles to identical JavaScript as TypeScript
- 🛠️ **CLI tooling** with build, run, and type-check commands
- 🎨 **VS Code syntax highlighting** (see setup below)

## 🚀 Installation

```bash
npm install -g nullscript
# or locally
npm install nullscript
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

### Transpile to TypeScript
```bash
# Single file
nullc build hello.ns --ts --outDir dist

# Directory
nullc build src/ --ts --outDir dist

# Direct to JavaScript
nullc build src/ --js --outDir dist
```

### Run NullScript directly
```bash
nullc run examples/hello-world.ns
```

### Type checking
```bash
nullc check src/
```

## 💻 Example Code

**hello-world.ns:**
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

**Transpiles to standard TypeScript:**
```typescript
import { readFileSync } from 'fs';

interface Person {
  name: string;
  age: number;
  isStudent?: boolean;
}

class Greeter {
  private message: string;

  constructor(greeting: string) {
    this.message = greeting;
  }

  function greet(person: Person): string {
    if (person.isStudent) {
      return `${this.message}, ${person.name}! Hope your studies are going well.`;
    } else {
      return `${this.message}, ${person.name}!`;
    }
  }
}

const greeter = new Greeter("Hey there");
let people: Person[] = [
  { name: "Alice", age: 25, isStudent: true },
  { name: "Bob", age: 30, isStudent: false }
];

for (const person of people) {
  console.log(greeter.greet(person));
}

export { Greeter };
```

## 🎨 VS Code Setup

1. Copy `.vscode/nullscript.tmLanguage.json` to your VS Code extensions folder
2. Add to your VS Code `settings.json`:
```json
{
  "files.associations": {
    "*.ns": "nullscript"
  }
}
```

## 🏗️ Building from Source

```bash
git clone https://github.com/your-org/nullscript
cd nullscript
npm install
npm run build
npm run test:examples
```

## 🧪 Running Examples

```bash
npm run test:examples  # Transpile examples
npm run test:run      # Run hello-world example
```

## 🤝 TypeScript Compatibility

NullScript maintains **100% compatibility** with TypeScript:

- ✅ All TypeScript types work identically
- ✅ Same type checking rules
- ✅ Same compilation output
- ✅ Compatible with existing TypeScript tooling
- ✅ Same runtime behavior
- ✅ Can import/export with TypeScript projects

## 📜 Philosophy

NullScript exists to prove that programming languages are just syntax sugar over ideas. While the keywords look different and playful, the underlying semantics remain identical to TypeScript. It's TypeScript wearing a funny hat! 🎩

## 🤔 Why NullScript?

- **Educational**: Understand how transpilers work
- **Fun**: Code with personality and humor
- **Compatibility**: Drop-in replacement for TypeScript
- **Experimental**: Play with language design without breaking things

## 📄 License

MIT License - Feel free to fork, modify, and have fun!

---

*"NullScript: Because programming should be fun, even when it's serious."* 🎭
