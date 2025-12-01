# https://adventofcode.com/2025/day/1

# Part 1

test_input = """L68
L30
R48
L5
R60
L55
L1
L99
R14
L82""".split(
    "\n"
)


def sgn(num):
    return 1 if num >= 0 else -1


def get_secret_code(rotations):
    """
    Counts the number o times the dial points at 0 AFTER a rotation
    """
    nb_zeros = 0
    current_rotation_result = 50

    for rotation in rotations:
        sign = -1 if rotation.startswith("L") else 1
        value = sign * int(rotation[1:])
        current_rotation_result += value
        if current_rotation_result % 100 == 0:
            nb_zeros += 1

    return nb_zeros


def get_secret_code_0x434C49434B(rotations):
    """
    Counts the number of times the dial points at 0 DURING or AFTER a rotation
    """

    nb_zeros = 0
    current_rotation_result = 50
    print(f"The dial starts at {current_rotation_result}.")

    for rotation in rotations:

        sign = -1 if rotation.startswith("L") else 1
        value = sign * int(rotation[1:])
        current_tick = current_rotation_result % 100
        new_tick = (current_tick + value) % 100

        error_msg = f"The dial is rotated {rotation} to point at {new_tick}."

        if new_tick == 0:
            nb_zeros += 1

        nb_passes_0 = abs((current_tick + value) // 100)
        if nb_passes_0 > 0:
            error_msg += f" During this rotation, it points at '0' {nb_passes_0} times."
        nb_zeros += nb_passes_0

        print(error_msg)

        current_rotation_result += value

    return nb_zeros


with open("input_day1.txt", "r") as f:
    rotations = f.readlines()

test_input = """L68
L30
R48
L5
R60
L55
L1
L99
R14
L82""".split(
    "\n"
)

print(f"The secret code : {get_secret_code(test_input)}")

print(
    f"The secret code (method 0x434C49434B): {get_secret_code_0x434C49434B(test_input)}"
)
