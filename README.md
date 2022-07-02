# sysy

## 环境配置

需要 Linux 或 WSL 环境

- LLVM 版本：13.0.1 (理论上只要是 LLVM 13 均可)
- rust 版本：1.61.0（该版本及以上均可）

### 安装 rust 环境

安装 rustup，参考 https://rustup.rs/ ，下载脚本并运行，会自动将 rust 工具链安装到当前用户

安装完成后，输入 `rustup --version`，应有类似的信息输出

```
❯ rustup --version
rustup 1.24.3 (ce5817a94 2021-05-31)
info: This is the version for the rustup toolchain manager, not the rustc compiler.
info: The currently active `rustc` version is `rustc 1.61.0 (fe5b13d68 2022-05-18)`
```

名词解释：

- rustup：rust 语言的工具链管理器，用于安装和管理不同平台（x86/arm, macos/linux, ...）、版本的工具链
  - 在 C 语言工具链中的对应物：无
- rustc：rust 语言编译器，但是一般并不直接使用，通常搭配下文提到的 cargo 使用
  - 在 C 语言工具链中的对应物：gcc
- cargo：rust 语言的包管理器和构建工具：根据项目中的 `Cargo.toml` 文件，自动下载并编译相关第三方库，并编译代码
  - 在 C 语言工具链中的对应物：make + cmake

### 编译本项目

在项目根目录下，执行 `cargo run -- -h`，cargo 会自动编译第三方库和本项目的代码，并运行本项目。

- `cargo run` 用于运行项目
  - `--` 之后的部分，将作为命令行参数传递给本项目

应有类似的信息输出：

```
sysy 0.1.0

USAGE:
    sysy [OPTIONS] <INPUT>

ARGS:
    <INPUT>    input file path

OPTIONS:
    -a, --ast           print ast
    -e, --exe-enable    enable exe output
    -g, --global        print global vars
    -h, --help          Print help information
    -i, --ir-enable     enable ir output
    -p, --prototype     print function prototypes
    -V, --version       Print version information
```

这打印出了本项目的命令行帮助。这说明本项目可以正确编译。

### 运行本项目

在编译完成后，`./target/debug` 文件夹中会多出来一个 `sysy` 可执行文件，这便是本项目的二进制。

```
❯ ./target/debug/sysy 
error: The following required arguments were not provided:
    <INPUT>

USAGE:
    sysy [OPTIONS] <INPUT>

For more information try --help
```

同时，我们也可以使用 `cargo run -- `来运行本项目，这与直接使用 `./target/debug` 具有相同的效果。
区别是如果代码更新了， `cargo run`会自动重新编译代码。

为了使用本项目编译 sysy 程序，我们需要使用以下命令行参数：

```
./target/debug/sysy ./tests/cases/hello_world.sy -e
```

其中， `-e` 代表生成可执行程序。编译完成后，在当前目录下，会出现可执行文件：

```
❯ ll hello_world.*
.rw-r--r-- 1.5k mgt  2 7月  22:04 hello_world.bc
.rwxr-xr-x  22k mgt  2 7月  22:04 hello_world.exe

```

可以执行该文件

```
❯ ./hello_world.exe 
TOTAL: 0H-0M-0S-0us
hello world⏎ 

```
## 文件结构
```
❯ ls --tree
.
├── Cargo.lock
├── Cargo.toml
├── compiler2022/ -> 编译大赛相关文件
├── README.md
├── rustfmt.toml
├── src -> 源代码
│  ├── cli.rs -> 命令行参数相关
│  ├── codegen -> 中间代码生成
│  │  ├── dbg.rs -> 调试用函数
│  │  ├── expression -> 表达式的中间代码生成
│  │  │  ├── binary.rs -> 二元表达式的中间代码生成
│  │  │  ├── call.rs -> 函数调用表达式的中间代码生成
│  │  │  ├── identifier.rs
│  │  │  ├── literal.rs
│  │  │  ├── mod.rs
│  │  │  ├── subscript.rs
│  │  │  └── unary.rs
│  │  ├── function.rs -> 函数的中间代码生成
│  │  ├── global.rs -> 全局变量的中间代码生成
│  │  ├── mod.rs -> **中间代码生成所用到的工具类的定义**
│  │  ├── statememt -> 语句的中间代码生成
│  │  │  ├── assignment.rs -> 赋值语句
│  │  │  ├── compound.rs -> 符合语句
│  │  │  ├── expression.rs
│  │  │  ├── if_statement.rs
│  │  │  ├── mod.rs
│  │  │  └── while_statement.rs
│  │  ├── utils.rs -> 工具代码
│  │  └── var.rs -> 变量相关代码
│  ├── error.rs -> 错误类型
│  ├── lib.rs
│  ├── main.rs -> 主函数所在的文件，程序的入口点
│  ├── parser.rs -> parser 相关函数
│  └── util.rs -> 相关工具函数
├── tests -> 测试
│  ├── cases
│  │  ├── break_out_loop.sy
│  │  ├── duplicate_global_sym.sy
│  │  ├── tree.sy
│  │  └── var_not_found.sy
│  └── sysy-tests.rs -> 调用 sysy 官方测试用例进行自动化测试
└── tree-sitter-sysy -> 使用 tree-sitter 编写的 sysy 语言语法文件
   ├── bindings
   │  └── rust -> 将生成的parser绑定到rust语言，使得可以在rust语言中使用
   │     ├── build.rs
   │     ├── lib.rs
   │     └── README.md
   ├── grammar.js -> 语法文件
   └── src -> 根据语法文件自动生成的parser
      ├── grammar.json
      ├── node-types.json
      ├── parser.c
      └── tree_sitter
         └── parser.h
```
