use rand::prelude::*;
#[derive(Debug, PartialEq, Eq)]
pub enum CatStatus {
    Alive,
    Sick,
    Death,
}

fn melapelas(a,){
    a + b
}

// impl PartialEq for CatStatus{
//     fn eq(&self, _: &CatStatus) -> bool { todo!() }
// }

pub struct Cat {
    pub name: String,
    // status: String,
    pub status: CatStatus, // pero agregar aqui
    pub stats: Stats,  
    //sexuality: gai,    // cuando se implemente
                       // alive: bool
}

// trait Status {
//     fn calculatestatus(&self) -> String{
//         let yomama = String::default();
//         yomama
//     }
// }
#[cfg(test)]
impl Cat {
    pub fn new(name: String) -> Cat { // 
        Cat {
            name,
            status: CatStatus::Alive,
            stats: Stats::new(),
        }
    }
    pub fn check_status(&mut self) {
        // donde deberia estar el tick en esto o en el loop arriba?
        let health = self.stats.health; // no
        if health == 0 {
            // self.status = String::from("death"); // no necesita returns
            self.status = CatStatus::Death;
        } else if health > 0 && health < 50 {
            // self.status = String::from("sick"); // no necesita returns
            self.status = CatStatus::Sick;
        } else {
            // self.status = String::from("Alive"); // *carita sonrojada*
            self.status = CatStatus::Alive;
        }
    }
    // como es vivir?
    pub fn live(&mut self) {
        // da hambre sip esa es la idea
        self.stats.tick(); // tenes copilot o que es lo que auto completa?
    }
    pub fn feed(&mut self) {
        self.stats.feed();
    }
}

pub struct Stats {
    pub health: u8,
    pub hambre: u8,
    // dream on: u8,
    // tired: u8,
}

impl Stats {
    fn new() -> Stats {
        // por ahora eso
        Stats {
            health: 20, // si com lo estopeamos? si llega a cero bueno
            hambre: 10,  // no
        }
    }
    fn feed(&mut self) {
        self.hambre -= 5;
    }
    fn tick(&mut self) {
        if self.health > 0 {
            self.health -= 1;
            let _num = randnum();
            if _num % 2u8 == 0 {
                self.hambre += 1;
            }
            // self.hambre += 1;
        }
    }
}

fn randnum() -> u8 {
    let rand_number: u8 = rand::thread_rng().gen_range(1..=10);

    rand_number
}

#[cfg(test)]
mod tests {
    // Import the necessary testing module
    use super::*;

    #[test]
    fn test_addition() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_subtraction() {
        assert_eq!(5 - 3, 2);
    }
}
