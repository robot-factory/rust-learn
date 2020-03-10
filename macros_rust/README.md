# rust 宏学习

## 分类

首先对宏进行分类：

1. 声明（Declarative）宏，使用 macro_rules!
2. 过程（Procedural），其有三种类型：

   1. 自定义 #[derive] 宏
   2. 类属性（Attribute）宏
   3. 类函数宏

## 宏是什么？宏和函数的区别？

宏有点像一个自动帮你写代码的小助手，和函数相比不存在运行时，即宏在编译的时候就会转化为相应的代码。可以理解出一个帮你写程序的程序。

## 声明宏

声明宏相当常见，比如 `println!`, `vec!`。以下是 vec 的一个简单实现。

```rust
#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

```

因为大部分 rust 编写过程都是使用声明宏而不是编写，所以这部分就先跳过

## 自定义 `#[derive]` 宏

自定义#[derive]宏可以说是写的最多的宏了，至少我见过最多的。在大部分框架里都会有很多自定义宏， 我见过的 actix-web 里就有很多，可以实现出很多看起来很风骚的操作。自定义宏最主要的功能是为某个数据类型实现 trait。
现在，我们已经引入了 三个新的 crate：proc_macro 、 syn 和 quote 。Rust 自带 proc_macro crate，因此无需将其加到 Cargo.toml 文件的依赖中。proc_macro crate 是编译器用来读取和操作我们 Rust 代码的 API。syn crate 将字符串中的 Rust 代码解析成为一个可以操作的数据结构。quote 则将 syn 解析的数据结构反过来传入到 Rust 代码中。这些 crate 让解析任何我们所要处理的 Rust 代码变得更简单：为 Rust 编写整个的解析器并不是一件简单的工作。

当用户在一个类型上指定 #[derive(HelloMacro)] 时，hello_macro_derive 函数将会被调用。原因在于我们已经使用 proc_macro_derive 及其指定名称对 hello_macro_derive 函数进行了注解：HelloMacro ，其匹配到 trait 名，这是大多数过程宏遵循的习惯。

```rust
extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // 构建 Rust 代码所代表的语法树
    // 以便可以进行操作
    let ast = syn::parse(input).unwrap();

    // 构建 trait 实现
    impl_hello_macro(&ast)
}

```


stringify!是一个很神奇的系统提供的声明宏。

```rust
fn main() {
    println!("{}",stringify!(1+1));
}
// 1 + 1 自动格式化出空格
```

## 类属性宏

类属性宏与自定义派生宏相似，不同于为 derive 属性生成代码，它们允许你创建新的属性。它们也更为灵活；derive 只能用于结构体和枚举；属性还可以用于其它的项，比如函数。作为一个使用类属性宏的例子，可以创建一个名为 route 的属性用于注解 web 应用程序框架（web application framework）的函数：

```rust
#[route(GET, "/")]
fn index() {
```

#[route] 属性将由框架本身定义为一个过程宏。其宏定义的函数签名看起来像这样：

```rust
#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {}
```

这里有两个 TokenStream 类型的参数；第一个用于属性内容本身，也就是 GET, "/" 部分。第二个是属性所标记的项，在本例中，是 fn index() {} 和剩下的函数体。

除此之外，类属性宏与自定义派生宏工作方式一致：创建 proc-macro crate 类型的 crate 并实现希望生成代码的函数！

## 类函数宏
最后，类函数宏定义看起来像函数调用的宏。例如，sql! 宏可能像这样被调用：
```rust
let sql = sql!(SELECT * FROM posts WHERE id=1);
```
这个宏会解析其中的 SQL 语句并检查其是否是句法正确的。该宏应该被定义为如此：
```rust
#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
```