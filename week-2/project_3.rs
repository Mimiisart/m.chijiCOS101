fn main() {
    let principle:f64 = 210_000.0;
    let rate:f64 = 5.0;
    let time:f64 = 3.0;
    let amount:f64 = principle * (1.0 - (rate / 100.0)) .powf(time);
    println!("the amount is {}",amount);
}