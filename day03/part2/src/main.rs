use atoi;
pub fn main() {
    // Step 1: Read the content of the file "input.txt" as bytes into the 'map' variable.
    let map = include_bytes!("../input.txt");

    // Step 2: Find the width of the map by locating the position of the first newline character.
    let width = map.iter().position(|b| b == &b'\n').unwrap() as isize;

    // Step 3: Create an empty vector 'nums' to store positions of ASCII digits around '*' characters.
    let mut nums = vec![];

    // Step 4: Perform the main computation, filtering positions with '*' characters,
    // and calculating the product of two adjacent ASCII digits around each '*'.
    let result = (0..map.len() - 2)
        .filter(|i| map[*i] == b'*')
        .filter_map(|i| {
            // Step 5: Iterate over positions around the '*' character to find adjacent ASCII digits.
            nums.clear();
            nums.extend(
                (-width - 2..=-width)
                    .chain([-1, 1])
                    .chain(width..=width + 2)
                    .map(|pos| (i as isize + pos) as usize)
                    .filter(|pos| map[*pos].is_ascii_digit())
                    .flat_map(|pos| {
                        // Step 6: For each position, find the last three ASCII digits before the current position.
                        (pos.saturating_sub(2)..=pos)
                            .rev()
                            .take_while(|i| map[*i].is_ascii_digit())
                            .last()
                    }),
            );
            // Step 7: Deduplicate the 'nums' vector and check if there are exactly two adjacent ASCII digits.
            nums.dedup();
            (nums.len() == 2).then(|| {
                // Step 8: If there are two adjacent ASCII digits, calculate their product.
                nums.iter()
                    .map(|i| atoi::atoi::<usize>(&map[*i..*i + 3]).unwrap())
                    .product::<usize>()
            })
        })
        .sum::<usize>();

    // Step 9: Print the final result.
    println!("{}", result);
}
