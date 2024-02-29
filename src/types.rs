use serde::{Serialize,Deserialize};
use std::io;
use std::fs;

const FILE_PATH:&str = "/home/mr_platypus/Documents/Code/Rust-Projects/restaurant_statistics/src/";

pub fn get_input(prompt: &str) -> String {
    let mut input = String::new();
    println!("{}",prompt);
    io::stdin().read_line(&mut input).unwrap();
    input
}
pub fn save(data: StatData ,file_name: &str) {
    fs::write(
        format!("{}{}",&FILE_PATH, file_name),
        serde_json::to_string_pretty(&data).unwrap() 
        ).unwrap();
}
pub fn load(file_name: &str) -> StatData {
    let json_from_file = 
        match fs::read_to_string(format!("{}{}",FILE_PATH,file_name)){
                Ok(file) => file,
                Err(_) => "{}".to_string(), 
        };
    fs::write(
        format!("{}{}",&FILE_PATH, ".data.json.bak"),
        &json_from_file
        ).unwrap();
    serde_json::from_str(&json_from_file).unwrap()
}

#[derive(Debug,Serialize,Deserialize)]
pub enum GenericFish {
    Patelnia,
    Kaszub,
    Piec,
    PiecMozzarella,
    Pomarancze,
}

#[derive(Debug,Serialize,Deserialize)]
pub enum Fish {
    Dorsz(GenericFish),
    Halibut(GenericFish),
    Fladra(GenericFish),
    Mietus(GenericFish),
    Turbot(GenericFish),
    Sandacz(GenericFish),
    Wegorz(GenericFish),
}

#[derive(Debug,Serialize,Deserialize,Clone,Copy)]
pub enum Frytki {
    Frytki,
    Opiekane,
    Bataty,
}
#[derive(Debug,Serialize,Deserialize,Clone,Copy)]
pub enum Surowka {
    Coleslaw,
    Marchewka,
    Wiosenna,
    Buraczki,
    Kapusta,
    Zestaw,
}
#[derive(Debug,Serialize,Deserialize,Clone,Copy)]
pub enum Addon {
    Frytki(Frytki),
    Surowka(Surowka),
    Ziemniaczki,
    Szprotki,
    Maslo,
    Sos,
}
#[derive(Debug,Serialize,Deserialize,Clone,Copy)]
pub enum Soup {
    Pomidorowa,
    Rosol,
    Rybna,
    Dnia,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct Order {
    #[serde(skip_serializing_if="Option::is_none")]
    fish: Option<Vec<Fish>>,
    #[serde(skip_serializing_if="Option::is_none")]
    soup : Option<Vec<Soup>>,
    #[serde(skip_serializing_if="Option::is_none")]
    addons: Option<Vec<Addon>>,
    time: String,
}
impl Order {
    pub fn new(time: String) -> Self {
        Order {
            fish: None,
            soup: None,
            addons: None,
            time: time.trim().to_string(),
        }
    }
    pub fn add_fish(&mut self) {
        let fish_to_add = 
        match get_input("1-Fladra | 2-Dorsz | 3-Mietus | 4-Halibut | 5-Turbot | 6-Sandacz | 7-Wegorz").trim() {
            "1" => Fish::Fladra,
            "2" => Fish::Dorsz,
            "3" => Fish::Mietus,
            "4" => Fish::Halibut,
            "5" => Fish::Turbot,
            "6" => Fish::Sandacz,
            "7" => Fish::Wegorz,
             _  => Fish::Dorsz,
        }(
        match get_input("1-Patelnia | 2-Piec | 3-PiecMozzarella | 4-Pomarancze").trim() {
            "1" => GenericFish::Patelnia,
            "2" => GenericFish::Piec,
            "3" => GenericFish::PiecMozzarella,
            "4" => GenericFish::Pomarancze,
             _  => GenericFish::Patelnia,
        });
        Self::add_fish_direct(self,fish_to_add);

        println!("{:?}",self.fish.as_ref().unwrap()[self.fish.as_ref().unwrap().len()-1]);
    }

    fn add_fish_direct(&mut self,fish_to_add: Fish) {
        match &mut self.fish {
            Some(fish) => fish.push(fish_to_add),
            None => self.fish = Some(vec![fish_to_add]),
        }
    }

    pub fn add_soup(&mut self) {
        let soup_to_add = 
        match get_input("1-Rosol | 2-Pomidorowa | 3-Rybna | 4-Dnia").trim() {
            "1" => Soup::Rosol,
            "2" => Soup::Pomidorowa,
            "3" => Soup::Rybna,
            "4" => Soup::Dnia,
             _  => Soup::Rosol,
        };
        let input = get_input("How many").chars().collect::<Vec<_>>();
        let input = input.get(0);
        let soup_count;
        if input.is_some() && input.unwrap().is_ascii_digit() {
            soup_count = input.unwrap().to_digit(10).unwrap();
        } else {
            soup_count = 1;
        }
        for _ in 0..soup_count {
            match &mut self.soup {
                Some(soup) => soup.push(soup_to_add),
                None => self.soup = Some(vec![soup_to_add]),
            }
        }
        println!("{:?}",self.soup.as_ref().unwrap()[self.soup.as_ref().unwrap().len()-1]);
    }

    pub fn add_addon(&mut self) {
        let addon_to_add = 
            match get_input("1-Frytki | 2-Surowka | 3-Ziemniaczki | 4-Szprotki | 5-Maslo | 6-Sos").trim() {
                "1" => match get_input("1-Frytki | 2-Opiekane | 3-Bataty").trim() {
                    "1" => Addon::Frytki(Frytki::Frytki),
                    "2" => Addon::Frytki(Frytki::Opiekane),
                    "3" => Addon::Frytki(Frytki::Bataty),
                     _  => Addon::Frytki(Frytki::Frytki),
                        }
                "2" => match get_input("1-Zestaw | 2-Coleslaw | 3-Buraczki | 4-Marchewka | 5-Kapusta | 6-Wiosenna").trim() {
                    "1" => Addon::Surowka(Surowka::Zestaw),
                    "2" => Addon::Surowka(Surowka::Coleslaw),
                    "3" => Addon::Surowka(Surowka::Buraczki),
                    "4" => Addon::Surowka(Surowka::Kapusta),
                    "5" => Addon::Surowka(Surowka::Wiosenna),
                     _  => Addon::Surowka(Surowka::Zestaw),
                        }
                "3" => Addon::Ziemniaczki,
                "4" => Addon::Szprotki,
                "5" => Addon::Maslo,
                "6" => Addon::Sos,
                 _  => Addon::Frytki(Frytki::Frytki),
            };
        let input = get_input("How many").chars().collect::<Vec<_>>();
        let input = input.get(0);
        let addon_count;
        if input.is_some() && input.unwrap().is_ascii_digit() {
            addon_count = input.unwrap().to_digit(10).unwrap();
        } else {
            addon_count = 1;
        }
        for _ in 0..addon_count {
            match &mut self.addons {
                Some(addons) => addons.push(addon_to_add),
                None => self.addons = Some(vec![addon_to_add]),
            }
        }
        println!("{:?}",self.addons.as_ref().unwrap()[self.addons.as_ref().unwrap().len()-1]);
    }

    pub fn parse(&mut self,token: String) -> Result<(),()> {
        let fish_kinds = vec!['f','d','m','h','t','s','w'];
        let fish_types = vec!['a','p','z','m'];
        let chars: Vec<char> = token.trim().chars().collect();

        if chars.is_empty() {
                //println!("SYNTAX ERROR empty string");
                return Err(());
        }

        let mut fish_to_add:Vec<Fish> = vec![];
        let mut fish_to_add_kind;
        let mut fish_to_add_type;

        let mut index = 0;
        while index <= chars.len()-1 {
            let mut fish_count = 1;

            if !fish_kinds.contains(&chars[index]) {
                println!("SYNTAX ERROR at:{} : ({}) is not a fish kind",index,chars[index]);
                return Err(());
            }
            if chars.get(index+1).is_none() {
                println!("SYNTAX ERROR at:{} (end) : ({}) is missing the type parameter",index,chars[index]);
                return Err(());
            }
            if !fish_types.contains(&chars[index+1]) {
                println!("SYNTAX ERROR at:{0} : {1}{2} ({2}) is not a type",index,chars[index],chars[index+1]);
                return Err(());
            }

            fish_to_add_kind = chars[index];
            fish_to_add_type = chars[index+1];

            if chars.get(index+2).is_some() && chars[index+2].is_ascii_digit() {
                fish_count = chars[index+2].to_digit(10).unwrap();
                index += 3;
            } else {
                index += 2;
            }
            for _ in 0.. fish_count {
                fish_to_add.push(Self::make_fish(fish_to_add_kind,fish_to_add_type));
            }
        }
        for f in fish_to_add {
            self.add_fish_direct(f);
        }
        Ok(())
    }

    fn make_fish(fish_kind: char, fish_type: char) -> Fish {
        let fish_to_add = 
        match fish_kind {
            'f' => Fish::Fladra,
            'd' => Fish::Dorsz,
            'm' => Fish::Mietus,
            'h' => Fish::Halibut,
            't' => Fish::Turbot,
            's' => Fish::Sandacz,
            'w' => Fish::Wegorz,
             _  => Fish::Dorsz,
        }(
        match fish_type {
            'a' => GenericFish::Patelnia,
            'p' => GenericFish::Piec,
            'z' => GenericFish::PiecMozzarella,
            'm' => GenericFish::Pomarancze,
             _  => GenericFish::Patelnia,
        });
        fish_to_add
    }
}

#[derive(Debug,Serialize,Deserialize)]
pub struct Day {
    #[serde(skip_serializing_if="Option::is_none")]
    orders: Option<Vec<Order>>,
    date: String,
}
impl Day {
    fn new(date: String) -> Self {
        Day {
            orders: None,
            date: date.trim().to_string(),
        }
    }
    pub fn new_order(&mut self,time: String) -> &mut Order{
        let new_order = Order::new(time);
        //let new_order_ref = &mut new_order;
        match &mut self.orders {
            Some(orders) => orders.push(new_order),
            None => self.orders = Some(vec![new_order]),
        };
        let len = self.orders.as_ref().unwrap().len()-1;
        let new_order_ref = &mut self.orders.as_mut().unwrap()[len];
        new_order_ref
    }
}

#[derive(Debug,Serialize,Deserialize)]
pub struct StatData {
    #[serde(skip_serializing_if="Option::is_none")]
    days: Option<Vec<Day>>
}
impl StatData {
    fn new() -> Self {
        StatData {
            days: None,
        }
    }
    pub fn new_day(&mut self,date: String) -> &mut Day {
        let mut new_day = Day::new(date);
        match &mut self.days {
            Some(days) => days.push(new_day),
            None => self.days = Some(vec![new_day]),
        };
        let len = self.days.as_ref().unwrap().len()-1;
        let new_day_ref = &mut self.days.as_mut().unwrap()[len];
        new_day_ref
    }
}
