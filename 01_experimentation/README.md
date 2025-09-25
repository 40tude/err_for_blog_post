
## Workspace
* From the root of the workspace `cargo run -p experimentation --example ex00`

## References
### Blog
* The associated [blog post](https://www.40tude.fr/docs/06_programmation/rust/016_errors/errors_00.html).

### GitHub
* https://github.com/40tude/err_for_blog_post

### Video
* Watch : https://www.youtube.com/watch?v=j-VQCYP7wyw



## TL;DR

* Avoid `.unwrap()`, `.expect()`, `.context()`
* In experimentation, write code to handle errors the same way you write code in production
    * See [xp_err_best_practices_2](https://github.com/40tude/xp_err_best_practices_2) 

```rust
pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

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

