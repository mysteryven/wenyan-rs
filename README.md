# Wenyan-rs

A bytecode interpreter written by Rust for wenyan language.

## Try it

[playground](./)

## Install

## Differences

Must use 「 for variable and 「「 for string. The handbook has this rule, but the [online IDE](https://ide.wy-lang.org/) can pass without a surrounded 「 for variable. I think it will remove ambiguity for some situation, so I added this rule in this implementation,

[或問曰。甲字上下有符如矩尺然者。何焉。今欲省之。可乎。曰。不可。此引號也。「單引號」者。所以別變數於其他也。「「雙引號」」者。所以別言語於其他也。微是。不能別歧義也。又問曰。句讀。挪抬。無之可乎。](https://github.com/wenyan-lang/book/blob/d73bb7b6f3aeb3ce13591fa120362e4065234d9f/01 明義第一.md#L71)

Has static type check. For example, we can't assign a number to `言`(the string type in wenyan language). The [online IDE](https://ide.wy-lang.org/) not check this rule(If there some consideration, please let me know!), I think it makes sense to throw a compiler error.

## Credits

- [Crafting Interpreters](http://craftinginterpreters.com/)
- [lox-rs](https://github.com/Darksecond/lox)
