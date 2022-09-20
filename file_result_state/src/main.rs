mod filestruct;

fn main() {
    let mut f5 = filestruct::File::new("5.txt");

    let mut buffer: Vec<u8> = vec![];

    if f5.read(&mut buffer).is_err() {
        println!("Error checking is working");
    }

    f5 = filestruct::open(f5).unwrap();
    let f5_length = f5.read(&mut buffer).unwrap();
    f5 = filestruct::close(f5).unwrap();

    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", f5);
    println!("{} is {} byte(s) long", &f5.name, f5_length);
    println!("{}", text);
}
