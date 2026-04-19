fn main() {
    task_1();
    task_2();
    task_3();
    task_4();
    task_5();
}

fn task_1() {
    let numbers = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let mapping: Vec<f64> = numbers.iter().map(|x| x * 3.0).collect();

    println!("{:?}", mapping);
}

fn task_2() {
    let numbers = vec![10, 15, 20, 25, 30];
    let filtering: Vec<_> = numbers.iter().filter(|&&x| x > 20).collect();

    println!("{:?}", filtering);
}

fn task_3() {
    let numbers = vec![10, 15, 20, 25, 30];
    let map_pow: Vec<_> = numbers
        .iter()
        .filter(|&&x| x % 2 == 0)
        .map(|x| *x * *x)
        .collect();

    println!("{:?}", map_pow);
}

fn task_4() {
    let numbers = vec![10, 15, 20, 25, 30];
    let fold_sum = numbers.iter().fold(0, |acc, x| acc + x);
    let fold_prod = numbers.iter().fold(1, |acc, x| acc * x);

    println!("sum : {:?}", fold_sum);
    println!("product : {:?}", fold_prod);
}

fn task_5() {
    let data = vec!["10", "20", "abc", "30", "xyz", "40"];

    let parser: Vec<i32> = data.iter().
    filter_map(|x| x.parse().ok())
    .collect();

    let filtering:Vec<&i32> = parser
    .iter()
    .filter(|&&x| x > 20)
    .collect();

    let sum = filtering
    .iter()
    .fold(0,|acc, x|acc + **x);

    println!("parse int only : {:?}",parser);
    println!("> 20 : {:?}",filtering);
    println!("sum : {:?}",sum);
}
