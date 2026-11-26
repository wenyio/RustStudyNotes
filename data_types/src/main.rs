fn main() {
    /*
    Rust 是 静态类型（statically typed）语言，也就是说，它必须在编译时就知道所有变量的类型。
    编译器通常可以根据值以及它的使用方式推断出我们想要使用的类型。
     */
    // let guess = "42".parse().expect("Not a number!");
    let guess: u32 = "42".parse().expect("Not a number!");
}
