use rayon::prelude::*;

fn parallel_average(salary: Vec<i32>) -> f64 {
    let (min, max) = salary.par_iter().fold(
        || (i32::MAX, i32::MIN),
        |(min, max), &salary| (min.min(salary), max.max(salary)),
    ).reduce(
        || (i32::MAX, i32::MIN),
        |(min1, max1), (min2, max2)| (min1.min(min2), max1.max(max2))
    );
    let total_sum: i32 = salary.par_iter().sum();

    let adjusted_sum = total_sum - min - max;
    let adjusted_len = (salary.len() - 2) as f64;

    adjusted_sum as f64 / adjusted_len
}

fn main() {
    let salary = vec![1000, 2000, 3000, 4000, 5000];
    let average = parallel_average(salary);
    println!("The average excluding the minimum and maximum is:", average);
}