fn sum(numbers: &[u32]) -> Option<u32> {
    let mut total: u32 = 0;
    for n in numbers {
        total = total.checked_add(*n)?;
    }
    return Some(total);
}

fn do_sum_with_print(numbers: &[u32]) {
    match sum(numbers) {
        Some(n) => println!("{:?} summary is {}", numbers, n),
        None => println!("{:?} summary is overflow!", numbers),
    }
}

fn main() {
    do_sum_with_print(&vec![
        234_u32,
        54654_u32,
        1138744389_u32,
        1138744389_u32,
        1138744389_u32,
    ]);

    do_sum_with_print(&vec![
        234_u32,
        54654_u32,
        1138744389_u32,
        1138744389_u32,
        744389_u32,
        1138744389_u32,
        1138744389_u32,
        1138744389_u32,
    ]);
}
