mod token;
mod error;

#[cfg(test)]
mod tests {
    use crate::token::{Token, TokenType};

    #[test]
    fn title() {
        let title1 = "# abc";
        let title2 = "## abc";
        let title3 = "### abc";

        // Title1
        assert_eq!(Token::from(title1).unwrap().token_type, TokenType::Title1);
        assert_eq!(Token::from(title1).unwrap().content, "abc");

        // Title1
        assert_eq!(Token::from(title2).unwrap().token_type, TokenType::Title2);
        assert_eq!(Token::from(title2).unwrap().content, "abc");

        // Title1
        assert_eq!(Token::from(title3).unwrap().token_type, TokenType::Title3);
        assert_eq!(Token::from(title3).unwrap().content, "abc");
    }

    #[test]
    fn font() {
        let italic = "*abc*";
        let bold = "**abc**";
        let italic_and_bold = "***abc***";

        // Italic
        assert_eq!(Token::from(italic).unwrap().token_type, TokenType::Italic);
        assert_eq!(Token::from(italic).unwrap().content, "abc");

        // Bold
        assert_eq!(Token::from(bold).unwrap().token_type, TokenType::Bold);
        assert_eq!(Token::from(bold).unwrap().content, "abc");

        // Italic
        assert_eq!(Token::from(italic_and_bold).unwrap().token_type, TokenType::ItalicAndBold);
        assert_eq!(Token::from(italic_and_bold).unwrap().content, "abc");
    }

    #[test]
    fn url() {
        let link = "[abc](https://abc.com/)";
        let image = "[abc](https://abc.com/a.jpeg)";

        // Link
        assert_eq!(Token::from(link).unwrap().token_type, TokenType::Link);
        assert_eq!(Token::from(link).unwrap().content, "abc");

        // Image
        assert_eq!(Token::from(image).unwrap().token_type, TokenType::Image);
        assert_eq!(Token::from(image).unwrap().content, "abc");
    }
}
