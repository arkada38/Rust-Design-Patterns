pub trait MainCourse {
    fn get_price(&self) -> f32;
    fn get_weight(&self) -> u32;
}

#[derive(Clone, Copy)]
pub struct CheapCourse {
    weight: u32,
}

impl CheapCourse {
    pub fn new(weight: u32) -> CheapCourse {
        CheapCourse { weight }
    }
}

impl MainCourse for CheapCourse {
    fn get_price(&self) -> f32 {
        9.99
    }

    fn get_weight(&self) -> u32 {
        self.weight
    }
}

#[derive(Clone, Copy)]
pub struct BusinessCourse {
    weight: u32,
}

impl BusinessCourse {
    pub fn new(weight: u32) -> BusinessCourse {
        BusinessCourse { weight }
    }
}

impl MainCourse for BusinessCourse {
    fn get_price(&self) -> f32 {
        19.99
    }

    fn get_weight(&self) -> u32 {
        self.weight
    }
}
