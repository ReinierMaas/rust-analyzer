use crate::SyntaxKind::{self, *};

use crate::lexer::ptr::Ptr;

pub(crate) fn is_string_literal_start(c: char, c1: Option<char>, c2: Option<char>) -> bool {
    match (c, c1, c2) {
        ('r', Some('"'), _)
        | ('r', Some('#'), _)
        | ('b', Some('"'), _)
        | ('b', Some('\''), _)
        | ('b', Some('r'), Some('"'))
        | ('b', Some('r'), Some('#')) => true,
        _ => false,
    }
}

pub(crate) fn scan_char(ptr: &mut Ptr) {
    while let Some(c) = ptr.current() {
        match c {
            '\\' => {
                ptr.bump();
                if ptr.at('\\') || ptr.at('\'') {
                    ptr.bump();
                }
            }
            '\'' => {
                ptr.bump();
                return;
            }
            '\n' => return,
            _ => {
                ptr.bump();
            }
        }
    }
}

pub(crate) fn scan_byte_char_or_string(ptr: &mut Ptr) -> SyntaxKind {
    // unwrapping and not-exhaustive match are ok
    // because of string_literal_start
    let c = ptr.bump().unwrap();
    match c {
        '\'' => {
            scan_byte(ptr);
            BYTE
        }
        '"' => {
            scan_byte_string(ptr);
            BYTE_STRING
        }
        'r' => {
            scan_raw_byte_string(ptr);
            RAW_BYTE_STRING
        }
        _ => unreachable!(),
    }
}

pub(crate) fn scan_string(ptr: &mut Ptr) {
    while let Some(c) = ptr.current() {
        match c {
            '\\' => {
                ptr.bump();
                if ptr.at('\\') || ptr.at('"') {
                    ptr.bump();
                }
            }
            '"' => {
                ptr.bump();
                return;
            }
            _ => {
                ptr.bump();
            },
        }
    }
}

pub(crate) fn scan_raw_string(ptr: &mut Ptr) {
    let mut hashes = 0;
    while ptr.at('#') {
        hashes += 1;
        ptr.bump();
    }
    if !ptr.at('"') {
        return;
    }
    ptr.bump();

    while let Some(c) = ptr.bump() {
        if c == '"' {
            let mut hashes_left = hashes;
            while ptr.at('#') && hashes_left > 0{
                hashes_left -= 1;
                ptr.bump();
            }
            if hashes_left == 0 {
                return;
            }
        }
    }
}

fn scan_byte(ptr: &mut Ptr) {
    scan_char(ptr)
}

fn scan_byte_string(ptr: &mut Ptr) {
    scan_string(ptr)
}

fn scan_raw_byte_string(ptr: &mut Ptr) {
    if !ptr.at('"') {
        return;
    }
    ptr.bump();

    while let Some(c) = ptr.bump() {
        if c == '"' {
            return;
        }
    }
}
