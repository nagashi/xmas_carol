/*
*********************************************************************************
*                                                                               *
* FILE: main.rs                                                                 *
*                                                                               *
* DESCRIPTION: This is one of may possible ways in which                        *
*              The 12 Days of Christmas can be coded.                           *
*              See README file for more details.                                *
*                                                                               *
* DEVELOPER: Charles E. O'Riley Jr.                                             *
* DEVELOPER PHONE: +1 (615) 983-1474                                            *
* DEVELOPER EMAIL: ceoriley@gmail.com                                           *
*                                                                               *
* RUST VERSION: 1.57.0                                                          *
* CREATED DATE-TIME: 20211216-21:50 Central Time Zone USA                       *                                                                                  #
*********************************************************************************
*/

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
    let xmas_tree = '\u{1F384}';
    let title = Red
        .bold()
        .italic()
        .underline()
        .paint("TWELVE DAYS OF CHRISTMAS!");
    println!("\n\t{}{}{}\n", xmas_tree, title, xmas_tree);

    for i in 1..=12 {
        println!("{}\n", gen_verse(i))
    }
}
