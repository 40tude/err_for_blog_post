// ex200.rs
// cargo run -p experimentation --example ex200

// ! First version with error

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    // let numbers = vec!["10", "20", "89", "30"];
    let numbers = vec!["10", "20", "oops", "30"];

    let total = sum_strings(&numbers)?;
    println!("The total is: {total}");
    Ok(())
}

fn sum_strings(values: &[&str]) -> Result<i32> {
    let sum: i32 = values.iter().map(|s| s.parse::<i32>().expect(&format!("Failed to parse '{}' as integer", s))).sum();
    Ok(sum)
}

// fn sum_strings(values: &[&str]) -> Result<i32> {
//     let mut sum = 0;
//     for s in values {
//         let current_val = s.parse::<i32>();
//         sum += current_val.unwrap();
//     }
//     Ok(sum)
// }

// fn sum_strings(values: &[&str]) -> Result<i32> {
//     let sum: i32 = values
//         .iter()
//         .map(|s| s.parse::<i32>().unwrap())
//         .sum();
//     Ok(sum)
// }

// fn sum_strings(values: &[&str]) -> Result<i32> {
//     let sum: i32 = values
//         .iter()
//         .filter_map(|s| s.parse::<i32>().ok()) // drop entries that fail to parse
//         .sum();
//     Ok(sum)
// }
