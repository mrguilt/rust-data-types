fn main() {
    println!("\n\n\tThe Rust Programming Language Chapter 3, Section 2: Data Types\n");
    
    println!("INTEGERS");
    let mut foo: u32=50; //32-bit unsigned integer
    println!("Foo={foo}");
    foo=foo*foo;
    println!("Foo^2={foo}");
    let mut bar: u32=0xffff;
    println!("Bar={bar}\tI put this in as a hex number (0xffff). It printed as decimal.");

    println!("\nFLOATING TYPES");
    let mut baz: f32=3.14;
    println!("Baz={baz}\tNot rocket science.");

    println!("\nLET'S DO SOME MATH!");
    let truncated=5/3;
    println!("5/3={truncated}\tIt assumed integer");
    let floatingtruncated=5.0/3.0;
    println!("5/3={floatingtruncated}\tI set it to a float by putting decimals (5.0/3.0).");
    let rock=10;
    let mixnumbers=rock as f64 +floatingtruncated; //"as" is key.
    println!("Mixing numbers gives me {mixnumbers}");

    println!("\nBOOLEAN\nJust setting a couple in the code. It's pretty straight-forward.");
    let t=true;
    let f: bool=false;

    println!("\nCHARACTER");
    let c='c';
    println!("The caracter is \"{c}\"");

}
