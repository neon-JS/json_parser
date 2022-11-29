use std::fmt;
use std::fmt::Formatter;

pub enum JsonParserError {
    InvalidArrayOpeningToken(char),
    InvalidArrayProperty(Box<JsonParserError>),

    InvalidBoolToken,

    InvalidNullToken,

    InvalidNumberFormat(String),
    InvalidNumberToken(char),
    MultipleNumberDecimalPoints,
    MultipleNumberExponentSigns,

    DuplicateKey(String),
    InvalidObjectKey(Box<JsonParserError>),
    InvalidObjectKeyValueSeparatorToken(char),
    InvalidObjectOpeningToken(char),
    InvalidObjectProperty(Box<JsonParserError>),

    InvalidStringOpeningToken(char),

    InvalidEscapeSequenceOpeningToken(char),
    InvalidEscapeSequenceToken(char),
    InvalidEscapeSequence,

    UnexpectedEndOfData,

    UnknownToken(char),
}

impl fmt::Display for JsonParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            JsonParserError::InvalidArrayOpeningToken(token)
            => write!(f, "Could not parse array: Invalid opening token. Expected: [[], Got: '{}'", token),
            JsonParserError::InvalidArrayProperty(inner)
            => write!(f, "Could not parse array: Invalid property. Inner: {}", inner),

            JsonParserError::InvalidBoolToken
            => write!(f, "Could not parse bool: Invalid token. Expected: (true|false)"),

            JsonParserError::InvalidNullToken
            => write!(f, "Could not parse null: Invalid token. Expected: (null)"),

            JsonParserError::InvalidNumberFormat(failed)
            => write!(f, "Could not parse number: Invalid number format. Got: '{}'", failed),
            JsonParserError::InvalidNumberToken(token)
            => write!(f, "Could not parse number: Invalid token. Expected: [0-9\\-eE]. Got: '{}'", token),
            JsonParserError::MultipleNumberDecimalPoints
            => write!(f, "Could not parse number: Contains multiple decimal points [.]"),
            JsonParserError::MultipleNumberExponentSigns
            => write!(f, "Could not parse number: Contains multiple exponent signs [eE]"),

            JsonParserError::DuplicateKey(key)
            => write!(f, "Could not parse object: Contains duplicate key. Got: '{}'", key),
            JsonParserError::InvalidObjectKey(inner)
            => write!(f, "Could not parse object: Contains invalid key. Inner: {}", inner),
            JsonParserError::InvalidObjectKeyValueSeparatorToken(token)
            => write!(f, "Could not parse object: Invalid key-value separator. Expected: [:], Got: '{}'", token),
            JsonParserError::InvalidObjectOpeningToken(token)
            => write!(f, "Could not parse object: Invalid opening token. Expected: [{{]. Got: '{}'", token),
            JsonParserError::InvalidObjectProperty(inner)
            => write!(f, "Could not parse object: Invalid property. Inner: {}", inner),

            JsonParserError::InvalidStringOpeningToken(token)
            => write!(f, "Could not parse string: Invalid opening token. Expected: [\"]. Got: '{}'", token),

            JsonParserError::InvalidEscapeSequenceOpeningToken(token)
            => write!(f, "Could not parse string: Invalid escape sequence token. Expected: [\\]. Got '{}'", token),
            JsonParserError::InvalidEscapeSequenceToken(token)
            => write!(f, "Could not parse string: Invalid escape sequence type. Expected: [\\/\"bfnrtu]. Got '{}'", token),
            JsonParserError::InvalidEscapeSequence
            => write!(f, "Could not parse string: Invalid escape sequence."),

            JsonParserError::UnexpectedEndOfData
            => write!(f, "Could not parse JSON. Unexpected end of data."),

            JsonParserError::UnknownToken(token)
            => write!(f, "Could not parse JSON. Unexpected token. Got: '{}'", token)
        }
    }
}

impl fmt::Debug for JsonParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        return write!(f, "{}", self);
    }
}