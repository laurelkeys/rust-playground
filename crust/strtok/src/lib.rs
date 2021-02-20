// @Note: Rust differs between three types of variance:
// covariance, contravariance and invariance.
//
// See https://doc.rust-lang.org/nomicon/subtyping.html
// See https://doc.rust-lang.org/reference/subtyping.html

// @Todo: continue from https://youtu.be/iVYWDIW71jk?t=3186

// pub fn strtok<'a>(s: &'a mut &'a str, delim: char) -> &'a str {
pub fn strtok<'a, 's>(s: &'a mut &'s str, delim: char) -> &'a str {
    if let Some(i) = s.find(delim) {
        let token = &s[..i];
        *s = &s[(i + delim.len_utf8())..];
        token
    } else {
        let token = *s;
        *s = "";
        token
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut x = "hello world";
        let hello = strtok(&mut x, ' ');
        assert_eq!(hello, "hello");
        assert_eq!(x, "world");
    }
}
