import sys
import os
import pyexcel as p

BASE_DIR = './libpsx-rs/tools/Memory Map Generation/'

try:
    file_name = sys.argv[1]
except IndexError:
    file_name = 'Instruction List.ods'

print(f'Parsing {file_name}')

records = p.get_records(file_name=os.path.join(BASE_DIR, file_name), sheet_name='Sheet1', start_row=4)
keys = list(records[0].keys())
