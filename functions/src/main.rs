fn main() {
    print_labeled_measurement(5, 'h');

    /*
    表达式会计算出一个值，并且你将编写的大部分 Rust 代码是由表达式组成的。
    函数调用是一个表达式。
    宏调用是一个表达式。
    用大括号创建的一个新的块作用域也是一个表达式
     */
    let y = 6; // 语句 statements
    let y = {
        let x = 3;
        /*
        表达式的结尾没有分号。
        如果在表达式的结尾加上分号，它就变成了语句，而语句不会返回值。
         */
        x + 1
    }; // 表达式 expressions

    let x = five();

    let x = plus_one(x);
}

/*
A function must declare the types of its parameters.
 */
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1 // 加 ; 就会丛表达式变为语句，编译会报错
}