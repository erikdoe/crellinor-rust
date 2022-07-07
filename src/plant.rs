
pub struct Plant {
    pub ep: u32,
}

impl Plant {

    pub fn new() -> Plant {
        Plant {
            ep: 0,
        }
    }

    pub fn with_ep(ep: u32) -> Plant {
        Plant {
            ep
        }
    }

}
