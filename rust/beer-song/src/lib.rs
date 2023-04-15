pub fn verse(n: u32) -> String {
    match n {
        0 => String::from("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n"),
        1 => String::from("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n"),
        n => format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} {} of beer on the wall.\n", n, n, n - 1, if n - 1 > 1 { "bottles" } else { "bottle" })
    }
}

pub fn sing(start: u32, end: u32) -> String {
    let mut song = String::new();
    let dist = start - end + 1;
    for n in 0..dist {
        if n > 0 {
            song.push_str("\n");
        }
        song.push_str(&verse(start - n));
    }
    song
}
