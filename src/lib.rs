use std::io::Write;

pub struct LatexWriter<W> {
    sink: W,
}

impl<W: Write> LatexWriter<W> {
    pub fn new(sink: W) -> Self {
        Self { sink }
    }

    pub fn write(&mut self, event: LatexEvent) -> Result<(), std::io::Error> {
        match event {
            LatexEvent::BeginDocument => self.sink.write_all(b"\\begin{document}\n"),
            LatexEvent::EndDocument => self.sink.write_all(b"\\end{document}\n"),
            LatexEvent::Text(text) => self.write_text(text),
        }
    }

    fn write_text(&mut self, mut text: String) -> Result<(), std::io::Error> {
        text.push_str("\n");
        self.sink.write_all(text.as_bytes())
    }
}

pub enum LatexEvent {
    BeginDocument,
    EndDocument,
    Text(String),
}
