pub fn solve_1(lines: &[&str]) -> usize {
    lines.iter().map(|l| {
        let l = l.chars().map(parse_char).collect::<Vec<_>>();
        let mut stack: Vec<u8> = Vec::new();
        let corruption = l.iter().position(|&c| {
            if (c & 0x4) == 0 {
                stack.push(c);
                false
            } else {
                let top = stack.pop();
                top != Some(c & 0x3)
            }
        });
        if let Some(corruption) = corruption {
            let bad = l[corruption];
            match bad {
                0x4 => 3,
                0x5 => 57,
                0x6 => 1197,
                0x7 => 25137,
                _ => unreachable!("Bad character {} unknown", bad)
            } 
        } else {
            0
        }
    }).sum()
}

pub fn solve_2(lines: &[&str]) -> usize {
    let mut scores = lines.iter().filter_map(|l| {
        let l = l.chars().map(parse_char).collect::<Vec<_>>();
        let mut stack: Vec<u8> = Vec::new();
        let corruption = l.iter().position(|&c| {
            if (c & 0x4) == 0 {
                stack.push(c);
                false
            } else {
                let top = stack.pop();
                top != Some(c & 0x3)
            }
        });
        if corruption.is_none() {
            stack.reverse();
            Some(stack.into_iter().fold(0, |score, c| {
                score * 5 + c as usize + 1
            }))
        } else {
            None
        }
    }).collect::<Vec<_>>();
    scores.sort();
    scores[scores.len()/2]
}

pub fn parse_char(c: char) -> u8 {
    "([{<)]}>".chars().position(|x| x == c).unwrap() as u8    
}