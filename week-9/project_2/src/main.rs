use std::io::Write;
fn main(){
    let sn = vec!["Student Name\t","Oluchi Mordi\t\t\t","Adams Aliyu\t\t\t","Shania Bolade\t\t","Adekunle Gold\t","Bianca Edemoh\t"];
    let mn = vec!["Matric No\t\t.","ACC10211111\t","ECO101110101\t","CSC10328828\t","EEE11020202\t","MEE10202001\t"];
    let d = vec!["Department\t","Accounting\t","Economics\t","Computer\t\t","Electrical\t\t","Mechanical\t"];
    let lv = vec!["Level","300","100","200","200","100"];
    for i in 0..sn.len(){
        let smis = format!("\nS/N {}\nStudent Name {}\nMatric No {}\nDepartment {}\nLevel",
            i + 1, sn[i],mn[i],d[i],lv[i]);
    }

    let mut file = std::fs::File::create("SMIS.ext").expect("create failed");
    file.write_all("\t\t\tPAU SMIS\n".as_bytes()).expect("write failed");
    file.write_all(smis.as_bytes()).expect("write failed");
}