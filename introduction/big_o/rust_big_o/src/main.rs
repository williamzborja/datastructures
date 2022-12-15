const N: usize = 15; // size of array
fn main() {
    // given and array of n size
    let numbers: [i32; N] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    
    print(&numbers); // time: O(n)  space: O(1)
}

fn print(nums: &[i32; N]) {
    for num in nums {  //  n * 1 = n;
        print!("{} ", num); // n(infinity) * 1 = n(infinity)
    }
    println!("") // n + 1 = n
}
