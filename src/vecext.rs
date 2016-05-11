use std::ffi::{OsString, OsStr};
#[cfg(windows)]
use osstrext::OsStrExt;
#[cfg(not(windows))]
use std::os::unix::ffi::OsStrExt;

pub trait VecExt<T> {
    fn join(&self, sep: &str) -> T;
}

#[cfg(not(windows))]
impl VecExt<OsString> for Vec<OsString> {
    fn join(&self, sep: &str) -> OsString {
        use std::os::unix::ffi::OsStringExt;
        use osstrext::OsStrExt;
        let size = self.iter().fold(0, |acc, v| acc + v._len());
        let mut result = Vec::with_capacity(size + self.len());
        let mut first = true;
        for v in self {
            if first {
                first = false
            } else {
                result.extend_from_slice(sep.as_bytes());
            }
            result.extend_from_slice(OsStr::as_bytes(v.as_os_str()))
        }
        OsString::from_vec(result)
    }
}

#[cfg(windows)]
impl VecExt<OsString> for Vec<OsString> {
    fn join(&self, sep: &str) -> OsString {
        use osstrext::OsStrExt;
        let mut result = OsString::new();
        let mut first = true;
        for v in self.split(b" ").iter() {
            if first {
                first = false
            } else {
                result.push(sep.as_bytes());
            }
            result.push(v)
        }
        OsString::from_vec(result)
    }
}

#[cfg(test)]
mod test {
    use std::ffi::OsString;
    use super::VecExt;

    #[test]
    fn vec_ext_join() {
        let vec = vec![OsString::from("one"), OsString::from("two"), OsString::from("three")];
        let oss = vec.join(" ");
        assert_eq!("one two three", &oss.to_string_lossy()[..]);
    }
}
