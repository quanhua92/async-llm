use serde::de::DeserializeOwned;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

/// Reads and deserializes a JSON file into a specified type.
///
/// # Arguments
/// * `path` - The path to the JSON file. Can be any type that implements `AsRef<Path>`.
///
/// # Returns
/// * `Ok(T)` - If the file is successfully read and deserialized.
/// * `Err(Box<dyn Error>)` - If an error occurs during file reading or deserialization.
///
/// # Example
/// ```
/// #[derive(serde::Deserialize)]
/// struct Info {
///     model_name: String,
///     provider_name: String,
///     test_name: String,
/// }
///
/// let info: Info = read_json("info.json")?;
/// println!("{:?}", info);
/// ```
pub fn read_json<P: AsRef<Path>, T: DeserializeOwned>(path: P) -> Result<T, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let value = serde_json::from_reader(reader)?;
    Ok(value)
}
