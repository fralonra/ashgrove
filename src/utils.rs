pub fn is_inputable_char(c: char) -> bool {
  let c = c as u8;
  (c >= 48 && c <= 57) || (c >= 97 && c <= 122)
}

pub fn to_lowercase(c: char) -> char {
  c.to_lowercase().collect::<Vec<_>>()[0]
}
