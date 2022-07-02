# sysy

## 环境配置



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
