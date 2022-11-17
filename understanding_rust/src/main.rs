fn main() {
    // immutable var
    let var1 = 5;
    //error:
    // var1 = 6;

    // mutable variable
    let mut var2;
    var2 = 50;
    var2 = 75;

    println!("Var1 = {var1}, var2 = {var2}");


    // tuple. Each value can be different type. 
    // Immutable
    let tup = (2, 4.5, 'a');

    // array type
    let arr = [1,2,3,4,5];
    let arr2 = [3;5]; // [3,3,3,3,3];

    fn_example(20, 'y');

    // 91
    let expression_var = {
        let x = 90;
        x+1
    }

    // if statement
    let condition = true;
    if (condition) {
        println!("Condition is true");
    } else {
        println!("Condition is false");
    }

    // loop
    // label always starts with a single quote (')
    'loop_label loop {
        println!("Inside loop, breaking it now");
        break 'loop_label;
    }

    // while loop
    let mut count = 3;
    while count != 0 {
        println!("{count}!");
        count -= 1;
    }
    println!("LIFTOFF!");

    // for loop
    let arr = [10, 20, 30, 40, 50];
    for element in arr {
        println!("the value is: {element}");
    }
}

fn fn_example(x: i32, y: char) -> i32 {
    x+1
}
