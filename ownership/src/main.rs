/*

栈与堆的简要总结：
栈：后进先出（LIFO），数据大小必须已知且固定。入栈和出栈都很快，因为分配器无需搜索空间，位置总在栈顶。访问也快，因为数据彼此相邻，对处理器友好。
堆：用于存放编译时大小未知或可能变化的数据。分配时需向内存分配器请求空间，分配器要找到足够大的空位并做记录，因此比入栈慢。访问堆数据必须通过指针，比访问栈慢。
函数调用：调用函数时，传入的值（包括指向堆数据的指针）和局部变量被压入栈；函数结束时这些值被移出栈。
与所有权的关系：所有权系统主要用来管理堆数据——跟踪哪些代码在使用堆上的哪些数据、减少重复数据、清理不再使用的数据以防耗尽空间。理解了栈与堆，就能明白所有权为何这样设计。

所有权的主要目的就是管理堆数据

所有权规则：
1. Rust 中的每一个值都有一个 所有者（owner）。
2. 值在任一时刻有且只有一个所有者。
3. 当所有者离开作用域，这个值将被丢弃。

变量作用域：
{                    // s 无效
    let s = "hello"; // s 有效
}                    // s 无效

String 类型
标量类型（基本数据类型）都是已知大小的，可以存储在栈中，String 存储在堆上。

内存与分配
就字符串字面值来说，我们在编译时就知道其内容，所以文本被直接硬编码进最终的可执行文件中。这使得字符串字面值快速且高效。
对于 String 类型，为了支持一个可变，可增长的文本片段，需要在堆上分配一块在编译时未知大小的内存来存放内容。这意味着：
- 必须在运行时向内存分配器（memory allocator）请求内存。
- 需要一个当我们处理完 String 时将内存返回给分配器的方法。
当调用 String::from 时，它的实现 (implementation) 请求其所需的内存。

在有 垃圾回收（garbage collector，GC）的语言中，GC 记录并清除不再使用的内存，而我们并不需要关心它。在大部分没有 GC 的语言中，识别出不再使用的内存并调用代码显式释放就是我们的责任了，跟请求内存的时候一样。从历史的角度上说正确处理内存回收曾经是一个困难的编程问题。如果忘记回收了会浪费内存。如果过早回收了，将会出现无效变量。如果重复回收，这也是个 bug。我们需要精确的为一个 allocate 配对一个 free。

Rust 采取了一个不同的策略：内存在拥有它的变量离开作用域后就被自动释放。
{
    let s = String::from("hello"); // s 有效
}                                  // s 无效，Rust 在结尾的 } 处自动调用 drop
在更复杂的场景下代码的行为可能是不可预测的，比如当有多个变量使用在堆上分配的内存时

使用移动的变量与数据交互
let x = 5;
let y = x; // 两个变量都等于 5，有两个 5 被压入栈中

let s1 = String::from("hello");
let s2 = s1; // 两个指针指向同一内存；离开作用域时会有二次释放（double free） 的错误

为了确保内存安全，在 let s2 = s1; 之后，Rust 认为 s1 不再有效，因此 Rust 不需要在 s1 离开作用域后清理任何东西。

let s1 = String::from("hello");
let s2 = s1;  // s1 失效
println!("{s1}, world!"); // 报错，禁止使用无效的引用

Rust 永远也不会自动创建数据的 “深拷贝”。因此，任何自动的复制都可以被认为是对运行时性能影响较小的

作用域与赋值
作用域、所有权和通过 drop 函数释放内存之间的关系反过来也同样成立。当你给一个已有的变量赋一个全新的值时，Rust 将会立即调用 drop 并释放原始值的内存。例如，考虑如下代码：
let mut s = String::from("hello");
s = String::from("ahoy");

使用克隆的变量与数据交互
let s1 = String::from("hello");
let s2 = s1.clone();
println!("s1 = {s1}, s2 = {s2}");

只在栈上的数据：拷贝
像整型这样的在编译时已知大小的类型被整个存储在栈上，所以拷贝其实际的值是快速的
let x = 5;
let y = x;
println("x = {x}, y = {y}");

Rust 有一个叫做 Copy trait 的特殊注解，可以用在类似整型这样的存储在栈上的类型上（第十章将会详细讲解 trait）。如果一个类型实现了 Copy trait，那么一个旧的变量在将其赋值给其他变量后仍然有效。
Rust 不允许自身或其任何部分实现了 Drop trait 的类型使用 Copy trait。如果我们对其值离开作用域时需要特殊处理的类型使用 Copy 注解，将会出现一个编译时错误。

实现了 Copy trait 的类型：
- 所有整数类型，比如 u32。
- 布尔类型，bool，它的值是 true 和 false。
- 所有浮点数类型，比如 f64。
- 字符类型，char。
- 元组，当且仅当其包含的类型也都实现 Copy 的时候。比如，(i32, i32) 实现了 Copy，但 (i32, String) 就没有。

所有权与函数
fn main() {
    let s = String::from("hello"); // s 进入作用域

    takes_ownership(s); // s 的值移动到函数里，后续使用就会有编译错误

    let x = 5; // x 进入作用域

    makes_copy(x); // 因为 i32 实现了 Copy trait，x 不会移动到函数里，后续仍然可以使用
}

fn takes_ownership(some_string: String) { // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 drop 方法，内存释放

fn makes_copy(some_integer: i32) { // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域

返回值与作用域
返回值也可以转移所有权
fn main() {
    let s1 = give_ownership();  // 返回值传递给 s1
    let s2 = String::from("hello"); // s2 进入作用域
    let s3 = takes_and_gives_back(s2); // s2 被传入函数，又被返回，传递给 s3
} // s3 移除作用域或丢弃；s2 被 move，无事发生；s1 移除作用域或丢弃

fn give_ownership() -> String {
    let some_string = String::from("yours"); // 进入作用域

    some_string // 返回 some_string 并将其移至调用函数
}

// 该函数将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String {
    // a_string 进入作用域
    a_string // 返回 a_string 并移出给调用的函数
}

如果我们想要函数使用一个值但不获取所有权该怎么办呢？如果我们还要接着使用它的话，每次都传进去再返回来就有点烦人了，除此之外，我们也可能想返回函数体中产生的一些数据
我们可以使用元组来返回多个值
 */
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() 返回字符串的长度
    (s, length)
}
// 但是这未免有些形式主义，而且这种场景应该很常见。幸运的是，Rust 对此提供了一个不用获取所有权就可以使用值的功能，叫做 引用（references）。
