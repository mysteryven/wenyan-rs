# Wenyan-rs

A bytecode interpreter for [Wenyan-lang](https://github.com/wenyan-lang/wenyan).

## Usage

```bash
wyw [file]
```

## Examples

```bash
吾有一術 名之曰「階乘」 欲行是術 必先得一數 曰「甲」 乃行是術曰
 若「甲」等於一者。
  乃得「甲」
 若非
  減「甲」以一名之曰「乙」
  施「階乘」於「乙」名之曰「丙」
  乘「丙」以「甲」。名之曰「丁」
  乃得「丁」
 云云
是謂「階乘」之術也

施「階乘」於五書之
```

Output:

```bash
120
```

This project are still work in progress, many features are not implemented yet. You can see more supported examples in [inputs](./tests/inputs/) folder.

## Differences

The [Online IDE](https://ide.wy-lang.org/) has many friendly enhances, some of my rules may be more strict for ease of implementation.

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

Boolean algebra statement always gets boolean.

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

Treats '也' as kind of whitespace, you can use it to make code more readable, but can't use it as close of block.

Function will be added into stack, and "書之" will drain values produced by current function.

```bash
吾有一數曰五名之曰「甲」
吾有一術名之曰「你好」是術曰
夫「「世界，你好」」書之
是謂 「你好」 之術也

施「你好」
```

After `夫「「世界，你好」」書之`, The stack will be:

```bash
[2] <fn 你好>
[1] Value::Number(5)
[0] <global context> 
```

When function `你好` returns, the stack will be:

```bash
[1] Value::Number(5)
[0] <global context> 
```

Take advantage of our own virtual machine, It has ability to report more kind of runtime errors.

```bash
吾有二言曰『你』曰『好』名之曰「甲」名之曰「乙」 
減「甲」以「乙」書之
```

Output:

```bash
[line 2] errors: two string can only be added
```

## Install

If you are Mac user, download binary file from the release page, and follow this [blog](https://zwbetz.com/how-to-add-a-binary-to-your-path-on-macos-linux-windows/#macos-and-linux-cli) to add it to your path. If you are noticed about "can’t be opened because Apple cannot check it for malicious software.", you can follow this [blog](https://support.apple.com/en-us/HT202491) to allow it.

The better install way will be added when this project is more stable.

## Future Work

There are still many features not implemented! Part of them are listed in [this issue](https://github.com/mysteryven/wenyan-rs/issues/1).

## Credits

- [Crafting Interpreters](http://craftinginterpreters.com/)
- [lox-rs](https://github.com/Darksecond/lox)

## License

[MIT.](./LICENSE)
