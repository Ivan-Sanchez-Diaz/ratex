use ratex::{LatexEvent, LatexWriter};

fn main() -> Result<(), std::io::Error> {
    let output = std::io::stdout();
    let mut writer = LatexWriter::new(output);

    writer.write(LatexEvent::BeginDocument)?;
    writer.write(LatexEvent::Text("Hello, world!".to_string()))?;
    writer.write(LatexEvent::EndDocument)?;

    Ok(())
}
