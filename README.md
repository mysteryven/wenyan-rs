# Wenyan-rs

A bytecode interpreter for [Wenyan-lang](https://github.com/wenyan-lang/wenyan).

## Try it online

[playground](./)

## Install

```
npm install wenyan-rs
```

## Differences

The [Online IDE](https://ide.wy-lang.org/) has many friendly enhances, but I not realized those implicit features. I added some checks in the complication stage: 

Must use `「` for variable and `「「` for string. The Online IDE not force it while the handbook has this rule.

Has static type check. For example, we can't assign a number to `言`(the string type in wenyan language). The Online IDE not check this rule, I think it makes sense to throw a error before run it.


## Future Work

There are still some features not implemented yet, and they are listed in [this issue](./).

## Credits

- [Crafting Interpreters](http://craftinginterpreters.com/)
- [lox-rs](https://github.com/Darksecond/lox)

## License

[MIT.](./LICENSE)
