#!/usr/bin/python3

import os
import re
import datetime

script_dir = os.path.dirname(os.path.realpath(__file__))


def main():
    print('\n'.join(process_file(script_dir + '/main.rs')))
    print(f'// {datetime.datetime.now().isoformat()}')


def process_file(path):
    file_dir = os.path.dirname(path)
    file_name = os.path.basename(path)

    yield f'// {file_name}'
    yield ''

    with open(path, 'r') as file:
        lines = file.readlines()

    for line in lines:
        line = line.rstrip()

        if include_match := re.match(r'^include!\("(.*)"\);$', line):
            yield from process_file(os.path.join(file_dir, include_match.group(1)))
        else:
            yield line


if __name__ == '__main__':
    main()
