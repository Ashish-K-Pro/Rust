
use std::env;


fn main() {

let args : Vec<String> = env :: args().collect();

for argument in args.iter()
{
println!("Command Line Arguments: {}",argument);
}


}
