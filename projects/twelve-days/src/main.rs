mod lyric;

// ======================================================================

fn main() {
    let mut lyric_lines: Vec<String> = Vec::new();

    println!("{}", print_lyric(&lyric_lines));
    for section in lyric::LYRIC {
        lyric_lines.insert(0, section.to_string());
        println!("{}", print_lyric(&lyric_lines));
    }
}

fn print_lyric(lyric_lines: &[String]) -> String {
    let lyric_text = lyric_lines.join("\n");
    return format!(
        "On the first day of Christmas,\nmy true love gave to me\n{lyric_text}\nA partridge in a pear tree.\n"
    );
}
