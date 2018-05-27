// Domains for the entity

extern crate rand;
use rand::Rng;


#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Domain {
    water: u8,
    earth: u8,
    fire: u8,
    air: u8,

    // TODO: Mudar para enum
    primary: Option<usize>,
    secundary: Option<usize>,
}

impl Domain {
    pub fn new() -> Domain {
        Domain {
            water: 0,
            earth: 0,
            fire: 0,
            air: 0,
            primary: None,
            secundary: None,
        }
    }

    pub fn get_water(&self) -> u8 { self.water }
    pub fn get_earth(&self) -> u8 { self.earth }
    pub fn get_fire(&self) -> u8 { self.fire }
    pub fn get_air(&self) -> u8 { self.air }
    pub fn get_primary(&self) -> Option<usize> { self.primary }
    pub fn get_secundary(&self) -> Option<usize> { self.secundary }

    pub fn gen_domain() -> Domain {
        debug!("gen_domain");

        let mut d = Domain::new();

        let primary_domain = rand::thread_rng().gen_range(0, 3);
        let primary_level = rand::thread_rng().gen_range(0, 255);
        println!("primary: {:?}", (primary_domain, primary_level));
        d.primary = Some(primary_domain);

        let sec_d  = rand::thread_rng().gen_range(0, 1);
        let sec_l = rand::thread_rng().gen_range(0, 127);
        let ter_l = rand::thread_rng().gen_range(0, 63);

        // TODO: Melhorar reusabilidade
        match primary_domain {
            0 => {
                d.water = primary_level;
                if sec_d == 0 {
                    d.secundary = Some(1);
                    d.earth = sec_l;
                } else {
                    d.secundary = Some(3);
                    d.air = ter_l
                };
            },

            1 => {
                d.earth = primary_level;
                if sec_d == 0 {
                    d.secundary = Some(2);
                    d.fire = sec_l;
                } else {
                    d.secundary = Some(0);
                    d.water = ter_l
                };
            },

            2 => {
                d.fire = primary_level;
                if sec_d == 0 {
                    d.secundary = Some(3);
                    d.air = sec_l;
                } else {
                    d.secundary = Some(1);
                    d.earth = ter_l
                };
            },

            3 => {
                d.air = primary_level;
                if sec_d == 0 {
                    d.secundary = Some(0);
                    d.water = sec_l;
                } else {
                    d.secundary = Some(2);
                    d.fire = ter_l
                };
            },

            _ => panic!(),
        }



        d
    }
}
