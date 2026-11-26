# RustStudyNotes

Rust 学习笔记与练习代码，记录学习过程中的示例。

## 示例

| 目录 | 内容 |
| --- | --- |
| `hello_world/` | 使用 `rustc` 编译并运行第一个 Rust 程序 |
| `hello_cargo/` | 使用 Cargo 管理并运行 Rust 项目 |

## 运行

先通过 [rustup](https://rustup.rs/) 安装 Rust 工具链。
`hello_cargo` 使用 Rust 2024 edition，需要 Rust 1.85 或更新版本。

### hello_world

```sh
cd hello_world
rustc main.rs
./main
```

### hello_cargo

在仓库根目录运行：

```sh
cargo run --manifest-path hello_cargo/Cargo.toml
```

## 参考资料

- [Rust 程序设计语言（简体中文版）](https://kaisery.github.io/trpl-zh-cn/)
- [The Rust Programming Language](https://doc.rust-lang.org/book/)

## 许可证

本项目使用 [MIT License](LICENSE)。
