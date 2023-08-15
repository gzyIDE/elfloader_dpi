use std::collections::HashMap;

// Word address to page offset, memsel, tag
fn addr_dec(ad: usize) -> (usize, usize) {
    // ad: word address
    let laddr   = ad & 0xfff;
    let tag     = ad >> 12;
    (laddr, tag)
}

pub type MemPages = HashMap<usize, Box<[u32; 4096]>>;

// Trait
pub trait Memory {
    fn new() -> Self;
    fn write64(&mut self, ad: usize, dt: u64);
    fn write32(&mut self, ad: usize, dt: u32);
    fn write16(&mut self, ad: usize, dt: u16);
    fn write8(&mut self,  ad: usize, dt: u8);
    fn read64(&self, ad: usize) -> u64;
    fn read32(&self, ad: usize) -> u32;
    fn read16(&self, ad: usize) -> u16;
    fn read8(&self,  ad: usize) -> u8;
}

impl Memory for MemPages {
    fn new() -> Self {
        Default::default()
    }

    // ad: byte address
    fn write64(&mut self, ad: usize, dt: u64) {
        let ldt : u32 = (dt & 0xffffffff) as u32;
        let hdt : u32 = ((dt >> 32) & 0xffffffff) as u32;

        let dwad         = ad >> 3;   // double word address
        let wad          = dwad << 1; // word address
        let (laddr, tag) = addr_dec(wad);
        let page         = self.get_mut(&tag);
        match page {
            Some(x) => {
                x[laddr]   = ldt;
                x[laddr+1] = hdt;
            },
            None    => {
                let mut new_page  = Box::new([0; 4096]);
                new_page[laddr]   = ldt;
                new_page[laddr+1] = hdt;
                self.insert(tag, new_page);
            },
        }
    }

    // ad: byte address
    fn write32(&mut self, ad: usize, dt: u32) {
        let wad          = ad >> 2;
        let (laddr, tag) = addr_dec(wad);
        let page         = self.get_mut(&tag);
        match page {
            Some(x) => {
                x[laddr] = dt;
            },
            None    => {
                let mut new_page = Box::new([0; 4096]);
                new_page[laddr]  = dt;
                self.insert(tag, new_page);
            },
        }
    }

    // ad: byte address
    fn write16(&mut self, ad: usize, dt: u16) {
        let hwad  = ad >> 1;
        let wad   = hwad >> 1;
        let shift = (hwad & 0x1) * 16;

        let wdt  : u32   = (dt as u32) << shift;
        let mask : u32   = 0xffffffff ^ (0xffff << shift);
        let (laddr, tag) = addr_dec(wad);
        let page         = self.get_mut(&tag);
        match page {
            Some(x) => {
                x[laddr] = (x[laddr] & mask) | wdt;
            },
            None    => {
                let mut new_page = Box::new([0; 4096]);
                new_page[laddr]  = wdt;
                self.insert(tag, new_page);
            },
        }
    }

    // ad: byte address
    fn write8(&mut self, ad: usize, dt: u8) {
        let wad   = ad >> 2;
        let shift = (ad & 0x3) * 8;

        let wdt  : u32   = (dt as u32) << shift;
        let mask : u32   = 0xffffffff ^ (0xff << shift);
        let (laddr, tag) = addr_dec(wad);
        let page         = self.get_mut(&tag);
        match page {
            Some(x) => {
                x[laddr] = (x[laddr] & mask) | wdt;
            },
            None    => {
                let mut new_page = Box::new([0; 4096]);
                new_page[laddr]  = wdt;
                self.insert(tag, new_page);
            },
        }
    }

    // ad: byte address
    fn read64(&self, ad: usize) -> u64 {
        let dwad         = ad >> 3;
        let wad          = dwad << 1;
        let (laddr, tag) = addr_dec(wad);
        let page         = self.get(&tag);
        match page {
            Some(x) => {
                let ldt : u64 = x[laddr] as u64;
                let hdt : u64 = x[laddr+1] as u64;
                (hdt << 32) | ldt
            },
            None    => 0,
        }
    }

    // ad: word address
    fn read32(&self, ad: usize) -> u32 {
        let wad          = ad >> 2;
        let (laddr, tag) = addr_dec(wad);
        let page         = self.get(&tag);
        match page {
            Some(x) => x[laddr],
            None    => 0,
        }
    }

    // ad : half word address
    fn read16(&self, ad: usize) -> u16 {
        let hwad  = ad >> 1;
        let wad   = hwad >> 1;
        let shift = (hwad & 0x1) * 16;

        let (laddr, tag) = addr_dec(wad);
        let page         = self.get(&tag);
        match page {
            Some(x) => {
                ((x[laddr] >> shift) & 0xffff) as u16
            },
            None    => 0,
        }
    }

    // ad : byte address
    fn read8(&self, ad: usize) -> u8 {
        let wad = ad >> 2;
        let shift = (ad & 0x3) * 8;

        let (laddr, tag) = addr_dec(wad);
        let page         = self.get(&tag);
        match page {
            Some(x) => {
                ((x[laddr] >> shift) & 0xff) as u8
            },
            None    => 0,
        }
    }
}
