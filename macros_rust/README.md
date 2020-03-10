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
声明宏相当常见，比如 `println!`, `vec!`。以下是vec的一个简单实现。
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
因为大部分rust编写过程都是使用声明宏而不是编写，所以这部分就先跳过

## 自定义 `#[derive]` 宏
自定义#[derive]宏可以说是写的最多的宏了，至少我见过最多的。在大部分框架里都会有很多自定义宏， 我见过的actix-web里就有很多，可以实现出很多看起来很风骚的操作。自定义宏最主要的功能是为某个数据类型实现trait。
