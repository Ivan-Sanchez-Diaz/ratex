use std::slice::Iter;

fn main() {
    hex_to_octal("65FE8");
}

pub fn hex_to_decimal(hex: &str) {
    let mut raw = String::new();

    begin_center(&mut raw);
    write(&mut raw, &format!("hexadecimal={}", hex));
    end_center(&mut raw);

    begin_center(&mut raw);
    begin_tabular(&mut raw, hex.len());
    hline(&mut raw);

    let chars: Vec<char> = hex.chars().collect();
    add_rows(&mut raw, chars.iter());

    let pows: Vec<String> = (0..hex.len()).rev().map(|i| format!("16^{}", i)).collect();
    add_rows(&mut raw, pows.iter());

    let digits: Vec<u32> = hex.chars().map(|c| c.to_digit(16).unwrap()).collect();
    let mut values: Vec<u128> = Vec::new();
    for (index, digit) in digits.iter().rev().enumerate() {
        let value = *digit as u128 * 16_u128.pow(index as u32);
        values.push(value);
    }

    values.reverse();
    add_rows(&mut raw, values.iter());

    newline(&mut raw);
    hline(&mut raw);
    end_tabular(&mut raw);
    end_center(&mut raw);

    begin_center(&mut raw);

    let mut decimal = 0;
    for (index, value) in values.iter().enumerate() {
        decimal += value;

        if index == hex.len() - 1 {
            raw.push_str(&format!("{} = {}", value, decimal));
            raw.push_str(r" \\");
            raw.push_str("\n");
            raw.push_str(&format!("decimal = {}", decimal));
        } else {
            raw.push_str(&format!("{} + ", value));
        }
    }

    newline(&mut raw);
    end_center(&mut raw);

    println!("{}", raw)
}

pub fn hex_to_octal(hex: &str) {
    let mut raw = String::new();
    write(&mut raw, &format!("hexadecimal={}", hex));

    let decimal = u128::from_str_radix(hex, 16).unwrap();
    let mut cociente = decimal;
    loop {
        raw.push_str(r"\opidiv{");
        raw.push_str(&cociente.to_string());
        raw.push_str("}{");
        raw.push_str(&8.to_string());
        raw.push_str("}");
        raw.push_str(r" \quad");
        newline(&mut raw);

        cociente = cociente / 8;
        if cociente < 8 {
            break;
        }
    }

    write(&mut raw, &format!("octal={:o}", decimal));

    println!("{}", raw)
}

pub fn write(raw: &mut String, msg: &str) {
    raw.push_str(&format!("{}", msg));
    raw.push_str(r" \\");
    newline(raw);
}

pub fn begin_center(raw: &mut String) {
    raw.push_str(r"\begin{center}");
    raw.push_str("\n");
}

pub fn end_center(raw: &mut String) {
    raw.push_str(r"\end{center}");
    raw.push_str("\n\n");
}

pub fn begin_tabular(raw: &mut String, colums: usize) {
    raw.push_str(r"\begin{tabular}");

    raw.push_str("{ ");
    for _ in 0..colums {
        raw.push_str(r"|c");
    }
    raw.push_str("| }");
    raw.push_str("\n");
}

pub fn add_rows<T: std::fmt::Display>(raw: &mut String, iter: Iter<T>) {
    let len = iter.len();
    for (index, item) in iter.enumerate() {
        if index == len - 1 {
            raw.push_str(&format!(" {}", item));
            raw.push_str(r" \\");
        } else {
            raw.push_str(&format!(" {} & ", item));
        }
    }

    newline(raw);
}

pub fn end_tabular(raw: &mut String) {
    raw.push_str(r"\end{tabular}");
    raw.push_str("\n");
}

pub fn newline(raw: &mut String) {
    raw.push_str("\n");
}

pub fn hline(raw: &mut String) {
    raw.push_str(r" \hline");
    raw.push_str("\n");
}
