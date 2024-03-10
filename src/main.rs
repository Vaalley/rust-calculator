fn main() {
    let mut calc_type = String::new();
    loop {
        // Greet user and ask for type of calculation
        println!("                                                   |--------------------------------------------------|");
        println!("Hello, what type of calculation do you want to do? |    +    |      -      |       *        |    /    |");
        println!("                                                   |addition | subtraction | multiplication | division|");
        println!("                                                   |--------------------------------------------------|");

        std::io::stdin()
            .read_line(&mut calc_type)
            .expect("Failed to read line");

        if calc_type.trim() != "+"
            && calc_type.trim() != "-"
            && calc_type.trim() != "*"
            && calc_type.trim() != "/"
        {
            println!("Invalid calculation type");
            continue;
        }
        break;
    }
    // Ask for two numbers
    println!("Now please enter two numbers separated by a space:");

    let mut nums = String::new();

    std::io::stdin()
        .read_line(&mut nums)
        .expect("Failed to read line");

    let num1: f32 = nums.split_whitespace().next().unwrap().parse().unwrap();
    let num2: f32 = nums.split_whitespace().nth(1).unwrap().parse().unwrap();

    // Perform calculation
    let result = match calc_type.trim() {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => num1 / num2,
        _ => panic!("Invalid calculation type"),
    };

    println!("{} {} {} = {}", num1, calc_type.trim(), num2, result);

    // Ask if user wants to do another calculation
    println!("Do you want to do another calculation? (y/n)");

    let mut again = String::new();

    std::io::stdin()
        .read_line(&mut again)
        .expect("Failed to read line");

    if again.trim() == "y" {
        main();
    }
}
