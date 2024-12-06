pub fn day_2_first(input: String) {
	let lines = input.lines()
		.map(|x| x.split_whitespace().map(|n: &str| n.parse::<i32>().unwrap()).collect::<Vec<_>>()
	).collect::<Vec<_>>();

    let mut res = 0;

	for l in lines {
        if l.len() == 1 {res += 1} else {
            if l[0] > l[1] {
                res += l.iter().zip(l.iter().skip(1)).all(|(a, b)|  (a-b) >= 1 && (a-b) <= 3) as usize
            } else if l[0] < l[1] {
                res += l.iter().zip(l.iter().skip(1)).all(|(a, b)|  (b-a) >= 1 && (b-a) <= 3) as usize
            }
        }
	}

    println!("{}", res);

}

pub fn day_2_second(input: String) {
	let lines = input.lines()
		.map(|x| x.split_whitespace().map(|n: &str| n.parse::<i32>().unwrap()).collect::<Vec<_>>()
	).collect::<Vec<_>>();

    let mut res = 0;

	for l in lines {
        if l.len() == 1 {res += 1}
        else {
            
            let mut g = false;
                
            g = g || l.iter().zip(l.iter().skip(1)).all(|(a, b)|  (b-a) >= 1 && (b-a) <= 3);
            g  = g || l.iter().zip(l.iter().skip(1)).all(|(a, b)|  (a-b) >= 1 && (a-b) <= 3);
            
            for i in 0..l.len() {
                let temp = l.iter().enumerate().filter(|(x, _)| *x != i).map(|(_, &v)| v).collect::<Vec<_>>();
                g = g || temp.iter().zip(temp.iter().skip(1)).all(|(a, b)|  (b-a) >= 1 && (b-a) <= 3);
            }

            for i in 0..l.len() {
                let temp = l.iter().enumerate().filter(|(x, _)|*x != i).map(|(_, &v)| v).collect::<Vec<_>>();
                g = g || temp.iter().zip(temp.iter().skip(1)).all(|(a, b)|  (a-b) >= 1 && (a-b) <= 3);
            }

            res += g as usize
        }
	}

    println!("{}", res);

}