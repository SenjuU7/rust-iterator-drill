fn main() {
    println!("\n=== TASK 1 ===");
    task_1();

    println!("\n=== TASK 2 ===");
    task_2();

    println!("\n=== TASK 3 ===");
    task_3();

    println!("\n=== TASK 4 ===");
    task_4();

    println!("\n=== TASK 5 ===");
    task_5();
}

fn task_1() {
    let numbers = vec![1.0, 2.0, 3.0, 4.0, 5.0];

    let tripled: Vec<f64> = numbers
        .iter()
        .map(|x| x * 3.0)
        .collect();

    println!("input   : {:?}", numbers);
    println!("tripled : {:?}\n", tripled);
}

fn task_2() {
    let numbers = vec![10, 15, 20, 25, 30];

    let greater_than_20: Vec<i32> = numbers
        .iter()
        .copied()
        .filter(|x| *x > 20)
        .collect();

    println!("input   : {:?}", numbers);
    println!("> 20    : {:?}\n", greater_than_20);
}

fn task_3() {
    let numbers = vec![10, 15, 20, 25, 30];

    let even_squared: Vec<i32> = numbers
        .iter()
        .copied()
        .filter(|x| x % 2 == 0)
        .map(|x| x * x)
        .collect();

    println!("input         : {:?}", numbers);
    println!("even squared  : {:?}\n", even_squared);
}

fn task_4() {
    let numbers = vec![10, 15, 20, 25, 30];

    let sum: i32 = numbers.iter().sum();
    let product: i32 = numbers.iter().product();

    println!("input   : {:?}", numbers);
    println!("sum     : {}", sum);
    println!("product : {}\n", product);
}

fn task_5() {
    let data = vec!["10", "20", "abc", "30", "xyz", "40"];

    let parsed: Vec<i32> = data
        .iter()
        .filter_map(|s| s.parse().ok())
        .collect();

    let filtered: Vec<i32> = parsed
        .iter()
        .copied()
        .filter(|x| *x > 20)
        .collect();

    let sum: i32 = filtered.iter().sum();

    println!("raw data : {:?}", data);
    println!("parsed   : {:?}", parsed);
    println!("> 20     : {:?}", filtered);
    println!("sum      : {}\n", sum);
}