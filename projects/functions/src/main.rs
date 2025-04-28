fn main() {
    print_label_measurment(5, 'h');
    let x = five();
    println!("The value of x is {x}");

    let x_plus_one = plus_one(x);
    println!("The value of x plus one is {x_plus_one}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn print_label_measurment(value: i32, unit_label: char) {
    println!("The measuremant is: {value}{unit_label}");
}