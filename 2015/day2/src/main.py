file_contents = open('./src/day2.txt')

lines = file_contents.readlines()


def get_surface_area(dimensions):
    [length, width, height] = dimensions

    side1 = length * width
    side2 = length * height
    side3 = height * width

    slack = min([side1, side2, side3])
    return (side1 + side2 + side3) * 2 + slack


total_area = 0
for line in lines:
    dimensions = [int(x) for x in line.split('x')]
    total_area += get_surface_area(dimensions)

print(total_area)
