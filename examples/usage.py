# from example_python_package_with_rust_backend import cram

# # Example arguments
# a = [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]
# n0 = [1.0, 2.0, 3.0]
# dt = 0.1

# # Call the function
# cram(a, n0, dt)


from example_python_package_with_rust_backend import cram
import math
import numpy
from scipy.sparse import csc_matrix
decay_constant = 2.0
n0 = 1.0
dt = 100.0
conc = numpy.array([[n0]])
A = numpy.zeros((1, 1))
A[0, 0] = -decay_constant
A = csc_matrix(A)
expected = n0 * math.exp(-decay_constant * dt)
actual = cram(A, conc, dt)