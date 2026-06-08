//! Unit tests for the launch-name parameter parser.

use super::Params;
use crate::error::AppError;

fn parse(name: &str) -> Result<Params, AppError> {
    let mut params = Params::default();
    params.parse(name)?;
    Ok(params)
}

#[test]
fn drops_base_name_and_reads_a_color() {
    let p = parse("LoneColor red").unwrap();
    assert!(p.color.is_some() && !p.silent && !p.choose);
}

#[test]
fn switches_are_case_insensitive_and_independent() {
    let p = parse("LoneColor S red").unwrap();
    assert!(p.silent); // a later color must not clobber the silent flag
    assert!(p.color.is_some());
}

#[test]
fn choose_switch_is_recognized() {
    assert!(parse("LoneColor c").unwrap().choose);
}

#[test]
fn two_colors_is_an_error() {
    assert!(matches!(
        parse("LoneColor red blue"),
        Err(AppError::DuplicateColor(..))
    ));
}

#[test]
fn unknown_word_is_an_error() {
    assert!(matches!(
        parse("LoneColor wat"),
        Err(AppError::InvalidParameter(_))
    ));
}

#[test]
fn bare_name_yields_no_parameters() {
    let p = parse("LoneColor").unwrap();
    assert!(p.color.is_none() && !p.silent && !p.choose);
}
