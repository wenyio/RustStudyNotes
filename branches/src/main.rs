fn main() {
    let number = 3;

    // if 期望得到一个 bool，不能是 if number
    // Rust 不会自动尝试把非布尔类型转换成布尔类型
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    /*
    if 和 else 分支的值类型是不相容的，如下就会编译报错
    let number = if condition { 5 } else { "six" };
     */
    let condition = true;
    let number = if condition { 5 } else { 6 };

    /*
    Rust 有三种循环：loop、while 和 for。
     */
    // 丛循环返回值
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    // 循环标签 loop label
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }

    // while
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    // for
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {}", element);
    }

    // for 遍历集合中的元素；rev，用于反转 range
    for number in (1..4).rev() {
        println!("{}!", number);
    }
}
