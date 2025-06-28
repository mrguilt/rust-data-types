fn main() {
    println!("\n\n\t# The Rust Programming Language Chapter 3, Section 2: Data Types\n");
    
    println!("## Scaler Types");
    println!("### INTEGERS");
    let mut foo: u32=50; //32-bit unsigned integer
    println!("Foo={foo}");
    foo=foo*foo;
    println!("Foo^2={foo}");
    let mut bar: u32=0xffff;
    println!("Bar={bar}\tI put this in as a hex number (0xffff). It printed as decimal.");

    println!("\n### FLOATING TYPES");
    let mut baz: f32=3.14;
    println!("Baz={baz}\tNot rocket science.");

    println!("\n### LET'S DO SOME MATH!");
    let truncated=5/3;
    println!("5/3={truncated}\tIt assumed integer");
    let floatingtruncated=5.0/3.0;
    println!("5/3={floatingtruncated}\tI set it to a float by putting decimals (5.0/3.0).");
    let rock=10;
    let mixnumbers=rock as f64 +floatingtruncated; //"as" is key.
    println!("Mixing numbers gives me {mixnumbers}");

    println!("\n### BOOLEAN\nJust setting a couple in the code. It's pretty straight-forward.");
    let t=true;
    let f: bool=false;

    println!("\n### CHARACTER");
    let c='c';
    println!("The caracter is \"{c}\"");

    println!("\n## Compound Types");
    println!("\n### Tuple Type");
    let trey: (i32,f64,u8)=(500,24.8,3);
    println!("Accessing the first element: {0}",trey.0);
    println!("Accessing the third element: {0}",trey.2);
    println!("Accessing the second element: {0}",trey.1);
    println!("Can a tuple have more than three elements?");
    let mut quint: (i32,i32,i32,i32,i32)=(10,20,30,40,50);
    println!("Fifth Element: {0}",quint.4);
    println!("Yep!");
    quint.3=35;
    println!("Put a new value in fourth element: {0}",quint.3);
    println!("You can (assuming you declared it mutable in the first place).");
    println!("\n### Array Type");
    let daarray=[1,2,3,4,5];
    let differentarray=["Beso","Luna","Nami"];
    println!("Our youngest cat it {}. Our girl cat is {}.",differentarray[2],differentarray[1]);
    println!("WHAT ABOUT STRINGS?!?");
}
