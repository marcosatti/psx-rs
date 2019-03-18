pub mod timer;

use crate::types::b8_memory_mapper::B8MemoryMap;
use crate::resources::Resources;
use crate::resources::timers::timer::Timer;

pub struct Timers {
    timers: [Timer; 3],
}

impl Timers {
    pub fn new() -> Timers {
        Timers {
            timers: [
                Timer::new(), Timer::new(), Timer::new()
            ],
        }
    }
}

pub fn initialize(resources: &mut Resources) {
    for timer_index in 0..3 {
        resources.r3000.memory_mapper.map::<u32>(0x1F80_1100 + timer_index * 0x10, 4, &mut resources.timers.timers[timer_index as usize].count as *mut B8MemoryMap);
        resources.r3000.memory_mapper.map::<u32>(0x1F80_1104 + timer_index * 0x10, 4, &mut resources.timers.timers[timer_index as usize].mode as *mut B8MemoryMap);
        resources.r3000.memory_mapper.map::<u32>(0x1F80_1108 + timer_index * 0x10, 4, &mut resources.timers.timers[timer_index as usize].target as *mut B8MemoryMap);
    }
}
