#[macro_use]
extern crate nom;

use nom::digit;
use std::str::{self,FromStr};

fn main(){
    println!("hello");
    let input = "(1+2)*(3-4)";
}

named!(unit<&str>,
   map_res!(digit, str::from_utf8)
);

#[test]
fn test_unit(){
    let u = unit("4");
    println!("u: {}", u);
}
