fn main() {
    /*
    可以使用 cargo new 创建项目。
    可以使用 cargo build 构建项目。
    可以使用 cargo run 一步构建并运行项目。
    可以使用 cargo check 在不生成二进制文件的情况下构建项目来检查错误。
    有别于将构建结果放在与源码相同的目录，Cargo 会将其放到 target/debug 目录。

    当项目最终准备好发布时，可以使用 cargo build --release 以启用优化方式编译项目。
    这会在 target/release 而不是 target/debug 下生成可执行文件。
    这些优化会让 Rust 代码运行得更快，不过开启优化也会延长编译时间。
     */
    println!("Hello, world!");
}
