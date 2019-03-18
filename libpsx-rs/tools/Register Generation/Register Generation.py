import os

BASE_DIR = './tools/Register Generation/'

NAMESPACE = 'resources.spu'

CHANNELS = [
    'voice' + str(i)
    for i in range(24)
]

REGISTERS = {
    'vollr': ('B32Register', 4, 0x1F801C00, 0x10),
    'srate': ('B16Register', 2, 0x1F801C04, 0x10),
    'saddr': ('B16Register', 2, 0x1F801C06, 0x10),
    'adsr': ('B32Register', 4, 0x1F801C08, 0x10),
    'cvol': ('B16Register', 2, 0x1F801C0C, 0x10),
    'raddr': ('B16Register', 2, 0x1F801C0E, 0x10),
}

declaration_lines = []
definition_lines = []
memory_map_lines = []
for i, channel in enumerate(CHANNELS):
    for register, info in REGISTERS.items():
        name = f'{channel}_{register}'
        declaration_lines.append(f'pub {name}: {info[0]},\n')
        definition_lines.append(f'{name}: {info[0]}::new(),\n')
        address = '0x{:08X}'.format(info[2] + i * info[3])
        address = address[:6] + '_' + address[6:]
        memory_map_lines.append(f'resources.r3000.memory_mapper.map::<u32>({address}, {info[1]}, &mut {NAMESPACE}.{name} as *mut B8MemoryMap);\n')
    declaration_lines.append('\n')

with open(os.path.join(BASE_DIR, 'declarations.txt'), 'w') as f:
    f.writelines(declaration_lines)

with open(os.path.join(BASE_DIR, 'definitions.txt'), 'w') as f:
    f.writelines(definition_lines)

with open(os.path.join(BASE_DIR, 'memory_mappings.txt'), 'w') as f:
    f.writelines(memory_map_lines)
