use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Copy, Clone)]
struct ButtonPrize {
    a: (u32, u32),
    b: (u32, u32),
    prize: (u32, u32),
}

fn main() {
    let button_prizes = parse("data/input.txt");
    let mut res = 0;
    for button_prize in button_prizes {
        let mut ans = 100000000;
        for i in 0..100 {
            for j in 0..100 {
                if (button_prize.a.0 * i) + (button_prize.b.0 * j) > button_prize.prize.0
                    || (button_prize.a.1 * i) + (button_prize.b.1 * j) > button_prize.prize.1
                {
                    continue;
                }
                if (button_prize.a.0 * i) + (button_prize.b.0 * j) == button_prize.prize.0
                    && (button_prize.a.1 * i) + (button_prize.b.1 * j) == button_prize.prize.1
                {
                    ans = ans.min((i * 3) + j);
                }
            }
        }
        if ans != 100000000 {
            res += ans;
        }
    }
    println!("{}", res)
}

fn parse(filename: &str) -> Vec<ButtonPrize> {
    let mut button_prizes = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        let mut line_buffer = Vec::new();
        for (index, line_result) in lines.enumerate() {
            if let Ok(line) = line_result {
                match index % 4 {
                    0 => {
                        if let Some(a_coords) = parse_line(&line, ": X+", ", Y+") {
                            line_buffer.push(a_coords);
                        }
                    }
                    1 => {
                        if let Some(b_coords) = parse_line(&line, ": X+", ", Y+") {
                            line_buffer.push(b_coords);
                        }
                    }
                    2 => {
                        if let Some(prize_coords) = parse_line(&line, ": X=", ", Y=") {
                            line_buffer.push(prize_coords);
                        }
                    }
                    3 => {
                        let button_prize = ButtonPrize {
                            a: line_buffer[0],
                            b: line_buffer[1],
                            prize: line_buffer[2],
                        };
                        button_prizes.push(button_prize);
                        line_buffer.clear();
                    }
                    _ => {}
                }
            }
        }
    }
    button_prizes
}

fn parse_line(line: &str, x_del: &str, y_del: &str) -> Option<(u32, u32)> {
    line.split_once(x_del).and_then(|(_, coords)| {
        coords.split_once(y_del).and_then(|(x, y)| {
            let x_parsed = x.parse::<u32>().ok()?;
            let y_parsed = y.parse::<u32>().ok()?;
            Some((x_parsed, y_parsed))
        })
    })
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
