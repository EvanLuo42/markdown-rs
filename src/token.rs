use crate::error::{ParseError, ParseErrorType};

type ParseReturn = Result<(TokenType, String, Option<String>), ParseError>;

#[derive(PartialEq, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub content: String,
    pub link: Option<String>
}

#[derive(PartialEq, Debug)]
pub enum TokenType {
    Title1, Title2, Title3, Italic, Bold, ItalicAndBold, Image, Link, PlainText
}

impl Token {
    pub fn from(string: &str) -> Result<Token, ParseError> {
        let (token_type, content, link) = parse_string_to_token(string)?;
        let token = Token {
            token_type,
            content,
            link
        };

        Ok(token)
    }
}

fn parse_string_to_token(string: &str) -> ParseReturn {
    match &string[..1] {
        "#" => parse_title(string),
        "*" => parse_font(string),
        "[" => parse_url(string),
        _ => Ok((TokenType::PlainText, string.to_string(), None))
    }
}

fn parse_title(string: &str) -> ParseReturn {
    let begin = &string.split_whitespace().collect::<Vec<&str>>()[0];
    let content = string
        .replace("#", "")
        .replace(" ", "");

    match *begin {
        "#" => Ok((TokenType::Title1, content, None)),
        "##" => Ok((TokenType::Title2, content, None)),
        "###" => Ok((TokenType::Title3, content, None)),
        _ => {
            let err = ParseError {
                error_type: ParseErrorType::TitleNotSupported,
                reason: String::from("The given title is not supported.")
            };
            Err(err)
        }
    }
}

fn parse_font(string: &str) -> ParseReturn {
    let mut count = 0;
    let content = string.replace("*", "");

    for str in string.chars().into_iter() {
        if str == '*' {
            count = count + 1;
            continue;
        }
        break;
    }
    match count {
        1 => Ok((TokenType::Italic, content, None)),
        2 => Ok((TokenType::Bold, content, None)),
        3 => Ok((TokenType::ItalicAndBold, content, None)),
        _ => {
            let err = ParseError {
                error_type: ParseErrorType::FontNotSupported,
                reason: String::from("The given font is not supported.")
            };
            Err(err)
        }
    }
}

fn parse_url(string: &str) -> ParseReturn {
    let split_string = string
        .split("(")
        .collect::<Vec<&str>>();
    let content = split_string[0]
        .replace("[", "")
        .replace("]", "");
    let link = split_string[1].replace(")", "");

    if link.contains(".jpg") || link.contains(".png") || link.contains(".jpeg") {
        return Ok((TokenType::Image, content, Some(link)))
    }

    Ok((TokenType::Link, content, Some(link)))
}