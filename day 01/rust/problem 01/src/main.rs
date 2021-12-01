use std::env;
use std::fs;

fn get_numbers() -> Vec<usize> {
    let args: Vec<String> = env::args().collect();
    let input = args.get(1).expect("Missing commandline argument!");

    let contents = fs::read_to_string(input)
        .expect("Could not read from file!");

    const ZERO: usize = 0;

    let numbers: Vec<usize> = contents
        .split(char::is_whitespace)
        .map(|s| s.parse::<usize>().unwrap_or(ZERO))
        .filter(|x| x > &ZERO)
        .collect();

    return numbers;
}

#[derive(Debug)]
struct IncreaseCount {
    counter: usize,
    last: usize,
}

fn find_increase_count(numbers: &mut Vec<usize>) -> usize {
    let (first, rest) = numbers.split_first().unwrap();

    let start: IncreaseCount = IncreaseCount { counter: 0, last: *first };
    
    let result: IncreaseCount = rest.iter().fold(start, |acc, element| {
        // println!("{:?} => {:?}, {:?}", acc, element, element > &acc.last);
        match element > &acc.last {
            true => {
                IncreaseCount {
                    counter: acc.counter+1,
                    last: *element,
                }
            },
            false => IncreaseCount {
                counter: acc.counter,
                last: *element,
            },
        }
    }, );

    let mut counter = 0;
    let mut last = *first;

    for elem in rest {
        if elem > &mut last {
            counter = counter + 1;
        }
        if elem == &mut last {
            println!("equal");
        }
        last = *elem
    }
    println!("{:?}", counter);

    result.counter
}

fn main() {
    let mut numbers: Vec<usize> = get_numbers();

    let number_of_increases = find_increase_count(&mut numbers);

    println!("{:?}", number_of_increases);
}
