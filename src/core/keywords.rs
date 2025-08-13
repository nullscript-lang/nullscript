pub static KEYWORDS: &[(&str, &str)] = &[

    ("run", "function"),
    ("return", "return"),
    ("let", "let"),
    ("fixed", "const"),
    ("var", "var"),
    ("share", "export"),
    ("use", "import"),
    ("whatever", "if"),
    ("otherwise", "else"),
    ("since", "for"),
    ("when", "while"),
    ("switch", "switch"),
    ("case", "case"),
    ("done", "default"),
    ("stop", "break"),
    ("keepgoing", "continue"),
    ("test", "try"),
    ("grab", "catch"),
    ("atLast", "finally"),
    ("fresh", "new"),
    ("self", "this"),
    ("parent", "super"),
    ("model", "class"),
    ("remove", "delete"),
    ("null", "null"),
    ("yes", "true"),
    ("no", "false"),
    ("undefined", "undefined"),
    ("is", "==="),
    ("isnt", "!=="),
    ("more", ">"),
    ("less", "<"),
    ("moreeq", ">="),
    ("lesseq", "<="),
    ("and", "&&"),
    ("or", "||"),
    ("not", "!"),


    ("trigger", "throw"),
    ("inherits", "extends"),
    ("__init__", "constructor"),
    ("forever", "static"),
    ("later", "async"),
    ("hold", "await"),
    ("what", "typeof"),
    ("kind", "instanceof"),
    ("inside", "in"),
    ("part", "of"),
    ("nothing", "void"),
    ("using", "with"),
    ("freeze", "debugger"),
    ("pause", "yield"),
    ("getter", "get"),
    ("setter", "set"),


    ("speak", "console"),
    ("say", "log"),
    ("yell", "warn"),
    ("scream", "error"),
    ("whisper", "info"),
    ("peek", "debug"),
    ("check", "assert"),
    ("wipe", "clear"),
    ("tally", "count"),
    ("resetcount", "countReset"),
    ("dir", "dir"),
    ("deepdir", "dirxml"),
    ("group", "group"),
    ("fold", "groupCollapsed"),
    ("ungroup", "groupEnd"),
    ("show", "table"),
    ("time", "time"),
    ("stoptimer", "timeEnd"),
    ("logtimer", "timeLog"),
    ("backtrace", "trace"),


    ("thing", "Object"),
    ("list", "Array"),
    ("text", "String"),
    ("num", "Number"),
    ("bool", "Boolean"),
    ("clock", "Date"),
    ("maths", "Math"),
    ("json", "JSON"),
    ("pattern", "RegExp"),
    ("fail", "Error"),
    ("promise", "Promise"),
    ("dict", "Map"),
    ("unique", "Set"),
    ("weakdict", "WeakMap"),
    ("weakunique", "WeakSet"),
    ("symbol", "Symbol"),
    ("proxy", "Proxy"),
    ("reflect", "Reflect"),
    ("intl", "Intl"),
    ("wasm", "WebAssembly"),


    ("toint", "parseInt"),
    ("tofloat", "parseFloat"),
    ("isnan", "isNaN"),
    ("isfinite", "isFinite"),
    ("encodeurl", "encodeURI"),
    ("encodeurlpart", "encodeURIComponent"),
    ("decodeurl", "decodeURI"),
    ("decodeurlpart", "decodeURIComponent"),
    ("esc", "escape"),
    ("unesc", "unescape"),
    ("runcode", "eval"),
    ("delay", "setTimeout"),
    ("repeat", "setInterval"),
    ("stopdelay", "clearTimeout"),
    ("stoprepeat", "clearInterval"),
    ("pull", "fetch"),
    ("need", "require"),
];




pub static FORBIDDEN_KEYWORDS: &[&str] = &[

    "interface", "enum",
    "abstract", "implements",
    "public", "private", "protected", "readonly",
    "generator",


    "Partial", "Required", "Readonly", "Record", "Pick", "Omit",
    "Exclude", "Extract", "NonNullable", "ReturnType", "InstanceType",
    "Parameters", "ConstructorParameters", "ThisType",


    "decorator", "metadata", "reflect",


    "namespace", "declare", "ambient",


    "satisfies", "asserts", "infer", "keyof",
    "out",
];


pub static INVALID_SYNTAX: &[&str] = &[

    ": string", ": number", ": boolean", ": any", ": void", ": never",
    ": unknown", ": object", ": array", ": tuple", ": union",
    ": String", ": Number", ": Boolean", ": Any", ": Void", ": Never",
    ": Unknown", ": Object", ": Array", ": Tuple", ": Union",


    "<T>", "<string>", "<number>", "<boolean>", "<any>",
    "extends T", "implements I", "infer U",


    "as string", "as number", "as boolean", "as any",
    "satisfies T", "asserts T", "const T",


    "interface", "enum", "namespace",


    "@decorator", "@Component", "@Injectable", "@Input", "@Output",
];
