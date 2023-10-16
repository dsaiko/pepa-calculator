use pepino::Computer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut computer = Computer::default();

    computer.compute(
        "\
        5 + 4
        13 +     8
        20
        15 * 5 + 3 / (5 / (4 + 1))
        15 * 5 + 30 / 5 * 3
        15 + 5 * 3
        1 / 3 * 5 * 3
        (15 - 5) / 2
        10 celsius + 1 kelvin
        10 celsius + 1 kelvin
        - 1 / - 3
    ",
    );

    Ok(())
}
