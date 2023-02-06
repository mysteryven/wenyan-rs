# Wenyan-rs

A bytecode interpreter implemented in Rust for wenyan lang.

## Try it online

[playground](./)

## Install

```
npm install wenyan-rs
```

or with pnpm:

```
pnpm install wenyan-rs
```

## Run playground in local

```bash
wasm-pack build
pnpm i
cd playground
pnpm dev
```

## Differences

Must use `「` for variable and `「「` for string. The handbook has this rule, but the [online IDE](https://ide.wy-lang.org/) can pass without a surrounded 「 for variable. I think it will remove ambiguity for some situation, so I added this rule in this implementation,

> 或問曰。甲字上下有符如矩尺然者。何焉。今欲省之。可乎。曰。不可。此引號也。「單引號」者。所以別變數於其他也。「「雙引號」」者。所以別言語於其他也。微是。不能別歧義也。又問曰。句讀。挪抬。無之可乎。 

Has static type check. For example, we can't assign a number to `言`(the string type in wenyan language). The [online IDE](https://ide.wy-lang.org/) not check this rule(If there some consideration, please let me know!), I think it makes sense to throw a compiler error.

## Credits

- [Crafting Interpreters](http://craftinginterpreters.com/)
- [lox-rs](https://github.com/Darksecond/lox)
