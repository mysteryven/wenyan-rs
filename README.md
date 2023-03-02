# Wenyan-rs

A bytecode interpreter for [Wenyan-lang](https://github.com/wenyan-lang/wenyan).

## Try it Online

[Playground](./)

## Install

```
npm install wenyan-rs
```

## Differences

The [Online IDE](https://ide.wy-lang.org/) has many friendly enhances, but some of my rules may be more strict for ease of implementation.

Treats single [data](https://wy-lang.org/spec.html#data) as expression and will be pushed into stack. 

```bash
吾有一數曰五名之曰「甲」
「甲」書之
「「黃河流水鳴濺濺」」書之
```

Output vs Online IDE

```diff
+5
+黃河流水鳴濺濺
```

Variable need to be wrapped in single quote. 

```bash
吾有一數曰五名之曰「甲」 // ✅
吾有一數曰五名之曰甲 // ❎
```

Disable partially define variables. 

```bash
吾有二數曰五曰六  // ✅
吾有二數曰五曰六名之曰「甲」名之曰「乙」 // ✅
吾有二數曰五曰六名之曰「甲」 // ❎
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

Boolean algebra statement always get boolean.

```bash
吾有一數曰五名之曰「甲」
吾有一數曰六名之曰「乙」
夫「甲」「乙」中有陽乎
書之

吾有一數曰五名之曰「甲」
吾有一數曰六名之曰「乙」
夫「甲」「乙」中無陰乎
書之
```

Output vs Online IDE:

```diff
-5
-6
+true
+true
```

## Future Work

There are still some features not implemented, part of them are listed in [this issue](https://github.com/mysteryven/wenyan-rs/issues/1).

## Credits

- [Crafting Interpreters](http://craftinginterpreters.com/)
- [lox-rs](https://github.com/Darksecond/lox)

## License

[MIT.](./LICENSE)
