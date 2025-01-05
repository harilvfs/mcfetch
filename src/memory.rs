use sysinfo::System;

pub struct MemoryInfo {
    pub total: String,
    pub used: String,
}

pub fn get_memory_info() -> MemoryInfo {
    let mut system = System::new_all();
    system.refresh_memory();

    let total_memory = system.total_memory();
    let used_memory = system.used_memory();

    MemoryInfo {
        total: format!("{} GB", total_memory / 1024 / 1024),
        used: format!("{} GB", used_memory / 1024 / 1024),
    }
}
