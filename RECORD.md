# Rust 简介

Rust 是一门系统编程语言，专注于安全性和性能。它与 C 和 C++ 等语言一样，使用编译器直接编译成机器码。

Rust 语言的设计目标是成为一门实用的语言，它的设计借鉴了现代编程语言的各种优秀实践。Rust 语言的设计者们从众多现代编程语言中吸取了精华，创造了一门既有系统编程语言的能力，又有高级语言的安全性和便利性的语言。

Rust 的一些主要应用领域包括：

- 低级系统编程领域，如操作系统、设备驱动、嵌入式系统等。
- 高性能服务器领域，如 Web 服务器、数据库、游戏服务器等。
- 大规模并发领域，如高性能计算、区块链等。
- WebAssembly 领域，如浏览器、桌面应用、移动应用等。
- 物联网领域，如智能家居、智能工厂等。
- 人工智能领域，如机器学习、深度学习等。
- 云计算领域，如云原生、边缘计算等。
- 游戏开发领域，如游戏引擎、游戏客户端等。
- 信息安全领域，如密码学、区块链等。
- 科学计算领域，如数值计算、仿真模拟等。

## 环境准备

### 安装 Rust

```sh
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
# 详情可见： https://www.rust-lang.org/zh-CN/tools/install
```

上面的命令将下载以上脚本并开始安装用来安装最新版 Rust 的 rustup 工具。 您可能需要输入密码。 若安装成功，终端将显示以下内容：  
![](https://assets.fedtop.com/picbed/202208220022863.png)

[所有系统的预先准备方案可以参考官网链接](https://tauri.app/zh/v1/guides/getting-started/prerequisites/)

https://tauri.app/zh-cn/v1/guides/getting-started/prerequisites/

### 升级 Rust

```sh
rustup update
```

### 卸载 Rust

```sh
rustup self uninstall
```

### IDE 配置

推荐使用 VSCode，安装 Rust 插件 rust-analyzer。

## Rust 基础

### 基本语法

- 变量

变量默认是不可变的，使用 `mut` 修饰后可变。

```rust
fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    let mut y = 5;
    println!("The value of y is: {}", y);
    y = 6;
    println!("The value of y is: {}", y);
}
```

- 常量

常量使用 `const` 关键字声明，必须注明类型，且只能在常量表达式中使用。

```rust
const MAX_POINTS: u32 = 100_000;
```

##### 隐藏

可以使用 `let` 关键字来进行隐藏，隐藏后的变量可以改变类型，但是不能改变变量名。

```rust
fn main() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);
}
```

## 待学习的概念

- [ ] RAII 的管理内存方式
- [ ] FFI 的使用

Rust 支持和 C/C++ 的零损耗 FFI。  
这代表着：所有 C 和 C++ 的生态，也同时是 Rust 的生态，Rust 都可以使用。

学会知识屏蔽，那什么叫知识屏蔽呢？  
知识屏蔽的意思是，把部分的知识当做黑盒，不要过于钻到细节里面去，也不要想着一次性把所有东西都学会。
