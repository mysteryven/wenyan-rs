# Wenyan-rs

A bytecode interpreter for [Wenyan-lang](https://github.com/wenyan-lang/wenyan).

## Usage

```bash
cargo run tests/inputs/for-enum-statement.wy
```

## Differences

The [Online IDE](https://ide.wy-lang.org/) has many friendly enhances, but some of my rules may be more strict for ease of implementation.

Variable need to be wrapped with single quote.

```bash
吾有一數曰五名之曰「甲」 // ✅
吾有一數曰五名之曰甲 // ❌

昔之「甲」者今其是矣 // ✅
昔之甲者今其是矣 // ❌
```

Disable partially define variables.

```bash
吾有二數曰五曰六  // ✅
吾有二數曰五曰六名之曰「甲」名之曰「乙」 // ✅
吾有二數曰五曰六名之曰「甲」 // ❌ 
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

Since take advantage of our own virtual machine, It has ability to report more kind of runtime errors.

```bash
吾有二言曰『你』曰『好』名之曰「甲」名之曰「乙」 
減「甲」以「乙」書之
```

Output:

```bash
[line 2] errors: two string can only be added
```

## Future Work

There are still some features not implemented, part of them are listed in [this issue](https://github.com/mysteryven/wenyan-rs/issues/1).

## Credits

- [Crafting Interpreters](http://craftinginterpreters.com/)
- [lox-rs](https://github.com/Darksecond/lox)

## License

[MIT.](./LICENSE)
