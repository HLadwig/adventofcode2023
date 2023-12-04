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
    .expect("Data for day1-Test not found");

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
    .expect("Data for day1-Test not found");

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
    .expect("Data for day1-Test not found");

    assert_eq!(day4_1(&data), 13);
    //assert_eq!(day4_2(&data), 30);
}

#[derive(Debug)]
struct Day4Card {
    card: i32,
    count: i32,
    winning_numbers: Vec<i32>,
    actual_numbers: Vec<i32>,
    winners: i32,
}

impl Day4Card {
    fn new(line: &str) -> Self {
        let card = line.split(':').next().unwrap().replace("Card ", "");
        let mut content = line.split(": ").last().unwrap().split(" | ");
        let wn = content.next().unwrap().trim().replace("  ", " ");
        let an = content.next().unwrap().trim().replace("  ", " ");
        let winning_numbers: Vec<i32> = wn.split(' ').map(|x| x.parse().unwrap()).collect();
        let actual_numbers = an.split(' ').map(|x| x.parse().unwrap()).collect();
        let mut count = 0;
        for num in &actual_numbers {
            count += if winning_numbers.contains(num) { 1 } else { 0 };
        }
        Self {
            card: card.trim().parse().unwrap(),
            count: 1,
            winning_numbers,
            actual_numbers,
            winners: count,
        }
    }

    fn get_winner_count(&self) -> i32 {
        let mut count = 0;
        for num in &self.actual_numbers {
            count += if self.winning_numbers.contains(num) {
                1
            } else {
                0
            };
        }
        count
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

/*
fn day4rec(mut cards: Vec<Day4Card>, sum: i32) -> i32 {
    cards.reverse();
    let act_card = cards.pop().unwrap();
    let mut treffer = act_card.get_winner_count();
    let anzahl = act_card.count;
    if cards.len() == 0 {
        return anzahl;
    } else {
        for card in cards {
            if card.card > act_card.card && card.card <= act_card.card + treffer {
                card.count += anzahl;
            }
        }
        5
    }
}
*/

/*
fn day4_2(input: &str) -> i32 {
    let cards: Vec<Day4Card> = input.lines().map(Day4Card::new).collect();
    let mut counts: Vec<i32> = cards.iter().map(|x| x.count).collect();
    for card in cards {
        let winners = card.get_winner_count();
        let act_card = card.card;
    }
    0
}*/

fn day4() {
    let data =
        std::fs::read_to_string("C:\\Users\\Hagen\\RustProjects\\adventofcode2023\\data\\day4.txt")
            .expect("Data for day4-Problem not found");

    println!("{}", day4_1(&data));
    //println!("{}", day4_2(&data));
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
}
