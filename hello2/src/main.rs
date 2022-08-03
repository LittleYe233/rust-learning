fn main() {
    let usa = "Hello, world!";
    let southern_germany = "Grüß Gott!";
    let japan = "ハロー・ワールド";
    let regions = [usa, southern_germany, japan];
    for region in regions.iter() {
        println!("{}", &region);
    }
}
