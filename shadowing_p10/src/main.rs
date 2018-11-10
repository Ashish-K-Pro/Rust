fn main() {

let mut x = 10;

{
// inside the shadow of current block
let x = 15;
}

println!("The value of x: {}",x);


let x = true;
println!("X is a boolean now");
let x = "This is a string now";
println!("X is a string now");

}
