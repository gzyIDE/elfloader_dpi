pub struct ShFlags{
    pub write : bool,
    pub alloc : bool,
    pub exec  : bool,
}

impl From<u64> for ShFlags {
    fn from(flag: u64) -> Self {
        let write : bool = (flag & 0x01) == 0x01;
        let alloc : bool = (flag & 0x02) == 0x02;
        let exec  : bool = (flag & 0x04) == 0x04;
        ShFlags {
            write : write,
            alloc : alloc,
            exec  : exec,
        }
    }
}

pub struct Section {
    name: String,
    flag: ShFlags,
    pub stad: usize,
    pub size: usize,
}

impl Section {
    pub fn new(name: &str, flag: u64, stad: usize, size: usize) -> Self {
        Section {
            name : String::from(name),
            flag : flag.into(),
            stad : stad,
            size : size,
        }
    }

    #[allow(dead_code)]
    pub fn get_name(&self) -> &str {
        &self.name
    }

    #[allow(dead_code)]
    pub fn get_flag(&self) -> &ShFlags {
        &self.flag
    }
}

pub type SecList = Vec<Section>;
