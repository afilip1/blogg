pub trait OsStrExt {
    fn into_string(&self) -> String;
}

impl OsStrExt for std::ffi::OsStr {
    fn into_string(&self) -> String {
        self.to_str().unwrap().to_owned()
    }
}
