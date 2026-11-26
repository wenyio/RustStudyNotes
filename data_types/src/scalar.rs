/*
Rust 有四种基本的标量类型：整型、浮点型、布尔类型和字符类型。

整形
长度	   有符号   无符号
8-bit	i8	    u8
16-bit	i16    	u16
32-bit	i32	    u32
64-bit	i64  	u64
128-bit	i128	u128
架构相关	isize	usize

数字字面值	     例子
Decimal（十进制）	98_222
Hex（十六进制）	0xff
Octal（八进制）	0o77
Binary（二进制）	0b1111_0000
Byte（字节字面值，仅限 u8）	b'A'

整型溢出
假设你有一个 u8 类型的变量，它可以保存 0 到 255 之间的值。
如果你试图把它改成超出该范围的值，比如 256，就会发生 整型溢出（integer overflow），并可能导致两种行为之一。
当你在 debug 模式下编译时，Rust 会加入整型溢出的检查，并在发生这种情况时让程序在运行时 panic。
Rust 用 panicking 这个术语表示程序因错误而退出；

当你使用 --release flag 在 release 模式下编译时，Rust 不会加入会导致 panic 的整型溢出检查。
相反，如果发生溢出，Rust 会执行一种叫做 two’s complement wrapping 的行为。
简而言之，超过该类型最大值的数会“回绕”到该类型所能表示的最小值。
对于 u8 来说，256 会变成 0，257 会变成 1，依此类推。
程序不会 panic，但变量得到的值很可能不是你原本期望的值。
依赖整型溢出的回绕行为通常被认为是一种错误。

为了显式地处理溢出的可能性，可以使用这几类标准库提供的原始数字类型方法：

所有编译模式下都可以使用 wrapping_* 方法进行 wrapping，如 wrapping_add
如果 checked_* 方法发生溢出，则返回 None 值
用 overflowing_* 方法返回值和一个布尔值，表示是否出现溢出
用 saturating_* 方法在值的最小值或最大值处进行饱和处理

浮点型
Rust 也有两个原生的 浮点数（floating-point numbers）类型，它们是带小数点的数字。
Rust 的浮点数类型是 f32 和 f64，分别占 32 位和 64 位。
默认类型是 f64，因为在现代 CPU 中，它与 f32 速度几乎一样，不过精度更高。
所有的浮点型都是有符号的。

浮点数按照 IEEE-754 标准表示
 */
fn main() {
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    /*
    Rust 中的所有数字类型都支持基本数学运算：加法、减法、乘法、除法和取余。
    整数除法会向零舍入到最接近的整数。
     */
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // 结果为 -1

    // remainder
    let remainder = 43 % 5;

    /*
    Rust 中的布尔类型有两个可能的值：true 和 false。
    Rust 中的布尔类型使用 bool 表示。
     */
    let t = true;
    let f: bool = false;

    /*
    Rust 的 char 类型是语言中最原始的字母类型。
    我们使用单引号来表示 char 字面值，而字符串字面值使用的是双引号。
    Rust 的 char 类型大小为 4 个字节，并表示一个 Unicode 标量值（Unicode Scalar Value），这意味着它所能表示的内容远不止 ASCII。
    带重音符号的字母，中文、日文、韩文字符，emoji，以及零宽空格，都是 Rust 中合法的 char 值。
    Unicode 标量值的范围包括 U+0000 到 U+D7FF，以及 U+E000 到 U+10FFFF。
    不过，“字符”并不是 Unicode 中一个严格对应的概念，因此你直觉上认为的“字符”未必和 Rust 中的 char 一一对应
     */
    let c = 'z';
    let z: char = 'Z';
    let heart_eyed_cat = '😻';

    /*
    复合类型（compound types）可以把多个值组合成一个类型。
    Rust 有两种原生的复合类型：元组（tuple）和数组（array）。
    元组类型
    元组是一种将多个不同类型的值组合成一个复合类型的通用方式。
    元组长度固定：一旦声明，它的大小就不能增长或缩小。

    不带任何值的元组有一个特殊名字，叫做 单元（unit）。
    这种值以及其对应的类型都写作 ()，表示空值或空的返回类型。
    如果一个表达式没有返回任何其他值，它就会隐式返回单元值。
     */
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    /*
    变量 tup 绑定到整个元组上，因为元组本身会被视为一个单独的复合值。为了从元组中取出单个值，我们可以使用模式匹配（pattern matching）来解构（destructure）元组
     */
    let (x, y, z) = tup; // 解构（destructuring）
    /*
    也可以使用点号（.）后跟值的索引来直接访问所需的元组元素
     */
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    /*
    数组类型
    另一种包含多个值的方式是 数组（array）。
    和元组不同，数组中的每个元素都必须具有相同类型。
    Rust 中的数组也不同于某些其他语言中的数组：Rust 的数组长度是固定的。

    ，数组不如 vector 类型灵活。vector 是标准库提供的一种类似数组的集合类型，它 允许 长度增长或缩小。
     */
    let a = [1, 2, 3, 4, 5]; // let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5]; // let a = [3, 3, 3, 3, 3];
    let first = a[0];
    let second = a[1];
    /*
    当你明确知道元素个数不会变化时，数组就更有用。
    例如，如果你在程序中使用月份名称，那么你大概会选择数组而不是 vector，因为你知道它始终只有 12 个元素。
     */
    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];
}