use rand::Rng;
use rand::rngs::ThreadRng;
use rand_distr::{Distribution, LogNormal};

enum Species {
    Herbivore(Herbivore),
    Carnivore(Carnivore),
}

#[derive(Debug, PartialEq)]
pub struct Herbivore {
    pub weight: f32,
    pub age: u32,
    pub fitness: f32,
}

impl Herbivore {
    pub const W_BIRTH: f32 = 10.0;
    pub const MU: f32 = 17.0;
    pub const SIGMA_BIRTH: f32 = 4.0;
    pub const BETA: f32 = 0.05;
    pub const ETA: f32 = 0.2;
    pub const A_HALF: f32 = 2.5;
    pub const PHI_AGE: f32 = 5.0;
    pub const W_HALF: f32 = 3.0;
    pub const PHI_WEIGHT: f32 = 0.09;
    pub const GAMMA: f32 = 0.9;
    pub const ZETA: f32 = 0.22;
    pub const XI: f32 = 0.42;
    pub const OMEGA: f32 = 0.4;
    pub const F: u16 = 20;
    pub const DELTA_PHI_MAX: u8 = 10;

    pub const STRIDE: u8 = 1;

    pub const PROCREATE: f32 = Herbivore::ZETA * (Herbivore::W_BIRTH + Herbivore::SIGMA_BIRTH);

    pub fn birthweight(rng: &mut ThreadRng) -> f32 {
        let mean = f32::ln(
            Herbivore::W_BIRTH.powf(2.0)
                / (Herbivore::W_BIRTH.powf(2.0) + Herbivore::SIGMA_BIRTH.powf(2.0)).sqrt(),
        );
        let std =
            f32::ln(1.0f32 + ((Herbivore::SIGMA_BIRTH.powf(2.0)) / (Herbivore::W_BIRTH.powf(2.0))))
                .sqrt();

        let log_normal = LogNormal::new(mean, std).unwrap();
        log_normal.sample(rng)
    }

    pub fn gain_weight(&mut self, food: u16) {
        self.weight += food as f32;
    }

    pub fn aging(&mut self) {
        self.age += 1;
    }

    pub fn lose_weight_year(&mut self) {
        self.weight -= Herbivore::ETA * self.weight
    }

    pub fn lose_weight_birth(&mut self, baby_weight: f32) -> bool {
        if self.weight > Herbivore::XI * baby_weight {
            self.weight -= Herbivore::XI * baby_weight;
            self.calculate_fitness();
            true
        } else {
            false
        }
    }

    pub fn calculate_fitness(&mut self) {
        if self.weight <= 0.0 {
            self.fitness = 0.0;
        } else {
            let q_pos = (1.0
                + f32::exp(Herbivore::PHI_AGE * (self.age as f32 - Herbivore::A_HALF)))
            .powf(-1.0);

            let q_neg = (1.0
                + f32::exp(-Herbivore::PHI_WEIGHT * (self.weight - Herbivore::W_HALF)))
            .powf(-1.0);

            self.fitness = q_pos * q_neg;
        }
    }

    pub fn graze(&mut self, available_fodder: u16) -> u16 {
        if available_fodder >= Herbivore::F {
            self.gain_weight(Herbivore::F);
            Herbivore::F
        } else {
            self.gain_weight(available_fodder);
            available_fodder
        }
    }
}

#[derive(Debug)]
pub struct Carnivore {
    pub weight: f32,
    pub age: u32,
    pub fitness: f32,
}

impl Carnivore {
    pub const W_BIRTH: f32 = 6.0;
    pub const MU: f32 = 0.4;
    pub const SIGMA_BIRTH: f32 = 1.0;
    pub const BETA: f32 = 0.6;
    pub const ETA: f32 = 0.125;
    pub const A_HALF: f32 = 40.0;
    pub const PHI_AGE: f32 = 0.45;
    pub const W_HALF: f32 = 4.0;
    pub const PHI_WEIGHT: f32 = 0.28;
    pub const GAMMA: f32 = 0.8;
    pub const ZETA: f32 = 3.5;
    pub const XI: f32 = 1.1;
    pub const OMEGA: f32 = 0.3;
    pub const F: u16 = 70;
    pub const DELTA_PHI_MAX: u8 = 10;

    pub const STRIDE: u8 = 3;

    pub const PROCREATE: f32 = Carnivore::ZETA * (Carnivore::W_BIRTH + Carnivore::SIGMA_BIRTH);

    pub fn birthweight(rng: &mut ThreadRng) -> f32 {
        let mean = f32::ln(
            Carnivore::W_BIRTH.powf(2.0)
                / (Carnivore::W_BIRTH.powf(2.0) + Carnivore::SIGMA_BIRTH.powf(2.0)).sqrt(),
        );
        let std =
            f32::ln(1.0f32 + ((Carnivore::SIGMA_BIRTH.powf(2.0)) / (Carnivore::W_BIRTH.powf(2.0))))
                .sqrt();

        let log_normal = LogNormal::new(mean, std).unwrap();
        log_normal.sample(rng)
    }

    pub fn gain_weight(&mut self, food: u16) {
        self.weight += food as f32;
    }

    pub fn aging(&mut self) {
        self.age += 1;
    }

    pub fn lose_weight_year(&mut self) {
        self.weight -= Carnivore::ETA * self.weight
    }

    pub fn lose_weight_birth(&mut self, baby_weight: f32) -> bool {
        if self.weight > Carnivore::XI * baby_weight {
            self.weight -= Carnivore::XI * baby_weight;
            self.calculate_fitness();
            true
        } else {
            false
        }
    }

    pub fn calculate_fitness(&mut self) {
        if self.weight <= 0.0 {
            self.fitness = 0.0f32;
        } else {
            let q_pos = (1.0
                + f32::exp(Carnivore::PHI_AGE * (self.age as f32 - Carnivore::A_HALF)))
            .powf(-1.0);

            let q_neg = (1.0
                + f32::exp(-Carnivore::PHI_WEIGHT * (self.weight - Carnivore::W_HALF)))
            .powf(-1.0);

            self.fitness = q_pos * q_neg;
        }
    }

    pub fn predation(&mut self, rng: &mut ThreadRng, herbivores: &mut Vec<Herbivore>) -> u16 {
        let mut eaten: u16 = 0;
        let delta_phi_max: f32 = Carnivore::DELTA_PHI_MAX as f32;
        let mut removing: Vec<usize> = Vec::new();

        for (idx, herbivore) in herbivores.iter_mut().enumerate() {
            let herbivore_fitness = herbivore.fitness;
            let carnivore_fitness = self.fitness;
            let difference = carnivore_fitness - herbivore_fitness;

            let prob: f32;
            if carnivore_fitness <= herbivore_fitness {
                prob = 0.0;
            } else if 0.0 < difference && difference < delta_phi_max {
                prob = difference / delta_phi_max;
            } else {
                prob = 1.0;
            }

            if rng.gen::<f32>() < prob {
                removing.push(idx);

                let rest = Carnivore::F - eaten;
                let herbivore_weight: u16 = herbivore.weight as u16;

                if herbivore_weight < rest {
                    eaten += herbivore_weight;
                    self.gain_weight(herbivore_weight);
                } else {
                    self.gain_weight(rest);
                    break;
                }
            }
        }
        for idx in removing.iter().rev() {
            herbivores.remove(*idx);
        }
        eaten
    }
}
