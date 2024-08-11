#![allow(unused, dead_code)]

pub trait Field {
    fn parse(block: &Vec<String>) -> Self;
}
// Color impl

#[derive(Debug)]
pub struct Color {
    pub name: String,
    pub value: String
}

impl Color {
    pub fn new(name: &String, value: &String) -> Self {
        Self {
            name: name.to_owned(), 
            value: value.to_owned()
        }
    }
}

impl Field for Color {
    fn parse(block: &Vec<String>) -> Self {
        println!("Color block: {:?}", block);

        Self {
            name: "Hello".to_string(),
            value: "World".to_string()
        }
    }
}

// Font impl

#[derive(Debug)]
pub struct Font {
    pub name: String,
    pub value: String,
    pub source: Option<String>
}

impl Font {
    pub fn without_source(name: &String, value: &String) -> Self {
        Self {
            name: name.to_owned(),
            value: value.to_owned(),
            source: None
        }
    }

    pub fn with_source(name: &String, value: &String, source: &String) -> Self {
        Self {
            name: name.to_owned(),
            value: value.to_owned(),
            source: Some(source.to_owned())
        }
    }
}

impl Field for Font {
    fn parse(block: &Vec<String>) -> Self {
        println!("Fonts block: {:?}", block);

        Self {
            name: "Hello".to_string(),
            value: "World".to_string(),
            source: None
        }
    }
}

// Text style impl

#[derive(Debug)]
pub enum HtmlElementName {
    All,
    Div,
    H1,
    H2,
    Button,
    Form,
    P
}

#[derive(Debug)]
pub struct Text {
    pub name: String,
    pub styles: Vec<String>,
    pub for_element: Option<HtmlElementName>
}

impl Text {
    pub fn for_all(name: &String, styles: &Vec<String>) -> Self {
        Self {
            name: name.to_owned(),
            styles: styles.to_owned(),
            for_element: None
        }
    }

    pub fn for_element(name: &String, styles: &Vec<String>, for_element: HtmlElementName) -> Self {
        Self {
            name: name.to_owned(),
            styles: styles.to_owned(),
            for_element: Some(for_element)
        }
    }
}

impl Field for Text {
    fn parse(block: &Vec<String>) -> Self {
        println!("Color block: {:?}", block);

        Self {
            name: "Hello".to_string(),
            styles: vec!["hello".to_string(), "world".to_string()],
            for_element: None
        }
    }
}