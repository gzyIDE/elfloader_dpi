use elfloader::*; 
use crate::memory::{MemPages, Memory};
use crate::section::{Section, SecList};


struct Loader {
    _vbase: u64,
}

impl ElfLoader for Loader {
    fn allocate(&mut self, _load_headers: LoadableHeaders) -> Result<(), ElfLoaderErr> {
        Ok(())
    }

    fn relocate(&mut self, _entry: RelocationEntry) -> Result<(), ElfLoaderErr> {
        Ok(())
    }

    fn load(&mut self, _flags: Flags, _base: VAddr, _region: &[u8]) -> Result<(), ElfLoaderErr> {
        Ok(())
    }

    fn tls(
        &mut self,
        _tdata_start: VAddr,
        _tdata_length: u64,
        _total_size: u64,
        _align: u64
    ) -> Result<(), ElfLoaderErr> {
        Ok(())
    }
}

pub fn loadelf(fname: &str, memory: &mut MemPages, section: &mut SecList) -> usize {
    const FLAG_ALLOC: u64 = 0x02;
    let mut loader = Loader {_vbase: 0x0000_0000};
    let binary_blob = std::fs::read(fname).expect(&format!("Error: File ({}) does not exist", fname));
    let binary = ElfBinary::new(binary_blob.as_slice()).expect("Got proper ELF file");
    binary.load(&mut loader).expect("Error: ELF load failed");

    // locate binary to MemPool
    for s in binary.file.section_iter() {
        if let Ok(name) = s.get_name(&binary.file) {
            let flag  = s.flags();
            let htype = s.get_type().unwrap();
            let allocatable = (flag & FLAG_ALLOC) == FLAG_ALLOC;
            let progbits = match htype {
                xmas_elf::sections::ShType::ProgBits => true,
                _ => false,
            };
            let size  : usize = s.size() as usize;
            let fst   : usize = s.offset() as usize;
            let stad  : usize = s.address() as usize;
            section.push(Section::new(name, flag, stad, size));

            if allocatable { // allocatable
                if progbits {
                    let fend  : usize = fst + size;
                    let endad : usize = stad + size;
                    for (fad, mad) in (fst..fend).zip(stad..endad) {
                        memory.write8(mad, binary.file.input[fad]);
                    }
                } else {
                    let fend  : usize = fst + size;
                    let endad : usize = stad + size;
                    for (_, mad) in (fst..fend).zip(stad..endad) {
                        memory.write8(mad, 0);
                    }
                }
            }
        }
    }

    // return entry point
    binary.entry_point() as usize
}
