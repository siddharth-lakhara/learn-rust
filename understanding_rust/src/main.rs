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
}

fn fn_example(x: i32, y: char) -> i32 {
    x+1
}
