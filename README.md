# Wenyan-rs

A bytecode interpreter for [Wenyan-lang](https://github.com/wenyan-lang/wenyan).

## Try it online

[playground](./)

## Install

```
npm install wenyan-rs
```

## Notes

I followed the language grammar from [Wenyan-lang Specification](https://wy-lang.org/spec.html). there are still some features not implemented yet, and they are listed in [this issue](./).

I found that the [Online IDE](https://ide.wy-lang.org/) has many friendly enhances, but I not realized those implicit features now. I added some strict checks in the complication stage: 

Must use `「` for variable and `「「` for string. The handbook has this rule, the Online IDE can pass without a surrounded 「 for variable. I think it will remove ambiguity for some situation, so I added this rule in this implementation.

Has static type check. For example, we can't assign a number to `言`(the string type in wenyan language). The [online IDE](https://ide.wy-lang.org/) not check this rule(If there some consideration, please let me know!), I think it makes sense to throw a error before run it.

Not support NaN/Infi

## Credits

- [Crafting Interpreters](http://craftinginterpreters.com/)
- [lox-rs](https://github.com/Darksecond/lox)

## License

[MIT.](./LICENSE)
