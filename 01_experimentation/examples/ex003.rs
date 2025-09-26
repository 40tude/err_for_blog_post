// ex003.rs
// cargo run -p experimentation --example ex003

// ! error as &str

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let numbers = vec!["10", "20", "oops", "30"];

    let total = sum_strings(&numbers)?;
    println!("The total is: {total}");
    Ok(())
}

fn sum_strings(values: &[&str]) -> Result<i32> {
    let sum: i32 = values
        .iter()
        .filter_map(|s| s.parse::<i32>().ok()) // drop entries that fail to parse
        .sum();
    Ok(sum)
}
