fn get_input() -> String {
    let args: Vec<String> = std::env::args().collect();
    let input = args.get(1).expect("Missing commandline argument!");

     std::fs::read_to_string(input)
        .expect("Could not read from file!")
}

fn get_max_for_binary_bits(bits: usize) -> usize {
    2usize.pow(bits.try_into().unwrap()) - 1
}

fn get_gamma_value(numbers: Vec<&str>, bitcount: usize) -> usize {
    #[derive(Copy, Clone, Debug)]
    struct Counter(usize, usize);

    let mut counters: Vec<Counter> = Vec::new();
    for _ in 1..=bitcount {
        counters.push(Counter{0: 0, 1: 0})
    }

    for number in numbers {
        let mut entry = number.chars();
        println!("Entry: {:?}", entry);
        for bit in 0..bitcount {
            let mut counter = &mut counters[bit];
            match entry.nth(bit).unwrap() {
                '0' => counter.0 = counter.0 + 1,
                _ => counter.1 = counter.1 + 1,
            }
            counters[bit] = *counter;
        }
    }

    println!("Counter at end: {:?}", counters);

    bitcount
}

fn calc_epsilon(gamma: usize, maximum: usize) -> usize {
    maximum - gamma
}

fn calc_result(input: String) -> (usize, usize, usize) {
    let numbers : Vec<&str> = input.lines().collect();

    let count_bits = numbers.first().unwrap().len();
    let maximum = get_max_for_binary_bits(count_bits);
    println!("Maximum: {:?}", maximum);
    let gamma = get_gamma_value(numbers, count_bits);
    let epsilon = calc_epsilon(gamma, maximum);

    (gamma, epsilon, gamma * epsilon)
}

fn main() {
    let input = get_input();
    let (gamma, epsilon, power_consumption) = calc_result(input);

    println!("Result: {:?} + {:?} = {:?}", gamma, epsilon, power_consumption)
}

#[test]
fn test_gamma_calculcation() -> Result<(), String> {
    fn internal(input: Vec<&str>, expected: usize) -> Result<(), String> {
        let count_bits = input.first().unwrap().len();
        let res = get_gamma_value(input, count_bits);
        match res == expected{
            true => Ok(()),
            _ => Err(format!("Got {:?}, expected {:?}", res, expected))
        }
    }

    struct GammaTest(Vec<&'static str>, usize);

    impl GammaTest {
        fn new(input: Vec<&'static str>, expected: usize) -> Self {
            GammaTest{ 0: input, 1: expected }
        }
    }

    let tests : Vec<GammaTest> = vec![
        GammaTest::new(vec!["00100"], 4),
        GammaTest::new(vec!["00001"], 1),
        // GammaTest::new(vec!["10110"], 22),
    ];

    for test in tests {
        let res = internal(test.0, test.1);
        if  res.is_err() {
            return res
        }
    }

    Ok(())
}