import sys
import os
import pyexcel as p

HANDLERS = [
    'u8 read', 
    'u8 write',
    'u16 read',
    'u16 write',
    'u32 read',
    'u32 write',
]

try:
    file_name = sys.argv[1]
except IndexError:
    file_name = os.path.join(os.path.dirname(__file__), 'Memory Mappings.ods')

print(f'Parsing {file_name}', file=sys.stderr)


def make_match_statement(src_handler, start_address, length, handler_path, args, writing):
    end_address_int = (int(start_address, base=16) + int(length, base=16)) - 1
    end_address = '0x{:08X}'.format(end_address_int)
    end_address = '0x' + (end_address[2:6]).upper() + '_' + (end_address[-4:]).upper()

    match_address = f'{start_address}..={end_address}' 
    
    handler_path = handler_path.split('::')
    assert len(handler_path) == 2
    handler_full_path = '::'.join(['crate', 'system', handler_path[0], 'controllers', 'memory', handler_path[1] + '_' + src_handler])

    statement = f'        {match_address} => {handler_full_path}(state, address - {start_address}'
    if writing:
        statement += f', value'
    if args:
        statement += f', {args}'
    statement += f'),'
    
    return statement


records = p.get_records(name_columns_by_row=0, file_name=file_name, sheet_name='Sheet1', start_row=1)

print(f'use crate::system::types::State;')
print(f'use crate::system::bus::types::*;')

for handler in HANDLERS: 
    print(f'')

    primitive = handler.split(' ')[0]
    writing = 'write' in handler

    src_handler = '_'.join(reversed(handler.split(' ')))
    if not writing:
        print(f'pub fn bus_{src_handler}(state: &State, address: u32) -> ReadResult<{primitive}> {{')
    else:
        print(f'pub fn bus_{src_handler}(state: &State, address: u32, value: {primitive}) -> WriteResult {{')

    if primitive == 'u8':
        print(f'    assert_eq!(address % 1, 0);')
    elif primitive == 'u16':
        print(f'    assert_eq!(address % 2, 0);')
    elif primitive == 'u32':
        print(f'    assert_eq!(address % 4, 0);')
    else:
        raise ValueError(f'Invalid primitive {primitive}')

    print(f'    match address {{')

    for record in records:
        if not record[handler]:
            continue

        start_address = record['Bus address']
        length = record['Length (bytes)']
        handler_path = record['Handler function path']
        args = record['Additional arguments']
        statement = make_match_statement(src_handler, start_address, length, handler_path, args, writing)
        print(statement)

    print(f'        _ => panic!("Unhandled bus address 0x{{:08X}}", address),')

    print(f'    }}')
    print(f'}}')
