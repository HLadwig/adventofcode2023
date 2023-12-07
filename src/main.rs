use std::{cmp::Ordering, collections::HashSet};

#[test]
fn day1tests() {
    let data = std::fs::read_to_string(
        "C:\\Users\\Hagen\\RustProjects\\adventofcode2023\\data\\day1test.txt",
    )
    .expect("Data for day1-Test not found");

    assert_eq!(day1_1(&data), 142);

    let data2 = std::fs::read_to_string(
        "C:\\Users\\Hagen\\RustProjects\\adventofcode2023\\data\\day1_2test.txt",
    )
    .expect("Data for day1_2-Test not found");

    assert_eq!(day1_2(&data2), 281);
}

fn get_number_from_line(line: &str) -> i32 {
    let mut firstdigit = ' ';
    let mut lastdigit = ' ';
    for c in line.chars() {
        if !c.is_numeric() {
            continue;
        }
        if firstdigit == ' ' {
            firstdigit = c;
        }
        lastdigit = c;
    }
    let mut strnumber: String = String::from(firstdigit);
    strnumber.push(lastdigit);
    strnumber.parse().unwrap()
}

fn replace_word_numbers(input: &str) -> i32 {
    let mut line = String::from(input);
    let numberwords = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut smallest_index = 0;
    for c in line.chars() {
        smallest_index += 1;
        if c.is_numeric() {
            break;
        }
    }
    let mut found_word = "";
    let mut number = 0;
    let mut counter = 0;
    for word in &numberwords {
        counter += 1;
        match line.find(word) {
            Some(x) => {
                if x < smallest_index {
                    smallest_index = x;
                    found_word = word;
                    number = counter;
                }
            }
            None => continue,
        };
    }
    if number != 0 {
        line = line.replace(found_word, number.to_string().as_str());
    }

    let mut largest_index = 0;
    let mut count = 0;
    for c in line.chars() {
        if c.is_numeric() {
            largest_index = count;
        }
        count += 1;
    }
    found_word = "";
    number = 0;
    counter = 0;
    for word in &numberwords {
        counter += 1;
        match line.rfind(word) {
            Some(x) => {
                if x > largest_index {
                    largest_index = x;
                    found_word = word;
                    number = counter;
                }
            }
            None => continue,
        };
    }
    if number != 0 {
        line = line.replace(found_word, number.to_string().as_str());
    }
    get_number_from_line(line.as_str())
}

fn day1_1(input: &str) -> i32 {
    input.lines().map(get_number_from_line).sum()
}

fn day1_2(input: &str) -> i32 {
    input.lines().map(replace_word_numbers).sum()
}

fn day1() {
    let data =
        std::fs::read_to_string("C:\\Users\\Hagen\\RustProjects\\adventofcode2023\\data\\day1.txt")
            .expect("Data for day1-Problem not found");

    println!("{}", day1_1(&data));
    println!("{}", day1_2(&data));
}

#[test]
fn day2tests() {
    let data = std::fs::read_to_string(
        "C:\\Users\\Hagen\\RustProjects\\adventofcode2023\\data\\day2test.txt",
    )
    .expect("Data for day2-Test not found");

    assert_eq!(day2_1(&data), 8);
    assert_eq!(day2_2(&data), 2286);
}

#[derive(Debug)]
struct Day2Pull {
    red: i32,
    green: i32,
    blue: i32,
}

impl Day2Pull {
    fn new(value: &str) -> Self {
        let parts = value.split(", ");
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        parts.for_each(|x| {
            let mut countcolor = x.trim().split(' ');
            let count: i32 = countcolor.next().unwrap().parse().unwrap();
            let color = countcolor.next().unwrap();
            match color {
                "red" => red = count,
                "green" => green = count,
                "blue" => blue = count,
                _ => (),
            }
        });
        Self { red, green, blue }
    }
}

#[derive(Debug)]
struct Day2Game {
    id: i32,
    pulls: Vec<Day2Pull>,
}

impl Day2Game {
    fn new(line: &str) -> Self {
        let mut parts = line.split(':');
        let id = parts.next().unwrap().replace("Game ", "").parse().unwrap();
        let pullsparts = parts.next().unwrap().split("; ");
        let pulls: Vec<Day2Pull> = pullsparts.map(Day2Pull::new).collect();
        Self { id, pulls }
    }
}

fn day2_1(input: &str) -> i32 {
    let games: Vec<Day2Game> = input.lines().map(Day2Game::new).collect();
    games.iter().map(day2game_is_possible).sum()
}

fn day2_2(input: &str) -> i32 {
    let games: Vec<Day2Game> = input.lines().map(Day2Game::new).collect();
    games.iter().map(day2game_power).sum()
}

fn day2game_is_possible(game: &Day2Game) -> i32 {
    for pull in &game.pulls {
        if pull.red > 12 || pull.green > 13 || pull.blue > 14 {
            return 0;
        }
    }
    game.id
}

fn day2game_power(game: &Day2Game) -> i32 {
    let mut redmax = 1;
    let mut greenmax = 1;
    let mut bluemax = 1;
    for pull in &game.pulls {
        if pull.red > redmax {
            redmax = pull.red;
        }
        if pull.green > greenmax {
            greenmax = pull.green;
        }
        if pull.blue > bluemax {
            bluemax = pull.blue;
        }
    }
    redmax * greenmax * bluemax
}

fn day2() {
    let data =
        std::fs::read_to_string("C:\\Users\\Hagen\\RustProjects\\adventofcode2023\\data\\day2.txt")
            .expect("Data for day2-Problem not found");

    println!("{}", day2_1(&data));
    println!("{}", day2_2(&data));
}

#[test]
fn day3tests() {
    let data = std::fs::read_to_string(
        "C:\\Users\\Hagen\\RustProjects\\adventofcode2023\\data\\day3test.txt",
    )
    .expect("Data for day3-Test not found");

    assert_eq!(day3_1(&data), 4361);
    assert_eq!(day3_2(&data), 467835);
}

#[derive(Debug)]
struct Day3Numbers {
    lineid: i32,
    positions: Vec<(i32, i32, i32)>,
}

impl Day3Numbers {
    fn new(lineid: i32, lines: &Vec<&str>) -> Self {
        let line = lines[lineid as usize];
        let mut innumber = false;
        let mut position = 0;
        let mut positions: Vec<(i32, i32, i32)> = vec![];
        let mut start = 0;
        let mut length = 0;
        for ch in line.chars() {
            if ch.is_numeric() {
                length += 1;
                if !innumber {
                    innumber = true;
                    start = position;
                }
            } else {
                if innumber {
                    positions.push((start, length, day3_ispart(lines, lineid, (start, length))));
                }
                innumber = false;
                start = 0;
                length = 0;
            }
            position += 1;
        }
        if start > 0 {
            positions.push((start, length, day3_ispart(lines, lineid, (start, length))));
        }
        Self { lineid, positions }
    }

    fn getsum(&self) -> i32 {
        self.positions
            .iter()
            .map(|x| {
                let (_, _, val) = x;
                val
            })
            .sum()
    }
}

fn day3_ispart(lines: &Vec<&str>, line: i32, position: (i32, i32)) -> i32 {
    let maxline = lines.len() as i32 - 1;
    let mut minl = if line == 0 { 0 } else { line - 1 };
    let maxl = if line == maxline { line } else { line + 1 };
    let (start, length) = position;
    let maxchar = lines.first().unwrap().len() as i32;
    let minc = if start == 0 { 0 } else { start - 1 };
    let maxc = if start + length == maxchar {
        start + length
    } else {
        start + length + 1
    };
    let number: i32 = lines[line as usize]
        .get(start as usize..(start + length) as usize)
        .unwrap()
        .parse()
        .unwrap();
    while minl <= maxl {
        let l = lines[minl as usize]
            .get(minc as usize..maxc as usize)
            .unwrap();
        for c in l.chars() {
            if !c.is_numeric() && c != '.' {
                return number;
            }
        }
        minl += 1;
    }
    0
}

fn day3_1(input: &str) -> i32 {
    let lines: Vec<&str> = input.lines().collect();
    let mut lineid = 0;
    let mut day3numbers: Vec<Day3Numbers> = vec![];
    for _line in &lines {
        day3numbers.push(Day3Numbers::new(lineid, &lines));
        lineid += 1;
    }
    day3numbers.iter().map(Day3Numbers::getsum).sum()
}

fn get_ratio(numbers: &Vec<Day3Numbers>, line: i32, column: i32) -> i32 {
    let mut possible_numbers: Vec<i32> = vec![];
    for numberline in numbers {
        if numberline.lineid < line - 1 || numberline.lineid > line + 1 {
            continue;
        }
        for pos in &numberline.positions {
            if pos.0 <= column + 1 && column - 1 < pos.0 + pos.1 {
                possible_numbers.push(pos.2);
            }
        }
    }
    if possible_numbers.len() == 2 {
        return possible_numbers.first().unwrap() * possible_numbers.last().unwrap();
    }
    0
}

fn day3_2(input: &str) -> i32 {
    let lines: Vec<&str> = input.lines().collect();
    let mut lineid = 0;
    let mut day3numbers: Vec<Day3Numbers> = vec![];
    for _line in &lines {
        day3numbers.push(Day3Numbers::new(lineid, &lines));
        lineid += 1;
    }
    lineid = 0;
    let mut charid = 0;
    let mut sum = 0;
    for line in &lines {
        for ch in line.chars() {
            if ch == '*' {
                sum += get_ratio(&day3numbers, lineid, charid);
            }
            charid += 1;
        }
        charid = 0;
        lineid += 1;
    }
    sum
}

fn day3() {
    let data =
        std::fs::read_to_string("C:\\Users\\Hagen\\RustProjects\\adventofcode2023\\data\\day3.txt")
            .expect("Data for day3-Problem not found");

    println!("{}", day3_1(&data));
    println!("{}", day3_2(&data));
}

#[test]
fn day4tests() {
    let data = std::fs::read_to_string(
        "C:\\Users\\Hagen\\RustProjects\\adventofcode2023\\data\\day4test.txt",
    )
    .expect("Data for day4-Test not found");

    assert_eq!(day4_1(&data), 13);
    assert_eq!(day4_2(&data), 30);
}

#[derive(Debug)]
struct Day4Card {
    card: i32,
    // count: i32,
    // winning_numbers: Vec<i32>,
    // actual_numbers: Vec<i32>,
    winners: i32,
}

impl Day4Card {
    fn new(line: &str) -> Self {
        let card = line.split(':').next().unwrap().replace("Card ", "");
        let mut content = line.split(": ").last().unwrap().split(" | ");
        let wn = content.next().unwrap().trim().replace("  ", " ");
        let an = content.next().unwrap().trim().replace("  ", " ");
        let winning_numbers: Vec<i32> = wn.split(' ').map(|x| x.parse().unwrap()).collect();
        let actual_numbers: Vec<i32> = an.split(' ').map(|x| x.parse().unwrap()).collect();
        let mut count = 0;
        for num in &actual_numbers {
            count += if winning_numbers.contains(num) { 1 } else { 0 };
        }
        Self {
            card: card.trim().parse().unwrap(),
            // count: 1,
            // winning_numbers,
            // actual_numbers,
            winners: count,
        }
    }
}

fn day4_1(input: &str) -> i32 {
    let cards: Vec<Day4Card> = input.lines().map(Day4Card::new).collect();
    let base: i32 = 2;
    cards
        .iter()
        .map(|x| {
            let wc = x.winners;
            if wc > 0 {
                base.pow((wc - 1) as u32)
            } else {
                0
            }
        })
        .sum::<i32>()
}

fn day4_2(input: &str) -> i32 {
    let cards: Vec<Day4Card> = input.lines().map(Day4Card::new).collect();
    let mut counts = [0; 200];
    let mut count = 0;
    while count < cards.len() {
        counts[count] = 1;
        count += 1;
    }
    for card in cards {
        let mut winners = card.winners;
        let act_card = card.card - 1;
        while winners >= 1 {
            counts[(act_card + winners) as usize] += counts[act_card as usize];
            winners -= 1;
        }
    }
    counts.iter().sum()
}

fn day4() {
    let data =
        std::fs::read_to_string("C:\\Users\\Hagen\\RustProjects\\adventofcode2023\\data\\day4.txt")
            .expect("Data for day4-Problem not found");

    println!("{}", day4_1(&data));
    println!("{}", day4_2(&data));
}

#[test]
fn day5tests() {
    let data = std::fs::read_to_string(
        "C:\\Users\\Hagen\\RustProjects\\adventofcode2023\\data\\day5test.txt",
    )
    .expect("Data for day5-Test not found");

    assert_eq!(day5_1(&data), 35);
    assert_eq!(day5_2(&data), 46);
}

fn day5_1(input: &str) -> i64 {
    let mut source: Vec<i64> = input
        .lines()
        .next()
        .unwrap()
        .split(": ")
        .last()
        .unwrap()
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let transformation_blocks = input.split("map:");
    for tf in transformation_blocks {
        if tf.starts_with("seeds:") || tf.is_empty() {
            continue;
        }
        source = source.iter().map(|x| day5transform(x, tf)).collect();
    }
    *source.iter().min().unwrap()
}

fn day5transform(seed: &i64, transformations: &str) -> i64 {
    for line in transformations.lines() {
        if line.contains("-to-") || line.is_empty() {
            continue;
        }
        let values: Vec<i64> = line
            .split_ascii_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let destination = values[0];
        let source = values[1];
        let range = values[2];
        if *seed >= source && *seed < source + range {
            let diff = destination - source;
            return *seed + diff;
        }
    }
    *seed
}

#[derive(Debug)]
struct Day5tf {
    destination: i64,
    source: i64,
    range: i64,
}

impl Day5tf {
    fn new(destination: i64, source: i64, range: i64) -> Self {
        Self {
            destination,
            source,
            range,
        }
    }
}

fn day5_2(input: &str) -> i64 {
    let mut source: Vec<i64> = input
        .lines()
        .next()
        .unwrap()
        .split(": ")
        .last()
        .unwrap()
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let mut ranges: Vec<(i64, i64)> = vec![];
    while !source.is_empty() {
        let length = source.pop().unwrap();
        let start = source.pop().unwrap();
        ranges.push((start, length));
    }
    //println!("{:?}", ranges);
    let transformation_blocks = input.split("map:");
    let mut mappings: Vec<Vec<Day5tf>> = vec![];
    for tf in transformation_blocks {
        if tf.starts_with("seeds:") || tf.is_empty() {
            continue;
        }
        let mut transformations: Vec<Day5tf> = vec![];
        for line in tf.lines() {
            if line.contains("-to-") || line.is_empty() {
                continue;
            }
            let values: Vec<i64> = line
                .split_ascii_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            transformations.push(Day5tf::new(values[0], values[1], values[2]));
        }
        mappings.push(transformations);
    }
    //day5transform_ranges_rec(ranges.pop().unwrap(), ranges, &mut mappings);
    //println!("{:?}", ranges);
    for mapping in mappings {
        let mut switch_ranges: Vec<(i64, i64)> = vec![];
        for range in ranges {
            switch_ranges.append(&mut day5apply_mapping(range, &mapping));
        }
        ranges = switch_ranges;
        //println!("{:?}", ranges);
    }
    ranges.iter().map(|x| x.0).min().unwrap()
}

fn day5apply_mapping(seeds: (i64, i64), mapping: &Vec<Day5tf>) -> Vec<(i64, i64)> {
    //println!("Mapping: {:?}", mapping);
    let mut result: Vec<(i64, i64)> = vec![];
    let mut ranges_to_test: Vec<(i64, i64)> = vec![seeds];
    while !ranges_to_test.is_empty() {
        let originalrest = ranges_to_test.pop().unwrap();
        let mut splitted = false;
        //println!("Ranges to test: {:?}", ranges_to_test);
        //println!("Now testing: {:?}", originalrest);
        for tf in mapping {
            if tf.source + tf.range <= originalrest.0
                || tf.source >= originalrest.0 + originalrest.1
            {
                continue;
            }
            splitted = true;
            let diff = tf.destination - tf.source;
            if tf.source <= originalrest.0 {
                //println!("tfs<orig");
                if tf.source + tf.range >= originalrest.0 + originalrest.1 {
                    //println!("komplett enthalten");
                    result.push((originalrest.0 + diff, originalrest.1));
                } else {
                    //println!("aufteilung");
                    let splitposition = tf.source + tf.range - originalrest.0;
                    result.push((originalrest.0 + diff, splitposition));
                    ranges_to_test.push((
                        originalrest.0 + splitposition,
                        originalrest.1 - splitposition,
                    ));
                }
            } else {
                // tf.source > originalrest.0
                //println!("tfs<orig");
                if tf.source + tf.range >= originalrest.0 + originalrest.1 {
                    //println!("aufteilung in zwei");
                    let splitposition = originalrest.0 + originalrest.1 - tf.source;
                    result.push((tf.source + diff, splitposition));
                    ranges_to_test.push((originalrest.0, originalrest.1 - splitposition));
                } else {
                    //println!("aufteilung in drei");
                    result.push((tf.source + diff, tf.range));
                    ranges_to_test.push((originalrest.0, tf.source - originalrest.0));
                    ranges_to_test.push((
                        tf.source + tf.range,
                        originalrest.0 + originalrest.1 - tf.source - tf.range,
                    ));
                }
            }
        }
        if !splitted {
            result.push(originalrest);
        }
    }

    result
}

fn day5() {
    let data =
        std::fs::read_to_string("C:\\Users\\Hagen\\RustProjects\\adventofcode2023\\data\\day5.txt")
            .expect("Data for day5-Problem not found");

    println!("{}", day5_1(&data));
    println!("{}", day5_2(&data));
}

#[test]
fn day6tests() {
    let data = std::fs::read_to_string(
        "C:\\Users\\Hagen\\RustProjects\\adventofcode2023\\data\\day6test.txt",
    )
    .expect("Data for day6-Test not found");

    assert_eq!(day6_1(&data), 288);
    assert_eq!(day6_2(&data), 71503);
}

fn day6get_racedistances(timedist: (&i32, &i32)) -> i32 {
    let mut result = 0;
    let mut i = 1;
    while i < *timedist.0 {
        if i * (timedist.0 - i) > *timedist.1 {
            result += 1;
        }
        i += 1;
    }
    result
}

fn day6fast_racedistances(timedist: (&i64, &i64)) -> i64 {
    let mut firstwin = 1;
    while firstwin < *timedist.0 {
        if firstwin * (timedist.0 - firstwin) > *timedist.1 {
            break;
        }
        firstwin += 1;
    }
    let loosing_games = (firstwin - 1) * 2 + 1; // add the 1 for the last game since the 0 game isn't counted
    timedist.0 - loosing_games
}

fn day6_1(input: &str) -> i32 {
    let mut lines = input.lines();
    let times: Vec<i32> = lines
        .next()
        .unwrap()
        .split(':')
        .last()
        .unwrap()
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let win_distances: Vec<i32> = lines
        .next()
        .unwrap()
        .split(':')
        .last()
        .unwrap()
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let racetimedist = times.iter().zip(win_distances.iter());
    let racedistances: Vec<i32> = racetimedist.map(day6get_racedistances).collect();
    racedistances.iter().product()
}

fn day6_2(input: &str) -> i64 {
    let mut lines = input.lines();
    let time: i64 = lines
        .next()
        .unwrap()
        .split(':')
        .last()
        .unwrap()
        .replace(" ", "")
        .parse()
        .unwrap();
    let win_distance: i64 = lines
        .next()
        .unwrap()
        .split(':')
        .last()
        .unwrap()
        .replace(" ", "")
        .parse()
        .unwrap();
    day6fast_racedistances((&time, &win_distance))
}

fn day6() {
    let data =
        std::fs::read_to_string("C:\\Users\\Hagen\\RustProjects\\adventofcode2023\\data\\day6.txt")
            .expect("Data for day6-Problem not found");

    println!("{}", day6_1(&data));
    println!("{}", day6_2(&data));
}

#[test]
fn day7tests() {
    let data = std::fs::read_to_string(
        "C:\\Users\\Hagen\\RustProjects\\adventofcode2023\\data\\day7test.txt",
    )
    .expect("Data for day7-Test not found");

    assert_eq!(day7_1(&data), 6440);
    assert_eq!(day7_2(&data), 5905);
}

#[derive(Debug)]
struct Day7hand {
    cards: String,
    bid: i32,
    typ: i32, // values: (10,8,7,6,4,2,1); double of same cards, full house=7, and with pairs count the involved cards
}

impl Ord for Day7hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.typ != other.typ {
            return self.typ.cmp(&other.typ);
        } else {
            let order = "AKQJT98765432";
            let strcmp = self.cards.chars().zip(other.cards.chars());
            for pair in strcmp {
                if pair.0 != pair.1 {
                    let selfpos = order.find(pair.1).unwrap();
                    let otherpos = order.find(pair.0).unwrap();
                    return selfpos.cmp(&otherpos);
                }
            }
            Ordering::Equal
        }
    }
}

impl PartialOrd for Day7hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.typ != other.typ {
            return self.typ.partial_cmp(&other.typ);
        } else {
            let order = "AKQJT98765432";
            let strcmp = self.cards.chars().zip(other.cards.chars());
            for pair in strcmp {
                if pair.0 != pair.1 {
                    let selfpos = order.find(pair.1).unwrap();
                    let otherpos = order.find(pair.0).unwrap();
                    return selfpos.partial_cmp(&otherpos);
                }
            }
            None
        }
    }
}

impl PartialEq for Day7hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl Eq for Day7hand {}

impl Day7hand {
    fn new(hand: &str, jokerrule: bool) -> Self {
        let mut parts = hand.split_ascii_whitespace();
        let cards = String::from(parts.next().unwrap());
        let bid = parts.next().unwrap().parse().unwrap();
        let typ = get_type(&cards, jokerrule);
        Self { cards, bid, typ }
    }
}

fn get_type(cards: &str, jokerrule: bool) -> i32 {
    let unique_cards = cards
        .chars()
        .into_iter()
        .filter(|&x| if jokerrule { x != 'J' } else { true })
        .collect::<HashSet<_>>();
    let mut counts: Vec<i32> = vec![];
    for c in unique_cards {
        counts.push(cards.chars().filter(|&x| x == c).count() as i32);
    }
    let jokercount = cards.chars().filter(|&x| x == 'J').count();
    //println!("Hand: {:?}, Jokercount: {:?}", cards, jokercount);
    if counts.contains(&5) {
        return 10;
    }
    if counts.contains(&4) {
        if jokerrule && jokercount > 0 {
            return 10;
        }
        return 8;
    }
    if counts.contains(&3) && counts.contains(&2) {
        return 7;
    }
    if counts.contains(&3) {
        if jokerrule && jokercount > 0 {
            if jokercount == 2 {
                return 10;
            }
            if jokercount == 1 {
                return 8;
            }
        }
        return 6;
    }
    if counts.contains(&2) && counts.len() <= 3 {
        if jokerrule && jokercount > 0 {
            if jokercount == 3 {
                return 10;
            }
            if jokercount == 2 {
                return 8;
            }
            if jokercount == 1 {
                if counts.len() == 3 {
                    return 6;
                }
                if counts.len() == 2 {
                    return 7;
                }
            }
        }
        return 4;
    }
    if counts.contains(&2) {
        if jokerrule && jokercount > 0 {
            if jokercount == 3 {
                return 10;
            }
            if jokercount == 2 {
                return 8;
            }
            if jokercount == 1 {
                return 6;
            }
        }
        return 2;
    }
    if jokerrule && jokercount > 0 {
        if jokercount == 5 {
            return 10;
        }
        if jokercount > 1 {
            return (jokercount as i32 + 1) * 2;
        } else {
            return 2;
        }
    }
    1
}

fn day7_1(input: &str) -> i32 {
    let mut hands: Vec<Day7hand> = input.lines().map(|x| Day7hand::new(x, false)).collect();
    //println!("Unordered: {:?}", hands);
    hands.sort();
    //hands.sort_by(|a, b| a.partial_cmp(b).unwrap());
    //println!("Ordered: {:?}", hands);
    let mut total_winnings = 0;
    for (rank, hand) in hands.iter().enumerate() {
        //println!("Rank: {:?} Hand: {:?}", rank, hand);
        total_winnings += (rank as i32 + 1) * hand.bid;
    }
    total_winnings
}

fn jokercmp(lhs: &Day7hand, rhs: &Day7hand) -> std::cmp::Ordering {
    if lhs.typ != rhs.typ {
        return lhs.typ.cmp(&rhs.typ);
    } else {
        let order = "AKQT98765432J";
        let strcmp = lhs.cards.chars().zip(rhs.cards.chars());
        for pair in strcmp {
            if pair.0 != pair.1 {
                let lhspos = order.find(pair.1).unwrap();
                let rhspos = order.find(pair.0).unwrap();
                return lhspos.cmp(&rhspos);
            }
        }
        Ordering::Equal
    }
}

fn day7_2(input: &str) -> i32 {
    let mut hands: Vec<Day7hand> = input.lines().map(|x| Day7hand::new(x, true)).collect();
    //println!("Unordered: {:?}", hands);
    hands.sort_by(|a, b| jokercmp(a, b));
    //println!("Ordered: {:?}", hands);
    let mut total_winnings = 0;
    for (rank, hand) in hands.iter().enumerate() {
        //println!("Rank: {:?} Hand: {:?}", rank, hand);
        total_winnings += (rank as i32 + 1) * hand.bid;
    }
    total_winnings
}

fn day7() {
    let data =
        std::fs::read_to_string("C:\\Users\\Hagen\\RustProjects\\adventofcode2023\\data\\day7.txt")
            .expect("Data for day7-Problem not found");

    println!("{}", day7_1(&data));
    println!("{}", day7_2(&data));
}

fn main() {
    println!("Day1 results:");
    day1();
    println!("Day2 results:");
    day2();
    println!("Day3 results:");
    day3();
    println!("Day4 results:");
    day4();
    println!("Day5 results:");
    day5();
    println!("Day6 results:");
    day6();
    println!("Day7 results:");
    day7();
}

/*
#[test]
fn dayxtests() {
    let data = std::fs::read_to_string(
        "C:\\Users\\Hagen\\RustProjects\\adventofcode2023\\data\\dayxtest.txt",
    )
    .expect("Data for dayx-Test not found");

    assert_eq!(dayx_1(&data), 13);
    //assert_eq!(dayx_2(&data), 30);
}

fn dayx_1(input: &str) -> i32 {
    0
}

//fn dayx_2(input: &str) -> i32 {}

fn dayx() {
    let data =
        std::fs::read_to_string("C:\\Users\\Hagen\\RustProjects\\adventofcode2023\\data\\dayx.txt")
            .expect("Data for dayx-Problem not found");

    println!("{}", dayx_1(&data));
    //println!("{}", dayx_2(&data));
}
*/
