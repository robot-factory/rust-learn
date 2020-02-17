# 环境安装
第一步是安装 Rust。我们通过 rustup 下载 Rust，这是一个管理 Rust 版本和相关工具的命令行工具。下载时需要联网。

> 注意：如果你出于某些理由倾向于不使用 rustup，请到 Rust 安装页面 查看其它安装选项。

接下来的步骤会安装最新的稳定版 Rust 编译器。Rust 的稳定性确保本书所有示例在最新版本的 Rust 中能够继续编译。不同版本的输出可能略有不同，因为 Rust 经常改进错误信息和警告。也就是说，任何通过这些步骤安装的最新稳定版 Rust，都应该能正常运行本书中的内容。

> 命令行标记
> 本章和全书中，我们会展示一些在终端中使用的命令。所有需要输入到终端的行都以 $ 开头。但无需输入$；它代表每行命令的起点。不以 $ 起始的行通常展示之前命令的输出。另外，PowerShell 专用的示例会采用 > 而不是 $。

## 在 Linux 或 macOS 上安装 rustup
如果你使用 Linux 或 macOS，打开终端并输入如下命令：


>$ curl https://sh.rustup.rs -sSf | sh
此命令下载一个脚本并开始安装 rustup 工具，这会安装最新稳定版 Rust。过程中可能会提示你输入密码。如果安装成功，将会出现如下内容：

>Rust is installed now. Great!
如果你愿意，可在运行前下载并检查该脚本。

此安装脚本自动将 Rust 加入系统 PATH 环境变量中，在下一次登录时生效。如果你希望立刻就开始使用 Rust 而不重启终端，在 shell 中运行如下命令，手动将 Rust 加入系统 PATH 变量中：


>$ source $HOME/.cargo/env
或者，可以在 ~/.bash_profile 文件中增加如下行：

>$ export PATH="$HOME/.cargo/bin:$PATH"
另外，你需要一个某种类型的链接器（linker）。很有可能已经安装，不过当你尝试编译 Rust 程序时，却有错误指出无法执行链接器，这意味着你的系统上没有安装链接器，你需要自行安装一个。C 编译器通常带有正确的链接器。请查看你使用平台的文档，了解如何安装 C 编译器。并且，一些常用的 Rust 包依赖 C 代码，也需要安装 C 编译器。因此现在安装一个是值得的。

> p.s. 事实上我在Ubuntu18.04的环境也遇到这个问题，通过下面代码解决   
sudo apt-get install gcc

## 在 Windows 上安装 rustup
在 Windows 上，前往 https://www.rust-lang.org/install.html 并按照说明安装 Rust。在安装过程的某个步骤，你会收到一个信息说明为什么需要安装 Visual Studio 2013 或更新版本的 C++ build tools。获取这些 build tools 最方便的方法是安装 Build Tools for Visual Studio 2019。这个工具在 “Other Tools and Frameworks” 部分。

本书的余下部分会使用能同时运行于 cmd.exe 和 PowerShell 的命令。如果存在特定差异，我们会解释使用哪一个。

## 更新和卸载
通过 rustup 安装了 Rust 之后，很容易更新到最新版本。在 shell 中运行如下更新脚本：


> $ **rustup update**
为了卸载 Rust 和 rustup，在 shell 中运行如下卸载脚本:


>$ **rustup self uninstall**

## 故障排除（Troubleshooting）
要检查是否正确安装了 Rust，打开 shell 并运行如下行：


>$ rustc --version
你应能看到已发布的最新稳定版的版本号、提交哈希和提交日期，显示为如下格式：


>rustc x.y.z (abcabcabc yyyy-mm-dd)
如果出现这些内容，Rust 就安装成功了！如果并没有看到这些信息，并且使用的是 Windows，请检查 Rust 是否位于 %PATH% 系统变量中。如果一切正确但 Rust 仍不能使用，有许多地方可以求助。最简单的是 irc.mozilla.org 上的 #rust IRC 频道 ，可以使用 Mibbit 来访问它。然后就能和其他 Rustacean（Rust 用户的称号，有自嘲意味）聊天并寻求帮助。其它给力的资源包括用户论坛和 Stack Overflow。


## 本地文档
安装程序也自带一份文档的本地拷贝，可以离线阅读。运行 rustup doc 在浏览器中查看本地文档。

任何时候，如果你拿不准标准库中的类型或函数的用途和用法，请查阅应用程序接口（application programming interface，API）文档！

## 更换软件源，配置镜像
> vim /etc/profile
> export RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static
> export RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup
本来我也不太喜欢用镜像，但是没办法，国内的网络实在是太卡了。

## 有时安装的内容会被墙

解决方案：
1. 打开终端， 输入 RUSTUP_```DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static``` 和 ```RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup```，使当前终端有这两个环境变量，然后执行 ``` curl https://sh.rustup.rs -sSf | sh ```
2. 使用代理执行命令，

## 安装toolchain

rustup toolchain install nightly-x86_64-unknown-linux-gnu
rustup toolchain install nightly
rustup default nightly

查看toolchain

rustup toolchain list
stable-x86_64-unknown-linux-gnu
nightly-x86_64-unknown-linux-gnu (default)