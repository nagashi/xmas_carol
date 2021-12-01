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

    let mut begin = format!(
        "On the {} day of Christmas, my true love gave to me:",
        ordinal_suffix
    );

    for line in TWELVE_DAYS.iter().skip(12 - day) {
        begin.push('\n');
        begin.push_str(line);
    }

    begin
}

fn main() {
    for i in 1..=12 {
        println!("{}\n", gen_verse(i))
    }
}
