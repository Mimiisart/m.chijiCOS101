fn main() {
    let principle = 520_000_000.0;
    let rate = 10.0;
    let time = 5.0;
    let amount = principle * ((1.0 + (rate/100.0)) as f64).powf(time);
    println!("amount is {}", amount);

    let compound_interest = amount - principle;
    println!("compound interested {}", compound_interest);
}