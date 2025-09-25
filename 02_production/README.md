## Workspace
* From the root of the workspace `cargo run -p step_00`
* The working directory is the root of the workspace

## References
### Blog
* The associated [blog post](https://www.40tude.fr/docs/06_programmation/rust/016_errors/errors_00.html).

### GitHub
* ...

### Video
* Watch : https://www.youtube.com/watch?v=j-VQCYP7wyw





Dependencies
* Starting with `03_prod_code`
    * `cargo add derive_more --features from`

## How to
I wanted to have multiple version of the different files in the same projet and not multiple projects. This is why we have to rename the files while watching the video.

1. Open `01_prod_code.rs` 
1. Read the comments at the top
    * They ask to rename `01_prod_code.rs` to `main.rs`
        * If a `main.rs` file already exists, open it
        * At the top, its name is in the comments
        * For example, rename it `04_prod_code.rs`
        * Now you can rename `01_prod_code.rs` as `main.rs` 
    * The comments also ask to rename `mod.rs.01` in `mod.rs`
        * If `fs/mod.rs` already exists open it
        * At the top, its name is in the comments
        * For example, rename `fs/mod.rs` as `fs/mod.rs.02`
        * Now you can rename `fs/mod.rs.01` as `fs/mod.rs` 
1. `cargo run`