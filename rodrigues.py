import numpy as np


def rotate_vector(v, k, theta):
    v = np.asarray(v)
    k = np.asarray(k) / np.linalg.norm(k)  # Normalize k to a unit vector
    cos_theta = np.cos(theta)
    sin_theta = np.sin(theta)

    term1 = v * cos_theta
    term2 = np.cross(k, v) * sin_theta
    term3 = k * np.dot(k, v) * (1 - cos_theta)

    return term1 + term2 + term3
