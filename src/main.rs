mod data;
#[allow(dead_code)]
mod decks;

use crate::{
    data::Data,
    decks::{ONE_DECK, TWO_DECK},
};
use rand::seq::SliceRandom;
use std::{fmt::Write as FmtWrite, fs::File, io::Write};
use time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();
    let mut readme = String::new();
    const SAMPLES: usize = 100_0000;
    if let Ok(o) = one_deck(4 * SAMPLES) {
        writeln!(readme, "{}", o)?
    }
    if let Ok(o) = two_deck(2 * SAMPLES) {
        writeln!(readme, "{}", o)?
    }
    let mut file = File::create("readme.md")?;
    file.write_all(readme.as_bytes())?;
    let end = Instant::now();
    println!("finished in {} s", (start - end).as_seconds_f32());
    Ok(())
}

fn one_deck(samples: usize) -> Result<String, Box<dyn std::error::Error>> {
    let mut cards17 = Data::new(1, samples * 2);
    let mut cards18 = Data::new(1, samples * 3);
    let mut cards20 = Data::new(1, samples);
    for _ in 1..=samples {
        let mut rng = rand::thread_rng();
        let mut cards = ONE_DECK.to_vec();
        cards.shuffle(&mut rng);
        cards17.check(&cards[0..17]);
        cards17.check(&cards[17..34]);
        cards20.check(&cards[34..54]);
    }
    for _ in 1..=samples {
        let mut rng = rand::thread_rng();
        let mut cards = ONE_DECK.to_vec();
        cards.shuffle(&mut rng);
        cards18.check(&cards[0..18]);
        cards18.check(&cards[18..36]);
        cards18.check(&cards[36..54]);
    }
    let mut out = String::new();
    writeln!(out, "###  三人斗地主")?;
    writeln!(out, "- 地主: 20 张")?;
    writeln!(out, "{}", cards20)?;
    writeln!(out, "- 农民: 17 张")?;
    writeln!(out, "{}", cards17)?;
    writeln!(out, "###  三人扑克")?;
    writeln!(out, "- 均分: 18 张")?;
    writeln!(out, "{}", cards18)?;
    return Ok(out);
}

fn two_deck(samples: usize) -> Result<String, Box<dyn std::error::Error>> {
    let mut cards25 = Data::new(2, samples * 3);
    let mut cards27 = Data::new(2, samples * 4);
    let mut cards33 = Data::new(2, samples);
    for _ in 1..=samples {
        let mut rng = rand::thread_rng();
        let mut cards = TWO_DECK.to_vec();
        cards.shuffle(&mut rng);
        cards25.check(&cards[0..25]);
        cards25.check(&cards[25..50]);
        cards25.check(&cards[50..75]);
        cards33.check(&cards[75..108]);
    }
    for _ in 1..=samples {
        let mut rng = rand::thread_rng();
        let mut cards = TWO_DECK.to_vec();
        cards.shuffle(&mut rng);
        cards27.check(&cards[0..27]);
        cards27.check(&cards[27..54]);
        cards27.check(&cards[54..81]);
        cards27.check(&cards[81..108]);
    }
    let mut out = String::new();
    writeln!(out, "###  四人斗地主")?;
    writeln!(out, "- 地主: 33 张")?;
    writeln!(out, "{}", cards33)?;
    writeln!(out, "- 农民: 25 张")?;
    writeln!(out, "{}", cards25)?;
    writeln!(out, "###  四人扑克")?;
    writeln!(out, "- 均分: 27 张")?;
    writeln!(out, "{}", cards27)?;
    return Ok(out);
}

/*
fn three_deck() -> Result<String, Box<dyn std::error::Error>> {
    const SAMPLES: usize = 100_0000;
    let mut cards17 = Data::new(1, SAMPLES * 4);
    let mut cards18 = Data::new(1, SAMPLES * 5);
    let mut cards20 = Data::new(1, SAMPLES);
    for _ in 1..=SAMPLES {
        let mut rng = rand::thread_rng();
        let mut cards = THREE_DECK.to_vec();
        cards.shuffle(&mut rng);
        cards17.check(&cards[0..17]);
        cards17.check(&cards[17..34]);
        cards17.check(&cards[17..34]);
        cards17.check(&cards[17..34]);
        cards20.check(&cards[34..54]);
    }
    for _ in 1..=SAMPLES {
        let mut rng = rand::thread_rng();
        let mut cards = THREE_DECK.to_vec();
        cards.shuffle(&mut rng);
        cards18.check(&cards[0..18]);
        cards18.check(&cards[18..36]);
        cards18.check(&cards[36..54]);
        cards18.check(&cards[36..54]);
        cards18.check(&cards[36..54]);
    }
    let mut out = String::new();
    writeln!(out, "地主: 20 张")?;
    writeln!(out, "{}", cards20)?;
    writeln!(out, "农民: 17 张")?;
    writeln!(out, "{}", cards17)?;
    writeln!(out, "均分: 18 张")?;
    writeln!(out, "{}", cards17)?;
    return Ok(out);
}
*/
