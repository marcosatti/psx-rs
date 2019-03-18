import os

TEXT = \
"""
1F801DC0h 2  dAPF1  Reverb APF Offset 1
1F801DC2h 2  dAPF2  Reverb APF Offset 2
1F801DC4h 2  vIIR   Reverb Reflection Volume 1
1F801DC6h 2  vCOMB1 Reverb Comb Volume 1
1F801DC8h 2  vCOMB2 Reverb Comb Volume 2
1F801DCAh 2  vCOMB3 Reverb Comb Volume 3
1F801DCCh 2  vCOMB4 Reverb Comb Volume 4
1F801DCEh 2  vWALL  Reverb Reflection Volume 2
1F801DD0h 2  vAPF1  Reverb APF Volume 1
1F801DD2h 2  vAPF2  Reverb APF Volume 2
1F801DD4h 4  mSAME  Reverb Same Side Reflection Address 1 Left/Right
1F801DD8h 4  mCOMB1 Reverb Comb Address 1 Left/Right
1F801DDCh 4  mCOMB2 Reverb Comb Address 2 Left/Right
1F801DE0h 4  dSAME  Reverb Same Side Reflection Address 2 Left/Right
1F801DE4h 4  mDIFF  Reverb Different Side Reflection Address 1 Left/Right
1F801DE8h 4  mCOMB3 Reverb Comb Address 3 Left/Right
1F801DECh 4  mCOMB4 Reverb Comb Address 4 Left/Right
1F801DF0h 4  dDIFF  Reverb Different Side Reflection Address 2 Left/Right
1F801DF4h 4  mAPF1  Reverb APF Address 1 Left/Right
1F801DF8h 4  mAPF2  Reverb APF Address 2 Left/Right
1F801DFCh 4  vIN    Reverb Input Volume Left/Right
"""

BASE_DIR = './tools/Register Parser/'

NAMESPACE = 'resources.spu'

declaration_lines = []
definition_lines = []
memory_map_lines = []

splitted_text_raw = TEXT.split('\n')

splitted_text = []
for row in splitted_text_raw:
    if not row:
        continue
    row = row.split()
    address = row[0]
    size = row[1]
    name = row[2]
    description = ' '.join(row[3:])
    splitted_text.append([address, size, name, description])

for row in splitted_text:
    address = '0x{:08X}'.format(int(row[0][:-1], base=16))
    address = address[:6] + '_' + address[6:]

    size = int(row[1])

    if size == 2:
        rtype = 'B16Register'
    elif size == 4:
        rtype = 'B32Register'
    else:
        raise ValueError('Unhandled size')

    name = row[2].lower()

    declaration_lines.append(f'pub {name}: {rtype},\n')
    definition_lines.append(f'{name}: {rtype}::new(),\n')
    memory_map_lines.append(f'resources.r3000.memory_mapper.map::<u32>({address}, {size}, &mut {NAMESPACE}.{name} as *mut B8MemoryMap);\n')

with open(os.path.join(BASE_DIR, 'declarations.txt'), 'w') as f:
    f.writelines(declaration_lines)

with open(os.path.join(BASE_DIR, 'definitions.txt'), 'w') as f:
    f.writelines(definition_lines)

with open(os.path.join(BASE_DIR, 'memory_mappings.txt'), 'w') as f:
    f.writelines(memory_map_lines)
