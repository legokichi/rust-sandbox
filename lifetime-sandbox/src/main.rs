trait Schema {
    fn name(&self) -> &str;
    fn r#type(&self)-> char;
    fn level(&self)-> i64;
}

struct Item(String);
impl Schema for Item {
    fn name(&self) -> &str { &self.0 }
    fn r#type(&self) -> char { unimplemented!() }
    fn level(&self) -> i64 { unimplemented!() }
}
trait SchemaDict<'a> {
    type Item: Schema;
    fn get(&'a self, id: i64)-> Option<&'a Self::Item>;
}
struct Dict(std::collections::HashMap<i64, Item>);
impl<'a> SchemaDict<'a> for Dict {
    type Item = Item;
    fn get(&'a self, id: i64)-> Option<&'a Self::Item>{
        self.0.get(&id)
    }
}
struct Decoder<'a, S: SchemaDict<'a>>(&'a S);
impl<'a, S: SchemaDict<'a>> Decoder<'a, S> {
    fn decode(&self){
        println!("{}", self.0.get(0).unwrap().name());
    }
}
fn main() {
    let item = Item("foo".to_string());
    let mut map = std::collections::HashMap::new();
    map.insert(0, item);
    let dict = Dict(map);
    println!("{}", dict.get(0).unwrap().name());
    {
        let dict = Decoder(&dict);
    }
}

