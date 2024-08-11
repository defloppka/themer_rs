use std::path::PathBuf;

use super::{types::{Color, Field, Font, Text}, utils::read_file_lines};

#[derive(Debug)]
pub struct ThemerProgram {
    pub path: PathBuf,
    pub colors: Vec<Color>,
    pub fonts: Vec<Font>,
    pub text_styles: Vec<Text>
}

impl ThemerProgram {
    pub fn new(path: &PathBuf) -> Self {
        Self {
            path: path.to_owned(),
            colors: Vec::with_capacity(0),
            fonts: Vec::with_capacity(0),
            text_styles: Vec::with_capacity(0)
        }
    }

    pub fn parse_file(&mut self) {
        let content = read_file_lines(&self.path);

        // (0, 0, 0) -> (open block pos, close block pos, count)
        let mut colors_block = (0, 0, 0);
        let mut fonts_block = (0, 0, 0);
        let mut text_styles_block = (0, 0, 0);

        for (i, line) in content.iter().enumerate() {
            match line.as_str() {
                "<colors>" => {
                    colors_block.0 = i;
                    colors_block.2 += 1;
                },
                "</colors>" => colors_block.1 = i,

                "<fonts>" => {
                    fonts_block.0 = i;
                    fonts_block.2 += 1;
                },
                "</fonts>" => fonts_block.1 = i,

                "<texts>" => {
                    text_styles_block.0 = i;
                    text_styles_block.2 += 1;
                },
                "</texts>" => text_styles_block.1 = i,
                _ => ()
            };
        }

        if colors_block.2 != 0 {
            self.colors = Vec::with_capacity(colors_block.2 + 1);
            let colors_block = &content[colors_block.0 + 1..colors_block.1];
            
            for color_raw in colors_block {
                let color = Color::parse(color_raw);
                self.colors.push(color);
            }
        }
        if fonts_block.2 != 0 {
            self.fonts = Vec::with_capacity(fonts_block.2 + 1);
            let fonts_block = &content[fonts_block.0 + 1..fonts_block.1];
            
            for font_raw in fonts_block {
                let font = Font::parse(font_raw);
                self.fonts.push(font);
            }
        }

        if text_styles_block.2 != 0 {
            self.text_styles = Vec::with_capacity(text_styles_block.2 + 1);
            let text_styles_block = &content[text_styles_block.0 + 1..text_styles_block.1];
            
            for text_style_raw in text_styles_block {
                let text_style = Text::parse(text_style_raw);
                self.text_styles.push(text_style);
            }
        }
    }
}

#[allow(unused)]
pub fn create_program(path: &PathBuf) -> ThemerProgram {
    let mut program = ThemerProgram::new(path);
    program.parse_file();
    program
}