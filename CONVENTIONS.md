* Use `Outcome` type as a return type of functions that contain multiple short-circuit operators `?`. For example: 
  ```rust
  pub fn parse_one<T: DeserializeOwned>(path: &Path) -> Outcome<T> {
    let contents = fs::read_to_string(path)?;
    let value = serde_json::from_str(&contents)?;
    Ok(value)
  }
  ```
* Use `pub fn`
