use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt::{self, Display};

use crate::error::SassResult;
use crate::function::Function;
use crate::mixin::Mixin;
use crate::value::Value;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Symbol {
    /// .
    Period,
    /// #
    Hash,
    /// @
    At,
    /// $
    Dollar,
    /// (
    OpenParen,
    /// )
    CloseParen,
    /// {
    OpenCurlyBrace,
    /// }
    CloseCurlyBrace,
    /// [
    OpenSquareBrace,
    /// ]
    CloseSquareBrace,
    /// ,
    Comma,
    /// +
    Plus,
    /// -
    Minus,
    /// *
    Mul,
    /// /
    Div,
    /// :
    Colon,
    /// ;
    SemiColon,
    /// ~
    Tilde,
    /// >
    Gt,
    /// <
    Lt,
    /// ^
    Xor,
    /// =
    Equal,
    /// |
    BitOr,
    /// &
    BitAnd,
    /// %
    Percent,
    /// "
    DoubleQuote,
    /// '
    SingleQuote,
    /// ?
    QuestionMark,
    /// \
    BackSlash,
}

impl Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Period => write!(f, "."),
            Self::Hash => write!(f, "#"),
            Self::At => write!(f, "@"),
            Self::Dollar => write!(f, "$"),
            Self::OpenParen => write!(f, "("),
            Self::CloseParen => write!(f, ")"),
            Self::OpenCurlyBrace => write!(f, "{{"),
            Self::CloseCurlyBrace => write!(f, "}}"),
            Self::OpenSquareBrace => write!(f, "["),
            Self::CloseSquareBrace => write!(f, "]"),
            Self::Comma => write!(f, ","),
            Self::Plus => write!(f, "+"),
            Self::Minus => write!(f, "-"),
            Self::Mul => write!(f, "*"),
            Self::Div => write!(f, "/"),
            Self::Colon => write!(f, ":"),
            Self::SemiColon => write!(f, ";"),
            Self::Tilde => write!(f, "~"),
            Self::Gt => write!(f, ">"),
            Self::Lt => write!(f, "<"),
            Self::Xor => write!(f, "^"),
            Self::Equal => write!(f, "="),
            Self::BitOr => write!(f, "|"),
            Self::BitAnd => write!(f, "&"),
            Self::Percent => write!(f, "%"),
            Self::DoubleQuote => write!(f, "\""),
            Self::SingleQuote => write!(f, "'"),
            Self::QuestionMark => write!(f, "?"),
            Self::BackSlash => write!(f, "\\"),
        }
    }
}

impl TryFrom<char> for Symbol {
    type Error = &'static str;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' => Ok(Self::Period),
            '#' => Ok(Self::Hash),
            '@' => Ok(Self::At),
            '$' => Ok(Self::Dollar),
            '(' => Ok(Self::OpenParen),
            ')' => Ok(Self::CloseParen),
            '{' => Ok(Self::OpenCurlyBrace),
            '}' => Ok(Self::CloseCurlyBrace),
            '[' => Ok(Self::OpenSquareBrace),
            ']' => Ok(Self::CloseSquareBrace),
            ',' => Ok(Self::Comma),
            '+' => Ok(Self::Plus),
            '-' => Ok(Self::Minus),
            '*' => Ok(Self::Mul),
            '/' => Ok(Self::Div),
            ':' => Ok(Self::Colon),
            ';' => Ok(Self::SemiColon),
            '~' => Ok(Self::Tilde),
            '>' => Ok(Self::Gt),
            '<' => Ok(Self::Lt),
            '^' => Ok(Self::Xor),
            '=' => Ok(Self::Equal),
            '|' => Ok(Self::BitOr),
            '&' => Ok(Self::BitAnd),
            '%' => Ok(Self::Percent),
            '"' => Ok(Self::DoubleQuote),
            '\'' => Ok(Self::SingleQuote),
            '?' => Ok(Self::QuestionMark),
            '\\' => Ok(Self::BackSlash),
            _ => Err("invalid symbol"),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MediaQuery {}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Whitespace {
    Space,
    Tab,
    Newline,
    CarriageReturn,
}

impl Display for Whitespace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Space => write!(f, " "),
            Self::Tab => write!(f, "\t"),
            Self::Newline => writeln!(f),
            Self::CarriageReturn => write!(f, "\r"),
        }
    }
}

impl TryFrom<char> for Whitespace {
    type Error = &'static str;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            ' ' => Ok(Self::Space),
            '\t' => Ok(Self::Tab),
            '\n' => Ok(Self::Newline),
            '\r' => Ok(Self::CarriageReturn),
            _ => Err("invalid whitespace"),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Op {
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanEqual,
    LessThan,
    LessThanEqual,
    Plus,
    Minus,
    Mul,
    Div,
    Rem,
}

impl Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Equal => write!(f, "=="),
            Self::NotEqual => write!(f, "!="),
            Self::GreaterThanEqual => write!(f, ">="),
            Self::LessThanEqual => write!(f, "<="),
            Self::GreaterThan => write!(f, ">"),
            Self::LessThan => write!(f, "<"),
            Self::Plus => write!(f, "+"),
            Self::Minus => write!(f, "-"),
            Self::Mul => write!(f, "*"),
            Self::Div => write!(f, "/"),
            Self::Rem => write!(f, "%"),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Keyword {
    Important,
    True,
    False,
    Null,
    Default,
    From(String),
    To(String),
    Through(String),
    // Infinity,
    // NaN,
    // Auto,
    // Inherit,
    // Initial,
    // Unset,
    // Not,
    // And,
    // Or,
    // In,
}

impl Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Important => write!(f, "!important"),
            Self::True => write!(f, "true"),
            Self::False => write!(f, "false"),
            Self::Null => write!(f, "null"),
            Self::Default => write!(f, "!default"),
            // todo!(maintain casing for keywords)
            Self::From(s) => write!(f, "{}", s),
            Self::To(s) => write!(f, "{}", s),
            Self::Through(s) => write!(f, "{}", s),
            // Self::Infinity => write!(f, "Infinity"),
            // Self::NaN => write!(f, "NaN"),
            // Self::Auto => write!(f, "auto"),
            // Self::Inherit => write!(f, "inherit"),
            // Self::Initial => write!(f, "initial"),
            // Self::Unset => write!(f, "unset"),
            // Self::Not => write!(f, "not"),
            // Self::And => write!(f, "and"),
            // Self::Or => write!(f, "or"),
            // Self::In => write!(f, "in"),
        }
    }
}

impl Into<&'static str> for Keyword {
    fn into(self) -> &'static str {
        match self {
            Self::Important => "!important",
            Self::True => "true",
            Self::False => "false",
            Self::Null => "null",
            Self::Default => "!default",
            Self::From(_) => "from",
            Self::To(_) => "to",
            Self::Through(_) => "through",
            // Self::Infinity => "Infinity",
            // Self::NaN => "NaN",
            // Self::Auto => "auto",
            // Self::Inherit => "inherit",
            // Self::Initial => "initial",
            // Self::Unset => "unset",
            // Self::Not => "not",
            // Self::And => "and",
            // Self::Or => "or",
            // Self::In => "in",
        }
    }
}

impl TryFrom<&str> for Keyword {
    type Error = &'static str;

    fn try_from(kw: &str) -> Result<Self, Self::Error> {
        match kw.to_ascii_lowercase().as_str() {
            "important" => Ok(Self::Important),
            "true" => Ok(Self::True),
            "false" => Ok(Self::False),
            "null" => Ok(Self::Null),
            "default" => Ok(Self::Default),
            "from" => Ok(Self::From(kw.to_owned())),
            "to" => Ok(Self::To(kw.to_owned())),
            "through" => Ok(Self::Through(kw.to_owned())),
            // "infinity" => Ok(Self::Infinity),
            // "nan" => Ok(Self::NaN),
            // "auto" => Ok(Self::Auto),
            // "inherit" => Ok(Self::Inherit),
            // "initial" => Ok(Self::Initial),
            // "unset" => Ok(Self::Unset),
            // "not" => Ok(Self::Not),
            // "and" => Ok(Self::And),
            // "or" => Ok(Self::Or),
            // "in" => Ok(Self::In),
            _ => Err("invalid keyword"),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Pos {
    line: u32,
    column: u32,
}

impl Pos {
    pub const fn new() -> Self {
        Pos { line: 1, column: 1 }
    }

    pub const fn line(self) -> u32 {
        self.line
    }

    pub const fn column(self) -> u32 {
        self.column
    }

    pub fn newline(&mut self) {
        self.line += 1;
        self.column = 0;
    }

    pub fn next_char(&mut self) {
        self.column += 1;
    }

    pub fn chars(&mut self, num: u32) {
        self.column += num;
    }
}

impl Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "line:{} col:{}", self.line, self.column)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Scope {
    vars: HashMap<String, Value>,
    mixins: HashMap<String, Mixin>,
    functions: HashMap<String, Function>,
}

impl Scope {
    #[must_use]
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
            mixins: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    pub fn get_var(&self, v: &str) -> SassResult<&Value> {
        match self.vars.get(&v.replace('_', "-")) {
            Some(v) => Ok(v),
            None => Err("Undefined variable.".into()),
        }
    }

    pub fn insert_var(&mut self, s: &str, v: Value) -> SassResult<Option<Value>> {
        Ok(self.vars.insert(s.replace('_', "-"), v.eval()?))
    }

    pub fn var_exists(&self, v: &str) -> bool {
        self.vars.contains_key(&v.replace('_', "-"))
    }

    pub fn get_mixin(&self, v: &str) -> SassResult<&Mixin> {
        match self.mixins.get(&v.replace('_', "-")) {
            Some(v) => Ok(v),
            None => Err("Undefined mixin.".into()),
        }
    }

    pub fn insert_mixin(&mut self, s: &str, v: Mixin) -> Option<Mixin> {
        self.mixins.insert(s.replace('_', "-"), v)
    }

    pub fn mixin_exists(&self, v: &str) -> bool {
        self.mixins.contains_key(&v.replace('_', "-"))
    }

    pub fn get_fn(&self, v: &str) -> SassResult<&Function> {
        match self.functions.get(&v.replace('_', "-")) {
            Some(v) => Ok(v),
            None => Err("Undefined function.".into()),
        }
    }

    pub fn insert_fn(&mut self, s: &str, v: Function) -> Option<Function> {
        self.functions.insert(s.replace('_', "-"), v)
    }

    pub fn fn_exists(&self, v: &str) -> bool {
        self.functions.contains_key(&v.replace('_', "-"))
    }

    pub fn extend(&mut self, other: Scope) {
        self.vars.extend(other.vars);
        self.mixins.extend(other.mixins);
        self.functions.extend(other.functions);
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) enum QuoteKind {
    Single,
    Double,
    None,
}

impl Display for QuoteKind {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Single => write!(f, "'"),
            Self::Double => write!(f, "\""),
            Self::None => write!(f, ""),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ListSeparator {
    Space,
    Comma,
}

impl ListSeparator {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Space => " ",
            Self::Comma => ", ",
        }
    }

    #[allow(dead_code)]
    pub fn name(self) -> &'static str {
        match self {
            Self::Space => "space",
            Self::Comma => "comma",
        }
    }
}

impl Display for ListSeparator {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Space => write!(f, " "),
            Self::Comma => write!(f, ", "),
        }
    }
}
