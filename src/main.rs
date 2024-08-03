
use qr_meow::{ QrCat, QrStyle };


fn main() {
    println!("Data: ");
    let data = format!("{}", read_str());
    println!("File name: ");
    let output_path = format!("assets/{}.png", read_str().trim());

    // let ssid = "SSID";
    // let password = "PASSWORD";
    // let data = format!("WIFI:T:WPA;S:{};P:{};;", ssid, password);
    // println!("File name: ");
    // let output_path = format!("assets/{}.png", read_str().trim());

    println!("{:?} --> {}", data.as_bytes(), data.as_bytes().len());

    QrCat::new().version(4).data(&data).style(QrStyle::Rounded).to_png(&output_path);
}



fn read_str() -> String {
    let mut input = String::new();

    std::io::stdin().read_line(&mut input).expect("Failed");

    return input;
}
