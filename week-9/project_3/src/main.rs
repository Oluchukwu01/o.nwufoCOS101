use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // ----------- DATASETS (as arrays) -----------
    let commissioners = [
        "Aigbogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbona",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etieye",
    ];

    let zones = [
        "South West",
        "North East",
        "South South",
        "South West",
        "South East",
    ];

    let ministries = [
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum",
    ];

    // ----------- MERGE INTO A SINGLE VECTOR -----------
    let mut merged_output = Vec::new();

    for i in 0..commissioners.len() {
        let record = format!(
            "S/N {}\nName: {}\nGeopolitical Zone: {}\nMinistry: {}\n",
            i + 1,
            commissioners[i],
            zones[i],
            ministries[i]
        );
        merged_output.push(record);
    }

    // ----------- DISPLAY MERGED DATA -----------
    println!("=== MERGED EFCC MINISTER DATASET ===\n");

    for entry in &merged_output {
        println!("{}", entry);
    }

    Ok(())
}
