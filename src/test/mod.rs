
use std::fmt::{Display, Formatter};

// Define the all the arrays of char describing the language.
// Note that the first char is used as an identifier of the array.

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum TokenType {
    Id,
    IntNum,
    FloatNum,
    Keyword(KeywordType),
    Operator(OperatorType),
    Separator(SeparatorType),
    Comment(CommentType),
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        use TokenType::*;
        match self {
            Id => write!(f, "id"),
            IntNum => write!(f, "intNum"),
            FloatNum => write!(f, "floatNum"),
            Keyword(keyword) => write!(f, "{:?}", keyword),
            Operator(operator) => write!(f, "{:?}", operator),
            Separator(separator) => write!(f, "{:?}", separator),
            Comment(comment) => write!(f, "{:?}", comment),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum KeywordType {
    If,
    Then,
    Else,
    For,
    Class,
    Integer,
    Float,
    Read,
    Write,
    Return,
    Main,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum OperatorType {
    LT,
    LEq,
    NEq,
    GT,
    GEq,
    Assignment,
    Eq,
    Addition,
    Subtraction,
    Multiplication,
    Division,
    And,
    Not,
    Or,
    SR,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum SeparatorType {
    SemiColon,
    Coma,
    Period,
    Colon,
    LeftParenthesis,
    RightParenthesis,
    LeftCurlyBracket,
    RightCurlyBracket,
    LeftSquareBracket,
    RightSquareBracket,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum CommentType {
    BlockComment,
    LineComment,
}

#[allow(dead_code)]
pub fn get_small_language() -> Vec<(&'static str, TokenType)> {
    vec![
        ("for", TokenType::Keyword(KeywordType::For)),
        ("(a|f|r|o|x)+", TokenType::Id),
    ]
}

pub fn get_language() -> Vec<(&'static str, TokenType)> {
    vec![
        ("([1-9][0-9]*)|0", TokenType::IntNum),
        //("(([1-9][0-9]*)|0).(([0-9]*[1-9])|0)((e(+|-)?)(([1-9][0-9]*)|0))?", TokenType::floatNum),
        ("==", TokenType::Operator(OperatorType::Eq)),
        ("<>", TokenType::Operator(OperatorType::NEq)),
        ("<", TokenType::Operator(OperatorType::LT)),
        (">", TokenType::Operator(OperatorType::GT)),
        ("<=", TokenType::Operator(OperatorType::LEq)),
        (">=", TokenType::Operator(OperatorType::GEq)),
        ("::", TokenType::Operator(OperatorType::SR)),
        //("+", TokenType::Operator(OperatorType::Addition)),
        ("-", TokenType::Operator(OperatorType::Subtraction)),
        //("*", TokenType::Operator(OperatorType::Multiplication)),
        ("/", TokenType::Operator(OperatorType::Division)),
        ("=", TokenType::Operator(OperatorType::Assignment)),
        ("and", TokenType::Operator(OperatorType::And)),
        ("not", TokenType::Operator(OperatorType::Not)),
        ("or", TokenType::Operator(OperatorType::Or)),
        (";", TokenType::Separator(SeparatorType::SemiColon)),
        (",", TokenType::Separator(SeparatorType::Coma)),
        (".", TokenType::Separator(SeparatorType::Period)),
        (":", TokenType::Separator(SeparatorType::Colon)),
        //("(", TokenType::Separator(SeparatorType::LeftParenthesis)),
        //)")", TokenType::Separator(SeparatorType::RightParenthesis)),
        ("{", TokenType::Separator(SeparatorType::LeftCurlyBracket)),
        ("}", TokenType::Separator(SeparatorType::RightCurlyBracket)),
        //("[", TokenType::Separator(SeparatorType::LeftSquareBracket)),
        //("]", TokenType::Separator(SeparatorType::RightSquareBracket)),
        //("if", TokenType::Keyword(KeywordType::If)),
        //("then", TokenType::Keyword(KeywordType::Then)),
        //("else", TokenType::Keyword(KeywordType::Else)),
        //("for", TokenType::Keyword(KeywordType::For)),
        //("class", TokenType::Keyword(KeywordType::Class)),
        //("int", TokenType::Keyword(KeywordType::Integer)),
        //("float", TokenType::Keyword(KeywordType::Float)),
        //("get", TokenType::Keyword(KeywordType::Read)),
        //("put", TokenType::Keyword(KeywordType::Write)),
        //("return", TokenType::Keyword(KeywordType::Return)),
        //("program", TokenType::Keyword(KeywordType::Main)),
        ("[a-zA-Z][a-zA-Z0-9]*", TokenType::Id),
    ]
}
