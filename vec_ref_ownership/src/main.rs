struct Person {
    name: String,
    birth: i32,
}

fn main() {
    let mut composers = Vec::new();
    composers.push(Person {
        name: "Palestrina".to_string(),
        birth: 1525,
    });
    composers.push(Person {
        name: "Dowland".to_string(),
        birth: 1563
    });
    composers.push(Person {
        name: "Lully".to_string(),
        birth: 1632
    });
    for composer in composers {
        println!("{}, born {}", composer.name, composer.birth);
    }
    /*
        You can't write down the lines below, for the "for" block moved `composers` variable.

        To operate the variable again, use `&composers` in "for" block instead.
    */
    // composers.push(Person {
    //     name: "Test".to_string(),
    //     birth: -1
    // });
}
