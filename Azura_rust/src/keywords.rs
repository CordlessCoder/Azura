use std::{collections::HashMap, ops::Index};

use phf::phf_map;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Keyword {
    Class,
    Switch,
    Case,
}

pub static KEYWORDS: phf::Map<&'static str, Keyword> = phf_map! {
    "class" => Keyword::Class,
    "switch" => Keyword::Switch,
    "case" => Keyword::Case,
};

pub struct ScannerWithKeywords<'a, K> {
    scanner: Scanner<'a>,
    keywords: K,
}

use lending_iterator::prelude::*;

use crate::{
    error::ScannerError,
    scanner::{Scanner, TokenKind},
};
#[gat]
impl<'iter> LendingIterator for ScannerWithKeywords<'iter, &'static phf::Map<&'iter str, Keyword>> {
    type Item<'next>
    where
        Self: 'next,
    = Result<TokenKind<'next>, ScannerError<'next>>;

    fn next<'next>(
        self: &'next mut ScannerWithKeywords<'iter, &'static phf::Map<&'iter str, Keyword>>,
    ) -> Option<Result<TokenKind<'next>, ScannerError<'next>>> {
        match self.scanner.next() {
            Some(Ok(TokenKind::Ident(ident))) => Some(Ok(self
                .keywords
                .get(ident)
                .map_or(TokenKind::Ident(ident), |keyword| {
                    TokenKind::Keyword(*keyword)
                }))),
            other => other,
        }
    }
}

#[gat]
impl<'iter> LendingIterator for ScannerWithKeywords<'iter, &'iter HashMap<&'iter str, Keyword>> {
    type Item<'next>
    where
        Self: 'next,
    = Result<TokenKind<'next>, ScannerError<'next>>;

    fn next<'next>(
        self: &'next mut ScannerWithKeywords<'iter, &'iter HashMap<&'iter str, Keyword>>,
    ) -> Option<Result<TokenKind<'next>, ScannerError<'next>>> {
        match self.scanner.next() {
            Some(Ok(TokenKind::Ident(ident))) => Some(Ok(self
                .keywords
                .get(ident)
                .map_or(TokenKind::Ident(ident), |keyword| {
                    TokenKind::Keyword(*keyword)
                }))),
            other => other,
        }
    }
}

impl<'a> Scanner<'a> {
    pub fn with_keywords<T>(self: Scanner<'a>, keywords: T) -> ScannerWithKeywords<'a, T> {
        ScannerWithKeywords {
            scanner: self,
            keywords,
        }
    }
}
