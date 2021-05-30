use validator::{Validate, ValidationErrors, ValidationErrorsKind};

#[derive(Validate)]
struct C{
    #[validate]
    values: Vec<D>
}
#[derive(Validate)]
struct D{
    #[validate(length(min = 1, max = 10, message = "field string must be between 1 and 10"))]
    field: String
}


fn to_string(e: &ValidationErrors) -> String {
    e.errors().iter().map(|(field_name, kind)|{
        match kind {
            ValidationErrorsKind::Struct(o) => to_string(o),
            ValidationErrorsKind::List(o) => o.iter().map(|(id, o)|to_string(o)).fold(String::new(), |a, b| a+&b),
            ValidationErrorsKind::Field(o) => o.iter().map(|o| o.message.as_ref().map(|a| format!("{}", a)).unwrap_or("".to_string()) ).fold(String::new(), |a, b| a+&b)
        }
    }).fold(String::new(), |a, b| a+&b)
}


fn main() {
    let a = C{values: vec![D{field: "".to_string()}]};
    println!("{}", to_string(&dbg!(a.validate()).unwrap_err()));
    
}
