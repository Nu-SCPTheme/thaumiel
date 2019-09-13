/*
 * normalize.rs
 *
 * kant-router - Wikidot-compatible router for web applications
 * Copyright (C) 2019 Ammon Smith
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

use crate::StdResult;
use regex::Regex;
use std::borrow::Cow;
use std::mem;
use std::str::Utf8Error;

lazy_static! {
    static ref NON_URL: Regex = Regex::new(r"([^\w/\-]+|-{2,})").unwrap();
    static ref START_DASHES: Regex = Regex::new(r"(^|/+)(?P<dash>-+)").unwrap();
    static ref END_DASHES: Regex = Regex::new(r"(?P<dash>-+)($|/+)").unwrap();
}

#[inline]
fn percent_decode(input: &str) -> StdResult<Cow<str>, Utf8Error> {
    use percent_encoding::percent_decode_str;

    percent_decode_str(input).decode_utf8()
}

/// Converts an arbitrary string into Wikidot normalized form.
///
/// This will convert non-alphanumeric characters to dashes and
/// makes it lowercase.
///
/// Examples:
/// * `Big Cheese Horace` -> `big-cheese-horace`
/// * `bottom--Text` -> `bottom-text`
/// * `Tufto's Proposal` -> `tufto-s-proposal`
/// * `-test-` -> `test`
pub fn normalize(name: &mut String) {
    // Perform percent-decoding, if needed
    match percent_decode(&name) {
        Ok(Cow::Borrowed(_)) => (),
        Ok(Cow::Owned(mut decoded)) => mem::swap(name, &mut decoded),
        Err(_) => warn!("Error decoding percent string"),
    }

    // Lowercase
    name.make_ascii_lowercase();

    // Convert non-URL characters to dashes
    while let Some(mtch) = NON_URL.find(name) {
        let start = mtch.start();
        let end = mtch.end();
        name.replace_range(start..end, "-");
    }

    // Remove leading and trailing dashes
    let get_range = |captures: regex::Captures| {
        let mtch = captures.name("dash").unwrap();
        let start = mtch.start();
        let end = mtch.end();

        start..end
    };

    while let Some(captures) = START_DASHES.captures(name) {
        let range = get_range(captures);
        name.replace_range(range, "");
    }

    while let Some(captures) = END_DASHES.captures(name) {
        let range = get_range(captures);
        name.replace_range(range, "");
    }
}

/// Determines if an arbitrary string is already in Wikidot normalized form.
pub fn is_normal(name: &str) -> bool {
    // Is all lowercase
    fn is_valid_char(ch: char) -> bool {
        ch.is_ascii_lowercase() || ch.is_digit(10) || ch == ':' || ch == '-' || ch == '_' || ch == '/'
    }

    if !name.chars().all(is_valid_char) {
        return false;
    }

    // No special characters
    if let Some(_) = NON_URL.find(name) {
        return false;
    }

    // Has leading or trailing dashes
    if let Some(_) = START_DASHES.find(name) {
        return false;
    }

    if let Some(_) = END_DASHES.find(name) {
        return false;
    }

    true
}

#[test]
fn test_normalize() {
    macro_rules! check {
        ($input:expr, $expected:expr) => {{
            let mut text = str!($input);
            normalize(&mut text);
            assert_eq!(text, $expected, "Normalized text doesn't match expected");
        }};
    }

    check!("", "");
    check!("Big Cheese Horace", "big-cheese-horace");
    check!("bottom--Text", "bottom-text");
    check!("Tufto's Proposal", "tufto-s-proposal");
    check!("-test-", "test");
    check!("End of Death Hub", "end-of-death-hub");
    check!("$100 is a lot of money", "100-is-a-lot-of-money");
    check!("snake_case", "snake_case");
    check!("long__snake__case", "long__snake__case");
    check!(" <[ TEST ]> ", "test");
    check!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!", "");

    check!("/", "/");
    check!("/scp-1000/", "/scp-1000/");
    check!("/SCP 4447/ofFSEt/2", "/scp-4447/offset/2");
    check!("page/discuss", "page/discuss");
    check!("/-test-/", "/test/");
    check!("/Tufto's Proposal---", "/tufto-s-proposal");
    check!("page/-", "page/");
    check!("/ page /-yeah-/ thing ", "/page/yeah/thing");

    check!("/SCP%20xxxx", "/scp-xxxx");
    check!("/scp%20xxxx/", "/scp-xxxx/");
    check!("%20scp%20%20xxxx", "scp-xxxx");
}

#[test]
fn test_is_normal() {
    macro_rules! check {
        ($expected:expr, $input:expr) => {{
            assert_eq!(
                is_normal($input),
                $expected,
                "Normalization test failed: {}",
                $input,
            );
        }};
    }

    check!(true, "");
    check!(true, "big-cheese-horace");
    check!(false, "Big Cheese Horace");
    check!(true, "bottom-text");
    check!(false, "bottom-Text");
    check!(false, "-test-");
    check!(true, "scp-1000");
    check!(true, "end-of-death-hub");
    check!(false, "End of Death Hub");
    check!(false, "$200 please");
    check!(true, "snake_case");
    check!(true, "kebab-case");
    check!(false, "<[ TEST ]>");
    check!(false, " <[ TEST ]> ");
    check!(false, "!!!!!!!!!!!!");

    check!(true, "/");
    check!(true, "/scp-1000/");
    check!(false, "/SCP-1000/");
    check!(true, "/scp-4447/offset/2");
    check!(false, "/SCP 4447/ofFSEt/2");
    check!(true, "page/discuss");
    check!(false, "/-test-/");
    check!(true, "/test/");
    check!(false, "/Tufto's Proposal---");
    check!(false, "/ page /-yeah-/ thing");
    check!(false, "/ page /-yeah-/ ");
    check!(false, "/ page /-yeah-");
    check!(false, "/ page /-");
    check!(false, "/ page");

    check!(false, "/scp xxxx");
    check!(false, "/scp%20xxxx");
    check!(false, "/SCP%20xxxx");
    check!(true, "/scp-xxxx");
}
