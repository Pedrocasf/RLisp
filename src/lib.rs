#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), no_main)]

mod lisp;
use lisp::*;
#[cfg(feature = "std")]
mod tests {
    use crate::lisp::*;

    #[test]
    pub fn atom_parser_int_test() {
        let mut objects = [None;128];
        let actual = parse_atom("1", &mut objects).unwrap();
        let expected = Atom::Num(1);
        assert_eq!(actual, expected);
    }
    #[test]
    pub fn atom_parser_str_test() {
        let mut objects = [None;128];
        let actual = parse_atom("a", &mut objects).unwrap();
        let expected = Atom::Str("a");
        assert_eq!(actual, expected);
    }
    #[test]
    pub fn atom_parser_nil_test() {
        let mut objects = [None;128];
        let actual = parse_atom("", &mut objects).err();
        let expected = Some("the atom is empty");
        assert_eq!(actual, expected);
    }
    #[test]
    pub fn list_parser_nil_test() {
        let mut objects = [None;128];
        let actual = parse_list("", &mut objects).err();
        let expected = Some("the list is empty");
        assert_eq!(actual, expected);
    }
    #[test]
    pub fn list_parser_open_bracket_test() {
        let mut objects = [None;128];
        let actual = parse_list("12", &mut objects).err();
        let expected = Some("the list open delimiter is not set");
        assert_eq!(actual, expected);
    }
    #[test]
    pub fn list_parser_empty_brackets_test() {
        let mut objects = [None;128];
        let actual = parse_list("()", &mut objects).ok();
        println!("{:#?}", objects);
    }
    #[test]
    pub fn list_parser_one_element_test() {
        let mut objects = [None;128];
        let actual = parse_list("(1)", &mut objects).ok();
        println!("{:#?}", objects);
    }
}