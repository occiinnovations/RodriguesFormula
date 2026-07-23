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
    let v = [x, y, z];

    // Rotation axis (k = [1, 1, 1]) - normalize it for proper rotation
    let k_unnormalized = [1.0, 1.0, 1.0];
    let k_magnitude = (k_unnormalized[0].powi(2) + k_unnormalized[1].powi(2) + k_unnormalized[2].powi(2)).sqrt();
    let k = [
        k_unnormalized[0] / k_magnitude,
        k_unnormalized[1] / k_magnitude,
        k_unnormalized[2] / k_magnitude,
    ];

    // Rotation angle: 45 degrees in radians
    let theta = std::f64::consts::PI / 4.0;
    let sine_value = theta.sin();
    let cosine_value = theta.cos();

    println!("Input vector: [{}, {}, {}]", x, y, z);
    println!("Rotation axis (normalized): [{:.4}, {:.4}, {:.4}]", k[0], k[1], k[2]);
    println!("Rotation angle: {} radians ({:.1}°)", theta, theta.to_degrees());
    println!("sin({}) = {:.6}", theta, sine_value);
    println!("cos({}) = {:.6}", theta, cosine_value);
    println!();

    // Rodrigues' rotation formula:
    // v_rot = v*cos(θ) + (k × v)*sin(θ) + k*(k·v)*(1 - cos(θ))

    // 1. v * cos(θ)
    let term1 = scalar_mult(&v, cosine_value);

    // 2. (k × v) * sin(θ)
    let cross_product = cross(&k, &v);
    let term2 = scalar_mult(&cross_product, sine_value);

    // 3. k * (k · v) * (1 - cos(θ))
    let dot_product = dot(&k, &v);
    let k_scaled = scalar_mult(&k, dot_product * (1.0 - cosine_value));
    let term3 = k_scaled;

    // Sum all terms
    let v_rotated = [
        term1[0] + term2[0] + term3[0],
        term1[1] + term2[1] + term3[1],
        term1[2] + term2[2] + term3[2],
    ];

    println!("Rotated vector: [{:.6}, {:.6}, {:.6}]", v_rotated[0], v_rotated[1], v_rotated[2]);
}

// Helper functions for vector operations
fn dot(a: &[f64; 3], b: &[f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn cross(a: &[f64; 3], b: &[f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn scalar_mult(v: &[f64; 3], scalar: f64) -> [f64; 3] {
    [v[0] * scalar, v[1] * scalar, v[2] * scalar]
}