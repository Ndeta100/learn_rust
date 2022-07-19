fn main() {
    let mut x = 3;
    println!("The value of x is :{}", x);
    x = 9;
    println!("The value of x is :{}", x);
    let y = 6;
    let y = y + 3;
    let y = y / 3;
    println!("The value of y is {}", y);
    let number = 4;
    let is_positive = if number > 9 { true } else { false };
    println!("The value of is posiitive is {}", is_positive);
    //count_to_ten();
    //not_zero(34);
    for_loop([1, 2, 4, 5, 6, 7, 8, 35, 5, 6, 7, 76].to_vec());
}
fn count_to_ten() {
    let mut counter = 0;
    let results = loop {
        counter += 1;
        if counter == 10 {
            break counter + 2;
        }
    };
    println!("the result is {}", results);
}

fn not_zero(x: i32) {
    let mut number = x;
    while number != 0 {
        println!("{}", number);
        number -= 1
    }
    println!("LIFTOFF");
}
fn for_loop(x: Vec<i32>) {
    let mut vec = x;
    for element in vec.iter() {
        println!("{}", element);
    }
}
