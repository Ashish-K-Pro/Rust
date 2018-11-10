fn main() {

function_to_print_numbers(10);

if is_even(30)
{
println!("Its an even number");
}
else
{
println!("Odd number");
}

}

fn function_to_print_numbers(num : u32) {

for i in 1..num{
println!("number here is : {}",i);

}

}


// function to return a value

fn is_even(num: u32) -> bool {
return num % 2 == 0;
}
