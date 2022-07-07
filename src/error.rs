#[derive(Debug)]
pub struct ParseError {
    pub(crate) error_type: ParseErrorType,
    pub(crate) reason: String,
}

#[derive(Debug)]
pub enum ParseErrorType {
    TitleNotSupported, FontNotSupported
}