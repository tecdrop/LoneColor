//! Parses the launching file/shortcut name into the run parameters.

use crate::color::Rgb;
use crate::error::AppError;

const SILENT_LONG: &str = "silent";
const SILENT_SHORT: &str = "s";
const CHOOSE_LONG: &str = "choose";
const CHOOSE_SHORT: &str = "c";

/// The parameters extracted from the launch name.
#[derive(Default)]
pub struct Params {
    /// Suppresses the error sound.
    pub silent: bool,
    /// Opens the desktop-background control panel instead of setting a color.
    pub choose: bool,
    /// The explicit color, if one was given in the name.
    pub color: Option<Rgb>,
    /// The original text of the color token, for the duplicate-color error.
    color_token: Option<String>,
}

impl Params {
    /// Classifies each word of the name (the first word, the base app name, is ignored).
    pub fn parse(&mut self, name: &str) -> Result<(), AppError> {
        for word in name.split(' ').skip(1) {
            let word = word.trim();
            if word.is_empty() {
                continue;
            }

            if is_switch(word, SILENT_LONG, SILENT_SHORT) {
                self.silent = true;
            } else if is_switch(word, CHOOSE_LONG, CHOOSE_SHORT) {
                self.choose = true;
            } else if let Some(color) = Rgb::parse(word) {
                if let Some(first) = &self.color_token {
                    return Err(AppError::DuplicateColor(first.clone(), word.to_string()));
                }
                self.color = Some(color);
                self.color_token = Some(word.to_string());
            } else {
                return Err(AppError::InvalidParameter(word.to_string()));
            }
        }
        Ok(())
    }
}

fn is_switch(word: &str, long: &str, short: &str) -> bool {
    word.eq_ignore_ascii_case(long) || word.eq_ignore_ascii_case(short)
}

#[cfg(test)]
mod tests;
