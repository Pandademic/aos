#[allow(dead_code)]
pub struct Ffs<'a> {
    files: [&'a str;33]
}

impl Ffs<'_> {
    pub fn new() -> Self {
        Ffs { files: ["";33] }
    }
}
