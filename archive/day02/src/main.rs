fn main() {
    let lines = include_str!("../day02.txt").lines();
    /*
    let result = lines.fold((0,0),
        |coord, line| {
            let mut split = line.split(" ");
            let direction = split.next().unwrap();
            let distance = split.next().unwrap().parse::<i32>().unwrap();
            match direction {
                "forward" => (coord.0 + distance, coord.1),
                "up" => (coord.0, coord.1 - distance),
                "down" => (coord.0, coord.1 + distance),
                _ => unreachable!("Invalid direction")
            }
        });
        */
    let result = lines.fold((0,0,0),
        |coord, line| {
            let mut split = line.split(" ");
            let direction = split.next().unwrap();
            let distance = split.next().unwrap().parse::<i32>().unwrap();
            match direction {
                "forward" => (coord.0 + distance, coord.1 + coord.2 * distance, coord.2),
                "up" => (coord.0, coord.1, coord.2 - distance),
                "down" => (coord.0, coord.1, coord.2 + distance),
                _ => unreachable!("Invalid direction")
            }
        });
    println!("result: {}", result.0 * result.1);
}
