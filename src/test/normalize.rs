/*
 * test/normalize.rs
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

use crate::normalize::{is_normal, normalize};

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
