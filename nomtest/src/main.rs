#[macro_use]
extern crate nom;

use nom::{IResult,digit,alphanumeric,anychar};

use std::str;
use std::str::FromStr;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Equation {
    pub left: Operand,
    pub right: Operand,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Function {
    pub function: String,
    pub params: Vec<Operand>,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Operand {
    Column(String),
    Function(Function),
    Number(f64),
    Boolean(bool),
    Value(String),
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Connector {
    AND,
    OR,
}
#[derive(Debug)]
#[derive(PartialEq)]
pub enum Direction {
    ASC,
    DESC,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum NullsWhere {
    FIRST,
    LAST,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Order {
    pub operand: Operand,
    pub direction: Option<Direction>,
    pub nulls_where: Option<NullsWhere>,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Equality {
    EQ, // = ,  eq
    NEQ, // != , neq
    LT, // <,  lt
    LTE, // <=, lte
    GT, // >, gt
    GTE, // >=, gte
    IN, // IN, in
    NOT_IN, // NOT IN, not_in
    IS, // IS, is
    IS_NOT, // IS NOT, is_not
    LIKE, // LIKE, like
    ILIKE, // ILIKE case insensitive like, postgresql specific
    ST // Starts with, which will become ILIKE 'value%'
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Condition {
    pub left: Operand,
    pub equality: Equality,
    pub right: Operand,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Filter {
    pub connector: Option<Connector>,
    pub condition: Condition,
    pub sub_filters: Vec<Filter>,
}



#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Default)]
pub struct Query {
    pub from: Vec<Operand>,
    pub join: Vec<Join>,
    pub filters: Vec<Filter>,
    pub group_by: Vec<Operand>,
    pub having: Vec<Filter>,
    pub order_by: Vec<Order>,
    pub range: Option<Range>,
    pub equations: Vec<Equation>,
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Default)]
pub struct Page {
    pub page: i64,
    pub page_size: i64,
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Default)]
pub struct Limit {
    pub limit: i64,
    pub offset: Option<i64>,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Range {
    Page(Page),
    Limit(Limit),
}


#[derive(Debug)]
#[derive(PartialEq)]
pub enum JoinType {
    CROSS,
    INNER,
    OUTER,
    NATURAL,
}
#[derive(Debug)]
#[derive(PartialEq)]
pub enum Modifier {
    LEFT,
    RIGHT,
    FULL,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Join {
    pub modifier: Option<Modifier>,
    pub join_type: Option<JoinType>,
    pub table: Operand,
    pub column1: Vec<String>,
    pub column2: Vec<String>,
}



fn main() {
    println!("Hello, world!");
}

named!(column <&str>, map_res!(
        complete!(alphanumeric),
        str::from_utf8
    )
);

named!(boolean <bool>,
    alt!(tag!("true") => {|_| true} |
         tag!("false") => {|_| false}
        )
);

named!(operand <Operand>,
   alt_complete!(
        float => {|f| Operand::Number(f as f64)} |
        boolean => {|b| Operand::Boolean(b) } |
        column => {|c:&str| Operand::Column(c.to_string())}
   ) 
);

named!(equality<Equality>,
    alt!(tag!("eq") => {|_| Equality::EQ} | 
         tag!("neq") => {|_| Equality::NEQ} |
         tag!("lt") => {|_| Equality::LT} |
         tag!("lte") => {|_| Equality::LTE} |
         tag!("gt") => {|_| Equality::GT} |
         tag!("gte") => {|_| Equality::GTE} |
         tag!("in") => {|_| Equality::IN} |
         tag!("not_in") => {|_| Equality::NOT_IN} |
         tag!("is") => {|_| Equality::IS} |
         tag!("is_not") => {|_| Equality::IS_NOT} |
         tag!("like") => {|_| Equality::LIKE} |
         tag!("ilike") => {|_| Equality::ILIKE} |
         tag!("st") => {|_| Equality::ST}
    )
);

named!(condition <Condition>,
    map!(tuple!(
        column,
        tag!("="),
        equality,
        tag!("."),
        operand
    ),
    |(col,_,eq,_,op):(&str,_,Equality,_,Operand)|{
        Condition{
            left: Operand::Column(col.to_string()),
            equality: eq,
            right: op
        }
    }
    )
);


named!(unsigned_float <f64>, map_res!(
  map_res!(
    recognize!(
      alt_complete!(
        delimited!(digit, tag!("."), opt!(complete!(digit))) |
        delimited!(opt!(digit), tag!("."), complete!(digit)) |
        complete!(digit)
      )
    ),
    str::from_utf8
  ),
  FromStr::from_str
));

named!(float <f64>, map!(
  pair!(
    opt!(alt!(tag!("+") | tag!("-"))),
    unsigned_float
  ),
  |(sign, value): (Option<&[u8]>, f64)| {
    sign.and_then(|s| if s[0] == ('-' as u8) { Some(-1f64) } else { None }).unwrap_or(1f64) * value
  }
));

#[test]
fn test_boolean(){
    assert_eq!(boolean(&b"true"[..]), IResult::Done(&b""[..], true));
    assert_eq!(boolean(&b"false"[..]), IResult::Done(&b""[..], false));
}

#[test]
fn test_cond(){
    assert_eq!(condition(&b"product=eq.134"[..]), IResult::Done(&b""[..], 
        Condition{
            left: Operand::Column("product".to_string()),
            equality: Equality::EQ,
            right: Operand::Number(134f64)
          }
        ));

    assert_eq!(condition(&b"active=eq.true"[..]), IResult::Done(&b""[..], 
        Condition{
            left: Operand::Column("active".to_string()),
            equality: Equality::EQ,
            right: Operand::Boolean(true)
          }
        ));
    assert_eq!(condition(&b"price=lt.-0.3"[..]), IResult::Done(&b""[..], 
        Condition{
            left: Operand::Column("price".to_string()),
            equality: Equality::LT,
            right: Operand::Number(-0.3)
          }
        ));
    
    assert_eq!(condition(&b"name=st.John"[..]), IResult::Done(&b""[..], 
        Condition{
            left: Operand::Column("name".to_string()),
            equality: Equality::ST,
            right: Operand::Column("John".to_string())
          }
        ));
}

#[test]
fn test_equality(){
    assert_eq!(equality(&b"eq"[..]), IResult::Done(&b""[..], Equality::EQ));
    assert_eq!(equality(&b"neq"[..]), IResult::Done(&b""[..], Equality::NEQ));
    assert_eq!(equality(&b"st"[..]), IResult::Done(&b""[..], Equality::ST));
    assert_eq!(equality(&b"ilike"[..]), IResult::Done(&b""[..], Equality::ILIKE));
}



#[test]
fn test_operand() {
    assert_eq!(operand(&b"product"[..]), 
        IResult::Done(&b""[..],Operand::Column("product".to_string()))); 

    assert_eq!(operand(&b"1234"[..]), 
        IResult::Done(&b""[..],Operand::Number(1234f64))); 

    assert_eq!(operand(&b"true"[..]), 
        IResult::Done(&b""[..],Operand::Boolean(true))); 

    assert_eq!(operand(&b"false"[..]), 
        IResult::Done(&b""[..],Operand::Boolean(false))); 

    assert_eq!(operand(&b"trufalse"[..]), 
        IResult::Done(&b""[..],Operand::Column("trufalse".to_string()))); 
}

#[test]
fn test_column() {
    assert_eq!(column(&b"product"[..]), IResult::Done(&b""[..], "product"));
    //assert_eq!(column(&b"product_id"[..]), IResult::Done(&b""[..], "product_id"));
}

#[test]
fn unsigned_float_test() {
  assert_eq!(unsigned_float(&b"123.456"[..]), IResult::Done(&b""[..], 123.456));
  assert_eq!(unsigned_float(&b"0.123"[..]),   IResult::Done(&b""[..], 0.123));
  assert_eq!(unsigned_float(&b"123.0"[..]),   IResult::Done(&b""[..], 123.0));
  assert_eq!(unsigned_float(&b"123."[..]),    IResult::Done(&b""[..], 123.0));
  assert_eq!(unsigned_float(&b".123"[..]),    IResult::Done(&b""[..], 0.123));
  assert_eq!(unsigned_float(&b"123456"[..]), IResult::Done(&b""[..], 123456f64));
}

#[test]
fn float_test() {
  assert_eq!(float(&b"123.456"[..]),  IResult::Done(&b""[..], 123.456));
  assert_eq!(float(&b"+123.456"[..]), IResult::Done(&b""[..], 123.456));
  assert_eq!(float(&b"-123.456"[..]), IResult::Done(&b""[..], -123.456));
}
