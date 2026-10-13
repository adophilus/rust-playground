#[derive(Debug)]
pub struct Error<'a> {
    pub message: &'a str,
    pub source: Option<Box<dyn std::error::Error>>,
}

impl<'a> std::fmt::Display for Error<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.message)
    }
}

impl<'a> std::error::Error for Error<'a> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|v| &**v)
    }
}
