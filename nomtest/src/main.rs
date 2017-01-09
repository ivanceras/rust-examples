#[macro_use]
extern crate nom;

use nom::{IResult,digit,alphanumeric,anychar,is_alphanumeric};

use std::str;
use std::str::FromStr;

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub struct Equation {
    pub left: Operand,
    pub right: Operand,
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub struct Function {
    pub function: String,
    pub params: Vec<Operand>,
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub enum Operand {
    Column(String),
    Function(Function),
    Number(f64),
    Boolean(bool),
    Value(String),
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub enum Connector {
    AND,
    OR,
}
#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub enum Direction {
    ASC,
    DESC,
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub enum NullsWhere {
    FIRST,
    LAST,
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub struct Order {
    pub operand: Operand,
    pub direction: Option<Direction>,
    pub nulls_where: Option<NullsWhere>,
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
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
#[derive(Clone)]
pub struct Condition {
    pub left: Operand,
    pub equality: Equality,
    pub right: Operand,
}


#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
enum Param{
    Condition(Condition),
    Equation(Equation)
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub struct Filter {
    pub condition: Condition,
    /// the filter's condition will use this connector to connect to the rest of the filters (sub_filters)
    pub connector: Option<Connector>,
    pub sub_filters: Vec<Filter>,
}



#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Default)]
#[derive(Clone)]
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
#[derive(Clone)]
pub struct Page {
    pub page: i64,
    pub page_size: i64,
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Default)]
#[derive(Clone)]
pub struct Limit {
    pub limit: i64,
    pub offset: Option<i64>,
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub enum Range {
    Page(Page),
    Limit(Limit),
}


#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub enum JoinType {
    CROSS,
    INNER,
    OUTER,
    NATURAL,
}
#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub enum Modifier {
    LEFT,
    RIGHT,
    FULL,
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
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

named!(value<&str>, 
  map_res!(complete!(recognize!(many1!(is_not_s!(")"))))
    ,str::from_utf8
  )
);

named!(column<&str>, 
  map_res!(recognize!(many1!(one_of!("abcdefghijklmnopqrstuvwxyz0123456789_")))
    ,str::from_utf8
  )
);

/*
named!(column <&str>, map_res!(
        complete!(alphanumeric),
        str::from_utf8
    )
);
*/

named!(boolean <bool>,
    alt!(tag!("true") => {|_| true} |
         tag!("false") => {|_| false}
        )
);

named!(operand <Operand>,
   alt_complete!(
        float => {|f| Operand::Number(f as f64)} |
        boolean => {|b| Operand::Boolean(b) } |
        //column => {|c:&str| Operand::Column(c.to_string())} | //NOTE: assume the right value to be value, and the left to be always column
        value => {|v:&str| Operand::Value(v.to_string())}
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


named!(connector <Connector>,
   alt!(tag!("&") => {|_| Connector::AND} |
        tag!("|") => {|_| Connector::OR}
   )
);

named!(param <Param>,
    alt!(condition => {|c| Param::Condition(c)}| 
         equation => {|e| Param::Equation(e)}
    )
);

fn fold_conditions(initial: Condition, remainder: Vec<(Connector, Condition)>) -> Filter{
    let mut sub_filters = vec![];
    for (conn, cond) in remainder{
        let sub_filter = Filter{
                connector: Some(conn),
                condition: cond,
                sub_filters: vec![]
            };
        sub_filters.push(sub_filter);
    }
    Filter{
        connector: None,
        condition: initial,
        sub_filters: sub_filters
    }
}


named!(filter <Filter>,
    do_parse!(
        initial: condition_expr >>
        remainder: many0!(
           do_parse!(conn: connector >>
               cond: condition_expr >> 
                (conn, cond)
           )
        )
     >> (fold_conditions(initial, remainder))
    )
);

named!(filter_expr <Filter>,
    alt_complete!(filter | delimited!(tag!("("), filter_expr, tag!(")")))
);
    


named!(params < Vec<Param> >,
    separated_list!(tag!("&"), param)
);

named!(equation <Equation>, 
    map!(separated_pair!(column,
        tag!("="),
        operand 
    ),
    |(col,op):(&str,Operand)|{
        Equation{
            left: Operand::Column(col.to_string()),
            right: op
        }
    }
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

named!(condition_expr <Condition>,
    alt_complete!(condition | complete!(delimited!(tag!("("), condition_expr, tag!(")"))))
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
fn test_identifier(){
    assert_eq!(column("ahello".as_bytes()), IResult::Done(&b""[..],"ahello"));
    assert_eq!(column("hello_".as_bytes()), IResult::Done(&b""[..],"hello_"));
    assert_eq!(column("hello1".as_bytes()), IResult::Done(&b""[..],"hello1"));
}

#[test]
fn test_value(){
    assert_eq!(value("hello world!".as_bytes()), IResult::Done(&b""[..],"hello world!"));
    assert_eq!(value("技術通報".as_bytes()), IResult::Done(&b""[..],"技術通報"));
}

#[test]
fn test_param(){
    assert_eq!(param(&b"product=eq.134"[..]), IResult::Done(&b""[..], 
        Param::Condition(Condition{
            left: Operand::Column("product".to_string()),
            equality: Equality::EQ,
            right: Operand::Number(134f64)
          }
        )));

    assert_eq!(param(&b"product=134"[..]), IResult::Done(&b""[..], 
        Param::Equation(Equation{
            left: Operand::Column("product".to_string()),
            right: Operand::Number(134f64)
          }
        )));
}

#[test]
fn test_params(){
    assert_eq!(params(&b"product=eq.134"[..]), IResult::Done(&b""[..], 
        vec![Param::Condition(Condition{
            left: Operand::Column("product".to_string()),
            equality: Equality::EQ,
            right: Operand::Number(134f64)
          })]
        ));

    assert_eq!(params(&b"product=eq.134&page=2"[..]), IResult::Done(&b""[..], 
        vec![Param::Condition(Condition{
            left: Operand::Column("product".to_string()),
            equality: Equality::EQ,
            right: Operand::Number(134f64)
          }),
            Param::Equation(Equation{
                left: Operand::Column("page".to_string()),
                right: Operand::Number(2f64)
            })
          ]
        ));
}

// (filter)&condition wont match
#[test]
fn test_filter_issue1(){
    assert_eq!(filter(&b"age=lt.20&product=eq.134&price=lt.100.0"[..]), IResult::Done(&b""[..], 
        Filter{
            connector: None,
            condition: Condition{
                    left: Operand::Column("age".to_string()),
                    equality: Equality::LT,
                    right: Operand::Number(20f64)
                },
            sub_filters:vec![
                Filter{
                    condition:Condition{
                        left: Operand::Column("product".to_string()),
                        equality: Equality::EQ,
                        right: Operand::Number(134f64)
                    },
                    connector: Some(Connector::AND),
                    sub_filters: vec![
                    ]
                },
                Filter{
                    connector: Some(Connector::AND),
                    condition: Condition{
                        left: Operand::Column("price".to_string()),
                        equality: Equality::LT,
                        right: Operand::Number(100.0)
                    },
                    sub_filters: vec![]
                }
            ]
        }
        ));
}
// (filter)&(filter) wont match
//#[test]
fn test_filter_issue2(){

}

#[test]
fn test_filters(){
    assert_eq!(filter(&b"product=eq.134"[..]), IResult::Done(&b""[..], 
        Filter{
            connector: None,
            condition:Condition{
                left: Operand::Column("product".to_string()),
                equality: Equality::EQ,
                right: Operand::Number(134f64)
            },
            sub_filters: vec![]
        }
        ));

    assert_eq!(filter(&b"product=eq.134&price=lt.100.0"[..]), IResult::Done(&b""[..], 
        Filter{
            condition:Condition{
                left: Operand::Column("product".to_string()),
                equality: Equality::EQ,
                right: Operand::Number(134f64)
            },
            connector: None,
            sub_filters: vec![
                Filter{
                    connector: Some(Connector::AND),
                    condition: Condition{
                        left: Operand::Column("price".to_string()),
                        equality: Equality::LT,
                        right: Operand::Number(100.0)
                    },
                    sub_filters: vec![]
                }
            ]
        }
        ));

    assert_eq!(filter(&b"product=eq.134|price=lt.100.0"[..]), IResult::Done(&b""[..], 
        Filter{
            condition:Condition{
                left: Operand::Column("product".to_string()),
                equality: Equality::EQ,
                right: Operand::Number(134f64)
            },
            connector: None,
            sub_filters: vec![
                Filter{
                    connector: Some(Connector::OR),
                    condition: Condition{
                        left: Operand::Column("price".to_string()),
                        equality: Equality::LT,
                        right: Operand::Number(100.0)
                    },
                    sub_filters: vec![]
                }
            ]
        }
        ));

    assert_eq!(filter_expr(&b"(product=eq.134|price=lt.100.0)"[..]), IResult::Done(&b""[..], 
        Filter{
            condition:Condition{
                left: Operand::Column("product".to_string()),
                equality: Equality::EQ,
                right: Operand::Number(134f64)
            },
            connector: None,
            sub_filters: vec![
                Filter{
                    connector: Some(Connector::OR),
                    condition: Condition{
                        left: Operand::Column("price".to_string()),
                        equality: Equality::LT,
                        right: Operand::Number(100.0)
                    },
                    sub_filters: vec![]
                }
            ]
        }
        ));
    
}

#[test]
fn test_paren_filter_exprs(){
    assert_eq!(filter_expr(&b"(product=eq.134)|(price=lt.100.0)"[..]), IResult::Done(&b""[..], 
        Filter{
            condition:Condition{
                left: Operand::Column("product".to_string()),
                equality: Equality::EQ,
                right: Operand::Number(134f64)
            },
            connector: None,
            sub_filters: vec![
                Filter{
                    condition: Condition{
                        left: Operand::Column("price".to_string()),
                        equality: Equality::LT,
                        right: Operand::Number(100.0)
                    },
                    connector: Some(Connector::OR),
                    sub_filters: vec![]
                }
            ]
        }
        ));

    assert_eq!(filter_expr(&b"age=lt.20&(product=eq.134|price=lt.100.0)"[..]), IResult::Done(&b""[..], 
        Filter{
            condition: Condition{
                    left: Operand::Column("age".to_string()),
                    equality: Equality::LT,
                    right: Operand::Number(20f64)
                },
            connector: None,
            sub_filters:vec![
                Filter{
                    condition:Condition{
                        left: Operand::Column("product".to_string()),
                        equality: Equality::EQ,
                        right: Operand::Number(134f64)
                    },
                    connector: Some(Connector::OR),
                    sub_filters: vec![
                        Filter{
                            connector: None,
                            condition: Condition{
                                left: Operand::Column("price".to_string()),
                                equality: Equality::LT,
                                right: Operand::Number(100.0)
                            },
                            sub_filters: vec![]
                        }
                    ]
                }
            ]
        }
        ));
}

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
            right: Operand::Value("John".to_string())
          }
        ));

    assert_eq!(condition(&b"name=st.John Cena"[..]), IResult::Done(&b""[..], 
        Condition{
            left: Operand::Column("name".to_string()),
            equality: Equality::ST,
            right: Operand::Value("John Cena".to_string())
          }
        ));

    assert_eq!(condition_expr(&b"(name=st.John)"[..]), IResult::Done(&b""[..], 
        Condition{
            left: Operand::Column("name".to_string()),
            equality: Equality::ST,
            right: Operand::Value("John".to_string())
          }
        ));
    assert_eq!(condition_expr(&b"((name=st.John))"[..]), IResult::Done(&b""[..], 
        Condition{
            left: Operand::Column("name".to_string()),
            equality: Equality::ST,
            right: Operand::Value("John".to_string())
          }
        ));
    assert_eq!(condition("name=st.技術通".as_bytes()), IResult::Done(&b""[..], 
        Condition{
            left: Operand::Column("name".to_string()),
            equality: Equality::ST,
            right: Operand::Value("技術通".to_string())
          }
        ));
    assert_eq!(condition("name=ilike.*° ͜ʖ ͡°*".as_bytes()), IResult::Done(&b""[..], 
        Condition{
            left: Operand::Column("name".to_string()),
            equality: Equality::ILIKE,
            right: Operand::Value("*° ͜ʖ ͡°*".to_string())
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
        IResult::Done(&b""[..],Operand::Value("product".to_string()))); 

    assert_eq!(operand(&b"1234"[..]), 
        IResult::Done(&b""[..],Operand::Number(1234f64))); 

    assert_eq!(operand(&b"true"[..]), 
        IResult::Done(&b""[..],Operand::Boolean(true))); 

    assert_eq!(operand(&b"false"[..]), 
        IResult::Done(&b""[..],Operand::Boolean(false))); 

    // half match?
    //assert_eq!(operand(&b"true false"[..]), 
    //    IResult::Done(&b""[..],Operand::Column("true false".to_string()))); 

    assert_eq!(operand(&b"Hello world!"[..]), 
        IResult::Done(&b""[..],Operand::Value("Hello world!".to_string()))); 

    assert_eq!(operand(&b"hello world!"[..]), 
        IResult::Done(&b""[..],Operand::Value("hello world!".to_string()))); 
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
