# Wenyan-rs

A bytecode interpreter for [Wenyan-lang](https://github.com/wenyan-lang/wenyan).

## Try it Online

[Playground](./)

## Install

```
npm install wenyan-rs
```

## Differences

The [Online IDE](https://ide.wy-lang.org/) has many friendly enhances, but some of my rules may be stricter for ease of implementation.

Variables need to be wrapped in single quotes. 

```bash
吾有一數曰五名之曰「甲」 // ✅
吾有一數曰五名之曰甲 // ❎
```

Disable partially define variables. 

```bash
吾有二數曰五曰六名之曰「甲」名之曰「乙」 // ✅
吾有二數曰五曰六名之曰「甲」 // ❎
吾有二數曰五曰六  // ✅
```

Has implicit block scope.

```bash
吾有一數曰十名之曰「甲」
若陽者
	吾有一數曰一名之曰「甲」
	加「甲」以五書之
云云
加「甲」以五書之
```

Output vs Online IDE:

```diff
-6
-6
+6
+15
```

## Future Work

There are still some features not implemented yet, part of them are listed in [this issue](./).

## Credits

- [Crafting Interpreters](http://craftinginterpreters.com/)
- [lox-rs](https://github.com/Darksecond/lox)

## License

[MIT.](./LICENSE)
