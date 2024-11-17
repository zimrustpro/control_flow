fn main() {
    let first = (200, 0, 0);
    let second = (50, 50, 50);
    let third = (200, 50, 0);

    match_colors(first);
    match_colors(second);
    match_colors(third);
}

fn match_colors(rbg: (i32, i32, i32)) {
    let (red, blue, green) = (rbg.0, rbg.1, rbg.2);
    println!("Comparing a color with {} red {} blue and {} green", red, blue, green);
    let color_vec = vec![(red, "red"), (blue, "blue"), (green, "green")];
    let mut all_have_at_least_10 = true;
    for (amount, color) in color_vec {
        if amount < 10 {
            all_have_at_least_10 = false;
            println!("Not much {}", color);
        }
    }
    if all_have_at_least_10 {
        println!("Each color has at least 10.")
    }
    println!()
}