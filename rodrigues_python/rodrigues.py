import numpy as np
from vector_init import VectorPrep


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
