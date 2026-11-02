import numpy as np


class VectorPrep:
    def __init__(self, x=0, y=0, z=0, k=None):
        self.x = float(x)
        self.y = float(y)
        self.z = float(z)
        self.vector = np.array([self.x, self.y, self.z])

    def choose_vector(self):
        while True:
            x = input("Enter the x component of the vector: ")
            y = input("Enter the y component of the vector: ")
            z = input("Enter the z component of the vector: ")

            try:
                x = float(x)
                y = float(y)
                z = float(z)

                self.vector = np.array([x, y, z])
                break

            except ValueError:
                print("Invalid input. Please enter an integer or float.")

        return self.vector

    def choose_k(self):
        while True:
            self.k = str(
                input("Enter the letter of the axis you want to rotate about (x, y, or z): "))

            if self.k != 'x' and self.k != 'y' and self.k != 'z':
                print("Invalid axis. Please choose x, y, or z.")
            else:
                break

        if self.k == 'x':
            self.k = np.array([1.0, 0.0, 0.0])

        elif self.k == 'y':
            self.k = np.array([0.0, 1.0, 0.0])

        elif self.k == 'z':
            self.k = np.array([0.0, 0.0, 1.0])

        return self.k


my_vector_tool = VectorPrep()
init_vector = my_vector_tool.choose_vector()
rotation_axis = my_vector_tool.choose_k()

print(f"User vector: {init_vector}")
print(f"Rotation axis: {rotation_axis}")

theta_deg = float(input("Theta: Enter an angle in degrees: "))
theta_rad = np.deg2rad(theta_deg)

first_segment = init_vector * np.cos(theta_rad)
second_segment = np.cross(rotation_axis, init_vector) * np.sin(theta_rad)
third_segment = rotation_axis * \
    (1 - np.cos(theta_rad)) * np.dot(rotation_axis, init_vector)

rotated_vector = first_segment + second_segment + third_segment
print(f"Rotated vector: {rotated_vector}")

# From here, in a standard application such as a video game or graphic simulation, you can put
# this into a while loop where your target vector gets set = to the rotated_vector each timestep update.

# It is also important to not make it snap to the new position, use a simple loop or range function to slowly increase theta between desired time points.
