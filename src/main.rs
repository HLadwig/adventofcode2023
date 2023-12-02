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

fn main() {
    println!("Day1 results:");
    day1();
    println!("Day2 results:");
    day2();
}
