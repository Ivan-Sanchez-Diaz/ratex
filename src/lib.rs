use std::{collections::VecDeque, io::Write};

pub struct LatexWriter<W> {
    sink: W,
}

impl<W: Write> LatexWriter<W> {
    pub fn new(sink: W) -> Self {
        Self { sink }
    }

    pub fn write_doc(&mut self, mut doc: Document) -> Result<(), std::io::Error> {
        match doc.events.back() {
            Some(event) => {
                if !event.eq(&LatexEvent::EndDocument) {
                    doc.push(LatexEvent::EndDocument)
                }
            }

            None => panic!("Empty document"),
        }

        while let Some(event) = doc.events.pop_front() {
            self.write(event)?;
        }

        Ok(())
    }

    pub fn write(&mut self, event: LatexEvent) -> Result<(), std::io::Error> {
        match event {
            LatexEvent::DocumentClasss => self.write_document_class(),
            LatexEvent::BeginDocument => self.write_document_begin(),
            LatexEvent::EndDocument => self.write_document_end(),
            LatexEvent::Text(text) => self.write_text(text),
            LatexEvent::Section(section) => self.write_section(section),
            LatexEvent::Newline => self.write_newline(),
            LatexEvent::ItalicText(text) => self.write_italic_text(text),
            LatexEvent::Subsection(section) => self.write_subsection(section),
        }
    }

    fn write_document_class(&mut self) -> Result<(), std::io::Error> {
        write!(self.sink, "\\documentclass{{article}}\n")?;
        Ok(())
    }

    fn write_document_begin(&mut self) -> Result<(), std::io::Error> {
        write!(self.sink, "\\begin{{document}}\n")?;
        Ok(())
    }

    fn write_document_end(&mut self) -> Result<(), std::io::Error> {
        write!(self.sink, "\\end{{document}}\n")?;
        Ok(())
    }

    fn write_italic_text(&mut self, text: &str) -> Result<(), std::io::Error> {
        write!(self.sink, "\\textit{{{text}}}\n")?;
        Ok(())
    }

    fn write_text(&mut self, text: &str) -> Result<(), std::io::Error> {
        write!(self.sink, "{text}\n")?;
        Ok(())
    }

    fn write_section(&mut self, section: &str) -> Result<(), std::io::Error> {
        write!(self.sink, "\\section{{{section}}}\n")?;
        let label = section.replace(" ", "");
        write!(self.sink, "\\label{{sec:{label}}}\n")?;
        Ok(())
    }

    fn write_subsection(&mut self, section: &str) -> Result<(), std::io::Error> {
        write!(self.sink, "\\subsection{{{section}}}\n")?;
        let label = section.replace(" ", "");
        write!(self.sink, "\\label{{subsec:{label}}}\n")?;
        Ok(())
    }

    fn write_newline(&mut self) -> Result<(), std::io::Error> {
        write!(self.sink, "\\newline\n")?;
        Ok(())
    }
}

#[derive(PartialEq, Eq)]
pub enum LatexEvent<'a> {
    DocumentClasss,
    BeginDocument,
    EndDocument,
    Newline,
    Text(&'a str),
    ItalicText(&'a str),
    Section(&'a str),
    Subsection(&'a str),
}

pub struct Document<'a> {
    pub events: VecDeque<LatexEvent<'a>>,
}

impl<'a> Document<'a> {
    pub fn new() -> Self {
        let mut events = VecDeque::new();
        events.push_back(LatexEvent::DocumentClasss);
        events.push_back(LatexEvent::BeginDocument);
        Self { events }
    }

    pub fn push(&mut self, event: LatexEvent<'a>) {
        self.events.push_back(event)
    }
}
