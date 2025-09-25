
## Workspace
* From the root of the workspace `cargo run -p experimentation --example ex00`

## References
### Blog
* The associated [blog post](https://www.40tude.fr/docs/06_programmation/rust/016_errors/errors_00.html).

### GitHub
* ...

### Video
* Watch : https://www.youtube.com/watch?v=j-VQCYP7wyw


## Test/POC/Experiment Code
* See `examples/`
* `cargo run --example ex00`
* No dependencies

## TL;DR

* Avoid `.unwrap()`, `.expect()`, `.context()`
* In experimentation, write code to handle errors the same way you write code in production
    * See [xp_err_best_practices_2](https://github.com/40tude/xp_err_best_practices_2) 
*   

```rust
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    ... 
    my_func(...)?; 
    Ok(())
}

// In my_func()
other_func()?;
.map_err(|_| "Error while reading dir.")?
return Err("Cannot list empty folder.".into());
Ok(my_values)
```

