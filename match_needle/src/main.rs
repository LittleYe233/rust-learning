fn main() {
    let haystack = [-1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862];

    for item in &haystack {
        let result = match item {
            1 => "hit 1!",
            2 | 5 => "hit 2 or 5!",
            14.. => "hit not less than 14!",
            _ => "miss!"
        };

        println!("{}: {}", item, result);
    }
}
