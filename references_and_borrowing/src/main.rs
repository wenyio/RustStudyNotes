/*
引用（reference）有点像指针，因为它是一个地址，我们可以沿着它访问存储在该地址中的数据，而这些数据归其他变量所有。
与指针不同，引用在其生命周期内保证会指向某个特定类型的有效值。

fn main() {
    let s1 = String::from("hello");
    let len = calculate_length_ref(&s1);
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

与使用 & 引用相反的操作是 解引用（dereferencing），它使用解引用运算符 * 实现

将创建一个引用的行为称为 借用（borrowing）。正如现实生活中，如果一个人拥有某样东西，你可以从他那里借来。当你使用完后，必须还回去。因为我们并不拥有它的所有权。
不能修改借用的变量：
fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world"); // error
}
正如变量默认是不可变的，引用默认也是不可变的。我们不允许通过引用修改它指向的值。

可变引用
fn main() {
    let mut s = String::from("hello");
    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

可变引用有一个很大的限制：
如果你有一个对该变量的可变引用，你就不能再创建对该变量的引用。
这些尝试创建两个 s 的可变引用的代码会失败：
let mut s = String::from("hello");
let r1 = &mut s; // first mutable borrow
let r2 = &mut 2; // second mutable borrow

不能在同一时间多次以可变方式借用 s。第一个可变借用在 r1 中，并且必须持续到它在 println! 中被使用；
但在这个可变引用被创建和被使用之间，我们又尝试在 r2 中创建另一个可变引用，它借用的是和 r1 相同的数据。

这一限制让可变性以一种受到严格控制的方式出现，从而防止在同一时间对同一数据存在多个可变引用。大多数语言都允许你随时修改变量。
这个限制的好处是 Rust 可以在编译时防止数据竞争。数据竞争（data race）类似于竞态条件，它会在以下三种行为同时发生时出现：
- 两个或更多指针同时访问同一数据。
- 至少有一个指针被用来写入数据。
- 没有同步数据访问的机制。
数据竞争会导致未定义行为，难以在运行时追踪，并且难以诊断和修复；Rust 通过拒绝编译存在数据竞争的代码来避免此问题！

一如既往，可以使用大括号来创建一个新的作用域，以允许拥有多个可变引用，只是不能同时拥有：
let mut s = String::from("hello");
{
    let r1 = &mut s;
}
let r2 = &mut s;

Rust 在同时使用可变与不可变引用时也强制采用类似的规则。这些代码会导致一个错误：
let mut s = String::from("hello");
let r1 = &s; // 没问题
let r2 = &s; // 没问题
let r3 = &mut s; // 大问题
也不能在拥有不可变引用的同时拥有可变引用
不可变引用的借用者可不希望在借用时值会突然发生改变！然而，多个不可变引用是可以的，因为没有哪个只能读取数据的引用者能够影响其他引用者读取到的数据。

注意一个引用的作用域从声明的地方开始一直持续到最后一次使用为止。例如，因为最后一次使用不可变引用的位置在 println!，它发生在声明可变引用之前，所以如下代码是可以编译的：
let mut s = String::from("hello");
let r1 = &s; // 没问题
let r2 = &s; // 没问题
println!("{r1} and {r2}");
// 此位置之后 r1 和 r2 不再使用

let r3 = &mut s; // 没问题
println!("{r3}");
不可变引用 r1 和 r2 的作用域在 println! 最后一次使用之后结束，这发生在可变引用 r3 被创建之前。因为它们的作用域没有重叠，所以代码是可以编译的。编译器可以在作用域结束之前判断不再使用的引用。

悬垂引用
在带有指针的语言中，如果释放了一块内存，却保留了指向它的指针，就很容易错误地制造出一个悬垂指针（dangling pointer）：这个指针指向的内存位置可能已经被分配作其他用途。
相比之下，在 Rust 中，编译器保证引用永远不会变成悬垂引用：如果你持有某些数据的引用，编译器会确保这些数据不会在它们的引用之前离开作用域。

fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String { // dangle 返回一个字符串的引用

    let s = String::from("hello"); // s 是一个新字符串

    &s // 返回字符串 s 的引用
} // 这里 s 离开作用域并被丢弃。其内存被释放。
  // 危险！

这里的解决方法是直接返回 String：
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}

- 在任意给定时间，要么只能有一个可变引用，要么只能有多个不可变引用。
- 引用必须总是有效的。
 */
fn main() {
    let mut s = String::from("hello");
    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}