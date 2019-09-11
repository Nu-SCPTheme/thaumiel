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

use regex::Regex;

lazy_static! {
    static ref NON_URL: Regex = Regex::new(r"([^\w-]+|-{2,})").unwrap();
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
    // Lowercase
    name.make_ascii_lowercase();

    // Convert non-URL characters to dashes
    while let Some(mtch) = NON_URL.find(name) {
        let start = mtch.start();
        let end = mtch.end();
        name.replace_range(start..end, "-");
    }

    // Remove leading and trailing dashes
    while name.starts_with("-") {
        name.remove(0);
    }

    while name.ends_with("-") {
        name.pop();
    }
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
    check!(" <[ TEST ]> ", "test");
    check!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!", "");
}
