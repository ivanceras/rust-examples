
#[derive(Debug)]
enum Operand{
    Value(u32),
    String(String)
}

impl <'a>From<&'a str> for Operand{
    fn from(s: &'a str) -> Self{
        Operand::String(s.into())
    }
}

#[derive(Debug)]
enum Field{
    Operand(Operand)
}

impl From<Operand> for Field{
    
    fn from(operand: Operand) -> Self{
        Field::Operand(operand)
    }
}

impl <'a>From<&'a str> for Field{
    
    fn from(s: &'a str) -> Self{
        let op: Operand = s.into();
        op.into()
    }
}


fn main(){
    println!("hello");
    let op = Operand::Value(1);
    let fl = Field::from(op);
    println!("{:#?}", fl);
    let op2 = Operand::Value(2);
    let fl2:Field = op2.into();

    let op3 = Operand::Value(3);
    do_something(op3);
    let fl3:Field = fl2.into();
    do_something(fl3);
    let s: Operand = "hello".into();
    do_something(s);
    do_something("hello")
}


fn do_something<F>(field: F)
    where F: Into<Field> + std::fmt::Debug {
    println!("doing something for {:?}", field.into());
}



