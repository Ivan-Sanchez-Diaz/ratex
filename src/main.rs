use ratex::{Document, LatexEvent, LatexWriter};

fn main() -> Result<(), std::io::Error> {
    let output = std::io::stdout();

    let mut writer = LatexWriter::new(output);
    let mut doc = Document::new();

    doc.push(LatexEvent::Section("The simple stuff"));
    doc.push(LatexEvent::Text("Some regular text and some"));
    doc.push(LatexEvent::ItalicText("italic text. "));
    doc.push(LatexEvent::Newline);
    doc.push(LatexEvent::Text("Also some crazy characters: \\$\\&\\#{}"));
    doc.push(LatexEvent::Subsection("Math that is incorrect"));
    doc.push(LatexEvent::Text("2*3=9"));

    writer.write_doc(doc)?;

    Ok(())
}

