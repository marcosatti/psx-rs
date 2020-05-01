import sys
import os
import csv


def main():    
    print('use crate::types::mips1::instruction::Instruction;')
    print('use crate::system::r3000::types::InstructionFn;')
    print('')

    tables = ['']
    while tables:
        table = tables.pop()
        tables += generate_lookup_fn(table)


def generate_lookup_fn(table_name):
    file_name = make_file_name(table_name)
    print(f'Parsing {file_name}', file=sys.stderr)
    file = open(file_name, 'r', newline='')
    records = list(csv.reader(file))
    headers = records[3]
    records = [dict(zip(headers, record)) for record in records[4:]]
    
    suffix = ''
    if table_name != '':
        suffix = f'_{table_name}'

    print(f'use crate::system::r3000::controllers::instruction_impl{suffix}::*;')
    print('')
    print(f'pub fn lookup{suffix}(instruction: Instruction) -> (InstructionFn, usize) {{')
    proxy_tables = generate_match(headers, 0, 'opcode', records)
    print('}')
    print('')

    return proxy_tables


def generate_match(headers, level, field, base_records):
    base_indent = (level + 1) * 4 * ' '
    print(f'{base_indent}match instruction.{field}() {{')
    indent = base_indent + 4 * ' '
    proxy_tables = []
    unique_values = set(base_record[field] for base_record in base_records)
    unique_values = sorted(unique_values, key=lambda v: int(v))
    for value in unique_values:
        records = list(filter(lambda r: r[field] == value, base_records))
        assert len(records) > 0
        if len(records) == 1:
            record = records[0]
            mnemonic = record['Mnemonic']
            proxy = record['Proxy']
            cpi = record['CPI']
            
            if proxy == 'TRUE':
                print(f'{indent}{value} => lookup_{mnemonic}(instruction),')
                proxy_tables.append(mnemonic)
            else:
                print(f'{indent}{value} => ({mnemonic}, {cpi}),')
        else:
            next_field = get_next_defined_field(headers, records, field)
            print(f'{indent}{value} => {{')
            generate_match(headers, level + 2, next_field, records)
            print(f'{indent}}},')

    print(f'{indent}_ => panic!("Unknown instruction {{:?}} (using field {field})", instruction),')
    print(f'{base_indent}}}')
    
    return proxy_tables


def get_next_defined_field(headers, records, current_field):
    next_field = headers.index(current_field) + 1
    while True:
        next_field_value = headers[next_field]
        if records[0][next_field_value]:
            break
        next_field += 1
    return next_field_value


def make_file_name(prefix):
    file_name = f'Instruction List.csv'
    if prefix:
        prefix = prefix.upper()
        file_name = f'{prefix} {file_name}'
    return os.path.join(os.path.dirname(__file__), file_name)


if __name__ == '__main__':
    main()
