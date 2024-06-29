use std::error::Error;

/// Slice a string using the char indices.
/// This caters to characters that are greater than a single byte in size.
///
/// An example problem is where a string has a length of 100 but the last char position may be 90,
/// or when an intermediate position that you are trying to select does not exist.
///
/// If the end position cannot be found, the last character position in the string will be used instead.
pub fn utf8_slice<'a>(
    string: &'a str,
    start: usize,
    end: usize,
) -> Result<&'a str, Box<dyn Error>> {
    let start_pos = match string.char_indices().nth(start) {
        Some(v) => v.0,
        None => {
            eprintln!(
                "Slice start position {} could not be found in the string",
                start
            );
            return Err("Slice start position could not be found".into());
        }
    };

    let end_pos = match string.char_indices().nth(end) {
        Some(v) => v.0,
        None => string.len(),
    };

    if end_pos < start_pos {
        eprintln!(
            "For string starting with \"{:1}\", the slice start {:2} was higher than the end {:3}",
            string[0..10].to_string(),
            start_pos,
            end_pos
        );

        return Err("This function assumes you are slicing from smallest to largest. Received largest to smallest instead.".into());
    }

    // note you may want to trim any resulting whitespace,
    // trimming may cause slight confusion depending on what you're expecting
    let sliced = &string[start_pos..end_pos];

    Ok(sliced)
}

/// Slicer handles slicing in a more flexible fashion.
pub trait Slicer {
    fn nice_slice(&self, start: usize, end: usize) -> &str;
}

impl Slicer for str {
    /// Slice a string without worrying about character byte size (not all chars are 1 byte)
    fn nice_slice<'a>(self: &'a str, start: usize, end: usize) -> &'a str {
        utf8_slice(&self, start, end).unwrap()
    }
}
