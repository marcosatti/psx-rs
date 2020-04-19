import os
import re
import pprint
import csv


BASE_DIR = './libpsx-rs/src'
MAP_EXPRESSION = r'map\((\w+), (\w+), (.*)\)'
RE_MAP_EXPRESSION = re.compile(MAP_EXPRESSION)
OBJECT_EXPRESSION = r'state\.(.*?) '
RE_OBJECT_EXPRESSION = re.compile(OBJECT_EXPRESSION)
CONSTANTS = {
    'BIOS_SIZE': 0x80000,
    'MAIN_MEMORY_SIZE': 0x200000,
}
OUT_DIR = os.path.dirname(os.path.realpath(__file__))


def walk_dir(dir, file_func):
    for root, dirs, files in os.walk(dir):
        for name in dirs:
            dir_path = os.path.join(root, name)
            walk_dir(dir_path, file_func)
        for name in files:
            file_path = os.path.join(root, name)
            file_func(file_path)


def parse_number(number):
    try:
        formatted_number = number.replace('_', '')
        if '0x' in formatted_number:
            return int(formatted_number, base=16)
        else:
            return int(formatted_number, base=10)
    except ValueError:
        return CONSTANTS[number]


def stringify_number_hex(number):
    number = '{:08X}'.format(number)
    number = '0x' + number[:4] + '_' + number[-4:]
    return number


def parse_bus_address(bus_address):
    return stringify_number_hex(parse_number(bus_address))


def parse_length(length):
    return stringify_number_hex(parse_number(length))


def parse_object(object_):
    match = re.search(RE_OBJECT_EXPRESSION, object_)
    if not match:
        raise ValueError('Couldn\'t determine object state path')
    return match.group(1)


def parse_line(line):
    match = re.search(RE_MAP_EXPRESSION, line)
    if not match:
        return None

    bus_address = parse_bus_address(match.group(1))
    length = parse_length(match.group(2))
    object_ = parse_object(match.group(3))

    return (bus_address, length, object_)


def main():
    results = set()

    def file_func(file_path):
        with open(file_path, 'r') as file:
            for line in file:
                result = parse_line(line)
                if result is not None:
                    results.add(result)

    walk_dir(BASE_DIR, file_func)

    out_file = os.path.join(OUT_DIR, 'results.csv')
    with open(out_file, 'w', newline='') as file:
        writer = csv.writer(file)
        writer.writerow(['Bus address', 'Length (bytes)', 'State object path'])
        for result in sorted(results):
            writer.writerow(result)


if __name__ == "__main__":
    main()
