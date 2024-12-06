pub fn day_3_first(input: String) {
    let input = input.chars().collect::<Vec<char>>();
    let mut res = 0;

    let n = input.len();
    let mut i = 0;

    while i < n {
        match input[i] {
            'm' => check(i, &input, n, &mut res),
            _ => {}
        }

        i += 1;
    }

    println!("{}", res);
}

pub fn day_3_second(input: String) {
    let input = input.chars().collect::<Vec<char>>();
    let mut res = 0;
    let mut x = true;

    let n = input.len();
    let mut i = 0;

    while i < n {
        match input[i] {
            'm' => if x {check(i, &input, n, &mut res)},
            'd' => flag(i, &input, n, &mut x),
            _ => {}
        }

        i += 1;
    }

    println!("{}", res);
}

fn flag(i: usize, input: &Vec<char>, n: usize, x: &mut bool) {
    if i + 3 < n && input[i..=(i+3)].iter().collect::<String>() == "do()".to_string() {
        *x = true;
        return
    }

    if i + 6 < n && input[i..=(i+6)].iter().collect::<String>() == "don't()".to_string() {
        *x = false;
    }
}

fn check(i:usize, input: &Vec<char>, n: usize, res: &mut usize) {
    if i + 2 >= n { return }
    if &input[i..=(i+2)] == &['m', 'u', 'l'] {
        let j = i + 3;
        parse_brackets(j, input, n, res);
    }
}

fn parse_brackets(j: usize, input: &Vec<char>, n: usize, res: &mut usize) {

    if j >= n || input[j] != '(' {return}

    let mut c = 0;

    while c <= 8 && j + c < n && input[j + c] != ')' {
        c += 1;
    }

    if j + c >= n || input[j+c] != ')' {return}

    let s = &input[(j+1)..(j+c)].iter().collect::<String>();

    let vals = s.split(',').map(|x| x.parse::<i32>()).collect::<Vec<_>>();

    if vals.len() != 2 {
        return
    }

    match (vals[0].clone(), vals[1].clone()) {
        (Ok(a),Ok(b)) => {
            *res += (a * b) as usize
        },
        _ => {}
    }
}
