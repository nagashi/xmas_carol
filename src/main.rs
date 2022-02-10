use ansi_term::Colour::Red;
use ordinal::Ordinal;

const TWELVE_DAYS: [&str; 12] = [
    "Twelve drummers drumming",
    "Eleven pipers piping",
    "Ten lords a-leaping",
    "Nine ladies dancing",
    "Eight maids a-milking",
    "Seven swans a-swimming",
    "Six geese a-laying",
    "Five golden rings",
    "Four calling birds",
    "Three french hens",
    "Two turtle doves, and",
    "A partridge in a pear tree",
];

fn gen_verse(day: usize) -> String {
    let ordinal_suffix = Ordinal(day);

    let mut begin = format!("On the {ordinal_suffix} day of Christmas, my true love gave to me:");

    for line in TWELVE_DAYS.iter().skip(12 - day) {
        begin.push('\n');
        begin.push_str(line);
    }

    begin
}

fn main() {
    let xmas_tree = '\u{1F384}';
    let title = Red
        .bold()
        .italic()
        .underline()
        .paint("TWELVE DAYS OF CHRISTMAS!");
    println!("\n\t{xmas_tree}{title}{xmas_tree}\n");

    for i in 1..=12 {
        println!("{}\n", gen_verse(i))
    }
}
