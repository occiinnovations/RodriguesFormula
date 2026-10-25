# Rodrigues' Rotation Formula

Rodrigues' Formula demonstrated in Python and Rust; intended to be used as a building block for other projects.

### What is it?

The Rodrigues' rotation formula calculates the position of a vector after rotation around an axis by an angle.
The calculation uses the vector, the unit vector of the axis, and the angle. The formula maps the inputs to a \(3\times3\) matrix. Multiplying the vector by this matrix yields the rotated vector.

This project is the Rodrigues' Formula demonstrated in Python and Rust; intended to be used as a building block for other projects.
_____________

### File Directory

```text
RodriguesFormula/
│
├── rodrigues_python/                 
│   ├── requirements.txt  
│   └── rodrigues.py 
│
├── rodrigues_rust/
│   ├── src/
│   │   └── main.rs
│   ├── cargo.lock
│   └── cargo.toml
│    
├─ .gitignore
├─ LICENSE
└─ README.md
```

### Latex Forms

If $v$ is a valid vector in $ℝ^3$, with $k$ being a unit vector such as $\hat{\imath}$, $\hat{\jmath}$, or $\hat{k}$, we can represent the formula as follows:

$${\Large v_{rot} = v cos\theta + (k \times v)sin{\theta} + k(k\cdot v)(1-cos{\theta})}$$

Alternatively:

$$\Large R = I + (\sin\theta)K + (1 - \cos\theta)K^2$$

$$\Large R = I + \sin(\theta)\big[\mathbf{k}\big]_\times + (1 - \cos(\theta))\big[\mathbf{k}\big]_\times^2$$

$$\Large [\mathbf{k}\big]_\times = \begin{pmatrix} 0 & -k_z & k_y \\ k_z & 0 & -k_x \\ -k_y & k_x & 0 \end{pmatrix}$$
________

### Applications

• Computer Graphics: Rotating 3D models and positioning virtual cameras.

• Robotics: Calculating robot arm joint movements and kinematics.

• Aerospace: Tracking aircraft and spacecraft orientation (pitch, roll, yaw).

• Computer Vision: Estimating 3D camera poses and tracking object motion.
