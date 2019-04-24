pub fn verse(n: i32) -> String {
    let mut ver = String::new();

    if n > 1 {
        ver.push_str(&format!(
            "{} bottles of beer on the wall, \
             {} bottles of beer.\n",
            n, n
        ));
        if n > 2 {
            ver.push_str(&format!(
                "Take one down and pass it around, \
                 {} bottles of beer on the wall.\n",
                n - 1
            ));
        } else {
            ver.push_str(
                &"Take one down and pass it around, \
                  1 bottle of beer on the wall.\n",
            );
        }
    } else if n == 1 {
        ver.push_str(&format!(
            "{} bottle of beer on the wall, \
             {} bottle of beer.\n",
            n, n
        ));
        ver.push_str(
            &"Take it down and pass it around, \
              no more bottles of beer on the wall.\n",
        );
    } else {
        ver.push_str(
            &"No more bottles of beer on the wall, \
              no more bottles of beer.\n",
        );
        ver.push_str(
            &"Go to the store and buy some more, \
              99 bottles of beer on the wall.\n",
        );
    }

    ver
}

pub fn sing(start: i32, end: i32) -> String {
    let mut song = String::new();

    for v in (end..=start).rev() {
        song.push_str(&verse(v));
        if v != end {
            song.push_str(&"\n");
        }
    }

    song
}
