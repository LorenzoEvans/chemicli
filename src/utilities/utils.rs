use serde_json::Value;

pub fn create_periodic_table() -> serde_json::Value {
    let file = std::fs::read_to_string("./data/periodic_table.json")
        .expect("Was not able to open .json file.");
    
    serde_json::from_str::<Value>(&file).expect("Unable to read value.")
}
