// ex100.rs
// cargo run -p experimentation --example ex100

// ! First version with error

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let my_bmi = bmi(70.0, 0.0)?;
    println!("BMI: {my_bmi:.2}");
    Ok(()) // we must return a Result whose value here is Ok(())
}

fn bmi(w: f64, h: f64) -> Result<f64> {
    if h.abs() < f64::EPSILON {
        // .into() convert &'static str into Box<dyn std::error::Error>
        return Err("Height cannot be 0.0".into());
    }
    Ok(w / (h * h))
}
