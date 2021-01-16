use ascii::AsciiStr;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct ZipPath<'a>(&'a AsciiStr);

#[derive(Debug)]
pub enum ZipPathError {}

impl<'a> ZipPath<'a> {
    pub fn create_from_string(string: &'a AsciiStr) -> Result<Self, ZipPathError> {
        // need to validate this in future
        Ok(ZipPath(string))
    }

    pub fn to_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }

    pub fn len(&self) -> usize {
        self.to_bytes().len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
