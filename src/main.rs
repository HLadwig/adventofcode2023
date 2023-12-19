use std::{cmp::Ordering, collections::HashSet, str::FromStr};

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

#[test]
fn day8tests() {
    let data1 = std::fs::read_to_string(
        "C:\\Users\\Hagen\\RustProjects\\adventofcode2023\\data\\day8test1.txt",
    )
    .expect("Data for day8-Test1 not found");
    let data2 = std::fs::read_to_string(
        "C:\\Users\\Hagen\\RustProjects\\adventofcode2023\\data\\day8test2.txt",
    )
    .expect("Data for day8-Test2 not found");
    let data3 = std::fs::read_to_string(
        "C:\\Users\\Hagen\\RustProjects\\adventofcode2023\\data\\day8test3.txt",
    )
    .expect("Data for day8-Test3 not found");

    assert_eq!(day8_1(&data1), 2);
    assert_eq!(day8_1(&data2), 6);
    assert_eq!(day8_2(&data3), 6);
}

#[derive(Debug)]
struct Day8node {
    name: String,
    left: String,
    right: String,
}

impl Day8node {
    fn new(line: &str) -> Self {
        let mut parts = line.split(" = ");
        let name = String::from(parts.next().unwrap());
        let mut lrparts = parts.next().unwrap().split(", ");
        let left = lrparts.next().unwrap().replace('(', "");
        let right = lrparts.next().unwrap().replace(')', "");
        Self { name, left, right }
    }
}

fn day8_1(input: &str) -> i32 {
    let mut lines = input.lines();
    let movements = lines.next().unwrap();
    lines.next();
    //let nodes: Vec<Day8node> = lines.map(Day8node::new).collect();
    let network: std::collections::HashMap<&str, Day8node> = lines
        .map(|x| {
            let newnode = Day8node::new(x);
            let nodename = x.split(" = ").next().unwrap();
            (nodename, newnode)
        })
        .collect();
    let mut stepcount = 0;
    let mut current_node_name = "AAA";
    for step in movements.chars().cycle() {
        let current_node = network.get(current_node_name).unwrap();
        if step == 'L' {
            current_node_name = current_node.left.as_str();
        }
        if step == 'R' {
            current_node_name = current_node.right.as_str();
        }
        stepcount += 1;
        if current_node_name == "ZZZ" {
            break;
        }
    }

    stepcount
}

fn day8_2(input: &str) -> i64 {
    let mut lines = input.lines();
    let movements = lines.next().unwrap();
    lines.next();
    //let nodes: Vec<Day8node> = lines.map(Day8node::new).collect();
    let network: std::collections::HashMap<&str, Day8node> = lines
        .map(|x| {
            let newnode = Day8node::new(x);
            let nodename = x.split(" = ").next().unwrap();
            (nodename, newnode)
        })
        .collect();
    let current_node_names: Vec<&str> = network
        .iter()
        .filter(|x| x.0.ends_with("A"))
        .map(|x| x.1.name.as_str())
        .collect();
    let mut step_pairs: Vec<(i64, i64)> = vec![];
    for mut current_node_name in current_node_names {
        let mut stepcount = 0;
        let mut steps_first = 0;
        for step in movements.chars().cycle() {
            let current_node = network.get(current_node_name).unwrap();
            if step == 'L' {
                current_node_name = current_node.left.as_str();
            }
            if step == 'R' {
                current_node_name = current_node.right.as_str();
            }
            stepcount += 1;
            if current_node_name.ends_with('Z') {
                if steps_first == 0 {
                    steps_first = stepcount;
                    stepcount = 0;
                } else {
                    step_pairs.push((steps_first, stepcount));
                    break;
                }
            }
        }
    }
    /*for step in movements.chars().cycle() {
            let mut current_nodes: Vec<&Day8node> = vec![];
            for node in current_node_names {
                current_nodes.push(network.get(node).unwrap());
            }
            current_node_names = current_nodes
                .iter()
                .map(|x| {
                    if step == 'L' {
                        x.left.as_str()
                    } else {
                        x.right.as_str()
                    }
                })
                .collect();

            stepcount += 1;
            if current_node_names.iter().all(|x| x.ends_with("Z")) {
                break;
            }
    }*/
    if step_pairs.iter().any(|x| x.0 != x.1) {
        println!(
            "Problem, first cycle is different from second {:?}",
            step_pairs
        );
        return 0;
    }
    step_pairs
        .iter()
        .fold(1, |acc: i64, num: &(i64, i64)| num_integer::lcm(acc, num.0))
}

fn day8() {
    let data =
        std::fs::read_to_string("C:\\Users\\Hagen\\RustProjects\\adventofcode2023\\data\\day8.txt")
            .expect("Data for day8-Problem not found");

    println!("{}", day8_1(&data));
    println!("{}", day8_2(&data));
}

#[test]
fn day9tests() {
    let data = std::fs::read_to_string(
        "C:\\Users\\Hagen\\RustProjects\\adventofcode2023\\data\\day9test.txt",
    )
    .expect("Data for day9-Test not found");

    assert_eq!(day9_1(&data), 114);
    assert_eq!(day9_2(&data), 2);
}

/*
fn day9get_next_sequence(sequence: &[i32]) -> Vec<i32> {
    let mut new_sequence = Vec::with_capacity(sequence.len() - 1);
    let mut elements = sequence.iter().peekable();

    while let Some(&first) = elements.next() {
        if let Some(&second) = elements.peek() {
            new_sequence.push(second - first);
        }
    }

    new_sequence
}

fn day9get_prediction(mut sequence: &Vec<i32>) -> i32 {
    let mut last_value = Vec::new();

    while sequence.iter().any(|&x| x != 0) {
        let mut temp: Vec<i32> = vec![];
        last_value.push(sequence.last().copied().unwrap());
        temp = day9get_next_sequence(sequence);
        sequence = &temp;
    }

    last_value.iter().sum()
}
 */

fn day9get_prediction(sequence: &Vec<i32>) -> i32 {
    let mut arr: Vec<i32> = sequence.iter().rev().copied().collect();
    let mut last_values: Vec<i32> = vec![];
    let mut check_positions = arr.len() - 1;
    while arr.iter().any(|&x| x != 0) {
        last_values.push(arr[0]); // *arr.first().unwrap());
        for i in 0..check_positions {
            arr[i] -= arr[i + 1];
        }
        arr[check_positions] = 0;
        check_positions -= 1;
    }
    last_values.iter().sum()
}

fn day9get_backward_prediction(sequence: &Vec<i32>) -> i32 {
    let mut arr: Vec<i32> = sequence.iter().rev().copied().collect();
    let mut first_values: Vec<i32> = vec![];
    let mut check_positions = arr.len() - 1;
    while arr.iter().any(|&x| x != 0) {
        first_values.push(arr[check_positions]); // *arr.first().unwrap());
        for i in 0..check_positions {
            arr[i] -= arr[i + 1];
        }
        arr[check_positions] = 0;
        check_positions -= 1;
    }
    let mut prediction = 0;
    for (position, value) in first_values.iter().enumerate() {
        if position % 2 == 0 {
            prediction += value;
        } else {
            prediction -= value;
        }
    }
    prediction
}

fn day9_1(input: &str) -> i32 {
    let sequences: Vec<Vec<i32>> = input
        .lines()
        .map(|x| {
            x.split_ascii_whitespace()
                .map(|x| x.parse().unwrap())
                .collect()
        })
        .collect();
    sequences.iter().map(day9get_prediction).sum()
}

fn day9_2(input: &str) -> i32 {
    let sequences: Vec<Vec<i32>> = input
        .lines()
        .map(|x| {
            x.split_ascii_whitespace()
                .map(|x| x.parse().unwrap())
                .collect()
        })
        .collect();
    sequences.iter().map(day9get_backward_prediction).sum()
}

fn day9() {
    let data =
        std::fs::read_to_string("C:\\Users\\Hagen\\RustProjects\\adventofcode2023\\data\\day9.txt")
            .expect("Data for day9-Problem not found");

    println!("{}", day9_1(&data));
    println!("{}", day9_2(&data));
}

#[test]
fn day10tests() {
    let data1 =
        std::fs::read("C:\\Users\\Hagen\\RustProjects\\adventofcode2023\\data\\day10test1.txt")
            .expect("Data for day10-Test1 not found");
    let data2 =
        std::fs::read("C:\\Users\\Hagen\\RustProjects\\adventofcode2023\\data\\day10test2.txt")
            .expect("Data for day10-Test2 not found");
    let mut data3 =
        std::fs::read("C:\\Users\\Hagen\\RustProjects\\adventofcode2023\\data\\day10test3.txt")
            .expect("Data for day10-Test2 not found");
    let mut data4 =
        std::fs::read("C:\\Users\\Hagen\\RustProjects\\adventofcode2023\\data\\day10test4.txt")
            .expect("Data for day10-Test2 not found");
    let mut data5 =
        std::fs::read("C:\\Users\\Hagen\\RustProjects\\adventofcode2023\\data\\day10test5.txt")
            .expect("Data for day10-Test2 not found");
    assert_eq!(day10_1(&data1), 4);
    assert_eq!(day10_1(&data2), 8);
    assert_eq!(day10_2(&mut data3), 4);
    assert_eq!(day10_2(&mut data4), 8);
    assert_eq!(day10_2(&mut data5), 10);
}

fn day10_1(input: &Vec<u8>) -> i32 {
    let columns = input.iter().position(|&x| x == b'\n').unwrap() + 1;
    let _lines = input.len() / columns;
    let start = input.iter().position(|&x| x == b'S').unwrap();
    let ystart = start / columns;
    let xstart = start - ystart * columns;
    let mut xprev = xstart;
    let mut yprev = ystart;
    let mut xact = xstart + 1;
    let mut yact = ystart;
    let mut steps = 1;
    while (xact, yact) != (xstart, ystart) {
        let pos = yact * columns + xact;
        match input[pos] {
            b'L' => {
                if yprev == yact - 1 {
                    yprev = yact;
                    xprev = xact;
                    xact += 1;
                } else {
                    xprev = xact;
                    yprev = yact;
                    yact -= 1;
                }
            }
            b'F' => {
                if yprev == yact + 1 {
                    yprev = yact;
                    xprev = xact;
                    xact += 1;
                } else {
                    xprev = xact;
                    yprev = yact;
                    yact += 1;
                }
            }
            b'7' => {
                if yprev == yact + 1 {
                    yprev = yact;
                    xprev = xact;
                    xact -= 1;
                } else {
                    xprev = xact;
                    yprev = yact;
                    yact += 1;
                }
            }
            b'J' => {
                if yprev == yact - 1 {
                    yprev = yact;
                    xprev = xact;
                    xact -= 1;
                } else {
                    xprev = xact;
                    yprev = yact;
                    yact -= 1;
                }
            }
            b'|' => {
                xprev = xact;
                if yprev == yact - 1 {
                    yprev = yact;
                    yact += 1;
                } else {
                    yprev = yact;
                    yact -= 1;
                }
            }
            b'-' => {
                yprev = yact;
                if xprev == xact - 1 {
                    xprev = xact;
                    xact += 1;
                } else {
                    xprev = xact;
                    xact -= 1;
                }
            }
            _ => (),
        }
        steps += 1;
    }

    steps / 2
}

fn day10_2(input: &mut Vec<u8>) -> usize {
    let columns = input.iter().position(|&x| x == b'\n').unwrap() + 1;
    let lines = input.len() / columns;
    let start = input.iter().position(|&x| x == b'S').unwrap();
    let ystart = start / columns;
    let xstart = start - ystart * columns;
    let mut xprev = xstart;
    let mut yprev = ystart;
    let mut xact = xstart;
    let mut yact = ystart;
    day10print_map(input, columns, lines);
    // find out to which two fields is the connection from the starting point
    // and set xact/yact and the symbol accordingly
    // we just assume S isn't on any edge
    let mut test = input[start + 1];
    if test == b'-' || test == b'J' || test == b'7' {
        xact += 1;
    } else {
        test = input[start - 1];
        if test == b'-' || test == b'F' || test == b'L' {
            xact -= 1;
        } else {
            yact += 1;
        }
    }
    let mut loop_points: Vec<usize> = vec![];
    loop_points.push(start);
    let mut right_turns = 0;
    while (xact, yact) != (xstart, ystart) {
        let pos = yact * columns + xact;
        let symbol = input[pos];
        loop_points.push(pos);
        match symbol {
            b'L' => {
                if yprev == yact - 1 {
                    yprev = yact;
                    xprev = xact;
                    xact += 1;
                    right_turns -= 1;
                } else {
                    xprev = xact;
                    yprev = yact;
                    yact -= 1;
                    right_turns += 1;
                }
            }
            b'F' => {
                if yprev == yact + 1 {
                    yprev = yact;
                    xprev = xact;
                    xact += 1;
                    right_turns += 1;
                } else {
                    xprev = xact;
                    yprev = yact;
                    yact += 1;
                    right_turns -= 1;
                }
            }
            b'7' => {
                if yprev == yact + 1 {
                    yprev = yact;
                    xprev = xact;
                    xact -= 1;
                    right_turns -= 1;
                } else {
                    xprev = xact;
                    yprev = yact;
                    yact += 1;
                    right_turns += 1;
                }
            }
            b'J' => {
                if yprev == yact - 1 {
                    yprev = yact;
                    xprev = xact;
                    xact -= 1;
                    right_turns += 1;
                } else {
                    xprev = xact;
                    yprev = yact;
                    yact -= 1;
                    right_turns -= 1;
                }
            }
            b'|' => {
                xprev = xact;
                if yprev == yact - 1 {
                    yprev = yact;
                    yact += 1;
                } else {
                    yprev = yact;
                    yact -= 1;
                }
            }
            b'-' => {
                yprev = yact;
                if xprev == xact - 1 {
                    xprev = xact;
                    xact += 1;
                } else {
                    xprev = xact;
                    xact -= 1;
                }
            }
            _ => (),
        }
    }
    loop_points.sort();
    xprev = xstart;
    yprev = ystart;
    xact = xstart;
    yact = ystart;
    // find out to which two fields is the connection from the starting point
    // and set xact/yact and the symbol accordingly
    // we just assume S isn't on any edge
    test = input[start + 1];
    if test == b'-' || test == b'J' || test == b'7' {
        xact += 1;
    } else {
        test = input[start - 1];
        if test == b'-' || test == b'F' || test == b'L' {
            xact -= 1;
        } else {
            yact += 1;
        }
    }
    while (xact, yact) != (xstart, ystart) {
        let pos = yact * columns + xact;
        let symbol = input[pos];
        input[pos] = b'#';
        match symbol {
            b'L' => {
                if yprev == yact - 1 {
                    yprev = yact;
                    xprev = xact;
                    xact += 1;
                } else {
                    xprev = xact;
                    yprev = yact;
                    yact -= 1;
                }
            }
            b'F' => {
                if yprev == yact + 1 {
                    yprev = yact;
                    xprev = xact;
                    xact += 1;
                } else {
                    xprev = xact;
                    yprev = yact;
                    yact += 1;
                }
            }
            b'7' => {
                if yprev == yact + 1 {
                    yprev = yact;
                    xprev = xact;
                    xact -= 1;
                } else {
                    xprev = xact;
                    yprev = yact;
                    yact += 1;
                }
            }
            b'J' => {
                if yprev == yact - 1 {
                    yprev = yact;
                    xprev = xact;
                    xact -= 1;
                } else {
                    xprev = xact;
                    yprev = yact;
                    yact -= 1;
                }
            }
            b'|' => {
                xprev = xact;
                if yprev == yact - 1 {
                    yprev = yact;
                    yact += 1;
                    if right_turns > 0 {
                        if !loop_points.contains(&(pos - 1)) {
                            input[pos - 1] = b'I';
                        }
                    } else {
                        if !loop_points.contains(&(pos + 1)) {
                            input[pos + 1] = b'I';
                        }
                    }
                } else {
                    yprev = yact;
                    yact -= 1;
                    if right_turns < 0 {
                        if !loop_points.contains(&(pos - 1)) {
                            input[pos - 1] = b'I';
                        }
                    } else {
                        if !loop_points.contains(&(pos + 1)) {
                            input[pos + 1] = b'I';
                        }
                    }
                }
                day10print_map(input, columns, lines);
            }
            b'-' => {
                yprev = yact;
                if xprev == xact - 1 {
                    xprev = xact;
                    xact += 1;
                    if right_turns < 0 {
                        if !loop_points.contains(&(pos - columns)) {
                            input[pos - columns] = b'I';
                        }
                    } else {
                        if !loop_points.contains(&(pos + columns)) {
                            input[pos + columns] = b'I';
                        }
                    }
                } else {
                    xprev = xact;
                    xact -= 1;
                    if right_turns > 0 {
                        if !loop_points.contains(&(pos - columns)) {
                            input[pos - columns] = b'I';
                        }
                    } else {
                        if !loop_points.contains(&(pos + columns)) {
                            input[pos + columns] = b'I';
                        }
                    }
                }
                day10print_map(input, columns, lines);
            }
            _ => (),
        }
    }
    day10print_map(input, columns, lines);
    input.iter().filter(|&&x| x == b'I').count()
}

fn day10print_map(input: &Vec<u8>, columns: usize, lines: usize) {
    for i in 0..lines {
        let line = i * columns;
        println!(
            "{:?}",
            String::from_utf8(input[line..line + columns].to_vec()).unwrap()
        );
    }
    println!();
}

/* the standard does the expected
fn loop_points_cmp(lhs: &(usize, usize), rhs: &(usize, usize)) -> std::cmp::Ordering {
    if lhs.0 != rhs.0 {
        lhs.0.cmp(&rhs.0)
    } else {
        lhs.1.cmp(&rhs.1)
    }
}
*/

fn day10() {
    let mut data =
        std::fs::read("C:\\Users\\Hagen\\RustProjects\\adventofcode2023\\data\\day10.txt")
            .expect("Data for day10-Problem not found");

    println!("{}", day10_1(&data));
    //println!("{}", day10_2(&mut data));
}

#[test]
fn day11tests() {
    let data =
        std::fs::read("C:\\Users\\Hagen\\RustProjects\\adventofcode2023\\data\\day11test.txt")
            .expect("Data for day11-Test not found");

    assert_eq!(day11_1(&data), 374);
    //assert_eq!(day11_2(&data), 30);
}

fn day11_1(input: &Vec<u8>) -> usize {
    let columns = input.iter().position(|&x| x == b'\n').unwrap() + 1;
    let lines = input.len() / columns;
    let mut horvec: Vec<usize> = vec![];
    let mut vervec: Vec<usize> = vec![];
    for i in 0..lines {
        let startpoint = i * columns;
        let line = &input[startpoint..startpoint + columns];
        //println!("{:?})", String::from_utf8_lossy(line));
        horvec.push(
            line.iter()
                .filter(|&&x| x == b'#')
                .collect::<Vec<&u8>>()
                .iter()
                .count(),
        );
    }
    for i in 0..columns - 2 {
        let mut counter = 0;
        for j in 0..lines {
            let pos = j * columns + i;
            if input[pos] == b'#' {
                counter += 1
            }
        }
        vervec.push(counter);
    }
    day11double_zeroes(&mut horvec);
    day11double_zeroes(&mut vervec);
    day11compute_distance(&horvec) + day11compute_distance(&vervec)
    //leaving the first solution in, just for the record; could of course be replaced with the following
    //day11compute_distance_with_drift(&horvec, 2) + day11compute_distance_with_drift(&vervec, 2)
}

fn day11compute_distance(vector: &Vec<usize>) -> usize {
    let mut sum: usize = 0;
    for i in 0..vector.len() {
        let mut dist: usize = 0;
        let num = vector[i];
        for j in i..vector.len() {
            sum += num * dist * vector[j];
            dist += 1;
        }
    }
    sum
}

fn day11compute_distance_with_drift(vector: &Vec<usize>, drift: usize) -> usize {
    let mut sum: usize = 0;
    for i in 0..vector.len() {
        let mut dist: usize = 0;
        let num = vector[i];
        for j in i..vector.len() {
            sum += num * dist * vector[j];
            if vector[j] == 0 {
                dist += drift;
            } else {
                dist += 1;
            }
        }
    }
    sum
}

fn day11double_zeroes(vector: &mut Vec<usize>) {
    let mut zero_positions: Vec<usize> = vec![];
    for (i, x) in vector.iter().enumerate() {
        if *x == 0 {
            zero_positions.push(i);
        }
    }
    while zero_positions.len() > 0 {
        vector.insert(zero_positions.pop().unwrap(), 0);
    }
}

fn day11_2(input: &Vec<u8>) -> usize {
    let columns = input.iter().position(|&x| x == b'\n').unwrap() + 1;
    let lines = input.len() / columns;
    let mut horvec: Vec<usize> = vec![];
    let mut vervec: Vec<usize> = vec![];
    for i in 0..lines {
        let startpoint = i * columns;
        let line = &input[startpoint..startpoint + columns];
        //println!("{:?})", String::from_utf8_lossy(line));
        horvec.push(
            line.iter()
                .filter(|&&x| x == b'#')
                .collect::<Vec<&u8>>()
                .iter()
                .count(),
        );
    }
    for i in 0..columns - 2 {
        let mut counter = 0;
        for j in 0..lines {
            let pos = j * columns + i;
            if input[pos] == b'#' {
                counter += 1
            }
        }
        vervec.push(counter);
    }
    day11compute_distance_with_drift(&horvec, 1000000)
        + day11compute_distance_with_drift(&vervec, 1000000)
}

fn day11() {
    let data = std::fs::read("C:\\Users\\Hagen\\RustProjects\\adventofcode2023\\data\\day11.txt")
        .expect("Data for day11-Problem not found");

    println!("{}", day11_1(&data));
    println!("{}", day11_2(&data));
}

#[test]
fn day13tests() {
    let data = std::fs::read_to_string(
        "C:\\Users\\Hagen\\RustProjects\\adventofcode2023\\data\\day13test.txt",
    )
    .expect("Data for day13-Test not found");

    assert_eq!(day13_1(&data), 405);
    assert_eq!(day13_2(&data), 400);
}

#[derive(Debug)]
struct Day13note {
    note: String,
}

impl Day13note {
    fn new(input: &str) -> Day13note {
        Self {
            note: String::from_str(input).unwrap(),
        }
    }

    fn transpose(&self) -> Self {
        let lines: Vec<&str> = self.note.lines().collect();
        let linelength = lines[0].len();
        let mut new_lines: Vec<char> = vec![];
        for i in 0..linelength {
            let mut colline: Vec<char> = lines.iter().map(|&x| x.chars().nth(i).unwrap()).collect();
            new_lines.append(&mut colline);
            new_lines.push('\r');
            new_lines.push('\n');
        }
        Self {
            note: new_lines.into_iter().collect(),
        }
    }

    fn get_possible_mirror_line(&self) -> Vec<usize> {
        let mut index_of_identical: Vec<usize> = vec![];
        let lines: Vec<&str> = self.note.lines().collect();
        let mut last_line = lines[0];
        let mut count = 0;
        for i in 1..lines.len() {
            count += 1;
            if last_line == lines[i] {
                index_of_identical.push(count);
            }
            last_line = lines[i];
        }
        index_of_identical
    }

    fn get_mirror_line(&self) -> usize {
        'outer: for check_line in self.get_possible_mirror_line() {
            if check_line < 2 {
                return check_line;
            }
            let lines: Vec<&str> = self.note.lines().collect();
            let mut back = check_line - 2;
            let mut fore = check_line + 1;
            while fore < lines.len() {
                if lines[back] != lines[fore] {
                    continue 'outer;
                };
                if back == 0 {
                    break;
                }
                back -= 1;
                fore += 1;
            }
            return check_line;
        }
        0
    }

    fn get_possible_smudge_mirror_line(&self) -> Vec<usize> {
        let mut index_of_identical: Vec<usize> = vec![];
        let lines: Vec<&str> = self.note.lines().collect();
        let mut last_line = lines[0];
        let mut count = 0;
        for i in 1..lines.len() {
            count += 1;
            if day13diff(last_line, lines[i]) < 2 {
                index_of_identical.push(count);
            }
            last_line = lines[i];
        }
        index_of_identical
    }

    fn get_smudge_mirror_line(&self) -> usize {
        for check_line in self.get_possible_smudge_mirror_line() {
            let mut diff_count = 0;
            let lines: Vec<&str> = self.note.lines().collect();
            let mut back = check_line - 1;
            let mut fore = check_line;
            while fore < lines.len() {
                diff_count += day13diff(lines[back], lines[fore]);
                if back == 0 {
                    break;
                }
                back -= 1;
                fore += 1;
            }
            if diff_count == 1 {
                return check_line;
            }
        }
        0
    }
}

fn day13diff(lhs: &str, rhs: &str) -> usize {
    let lhschars = lhs.as_bytes();
    let rhschars = rhs.as_bytes();
    let mut differences = 0;
    for i in 0..lhschars.len() {
        if lhschars[i] != rhschars[i] {
            differences += 1;
        }
    }
    differences
}

fn day13_1(input: &str) -> usize {
    let set_breaks = input.replace("\r\n\r\n", "|");
    let notes: Vec<Day13note> = set_breaks.split(|x| x == '|').map(Day13note::new).collect();
    let linesum: usize = notes.iter().map(|x| x.get_mirror_line()).sum();
    let colsum: usize = notes.iter().map(|x| x.transpose().get_mirror_line()).sum();
    colsum + linesum * 100
}

fn day13_2(input: &str) -> usize {
    let set_breaks = input.replace("\r\n\r\n", "|");
    let notes: Vec<Day13note> = set_breaks.split(|x| x == '|').map(Day13note::new).collect();
    let linesum: usize = notes.iter().map(|x| x.get_smudge_mirror_line()).sum();
    let colsum: usize = notes
        .iter()
        .map(|x| x.transpose().get_smudge_mirror_line())
        .sum();
    colsum + linesum * 100
}

fn day13() {
    let data = std::fs::read_to_string(
        "C:\\Users\\Hagen\\RustProjects\\adventofcode2023\\data\\day13.txt",
    )
    .expect("Data for day13-Problem not found");

    println!("{}", day13_1(&data));
    println!("{}", day13_2(&data));
}

#[test]
fn day14tests() {
    let mut data =
        std::fs::read("C:\\Users\\Hagen\\RustProjects\\adventofcode2023\\data\\day14test.txt")
            .expect("Data for day14-Test not found");

    assert_eq!(day14_1(&data), 136);
    //assert_eq!(day14_1ver2(&mut data), 136);
    assert_eq!(day14_2(&mut data), 64);
}

fn day14_1(input: &Vec<u8>) -> usize {
    let columns = input.iter().position(|&x| x == b'\n').unwrap() + 1;
    let lines = input.len() / columns;
    let mut sum = 0;
    for i in 0..columns - 2 {
        let mut add_line = lines;
        for j in 0..lines {
            let content = input[j * columns + i];
            if content == b'O' {
                sum += add_line;
                add_line -= 1;
            }
            if content == b'#' {
                add_line = lines - j - 1;
            }
        }
    }
    sum
}

fn day14_1ver2(input: &mut Vec<u8>) -> usize {
    let columns = input.iter().position(|&x| x == b'\n').unwrap() + 1;
    let lines = input.len() / columns;
    // north
    for i in 0..columns - 2 {
        let mut set_line = 0;
        for j in 0..lines {
            let content = input[j * columns + i];
            if content == b'O' {
                input[set_line * columns + i] = b'O';
                if set_line != j {
                    input[j * columns + i] = b'.';
                }
                set_line += 1;
            }
            if content == b'#' {
                set_line = j + 1;
            }
        }
    }
    day14_compute_load_without_tilting(input)
}

fn day14_compute_load_without_tilting(input: &Vec<u8>) -> usize {
    let columns = input.iter().position(|&x| x == b'\n').unwrap() + 1;
    let lines = input.len() / columns;
    let mut sum = 0;
    for i in 0..columns - 2 {
        let mut add_line = lines;
        for j in 0..lines {
            let content = input[j * columns + i];
            if content == b'O' {
                sum += add_line;
            }
            add_line -= 1;
        }
    }
    sum
}

fn day14_2(input: &mut Vec<u8>) -> usize {
    let columns = input.iter().position(|&x| x == b'\n').unwrap() + 1;
    let lines = input.len() / columns;
    let mut first_cycle = vec![];
    let run_cycles = 1000000000;
    let mut rest: i32 = 0;
    for i in 0..run_cycles {
        // north
        for i in 0..columns - 2 {
            let mut set_line = 0;
            for j in 0..lines {
                let content = input[j * columns + i];
                if content == b'O' {
                    input[set_line * columns + i] = b'O';
                    if set_line != j {
                        input[j * columns + i] = b'.';
                    }
                    set_line += 1;
                }
                if content == b'#' {
                    set_line = j + 1;
                }
            }
        }
        // west
        for i in 0..lines {
            let mut set_column = 0;
            for j in 0..columns - 2 {
                let content = input[i * columns + j];
                if content == b'O' {
                    input[i * columns + set_column] = b'O';
                    if set_column != j {
                        input[i * columns + j] = b'.';
                    }
                    set_column += 1;
                }
                if content == b'#' {
                    set_column = j + 1;
                }
            }
        }
        // south
        for i in 0..columns - 2 {
            let mut set_line = lines - 1;
            for j in (0..lines).rev() {
                let content = input[j * columns + i];
                if content == b'O' {
                    input[set_line * columns + i] = b'O';
                    if set_line != j {
                        input[j * columns + i] = b'.';
                    }
                    if set_line >= 1 {
                        set_line -= 1;
                    }
                }
                if content == b'#' && j > 0 {
                    set_line = j - 1;
                }
            }
        }
        // east
        for i in 0..lines {
            let mut set_column = columns - 3;
            for j in (0..columns - 2).rev() {
                let content = input[i * columns + j];
                if content == b'O' {
                    input[i * columns + set_column] = b'O';
                    if set_column != j {
                        input[i * columns + j] = b'.';
                    }
                    if set_column >= 1 {
                        set_column -= 1;
                    }
                }
                if content == b'#' && j > 0 {
                    set_column = j - 1;
                }
            }
        }
        let matching = first_cycle
            .iter()
            .zip(input.iter())
            .filter(|&(a, b)| a == b)
            .count();
        if matching == first_cycle.len() && matching == input.len() {
            let cycle_length = i - 100;
            let rest_cycles: i32 = run_cycles - i - 1;
            rest = rest_cycles % cycle_length;
            break;
        }
        if i == 100 {
            first_cycle = input.clone();
        }
    }
    for _i in 0..rest {
        // north
        for i in 0..columns - 2 {
            let mut set_line = 0;
            for j in 0..lines {
                let content = input[j * columns + i];
                if content == b'O' {
                    input[set_line * columns + i] = b'O';
                    if set_line != j {
                        input[j * columns + i] = b'.';
                    }
                    set_line += 1;
                }
                if content == b'#' {
                    set_line = j + 1;
                }
            }
        }
        // west
        for i in 0..lines {
            let mut set_column = 0;
            for j in 0..columns - 2 {
                let content = input[i * columns + j];
                if content == b'O' {
                    input[i * columns + set_column] = b'O';
                    if set_column != j {
                        input[i * columns + j] = b'.';
                    }
                    set_column += 1;
                }
                if content == b'#' {
                    set_column = j + 1;
                }
            }
        }
        // south
        for i in 0..columns - 2 {
            let mut set_line = lines - 1;
            for j in (0..lines).rev() {
                let content = input[j * columns + i];
                if content == b'O' {
                    input[set_line * columns + i] = b'O';
                    if set_line != j {
                        input[j * columns + i] = b'.';
                    }
                    if set_line >= 1 {
                        set_line -= 1;
                    }
                }
                if content == b'#' && j > 0 {
                    set_line = j - 1;
                }
            }
        }
        // east
        for i in 0..lines {
            let mut set_column = columns - 3;
            for j in (0..columns - 2).rev() {
                let content = input[i * columns + j];
                if content == b'O' {
                    input[i * columns + set_column] = b'O';
                    if set_column != j {
                        input[i * columns + j] = b'.';
                    }
                    if set_column >= 1 {
                        set_column -= 1;
                    }
                }
                if content == b'#' && j > 0 {
                    set_column = j - 1;
                }
            }
        }
    }
    day14_compute_load_without_tilting(input)
}

fn day14() {
    let mut data =
        std::fs::read("C:\\Users\\Hagen\\RustProjects\\adventofcode2023\\data\\day14.txt")
            .expect("Data for day14-Problem not found");

    println!("{}", day14_1(&data));
    //println!("{}", day14_1ver2(&mut data));
    println!("{}", day14_2(&mut data));
}

#[test]
fn day15tests() {
    let mut data = std::fs::read_to_string(
        "C:\\Users\\Hagen\\RustProjects\\adventofcode2023\\data\\day15test.txt",
    )
    .expect("Data for day15-Test not found");
    data = data.replace("\r\n", "");

    assert_eq!(day15_compute_hash("HASH"), 52);
    assert_eq!(day15_1(&data), 1320);
    assert_eq!(day15_2(&data), 145);
}

fn day15_compute_hash(step: &str) -> u32 {
    let mut sum: u32 = 0;
    for c in step.as_bytes() {
        sum += *c as u32;
        sum *= 17;
        sum %= 256;
    }
    //println!("{:?}, Hash: {:?}", step, sum);
    sum
}

fn day15_1(input: &str) -> u32 {
    input
        .split(|x| x == ',')
        .map(|x| day15_compute_hash(x))
        .sum()
}

#[derive(Debug)]
struct Day15Lens {
    label: String,
    focal: u32,
    action: char,
}

impl Day15Lens {
    fn new(input: &str) -> Self {
        if input.ends_with(|x| x == '-') {
            Self {
                label: String::from(input.trim_end_matches(|x| x == '-')),
                focal: 0,
                action: '-',
            }
        } else {
            let mut parts = input.split(|x| x == '=');
            Self {
                label: String::from(parts.next().unwrap()),
                focal: parts.next().unwrap().parse().unwrap(),
                action: '=',
            }
        }
    }
}

fn day15_2(input: &str) -> u32 {
    let mut boxes: Vec<Vec<Day15Lens>> = Vec::with_capacity(256);
    for _i in 0..256 {
        boxes.push(vec![]);
    }
    input.split(|x| x == ',').for_each(|x| {
        let lens = Day15Lens::new(x);
        let boxnumber = day15_compute_hash(lens.label.as_str()) as usize;
        let actbox = &mut boxes[boxnumber];
        if lens.action == '-' {
            let rem_index = actbox.iter().position(|x| x.label == lens.label);
            if let Some(ind) = rem_index {
                actbox.remove(ind);
            }
        } else {
            let rem_index = actbox.iter().position(|x| x.label == lens.label);
            if let Some(ind) = rem_index {
                actbox[ind].focal = lens.focal;
            } else {
                actbox.push(lens);
            }
        }
    });
    let mut sum = 0;
    for (boxnum, lenses) in boxes.iter().enumerate() {
        for (slotnum, lens) in lenses.iter().enumerate() {
            sum += ((boxnum + 1) * (slotnum + 1)) as u32 * lens.focal;
        }
    }
    sum
}

fn day15() {
    let mut data = std::fs::read_to_string(
        "C:\\Users\\Hagen\\RustProjects\\adventofcode2023\\data\\day15.txt",
    )
    .expect("Data for day15-Problem not found");
    data = data.replace("\r\n", "");

    println!("{}", day15_1(&data));
    println!("{}", day15_2(&data));
}

#[test]
fn day16tests() {
    let data =
        std::fs::read("C:\\Users\\Hagen\\RustProjects\\adventofcode2023\\data\\day16test.txt")
            .expect("Data for day16-Test not found");

    assert_eq!(day16_1(&data), 46);
    //assert_eq!(day16_2(&data), 30);
}

#[derive(Debug)]
struct Day16BeamPosition {
    x: usize,
    y: usize,
    direction: char,
}

impl Day16BeamPosition {
    fn new(x: usize, y: usize, direction: char) -> Self {
        Self { x, y, direction }
    }
}

fn day16_1(input: &Vec<u8>) -> usize {
    let columns = input.iter().position(|&x| x == b'\n').unwrap() + 1;
    let lines = input.len() / columns;
    let mut beam_positions: Vec<Day16BeamPosition> = vec![];
    let mut unchecked_beams: Vec<Day16BeamPosition> = vec![];
    unchecked_beams.push(Day16BeamPosition::new(0, 0, '>'));
    while !unchecked_beams.is_empty() {
        let position = unchecked_beams.pop().unwrap();
        if beam_positions
            .iter()
            .position(|x| {
                x.x == position.x && x.y == position.y && x.direction == position.direction
            })
            .is_some()
        {
            continue;
        }
        beam_positions.push(Day16BeamPosition::new(
            position.x,
            position.y,
            position.direction,
        ));
        let symbol = input[position.y * columns + position.x];
        match symbol {
            b'/' => match position.direction {
                '>' => {
                    if position.y >= 1 {
                        unchecked_beams.push(Day16BeamPosition::new(
                            position.x,
                            position.y - 1,
                            '^',
                        ));
                    }
                }
                '<' => {
                    if position.y + 1 < lines {
                        unchecked_beams.push(Day16BeamPosition::new(
                            position.x,
                            position.y + 1,
                            'v',
                        ));
                    }
                }
                '^' => {
                    if position.x + 1 < columns - 2 {
                        unchecked_beams.push(Day16BeamPosition::new(
                            position.x + 1,
                            position.y,
                            '>',
                        ));
                    }
                }
                'v' => {
                    if position.x >= 1 {
                        unchecked_beams.push(Day16BeamPosition::new(
                            position.x - 1,
                            position.y,
                            '<',
                        ));
                    }
                }
                _ => (),
            },
            b'\\' => match position.direction {
                '<' => {
                    if position.y >= 1 {
                        unchecked_beams.push(Day16BeamPosition::new(
                            position.x,
                            position.y - 1,
                            '^',
                        ));
                    }
                }
                '>' => {
                    if position.y + 1 < lines {
                        unchecked_beams.push(Day16BeamPosition::new(
                            position.x,
                            position.y + 1,
                            'v',
                        ));
                    }
                }
                'v' => {
                    if position.x + 1 < columns - 2 {
                        unchecked_beams.push(Day16BeamPosition::new(
                            position.x + 1,
                            position.y,
                            '>',
                        ));
                    }
                }
                '^' => {
                    if position.x >= 1 {
                        unchecked_beams.push(Day16BeamPosition::new(
                            position.x - 1,
                            position.y,
                            '<',
                        ));
                    }
                }
                _ => (),
            },
            b'|' => match position.direction {
                '^' => {
                    if position.y >= 1 {
                        unchecked_beams.push(Day16BeamPosition::new(
                            position.x,
                            position.y - 1,
                            '^',
                        ));
                    }
                }
                'v' => {
                    if position.y + 1 < lines {
                        unchecked_beams.push(Day16BeamPosition::new(
                            position.x,
                            position.y + 1,
                            'v',
                        ));
                    }
                }
                '>' | '<' => {
                    if position.y >= 1 {
                        unchecked_beams.push(Day16BeamPosition::new(
                            position.x,
                            position.y - 1,
                            '^',
                        ));
                    }
                    if position.y + 1 < lines {
                        unchecked_beams.push(Day16BeamPosition::new(
                            position.x,
                            position.y + 1,
                            'v',
                        ));
                    }
                }
                _ => (),
            },
            b'-' => match position.direction {
                '^' | 'v' => {
                    if position.x + 1 < columns - 2 {
                        unchecked_beams.push(Day16BeamPosition::new(
                            position.x + 1,
                            position.y,
                            '>',
                        ));
                    }
                    if position.x >= 1 {
                        unchecked_beams.push(Day16BeamPosition::new(
                            position.x - 1,
                            position.y,
                            '<',
                        ));
                    }
                }
                '>' => {
                    if position.x + 1 < columns - 2 {
                        unchecked_beams.push(Day16BeamPosition::new(
                            position.x + 1,
                            position.y,
                            '>',
                        ));
                    }
                }
                '<' => {
                    if position.x >= 1 {
                        unchecked_beams.push(Day16BeamPosition::new(
                            position.x - 1,
                            position.y,
                            '<',
                        ));
                    }
                }
                _ => (),
            },
            b'.' => match position.direction {
                '^' => {
                    if position.y >= 1 {
                        unchecked_beams.push(Day16BeamPosition::new(
                            position.x,
                            position.y - 1,
                            '^',
                        ));
                    }
                }
                'v' => {
                    if position.y + 1 < lines {
                        unchecked_beams.push(Day16BeamPosition::new(
                            position.x,
                            position.y + 1,
                            'v',
                        ));
                    }
                }
                '>' => {
                    if position.x + 1 < columns - 2 {
                        unchecked_beams.push(Day16BeamPosition::new(
                            position.x + 1,
                            position.y,
                            '>',
                        ));
                    }
                }
                '<' => {
                    if position.x >= 1 {
                        unchecked_beams.push(Day16BeamPosition::new(
                            position.x - 1,
                            position.y,
                            '<',
                        ));
                    }
                }
                _ => (),
            },
            _ => (),
        }
    }
    let mut unique_positions: Vec<(usize, usize)> = vec![];
    for pos in beam_positions {
        let coord = (pos.x, pos.y);
        if !unique_positions.contains(&coord) {
            unique_positions.push(coord);
        }
    }
    unique_positions.len()
}

//fn day16_2(input: &str) -> i32 {}

fn day16() {
    let data = std::fs::read("C:\\Users\\Hagen\\RustProjects\\adventofcode2023\\data\\day16.txt")
        .expect("Data for day16-Problem not found");

    println!("{}", day16_1(&data));
    //println!("{}", day16_2(&data));
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
    println!("Day8 results:");
    day8();
    println!("Day9 results:");
    day9();
    println!("Day10 results:");
    day10();
    println!("Day11 results:");
    day11();
    println!("Day12 skipped");
    //day12();
    println!("Day13 results:");
    day13();
    println!("Day14 results:");
    day14();
    println!("Day15 results:");
    day15();
    println!("Day16 results:");
    day16();
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
