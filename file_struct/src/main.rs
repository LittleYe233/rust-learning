mod filestruct;

static mut ERROR: i32 = 0;

fn main() {
    // f1
    let f1 = filestruct::File {
        name: String::from("f1.txt"),
        data: Vec::new(),
    };

    let f1_name = &f1.name;
    let f1_length = f1.data.len();

    println!("{:?}", f1);
    println!("{} is {} byte(s) long.", f1_name, f1_length);

    // f2
    let f2 = filestruct::File::new("f2.txt");

    let f2_name = &f2.name;
    let f2_length = f2.data.len();

    println!("{:?}", f2);
    println!("{} is {} byte(s) long.", f2_name, f2_length);

    // f3
    let f3_data: Vec<u8> = vec![114, 117, 115, 116, 33];
    let mut f3 = filestruct::File::new_with_data("f3.txt", &f3_data);
    let mut buffer: Vec<u8> = vec![];

    filestruct::open(&mut f3);
    let f3_length = f3.read(&mut buffer);
    filestruct::close(&mut f3);
    let f3_name = &f3.name;

    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", f3);
    println!("{} is {} byte(s) long.", f3_name, f3_length);
    println!("{}", text);

    // f4
    let mut f4 = filestruct::File::new("f4.txt");

    f4.read(&mut buffer);
    unsafe {
        if ERROR != 0 {
            panic!("An error has occurred while reading the file");
        }
    }

    filestruct::close(&mut f4);
    unsafe {
        if ERROR != 0 {
            panic!("An error has occurred while closing the file");
        }
    }
}
