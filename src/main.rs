// Golden ratio everywhere :D
fn main() {
    println!("{:?}\n", fibbonacci_seq(10)) // Put your number of choice here, don't explode your computer, this is recursive.
}

fn fibbonacci (num: u128) -> u128 {
    match num {
        0 => 1,
        1 => 1,
        _ => fibbonacci(num - 1) + fibbonacci(num - 2)
    }
}

fn fibbonacci_seq (size: u128) -> Vec<u128> {
    let mut series: Vec<u128> = Vec::new();
    let mut i: u128 = 0;

    // Benchmark I guess? Not sure if accurate.
    let now = std::time::Instant::now();
    while (series.len() as u128) < size {
        series.push(fibbonacci(i));
        i += 1;
    }
    println!("\nNumbers crunched recursively: {:?}\nTime taken: {:.2?}\n", size, now.elapsed());
    series
}