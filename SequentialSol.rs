fn sequential_average(salary: Vec<i32>) -> f64 {
    let min = *salary.iter().min().unwrap();
    let max = *salary.iter().max().unwrap();

    let sum: i32 = salary.iter().sum();
    
    let length = (salary.len() - 2) as f64;

    (sum - min - max) as f64/ length
}

fn main() {
    let salary = vec![1000, 2000, 3000, 4000, 5000];
    let average = sequential_average(salary);
    println!("The average excluding the minimum and maximum is: {}", average);
}