use std::ops::Range;
use mongodb::bson::{Document};
use serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Style {
    pub(crate) range: Option<Range<usize>>,
    pub(crate) bold: Option<bool>,
    pub(crate) italic: Option<bool>,
    pub(crate) underline: Option<bool>,
    pub(crate) strike: Option<bool>,
    pub(crate) background_color: Option<Color>,
    pub(crate) text_color: Option<Color>,
    pub(crate) text_alignment: Option<TextAlignment>,
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StyleBuilder {
    pub(crate) range: Option<Range<usize>>,
    pub(crate) bold: Option<bool>,
    pub(crate) italic: Option<bool>,
    pub(crate) underline: Option<bool>,
    pub(crate) strike: Option<bool>,
    pub(crate) background_color: Option<Color>,
    pub(crate) text_color: Option<Color>,
    pub(crate) text_alignment: Option<TextAlignment>,
}

impl StyleBuilder{
    pub fn new() -> StyleBuilder{
        StyleBuilder{
            range: None,
            bold: None,
            italic: None,
            underline: None,
            strike: None,
            background_color: None,
            text_color: None,
            text_alignment: None,
        }
    }

    pub fn range(mut self, range: Option<Range<usize>>) -> StyleBuilder{
        self.range = range;
        self
    }

    pub fn bold(mut self, bold: Option<bool>) -> StyleBuilder{
        self.bold = bold;
        self
    }

    pub fn italic(mut self, italic: Option<bool>) -> StyleBuilder{
        self.italic = italic;
        self
    }

    pub fn underline(mut self, underline: Option<bool>) -> StyleBuilder{
        self.underline = underline;
        self
    }
    pub fn strike(mut self, strike: Option<bool>) -> StyleBuilder{
        self.strike = strike;
        self
    }
    pub fn background_color(mut self, background_color: Option<Color>) -> StyleBuilder{
        self.background_color = background_color;
        self
    }

    pub fn text_color(mut self, text_color: Option<Color>) -> StyleBuilder{
        self.background_color = text_color;
        self
    }

    pub fn text_alignment(mut self, alignment: Option<TextAlignment>) -> StyleBuilder{
        self.text_alignment = alignment;
        self
    }

    pub fn build(self) -> Style{
        Style{
            range: self.range,
            bold: self.bold,
            italic: self.italic,
            underline: self.underline,
            strike: self.strike,
            background_color: self.background_color,
            text_color: self.text_color,
            text_alignment: self.text_alignment,
        }
    }
}

pub trait ToStyle{
    fn to_style(&self) -> Style;
}

impl ToStyle for Style {
    fn to_style(&self) -> Style {
        Style {
            range: self.range.to_owned(),
            bold: self.bold,
            italic: self.italic,
            underline: self.underline,
            strike: self.strike,
            text_color: self.text_color.to_owned(),
            background_color: self.background_color.to_owned(),
            text_alignment: self.text_alignment.to_owned()
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Color(pub String);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TextDecorationType{
    Bold, Italic, Underline, Strike
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TextAlignment(pub usize);

impl From<&Document> for Style {
    fn from(doc: &Document) -> Self {
        let bold = doc.get_bool("bold").unwrap();
        let italic = doc.get_bool("italic").unwrap();
        let underline = doc.get_bool("underline").unwrap();
        let strike = doc.get_bool("strike").unwrap();

        Style {
            range: None,
            bold: Option::from(bold.to_owned()),
            italic: Option::from(italic.to_owned()),
            underline: Option::from(underline.to_owned()),
            strike: Option::from(strike.to_owned()),
            background_color: None,
            text_color: None,
            text_alignment: None,
        }
    }
}