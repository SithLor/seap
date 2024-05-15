# Importing a module
import math

# Defining a function
def calculate_area(radius):
    # Using a function from a module
    return math.pi * radius ** 2

# Defining a class
class Circle:
    # Constructor
    def __init__(self, radius):
        self.radius = radius

    # Method
    def area(self):
        return calculate_area(self.radius)

# Creating an object
circle = Circle(5)

# Using a method
print(circle.area())

# Conditional statement
if circle.radius > 0:
    print("Positive radius")
else:
    print("Non-positive radius")

# Loop
for i in range(5):
    print(i)

# Exception handling
try:
    print(calculate_area(-1))
except ValueError:
    print("Caught an exception")

# List comprehension
squares = [x**2 for x in range(10)]
print(squares)