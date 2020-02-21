#[macro_use]
extern crate frunk;
#[macro_use]
extern crate derive_more;
extern crate frunk_enum_core;
#[macro_use]
extern crate frunk_enum_derive;

#[derive(Generic,LabelledGeneric, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Deref)]
pub struct NonEmptyString(pub String);
impl std::convert::TryFrom<String> for NonEmptyString {
    type Error = ();
    fn try_from(src: String) -> Result<Self, Self::Error>{
        Ok(Self(src))
    }
}

#[derive(Generic, LabelledGeneric, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Into, From, Deref)]
pub struct B(pub NonEmptyString);

#[derive(Generic,LabelledGeneric, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct B2{
    pub o: B,
}
#[derive(LabelledGeneric, Debug)]
pub struct B3 {
    pub name: String,
    pub friends: Vec<String>,
}
#[derive(LabelledGeneric, Debug)]
pub struct B4 {
    pub new_name: String,
}

#[derive(LabelledGenericEnum)]
enum Foo{
    Bar{name:String}
}
