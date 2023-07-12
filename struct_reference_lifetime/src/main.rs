// Must add explicit lifetime parameters.
struct S<'a> {
    r: &'a i32
}

fn main() {
    let s;
    {
        let x = 10;
        s = S {r: &x};          // Now s has the same lifetime to x's.
        assert_eq!(*s.r, 10);
    }
}
