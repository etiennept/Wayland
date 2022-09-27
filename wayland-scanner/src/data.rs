#[derive(Debug)]
pub struct Protocol {
    pub name: String,
    pub interfaces: Vec<Interface>,

}

#[derive(Debug)]
pub struct Entry {
    pub name: String,
    pub value: String,
    pub summary: Option<String>,
    pub since: Option<String>,
}

#[derive(Debug)]
pub struct Enum {
    pub name: String,
    pub since: Option<String>,
    pub bitfield: Option<String>,
    pub entries: Vec<Entry>,
}

#[derive(Debug)]
pub struct Arg {
    pub name: String,
    pub interface : Option<String> ,
    pub type_  : Type ,
    pub allow_null: bool ,
    pub enum_ : Option<String>

}

#[derive(Debug)]
pub struct Callable {
    pub name: String,
    pub since: Option<String>,
    pub types: Option<Type>,
    pub args: Vec<Arg>,
}

#[derive(Debug)]
pub struct Interface {
    pub name: String,
    pub version: String,
    pub enums: Vec<Enum>,
    pub requests: Vec<Callable>,
    pub event: Vec<Callable>,
}

#[derive(Debug , Eq ,  PartialEq , Clone )]
pub enum Type {
    Int,
    Uint,
    Fixed,
    String,
    Object,
    NewId,
    Array,
    Fd,
    Destructor,
}