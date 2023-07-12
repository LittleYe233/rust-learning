struct S<'a> {
    x: &'a i32,
    y: &'a i32
}

fn main() {
    let x = 10;
    let r;
    {
        let y = 20;
        {
            let s = S {x: &x, y: &y};    // (i)
            r = s.x;                            // (ii)
        }
    }
}

// In the book "Programming Rust", the authors think that (i) s.y's lifetime is
// no longer than y's, and that (ii) r's lifetime is no longer than s.x's. Also
// s.x and s.y should have the same lifetime due to the definition of S, so
// there shouldn't be a valid lifetime for 'a, and Rust should panic. However,
// no compile error is raised here.
