use rand::Rng;

pub struct Game {
    principal: u128,
    stake: u128,
    ini: u128,
}

impl Game {
    pub fn new(principal: u128, stake: u128) -> Game {
        Game {
            principal: principal,
            stake: stake,
            ini: stake,
        }
    }

    pub fn play(&mut self) -> bool {
        if rand::thread_rng().gen_range(0,2)==rand::thread_rng().gen_range(0,2) {
            self.principal = self.stake + self.principal;
            self.stake = self.ini;
            return true;
        }
        else {
            if self.stake > self.principal{
                return false;
            }
            self.principal = self.principal - self.stake;
            self.stake = (self.stake * 2) + self.ini;
            return false;
        }
    }

    pub fn get_principal(&self) -> u128 {
        self.principal
    }

    pub fn get_stake(&self) -> u128 {
        self.stake
    }
}