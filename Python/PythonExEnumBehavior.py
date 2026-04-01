from enum import Enum

class Shape(Enum):
    CIRCLE = ("circle", 5)
    SQUARE = ("square", 4)

    def __init__(self, label, size):
        self.label = label
        self.size = size

print(Shape.CIRCLE.label)
print(Shape.CIRCLE.size)

print(Shape.SQUARE.label)
print(Shape.SQUARE.size)