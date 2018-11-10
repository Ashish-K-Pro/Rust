struct Person
{
name : String,
age : u8
}


impl ToString for Person {
fn to_string(&self) -> String {

return format!("My name is : {} and I'm : {} years of age", self.name,self.age);
}
}


fn main() {

// Traits in Rust, just like interfaces in other languages

let per = Person { name : String ::from ("Ashish") , age : 27};
println!("{}",per.to_string());

}
