# NullScript 🎭

**JavaScript with Attitude** - A fun parody programming language that transpiles to JavaScript.

## ✨ Features

- 🎪 **Fun keyword aliases** (`speak` instead of `console`, `run` instead of `function`, etc.)
- 🔧 **Pure JavaScript output** - no type annotations, just clean JS
- 📁 **`.ns` file extension** for that special feeling
- ⚡ **Zero runtime overhead** - compiles directly to JavaScript
- 🛠️ **CLI tooling** with build and run commands

## 🚀 Quick Start

### Installation

```bash
npm install -g nullscript
```

### Your First NullScript Program

Create `hello.ns`:

```javascript
run greet(name) {
  return `Hello, ${name}! Welcome to NullScript! 🎭`;
}

fixed message = greet("Developer");
speak.say(message);
```

### Run it!

```bash
nsc run hello.ns
```

## 📚 Documentation

**📖 [Full Documentation Available Here](https://nullscript.js.org)**

The documentation site includes:
- Complete language reference
- All keyword aliases and their JavaScript equivalents
- Advanced examples and tutorials
- CLI usage guide
- Best practices and tips

## 🛠️ Basic CLI Usage

```bash
# Transpile to JavaScript
nsc build src/ --outDir dist

# Run NullScript directly
nsc run hello.ns

# Show all keywords
nsc keywords
```

## 💻 Quick Example

```javascript
use { readFileSync } from 'fs';

model Greeter {
  run greet(person) {
    whatever (person.age moreeq 18) {
      return `Hello, ${person.name}! You're an adult.`;
    } otherwise {
      return `Hello, ${person.name}! You're young.`;
    }
  }
}

fixed greeter = fresh Greeter();
let person = { name: "Alice", age: 25 };
speak.say(greeter.greet(person));
```

---

**📖 [Visit the full documentation](https://nullscript.js.org) for complete language reference, examples, and tutorials.**

*"NullScript: Because programming should be fun, even when it's serious."* 🎭
