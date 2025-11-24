use std::collections::HashMap;
use std::fs;
use std::error::Error;

fn build_catalog() -> HashMap<String, Vec<String>> {
    let mut catalog: HashMap<String, Vec<String>> = HashMap::new();

    catalog.insert(
        "Lager".to_string(),
        vec![
            "33 Export".to_string(),
            "Desperados".to_string(),
            "Goldberg".to_string(),
            "Gulder".to_string(),
            "Heineken".to_string(),
            "Star".to_string(),
        ],
    );

    catalog.insert(
        "Stout".to_string(),
        vec![
            "Legend".to_string(),
            "Turbo King".to_string(),
            "Williams".to_string(),
        ],
    );

    catalog.insert(
        "Non-Alcoholic".to_string(),
        vec![
            "Maltina".to_string(),
            "Amstel Malta".to_string(),
            "Malta Gold".to_string(),
            "Fayrouz".to_string(),
        ],
    );

    catalog
}

fn convert_to_text(catalog: &HashMap<String, Vec<String>>) -> String {
    let mut output = String::new();

    // Sort categories for neat output
    let mut categories: Vec<&String> = catalog.keys().collect();
    categories.sort();

    for category in categories {
        output.push_str(&format!("{}:\n", category));

        if let Some(drinks) = catalog.get(category) {
            for drink in drinks {
                output.push_str(&format!(" - {}\n", drink));
            }
        }

        output.push_str("\n");
    }

    output
}

fn save_to_file(filename: &str, content: &str) -> Result<(), Box<dyn Error>> {
    fs::write(filename, content)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let catalog = build_catalog();
    let text = convert_to_text(&catalog);

    let filename = "drinks.txt";
    save_to_file(filename, &text)?;

    println!("Successfully saved drink categories to '{}'", filename);

    Ok(())
}
