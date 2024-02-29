use serde_json::Result;

mod types;
use types::get_input as get_input;
use types::load as load;
use types::save as save;

fn main() -> Result<()> {
    let mut stat_data = load("data.json");

    /*let mut new_order =
    stat_data
        .new_day("08-03".to_string())
        .new_order("12:00".to_string());
    while new_order.parse(get_input("")).is_err(){};
    new_order.add_addon();*/

    let mut curr_day = stat_data.new_day(get_input("Date"));
    let mut curr_order;
    loop {
        match get_input("1-New Day | 2-New Order | q-Exit").trim() {
            "1" => {
                curr_day = stat_data.new_day(get_input("Date"));
            }
            "2" => {
                curr_order = curr_day.new_order(get_input("Time"));
                loop {
                    match get_input("1-Fish | 2-Soup | 3-Addon | q-Exit Order").trim() {
                        "1" => curr_order.parse(get_input("Fish (Parse)")).unwrap(),
                        "2" => curr_order.add_soup(),
                        "3" => curr_order.add_addon(),
                        "q" => break,
                         _  => (),
                    }
                }
            }
            "q" => break,
             _ => (),
        }

    }
    save(stat_data,"data.json");
    Ok(())
}

    /*

    let order = stat_data
        .new_day("01-01".to_string())
        .new_order("12:00".to_string());
    order.add_fish();
    order.add_fish();
    order.add_fish();

    order.add_soup();
    order.add_soup();
    order.add_soup();

    order.add_addon();
    order.add_addon();
    order.add_addon();



    let order = stat_data
        .new_day("01-01".to_string())
        .new_order("12:00".to_string());
    order.add_fish();
    order.add_fish();
    order.add_fish();
    */

    /*
    let json_from_file = 
        match fs::read_to_string(format!("{}{}",FILE_PATH,"data.json")){
                Ok(file) => file,
                Err(_) => "{}".to_string(), };
    fs::write(
        format!("{}{}",&FILE_PATH, ".data.json.bak"),
        &json_from_file
        ).unwrap();
    let mut stat_data: types::StatData = serde_json::from_str(&json_from_file)?;
    */
    //let test: StatData = StatData { orders: None };
    //println!("{}",serde_json::to_string(&test).unwrap());

/*let z1 = Zamowienie{ 
            ryby: vec![Ryba::Dorsz,Ryba::Dorsz,Ryba::Halibut],
            czas: "12:45".to_string(),
            };*/
//let json_string = serde_json::to_string(&z1).unwrap();
//println!("{}",json_string);

//fs::write(FILE_PATH,json_string).unwrap();
    /*
    fs::write(
        format!("{}{}",FILE_PATH, "data.json"),
        serde_json::to_string(&stat_data).unwrap() 
        ).unwrap();
    */
