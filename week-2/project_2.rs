// Sum and average project 

fn main () {
    // Quantity of each item
    let toshiba_quantity = 2;
    let mac_quantity = 1;
    let hp_quantity = 3;
    let dell_quantity = 3;
    let acer_quantity = 1;

    // Price
    let toshiba_price = 450000;
    let mac = 1500000;
    let hp = 750000;
    let dell = 2850000;
    let acer = 250000;

    let sum = (toshiba_quantity * toshiba_price) + (mac_quantity * mac) + (hp_quantity * hp) + (dell_quantity * dell) + (acer_quantity * acer);
    println! ("the sum is {}",sum);
    let quantity = toshiba_quantity + mac_quantity + hp_quantity + dell_quantity + acer_quantity;
    let average = sum / quantity;
    println! ("the average is {}",average);
}