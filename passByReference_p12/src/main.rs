struct Color {

red : u8,
green : u8,
blue : u8
}



fn main() {

let blueColor = Color { red: 0, green :0, blue : 255};
print_color(&blueColor);

}

fn print_color(c: &Color)
{
println!("Color R: {}, G: {}, B: {}",c.red, c.green, c.blue);

}
