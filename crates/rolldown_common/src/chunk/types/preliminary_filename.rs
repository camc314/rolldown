use std::ops::Deref;

use crate::FilePath;

#[derive(Debug)]
/// Represents a filename that might contains hash placeholder.
pub struct PreliminaryFilename {
  /// Might contains preliminary hash
  filename: FilePath,
  /// Something like `!~{abcd}~`
  hash_placeholder: Option<String>,
}

impl PreliminaryFilename {
  pub fn new(filename: String, hash_placeholder: Option<String>) -> Self {
    Self { filename: filename.into(), hash_placeholder }
  }

  pub fn hash_placeholder(&self) -> Option<&str> {
    self.hash_placeholder.as_deref()
  }
}

impl Deref for PreliminaryFilename {
  type Target = FilePath;

  fn deref(&self) -> &Self::Target {
    &self.filename
  }
}
