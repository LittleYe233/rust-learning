mod filestruct;

fn main() {
    let f4_data: Vec<u8> = vec![114, 117, 115, 116, 33];
    let mut f4 = filestruct::File::new_with_data("f4.txt", &f4_data);

    let mut buffer: Vec<u8> = vec![];

    f4 = filestruct::open(f4).unwrap();
    let f4_length = f4.read(&mut buffer).unwrap();
    f4 = filestruct::close(f4).unwrap();

    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", f4);
    println!("{} is {} byte(s) long", &f4.name, f4_length);
    println!("{}", text);
}
