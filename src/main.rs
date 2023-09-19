use std::slice::Iter;

fn main() {
    bin_to_hexa("10111010001");
    hex_to_octal("65FE8");
    //hex_to_octal("789FF");
    //hex_to_octal("EDF759");
    //hex_to_octal("FFABCD");
    //hex_to_octal("125EFA");
}

pub fn bin_to_hexa(bin: &str) {
    let mut raw = String::new();

    let vec: Vec<String> = bin
        .chars()
        .map(|ch| ch.to_digit(2).unwrap().to_string())
        .collect();

    let vec: Vec<String> = vec.rchunks(4).rev().map(|c| c.join("")).collect();

    begin_center(&mut raw);
    write(&mut raw, &format!("binario={}", bin));
    end_center(&mut raw);

    begin_center(&mut raw);
    begin_tabular(&mut raw, vec.len());
    hline(&mut raw);

    add_rows(&mut raw, vec.iter());

    let hex: Vec<u32> = vec
        .iter()
        .map(|g| u32::from_str_radix(g, 2).unwrap())
        .collect();

    let hex: Vec<String> = hex
        .iter()
        .map(|n| format!("{:x}", n).to_uppercase())
        .collect();

    add_rows(&mut raw, hex.iter());

    hline(&mut raw);
    end_tabular(&mut raw);
    end_center(&mut raw);

    println!("{}", raw)
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

    begin_center(&mut raw);
    write(&mut raw, &format!("hexadecimal={}", hex));
    end_center(&mut raw);

    let decimal = u128::from_str_radix(hex, 16).unwrap();
    let mut cociente = decimal;

    let mut count = 0;
    begin_center(&mut raw);
    loop {
        count = count + 1;

        raw.push_str(r"\opidiv[voperation=top]{");
        raw.push_str(&cociente.to_string());
        raw.push_str("}{");
        raw.push_str(&8.to_string());
        raw.push_str("}");

        cociente = cociente / 8;
        if cociente < 8 {
            newline(&mut raw);
            end_center(&mut raw);
            break;
        }

        raw.push_str(r" \quad");
        newline(&mut raw);

        if count == 3 {
            count = 0;
            end_center(&mut raw);
            begin_center(&mut raw);
        }
    }

    begin_center(&mut raw);
    write(&mut raw, &format!("octal={:o}", decimal));
    end_center(&mut raw);

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
