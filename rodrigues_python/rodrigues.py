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
user_vector = my_vector_tool.choose_vector()
rotation_axis = my_vector_tool.choose_k()

print(f"User vector: {user_vector}")
print(f"Rotation axis: {rotation_axis}")
