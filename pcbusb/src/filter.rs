use embedded_can::Id;

pub struct Filter {
    pub accept_all: bool,
    pub is_extended: bool,
    pub id: u32,
    pub mask: u32,
}

impl Filter {
    pub fn accept_all() -> Self {
        // TODO: Fix
        Self {
            accept_all: true,
            is_extended: true,
            id: 0,
            mask: 0,
        }
    }

    pub fn new(id: Id) -> Self {
        match id {
            Id::Standard(id) => Self {
                accept_all: false,
                is_extended: false,
                id: id.as_raw() as u32,
                mask: 0x7FF,
            },
            Id::Extended(id) => Self {
                accept_all: false,
                is_extended: true,
                id: id.as_raw(),
                mask: 0x1FFF_FFFF,
            },
        }
    }

    pub fn with_mask(&mut self, mask: u32) -> &mut Self {
        self.mask = mask;
        self
    }
}
