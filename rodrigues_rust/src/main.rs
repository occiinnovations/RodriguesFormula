use std::io;

fn main() {
    println!("Enter the x, y, and z components of the vector (separated by spaces):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    // Parse the input into three f64 values
    let components: Vec<f64> = input
        .split_whitespace()
        .map(|s| s.parse::<f64>().expect("Failed to parse number"))
        .collect();

    if components.len() != 3 {
        eprintln!("Please enter exactly 3 values");
        return;
    }

    let x = components[0];
    let y = components[1];
    let z = components[2];

    // Vector to rotate
    let v = [x,y,z];

    println!("Input vector: [{:.6}, {:.6}, {:.6}]", v[0], v[1], v[2]);

}

fn axis_of_rotation(){
    println!("Enter which plane you want to rotate about (x, y, or z):");
    let mut k_input = String::new();
    io::stdin().read_line(&mut k_input).expect("Failed to read input");
    
    let k: Vec<f64> = k_input
        .split_whitespace()
        .map(|s| s.parse::<f64>().expect("Failed to parse number"))
        .collect();
    if k.len() != 3 {
        eprintln!("Please enter exactly 3 values for the axis of rotation");
        return;
    }
    let mut temp_count = 0;
    
    for unit_vector in k.iter() {
        temp_count += 1;
        if *unit_vector != 0.0 && *unit_vector != 1.0 {
            eprintln!("Axis of rotation must be a unit vector (components must be 0 or 1)");
            return;
        }
            
        if *unit_vector == 1.0{
            println!("You have chosen to rotate about the {}-axis", match temp_count {
                1 => "x",
                2 => "y",
                3 => "z",
                _ => unreachable!(),               
            });
        }
    }
    println!("Axis of rotation: [{:.6}, {:.6}, {:.6}]", k[0], k[1], k[2]);

}