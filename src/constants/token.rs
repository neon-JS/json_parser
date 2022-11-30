/* Escape controls */
pub const ESC_SEQUENCE_CHARACTER: char = '\\';
pub const ESC_CARRIAGE_RETURN: char = 'r';
pub const ESC_REVERSE_SOLIDUS: char = '\\';
pub const ESC_QUOTATION_MARK: char = '"';
pub const ESC_TABULATION: char = 't';
pub const ESC_BACKSPACE: char = 'b';
pub const ESC_FORM_FEED: char = 'f';
pub const ESC_LINE_FEED: char = 'n';
pub const ESC_SOLIDUS: char = '/';

/* Unicode sequence */
pub const ESC_UNICODE_SEQUENCE_CHARACTER: char = 'u';
pub const ESC_UNICODE_IDENTIFIER: &str = "\\u";

/* Number */
pub const NUM_PLUS: char = '+';
pub const NUM_MINUS: char = '-';
pub const NUM_DECIMAL_POINT: char = '.';
pub const NUM_EXPONENT_SIGN_LOWERCASE: char = 'e';
pub const NUM_EXPONENT_SIGN_UPPERCASE: char = 'E';

pub const NUM_ZERO: char = '0';
pub const NUM_ONE: char = '1';
pub const NUM_TWO: char = '2';
pub const NUM_THREE: char = '3';
pub const NUM_FOUR: char = '4';
pub const NUM_FIVE: char = '5';
pub const NUM_SIX: char = '6';
pub const NUM_SEVEN: char = '7';
pub const NUM_EIGHT: char = '8';
pub const NUM_NINE: char = '9';

pub const NUM_HEX_A_CAPITAL: char = 'A';
pub const NUM_HEX_F_CAPITAL: char = 'F';
pub const NUM_HEX_A_SMALL: char = 'a';
pub const NUM_HEX_F_SMALL: char = 'F';

/* Array */
pub const ARRAY_BRACKET_OPEN: char = '[';
pub const ARRAY_BRACKET_CLOSE: char = ']';
pub const ARRAY_PROPERTY_SEPARATOR: char = ',';

/* Bool */
pub const TOKEN_BOOL_TRUE: &str = "true";
pub const TOKEN_BOOL_FALSE: &str = "false";
pub const IDENTIFIER_BOOL_TRUE: char = 't';
pub const IDENTIFIER_BOOL_FALSE: char = 'f';

/* Null */
pub const TOKEN_NULL: &str = "null";
pub const IDENTIFIER_NULL: char = 'n';

/* Object */
pub const OBJECT_BRACKET_OPEN: char = '{';
pub const OBJECT_BRACKET_CLOSE: char = '}';
pub const OBJECT_PROPERTY_SEPARATOR: char = ',';
pub const OBJECT_KEY_VALUE_SEPARATOR: char = ':';

/* String */
pub const STRING_START: char = '"';
pub const STRING_END: char = '"';

/* JsonStream */
pub const WHITESPACE: char = ' ';
pub const TABULATION: char = '\t';
pub const CARRIAGE_RETURN: char = '\r';
pub const LINE_FEED: char = '\n';