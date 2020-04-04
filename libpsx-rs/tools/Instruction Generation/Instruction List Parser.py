#%%
import os
import sys
import pyexcel as p

MATCH = "match inst.{}() {{"
VALUE_MATCH = "{} => {{"
SOME = "Some(({}, {}))"
NONE = "None"
BRACE_CLOSE = "},"
BRACE_CLOSE_NC = "}"
INSTRUCTION_FN_DECL = "type InstructionFn = fn(&mut State, Instruction) -> InstResult;"
HEADER = "pub fn lookup(inst: Instruction) -> Option<(InstructionFn, usize)> {"
INST_HEADER = "pub fn {}(_state: &mut State, _instruction: Instruction) -> InstResult {{"
UNIMPLEMENTED = "unimplemented!(\"Instruction {} not implemented\");"

START_KEY_COLUMN = 3 # opcode column
END_KEY_COLUMN = 7

BASE_DIR = './libpsx-rs/tools/Instruction Generation/'

#################################

def write_line(f, indent, lines, string):
    f.write('    ' * indent)
    f.write(string)
    f.write('\n' * lines)

try:
    file_name = sys.argv[1]
except IndexError:
    file_name = 'Instruction List.ods'
print(f'Parsing {file_name}')

records = p.get_records(file_name=os.path.join(BASE_DIR, file_name), sheet_name='Sheet1', start_row=4)
keys = list(records[0].keys())

# make instruction templates
# write out
  
with open(os.path.join(BASE_DIR, 'core.rs'), 'w') as f: 
    for record in records:
        if record['Mnemonic'] != '':
            write_line(f, 0, 1, INST_HEADER.format(record['Mnemonic']))
            write_line(f, 1, 1, UNIMPLEMENTED.format(record['Mnemonic']))
            write_line(f, 0, 1, BRACE_CLOSE_NC)
            write_line(f, 0, 1, '')

# make lookup
# make tree

def make_switch(d, record, col_idx):
    value = record[keys[col_idx]]
    if value == '':
        value = -1

    if col_idx != END_KEY_COLUMN:
        d.setdefault(value, { 'type' : keys[col_idx + 1] })
        make_switch(d[value], record, col_idx + 1)
    else:
        d[value] = (record['Mnemonic'], record['CPI'], record['Index'])

d = { 'type' : 'opcode' }
for record in records:
    if record['opcode'] != '':
        key = keys[START_KEY_COLUMN]
        d.setdefault(record[key], { 'type' : keys[START_KEY_COLUMN + 1] })
        make_switch(d[record[key]], record, START_KEY_COLUMN + 1)

import pprint
#pprint.pprint(d)

# clean tree

def clean_level(level):
    if len(level) == 2 and (-1 in level):
        if type(level[-1]) == dict:
            temp = clean_level(level[-1])
            return temp
        else:
            temp = level[-1]
            return temp
    else:
        if type(level) == dict:
            for k, v in level.items():
                if type(v) == dict:
                    level[k] = clean_level(v)
            temp = level
            return temp
        else:
            temp = level
            return temp

for k, v in d.items():
    temp = clean_level(v)
    d[k] = temp

# import json
# with open('output.json', 'w') as f:
#     json.dump(d, f, indent=4)
# for item in d.items():
#     print(item)

# write out
sorted_list = []
def write_lookup_level(f, level, indent):
    global sorted_list
    for k, v in level.items():
        if k == 'type':
            write_line(f, indent, 1, MATCH.format(v))
        else:
            if type(v) == dict:
                if k == -1:
                    write_line(f, indent + 1, 1, VALUE_MATCH.format('_'))
                else:
                    write_line(f, indent + 1, 1, VALUE_MATCH.format(k))
                write_lookup_level(f, v, indent + 2)
                write_line(f, indent + 1, 1, BRACE_CLOSE)
            else:
                write_line(f, indent + 1, 1, VALUE_MATCH.format(k))
                write_line(f, indent + 2, 1, SOME.format(v[0], v[1]))
                sorted_list.append(v)
                write_line(f, indent + 1, 1, BRACE_CLOSE)
    
    write_line(f, indent + 1, 1, VALUE_MATCH.format('_'))
    write_line(f, indent + 2, 1, NONE)
    write_line(f, indent + 1, 1, BRACE_CLOSE)
    write_line(f, indent, 1, BRACE_CLOSE_NC)

with open(os.path.join(BASE_DIR, 'lookup.rs'), 'w') as f:
    write_line(f, 0, 1, INSTRUCTION_FN_DECL)
    write_line(f, 0, 1, HEADER)
    write_lookup_level(f, d, 1)
    write_line(f, 0, 1, BRACE_CLOSE_NC)

sorted_list = sorted(sorted_list, key=lambda x: x[2])
    
# write out class fn table
# with open(os.path.join(BASE_DIR, 'table.rs'), 'w') as f:
#     write_line(f, 0, 0, INST_TABLE_HEADER)
#     for i, inst in enumerate(sorted_list):
#         if i % 8 == 0:
#             write_line(f, 0, 1, '')
#         elif i % 8 == 1:
#             write_line(f, 1, 0, inst[0] + ', ')
#         else:
#             write_line(f, 0, 0, inst[0] + ', ')
#     write_line(f, 0, 1, '')
#     write_line(f, 0, 1, SQUARE_BRACE_CLOSE)

#%%