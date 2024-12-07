fn main() {
    // an array of numbers
    let numbers = [1, 2, 3, 4, 5];
    println!("Original array = {:?}", numbers);

    // create a slice of 2nd and 3rd element
    let slice1 = &numbers[1..3];
    println!("2nd and 3rd elements sliced = {:?}", slice1);

    // omit the end index
    let slice2 = &numbers[..3];

    //this means the slice starts from index 0 and goes up to index 3(exclusivly)
    println!("index 0 to index 3 slices = {:?}",slice2);

    //omit the end index
    let slice3 = &numbers[2..];
    //this means the start from 2 and ends at 5
    println!("index 2 to index 5 slices = {:?}",slice3);

    //omit the start index and the end index
    //reference the whole array
    let slice4 = &numbers[..];
    //this meand the slice starts at 0 and ends at 5
    println!("index 0 to index 5 slices = {:?}",slice4);
}
