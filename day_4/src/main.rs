fn main() {
    let data = "input.test";
    println!("Using data file {}", data);

    let contents = std::fs::read_to_string(data).expect("Error opening input file");
    let chunks = contents.split("\n\n").collect::<Vec<&str>>();
    println!("{:?}", chunks);

    let input = chunks
        .first()
        .expect("Error parsing bingo input")
        .split(",")
        .map(|n| n.parse::<u32>().expect("Error parsing bingo input"))
        .collect::<Vec<u32>>();

    println!("Bingo input: {:?}", input);

    let boards = chunks
        .iter()
        .skip(1)
        .map(|board| board.split("\n").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    println!("Boards: {:?}", boards);
}
