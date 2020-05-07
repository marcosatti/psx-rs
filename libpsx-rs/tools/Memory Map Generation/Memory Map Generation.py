import sys
import os
import csv

HANDLERS = [
    'u8 read', 
    'u8 write',
    'u16 read',
    'u16 write',
    'u32 read',
    'u32 write',
]

def main():
    file_name = os.path.join(os.path.dirname(__file__), 'Memory Mappings.csv')
    print(f'Parsing {file_name}', file=sys.stderr)

    file = open(file_name, 'r', newline='')
    records = list(csv.reader(file))
    headers = records[1]

    print('use crate::system::types::State;')
    print('use crate::system::bus::types::*;')

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
            print('    assert_eq!(address % 1, 0);')
        elif primitive == 'u16':
            print('    assert_eq!(address % 2, 0);')
        elif primitive == 'u32':
            print('    assert_eq!(address % 4, 0);')
        else:
            raise ValueError(f'Invalid primitive {primitive}')

        print('    match address {')

        for record in records[2:]:
            record = dict(zip(headers, record))

            if not record[handler] == 'TRUE':
                continue

            start_address = record['Bus address']
            length = record['Length (bytes)']
            handler_path = record['Handler function path']
            args = record['Additional arguments']
            statement = make_match_statement(src_handler, start_address, length, handler_path, args, writing)
            print(statement)

        print('        _ => panic!("Unhandled bus address 0x{:08X}", address),')

        print('    }')
        print('}')


def make_match_statement(src_handler, start_address, length, handler_path, args, writing):
    end_address_int = (int(start_address, base=16) + int(length, base=16)) - 1
    end_address = '{:08X}'.format(end_address_int)
    end_address = '0x' + (end_address[:4]).upper() + '_' + (end_address[-4:]).upper()

    match_address = f'{start_address}..={end_address}' 
    
    handler_path = handler_path.split('::')
    assert len(handler_path) == 2
    handler_full_path = '::'.join(['crate', 'system', handler_path[0], 'memory', handler_path[1] + '_' + src_handler])

    statement = f'        {match_address} => {handler_full_path}(state, address - {start_address}'
    if writing:
        statement += ', value'
    if args:
        statement += f', {args}'
    statement += '),'
    
    return statement


if __name__ == '__main__':
    main()
