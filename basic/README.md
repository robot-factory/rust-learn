# Rust语法基础

## 编译器和项目管理器
rustc是个存粹的编译器，cargo就像是golang里的go mod或nodejs的npm，但反正一般都是用cargo，rustc知道有这个东西就行。

### 创建、运行、编译项目

1. 创建
```bash
cargo new hello_cargo
> Created binary (application) `hello_cargo` package
```

2. 运行
```bash
cd hello_cargo
cargo run 
>   Compiling hello_cargo v0.1.0 (/Users/liuboheng/Documents/indie/rust-learn/basic/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 7.76s
     Running `target/debug/hello_cargo`
>   Hello, world!
```
相比go，rust的编译速度有点感人

3. 编译
```bash
cargo build
> Finished dev [unoptimized + debuginfo] target(s) in 0.00s
```
为什么这么快呢？
`cargo build` 命令默认会构建一个开发版本的程序，该版本包含了调试信息并未进行优化，以便于调试和测试。如果您想构建一个生产版本，需要使用 `--release` 标志，如下所示：

```
cargo build --release

```

这将会构建一个优化后的生产版本，该版本通常运行速度更快，但不包含调试信息。

```
> Compiling hello_cargo v0.1.0 (/Users/liuboheng/Documents/indie/rust-learn/basic/hello_cargo)
    Finished release [optimized] target(s) in 0.09s
```
这次看起来可以了，在target的release文件夹里有了相应的文件

## 熟悉println!

println!是一个宏，暂时先不管，初始学习的时候我们会很依赖log，所以需要了解一个其他用法。

```
let x = 5;
let y = 10;

println!("x = {x} and y + 2 = {}", y + 2);
```
这个用法类似模版语言

### println!("{:?}")

println!("{:?}", now);

这里使用 {:?} 占位符，这是一个调试占位符，用于输出 Debug trait 格式化的内容。
当你对一个值使用 {:?}，你实际上是在请求该值的 Debug 实现的输出。这通常用于调试目的，因为它能展示更多的内部信息。
对于很多标准库类型和很多第三方库类型，{:?} 会输出一个对开发者有帮助的表示。但是这不一定是为了展示给最终用户的格式，而且具体的输出格式依赖于具体类型的 Debug 实现。

## 熟悉变量

```
cargo new variable
cd variable
```



```
fn main() {
    let mut x = 5; // 使用mut关键字来声明x为可变变量
    println!("x is {}", x);
    x = 6; // 修改x的值
    println!("x is {}", x);
}
```
这个是不行的，因为在 Rust 中，变量默认是不可变的（immutable）。

需要改成
```
fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
```

```
fn main() {
    let x = 5;
    println!("x is {x}");
    let x = 6;
    println!("x is {x}");
}
```
这是可以运行的，我把x重新绑定了

```
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

>The value of x in the inner scope is: 12
The value of x is: 6
```
重新绑定也只是在自己的作用域里重新绑定
如果使用const就无法重新绑定了

```
fn main() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    const THREE_HOURS_IN_SECONDS: u32 = 1;
    println!("The value of x is: {THREE_HOURS_IN_SECONDS}");
}
```
