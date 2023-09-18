fn main() {
    hex_to_decimal("FF45A8");
    hex_to_decimal("FDA897");
    hex_to_decimal("ABC123");
    hex_to_decimal("FFFFF");
    hex_to_decimal("3B5D8F");
    hex_to_decimal("789DEF");
    hex_to_decimal("2015FFAAB");
}

pub fn hex_to_decimal(hex: &str) {
    let mut raw = String::new();

    raw.push_str(r"\begin{center}");
    raw.push_str("\n");
    raw.push_str(&format!("hexadecimal={}", hex));
    raw.push_str(r" \\");
    raw.push_str("\n");
    raw.push_str(r"\end{center}");
    raw.push_str("\n\n");

    raw.push_str(r"\begin{center}");
    raw.push_str("\n");
    raw.push_str(r"\begin{tabular}");

    raw.push_str("{ ");
    for _ in 0..hex.len() {
        raw.push_str(r"|c");
    }
    raw.push_str("| }");
    raw.push_str("\n");

    raw.push_str(r" \hline");
    raw.push_str("\n");

    for (index, ch) in hex.chars().enumerate() {
        if index == hex.len() - 1 {
            raw.push_str(&format!(" {}", ch));
            raw.push_str(r" \\");
        } else {
            raw.push_str(&format!(" {} & ", ch));
        }
    }

    raw.push_str("\n");

    for (index, pos) in (0..hex.len()).rev().enumerate() {
        if index == hex.len() - 1 {
            raw.push_str(&format!(" 16^{}", pos));
            raw.push_str(r" \\");
        } else {
            raw.push_str(&format!(" 16^{} & ", pos));
        }
    }

    raw.push_str("\n");

    let hex_vec: Vec<char> = hex.chars().collect();
    for (index, pos) in (0..hex.len()).rev().enumerate() {
        let value: u128 = match hex_vec[index] {
            'A' => 10 * 16_u128.pow(pos as u32),
            'B' => 11 * 16_u128.pow(pos as u32),
            'C' => 12 * 16_u128.pow(pos as u32),
            'D' => 13 * 16_u128.pow(pos as u32),
            'E' => 14 * 16_u128.pow(pos as u32),
            'F' => 15 * 16_u128.pow(pos as u32),
            '0' => 0 * 16_u128.pow(pos as u32),
            '1' => 1 * 16_u128.pow(pos as u32),
            '2' => 2 * 16_u128.pow(pos as u32),
            '3' => 3 * 16_u128.pow(pos as u32),
            '4' => 4 * 16_u128.pow(pos as u32),
            '5' => 5 * 16_u128.pow(pos as u32),
            '6' => 6 * 16_u128.pow(pos as u32),
            '7' => 7 * 16_u128.pow(pos as u32),
            '8' => 8 * 16_u128.pow(pos as u32),
            '9' => 9 * 16_u128.pow(pos as u32),
            unknown => {
                panic!("Error bitch! {}", unknown);
            }
        };

        if index == hex.len() - 1 {
            raw.push_str(&format!(" {}", value));
            raw.push_str(r" \\");
        } else {
            raw.push_str(&format!(" {} & ", value));
        }
    }

    raw.push_str("\n");
    raw.push_str(r" \hline");

    raw.push_str("\n");
    raw.push_str(r"\end{tabular}");
    raw.push_str("\n");
    raw.push_str(r"\end{center}");
    raw.push_str("\n\n");

    raw.push_str(r"\begin{center}");
    raw.push_str("\n");
    let mut final_value = 0;
    let hex_vec: Vec<char> = hex.chars().collect();
    for (index, pos) in (0..hex.len()).rev().enumerate() {
        let value: u128 = match hex_vec[index] {
            'A' => 10 * 16_u128.pow(pos as u32),
            'B' => 11 * 16_u128.pow(pos as u32),
            'C' => 12 * 16_u128.pow(pos as u32),
            'D' => 13 * 16_u128.pow(pos as u32),
            'E' => 14 * 16_u128.pow(pos as u32),
            'F' => 15 * 16_u128.pow(pos as u32),
            '0' => 0 * 16_u128.pow(pos as u32),
            '1' => 1 * 16_u128.pow(pos as u32),
            '2' => 2 * 16_u128.pow(pos as u32),
            '3' => 3 * 16_u128.pow(pos as u32),
            '4' => 4 * 16_u128.pow(pos as u32),
            '5' => 5 * 16_u128.pow(pos as u32),
            '6' => 6 * 16_u128.pow(pos as u32),
            '7' => 7 * 16_u128.pow(pos as u32),
            '8' => 8 * 16_u128.pow(pos as u32),
            '9' => 9 * 16_u128.pow(pos as u32),
            unknown => {
                panic!("Error bitch! {}", unknown);
            }
        };

        final_value += value;

        if index == hex.len() - 1 {
            raw.push_str(&format!("{} = {}", value, final_value));
            raw.push_str(r" \\");
            raw.push_str("\n");
            raw.push_str(&format!("decimal = {}", final_value));
        } else {
            raw.push_str(&format!("{} + ", value));
        }
    }

    raw.push_str("\n");
    raw.push_str(r"\end{center}");
    raw.push_str("\n\n");

    println!("{}", raw)
}
