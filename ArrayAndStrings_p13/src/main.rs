fn main() {

let numbers = [10,12,15,13,18,20];

for i in numbers.iter(){
println!("{}",i);
}


// Strings

let mut my_string = String :: from("This is my sample string. ");

println!("Length : {}", my_string.len());

println!("If the string is empty: {}",my_string.is_empty());


// to figure out white spaces..

for token in my_string.split_whitespace()
{
println!("{}",token);
}

println!("Does the String contains 'my'? {}", my_string.contains("my"));

// Add a new string

my_string.push_str("This is second string.");

println!("{}",my_string);

// there are lot of other methods to explore in the string.


}
