use std::io::{self, BufRead};

/*
 * Complete the 'countResponseTimeRegressions' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER_ARRAY responseTimes as parameter.
 */

fn countResponseTimeRegressions(responseTimes: &[i32]) -> i32 {
    // Write your code here
    //println!("{:?}", &responseTimes);

    let mut divisor: u64 = 0;
    let mut counter = 0;
    let mut subtotal: u64 = 0;

    for entry in responseTimes {
		subtotal += *entry as u64;
		divisor += 1;
		if *entry as u64 > (subtotal / divisor) {
			counter += 1;
	}}
    counter
}

fn main() {
    println!("\x1b[2J\x1b[H\x1b[3J");
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let responseTimes_count = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    let mut responseTimes: Vec<i32> = Vec::with_capacity(responseTimes_count as usize);

    for _ in 0..responseTimes_count {
        let responseTimes_item = stdin_iterator
            .next()
            .unwrap()
            .unwrap()
            .trim()
            .parse::<i32>()
            .unwrap();
        responseTimes.push(responseTimes_item);
    }

    let result = countResponseTimeRegressions(&responseTimes);

    println!("{}", result);
}
