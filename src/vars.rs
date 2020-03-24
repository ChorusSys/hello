pub fn run() {
    let name = "Jess";
    let mut age = 25;
    println!("My name is {} and my age is {}", name, age);

    // constants require declaratio of datatype f64 => float64
    const PI:f64 = 3.1415;
    println!("PI = {}", PI);

    age = age + 1;
    println!("My name is {} and my age is {}", name, age);

}