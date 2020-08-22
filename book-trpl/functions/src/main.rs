fn main() {
    println!("Hello, world! (from main)");

    another_function();

    show_x(42);
    show_xy(42, 43);
    let v = doubleprod(10, 5);
    println!("here is val from func: {}", v)
}

fn another_function() {
    println!("Inside another_function");
}

fn show_x(x: i32) {
    println!("The value of x is: {}", x);
}

fn show_xy(x: i32, y: i32) {
    println!("The values of x,y are: {},{}", x, y);
}

fn doubleprod(x: i32, y: i32) -> i32 {
    // a statement
    let z = x * y;

    if x == y {
        // function as an expression
        let foo = {
            let d = x * x;
            d * 2
        };
        // explicit and early return
        return foo;
    }

    // an expression.  and an implicit return.
    // return z * 2;
    z * 2
}
