use std::io::Write;
fn main(){
    let lager = vec!["33 Export\n","Desperados\n","Goldberg\n","Gulder\n","Heineken\n","Star\n\n"];
    let stout = vec!["Legend\n","Turbo King\n","Williams\n\n"];
    let na = vec!["Maltina\n","Amstel Malta\n","Malta Gold\n","Fayrouz\n"];

    let mut file = std::fs::File::create("drinks.txt").expect("create failed");
    file.write_all("LAGER\n".as_bytes()).expect("write failed");
    for i in 0..lager.len(){
        file.write_all(lager[i].as_bytes()).expect("write failed");
    }
    file.write_all("STOUT\n".as_bytes()).expect("write failed");
    for i in 0..stout.len(){
        file.write_all(stout[i].as_bytes()).expect("write failed");
    }
    file.write_all("NON-ALCOHOLIC\n".as_bytes()).expect("write failed");
    for i in 0..na.len(){
        file.write_all(na[i].as_bytes()).expect("write failed");
    }
    print!("\nData written to file");

}