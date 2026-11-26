use std::cmp::Ordering;
use std::io;

use rand::prelude::*;

/*
在 Rust 中，变量默认是不可变的，
这意味着一旦我们给变量赋值，这个值就不可以再修改了。

let apples = 5; // 不可变
let mut bananas = 5; // 可变
 */
fn main() {
    println!("Guess the number!");

    /*
    调用 rand::rng 函数，得到我们将要使用的随机数生成器：它与当前执行线程局部相关，并由操作系统提供种子（seed）
    这里使用的范围表达式形式是 start..=end，包含上下边界，所以我们需要写 1..=100，来请求一个 1 到 100 之间的数
     */
    let secret_number = rand::rng().random_range(1..=100);

    // println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        /*
        ::new 那一行的 :: 语法表明 new 是 String 类型的一个 关联函数（associated function）。
        关联函数是针对某个类型实现的函数，在这个例子中是 String。
        这个 new 函数创建了一个新的空字符串。
        你会发现许多类型上都有一个 new 函数，因为这是为某种类型创建新值的常用函数名。
         */
        let mut guess = String::new();

        /*
        & 表示这个参数是一个 引用（reference），它允许多处代码访问同一处数据，而无需在内存中多次拷贝。
        引用是一个复杂的特性，Rust 的一个主要优势就是安全而简单的操纵引用。

        它像变量一样，默认是不可变的。因此，需要写成 &mut guess 来使其可变，而不是 &guess

        read_line 会将用户输入附加到传递给它的字符串中，不过它也会返回一个类型为 Result 的值。
        Result 是一种枚举类型，通常也写作 enum，它可以是多种可能状态中的一个。
        我们把每种可能的状态称为一种 枚举成员（variant）。

        Result 的成员是 Ok 和 Err，Ok 成员表示操作成功，内部包含成功时产生的值。
        Err 成员则意味着操作失败，并且 Err 中包含有关操作失败的原因或方式的信息。

        Result 类型的值，像其他类型一样，拥有定义于其实例上的方法。
        Result 的实例拥有 expect 方法。如果 Result 实例的值是 Err，expect 会导致程序崩溃，并输出当做参数传递给 expect 的信息。
        所以当 read_line 方法返回 Err，则可能是来源于底层操作系统错误的结果。
        如果 Result 实例的值是 Ok，expect 会获取 Ok 中的值并原样返回。

        如果不调用 expect，程序也能编译，不过会出现一个警告
         */
        // std::io::stdin
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        /*
        Rust 很贴心地允许我们用一个新值来 遮蔽（shadowing）之前的 guess。
        这样一来，我们就可以继续复用 guess 这个变量名，而不必被迫创建两个不同的变量，比如 guess_str 和 guess

        tring 实例上的 trim 方法会去掉字符串开头和结尾的空白字符；
        在把字符串转换为 u32 之前，我们必须先这么做，因为 u32 只能包含数值数据。
        用户必须按下 enter 才能让 read_line 返回并提交他们的猜测，这会在字符串中附加一个换行符（newline）。
        例如，如果用户输入 5 并按下 enter，guess 实际上会变成 5\n。
        （在 Windows 上，按下 enter 会得到回车加换行，也就是 \r\n。）trim 方法会去除 \n 或 \r\n，最后只留下 5。
         */
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        /*
        一个 match 表达式由 分支（arms） 构成。
        一个分支包含一个 模式（pattern）和表达式开头的值与分支模式相匹配时应该执行的代码。
        Rust 获取提供给 match 的值并挨个检查每个分支的模式。
         */
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
