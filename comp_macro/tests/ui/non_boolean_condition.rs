use comp_macro::comp;

fn main() {
    let _ = comp![x for x in 0..10 if "not_a_bool"];
}

