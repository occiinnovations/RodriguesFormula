# Rodrigues' Rotation Formula

Rodrigues' Formula demonstrated in Python and Rust; intended to be used as a building block for other projects.

### What is it?

The Rodrigues' rotation formula calculates the position of a vector after rotation around an axis by an angle.
The calculation uses the vector, the unit vector of the axis, and the angle. The formula maps the inputs to a \(3\times3\) matrix. Multiplying the vector by this matrix yields the rotated vector.

This project is the Rodrigues' Formula demonstrated in Python and Rust; intended to be used as a building block for other projects.
_____________

### Applications

• **3D Animation & Video Games**: Moving an item around a specific pivot point in a virtual 3D space, or calculating where a player's field-of-view camera is pointing when they move their mouse.

• **Robotics Control Pipelines**: Calculating the physical angles that an industrial motor needs to turn to position a robotic gripper at an exact XYZ coordinate in space.

• **Flight Dynamics**: Computing the changes in roll, pitch, and yaw angles to determine how a drone or rocket has tilted away from its vertical tracking axis.

• **Camera Tracking Systems**: Taking a live video stream and mathematically calculating where a camera is physically located relative to the object it is recording

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

### LaTeX Forms

If $v$ is a valid vector in $ℝ^3$, with $k$ being a unit vector such as $\hat{\imath}$, $\hat{\jmath}$, or $\hat{k}$, we can represent the formula as follows:

$${\Large v_{rot} = v cos\theta + (k \times v)sin{\theta} + k(k\cdot v)(1-cos{\theta})}$$

Alternatively:

$$\Large R = I + (\sin\theta)K + (1 - \cos\theta)K^2$$

$$\Large R = I + \sin(\theta)\big[\mathbf{k}\big]_\times + (1 - \cos(\theta))\big[\mathbf{k}\big]_\times^2$$

$$\Large [\mathbf{k}\big]_\times = \begin{pmatrix} 0 & -k_z & k_y \\ k_z & 0 & -k_x \\ -k_y & k_x & 0 \end{pmatrix}$$
________
