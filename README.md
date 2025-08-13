# NullScript 🎭

**JavaScript with Attitude** - A fun parody programming language that transpiles to JavaScript.

[![npm version](https://badge.fury.io/js/nullscript.svg)](https://badge.fury.io/js/nullscript)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

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

## 📝 Language Reference

### File Extension
- Use `.ns` extension for NullScript files
- Example: `main.ns`, `utils.ns`, `index.ns`

### Keyword Aliases

| NullScript | JavaScript | Usage |
|------------|------------|-------|
| `return` | `return` | `return myValue;` |
| `run` | `function` | `run add(a, b) { ... }` |
| `fixed` | `const` | `fixed name = "Alice";` |
| `let` | `let` | `let x = 5;` |
| `var` | `var` | `var legacy = "old";` |
| `use` | `import` | `use fs from 'fs';` |
| `share` | `export` | `share { myFunction };` |
| `whatever` | `if` | `whatever (condition) { ... }` |
| `otherwise` | `else` | `otherwise { ... }` |
| `since` | `for` | `since (item of items) { ... }` |
| `when` | `while` | `when (condition) { ... }` |
| `switch` | `switch` | `switch (value) { ... }` |
| `case` | `case` | `case "value":` |
| `done` | `default` | `done:` |
| `stop` | `break` | `stop;` |
| `keepgoing` | `continue` | `keepgoing;` |
| `test` | `try` | `test { ... }` |
| `grab` | `catch` | `grab (error) { ... }` |
| `atLast` | `finally` | `atLast { ... }` |
| `trigger` | `throw` | `trigger new Error();` |
| `fresh` | `new` | `fresh MyClass();` |
| `self` | `this` | `self.property` |
| `parent` | `super` | `parent.method();` |
| `model` | `class` | `model MyClass { ... }` |
| `__init__` | `constructor` | `__init__() { ... }` |
| `inherits` | `extends` | `model Child inherits Parent` |
| `forever` | `static` | `run forever method() { ... }` |
| `later` | `async` | `run later fetch() { ... }` |
| `hold` | `await` | `let data = hold fetch();` |
| `yes` | `true` | `fixed isValid = yes;` |
| `no` | `false` | `fixed isValid = no;` |
| `null` | `null` | `let value = null;` |
| `undefined` | `undefined` | `let value = undefined;` |
| `is` | `===` | `whatever (a is b) { ... }` |
| `isnt` | `!==` | `whatever (a isnt b) { ... }` |
| `more` | `>` | `whatever (a more b) { ... }` |
| `less` | `<` | `whatever (a less b) { ... }` |
| `moreeq` | `>=` | `whatever (a moreeq b) { ... }` |
| `lesseq` | `<=` | `whatever (a lesseq b) { ... }` |
| `and` | `&&` | `whatever (a and b) { ... }` |
| `or` | `||` | `let result = a or b;` |
| `not` | `!` | `whatever (not condition) { ... }` |
| `remove` | `delete` | `remove obj.property;` |
| `what` | `typeof` | `what value` |
| `kind` | `instanceof` | `whatever (obj kind Array) { ... }` |
| `inside` | `in` | `whatever ("key" inside obj) { ... }` |
| `part` | `of` | `since (item part items) { ... }` |
| `nothing` | `void` | `nothing 0` |
| `using` | `with` | `using (obj) { ... }` |
| `freeze` | `debugger` | `freeze;` |
| `pause` | `yield` | `pause value;` |
| `getter` | `get` | `getter property() { ... }` |
| `setter` | `set` | `setter property(value) { ... }` |

### Console API
| NullScript | JavaScript | Usage |
|------------|------------|-------|
| `speak` | `console` | `speak.say("Hello");` |
| `say` | `log` | `speak.say("message");` |
| `yell` | `warn` | `speak.yell("warning");` |
| `scream` | `error` | `speak.scream("error");` |
| `whisper` | `info` | `speak.whisper("info");` |
| `peek` | `debug` | `speak.peek("debug");` |
| `check` | `assert` | `speak.check(condition);` |
| `wipe` | `clear` | `speak.wipe();` |
| `tally` | `count` | `speak.tally("label");` |
| `resetcount` | `countReset` | `speak.resetcount("label");` |
| `dir` | `dir` | `speak.dir(obj);` |
| `deepdir` | `dirxml` | `speak.deepdir(obj);` |
| `group` | `group` | `speak.group("label");` |
| `fold` | `groupCollapsed` | `speak.fold("label");` |
| `ungroup` | `groupEnd` | `speak.ungroup();` |
| `show` | `table` | `speak.show(data);` |
| `time` | `time` | `speak.time("label");` |
| `stoptimer` | `timeEnd` | `speak.stoptimer("label");` |
| `logtimer` | `timeLog` | `speak.logtimer("label");` |
| `backtrace` | `trace` | `speak.backtrace();` |

### Built-in Objects
| NullScript | JavaScript | Usage |
|------------|------------|-------|
| `thing` | `Object` | `fresh thing()` |
| `list` | `Array` | `fresh list()` |
| `text` | `String` | `fresh text()` |
| `num` | `Number` | `fresh num()` |
| `bool` | `Boolean` | `fresh bool()` |
| `clock` | `Date` | `fresh clock()` |
| `maths` | `Math` | `maths.random()` |
| `json` | `JSON` | `json.parse()` |
| `pattern` | `RegExp` | `fresh pattern()` |
| `fail` | `Error` | `fresh fail()` |
| `promise` | `Promise` | `fresh promise()` |
| `dict` | `Map` | `fresh dict()` |
| `unique` | `Set` | `fresh unique()` |
| `weakdict` | `WeakMap` | `fresh weakdict()` |
| `weakunique` | `WeakSet` | `fresh weakunique()` |
| `symbol` | `Symbol` | `fresh symbol()` |
| `proxy` | `Proxy` | `fresh proxy()` |
| `reflect` | `Reflect` | `reflect.get()` |
| `intl` | `Intl` | `intl.DateTimeFormat()` |
| `wasm` | `WebAssembly` | `wasm.instantiate()` |

### Global Functions
| NullScript | JavaScript | Usage |
|------------|------------|-------|
| `toint` | `parseInt` | `toint("123")` |
| `tofloat` | `parseFloat` | `tofloat("123.45")` |
| `isnan` | `isNaN` | `isnan(value)` |
| `isfinite` | `isFinite` | `isfinite(value)` |
| `encodeurl` | `encodeURI` | `encodeurl(url)` |
| `encodeurlpart` | `encodeURIComponent` | `encodeurlpart(part)` |
| `decodeurl` | `decodeURI` | `decodeurl(url)` |
| `decodeurlpart` | `decodeURIComponent` | `decodeurlpart(part)` |
| `esc` | `escape` | `esc(string)` |
| `unesc` | `unescape` | `unesc(string)` |
| `runcode` | `eval` | `runcode(code)` |
| `delay` | `setTimeout` | `delay(callback, 1000)` |
| `repeat` | `setInterval` | `repeat(callback, 1000)` |
| `stopdelay` | `clearTimeout` | `stopdelay(id)` |
| `stoprepeat` | `clearInterval` | `stoprepeat(id)` |
| `pull` | `fetch` | `pull(url)` |
| `need` | `require` | `need("module")` |

## 🛠️ CLI Usage

```bash
# Transpile to JavaScript
nsc build src/ --outDir dist

# Run NullScript directly
nsc run hello.ns

# Show all keywords
nsc keywords

# Show keywords by category
nsc keywords --category control-flow
```

### Build Options
- `--outDir <dir>` - Output directory (default: `dist`)

## 💻 Example

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

## 🤝 JavaScript Focus

NullScript compiles directly to clean JavaScript:
- ✅ No type annotations in output
- ✅ Compatible with all JavaScript environments
- ✅ Same runtime behavior as regular JavaScript
- ✅ Can import/export with JavaScript and Node.js projects
- ✅ Simpler build process without TypeScript dependency

## 🔗 Links

- **GitHub**: https://github.com/nullscript-lang/nullscript
- **npm**: https://www.npmjs.com/package/nullscript
- **Documentation**: https://github.com/nullscript-lang/nullscript#readme

## 📄 License

MIT License

---

*"NullScript: Because programming should be fun, even when it's serious."* 🎭
