fn main() {

// loop is a keyword in Rust

let mut n = 10;

loop {
n +=1;

if n == 20{
continue;
}
if n > 50{
break;
}
println!("The value of n here is : {}",n)


}

}
