/**
   Copyright 2015 W. Max Lees

   This file is part of Jarvis OS.

   Jarvis OS is free software: you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation, either version 3 of the License, or
   (at your option) any later version.

   Jarvis OS is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with Jarvis OS.  If not, see <http://www.gnu.org/licenses/>.
*/

extern crate nalgebra as na;

const N: usize = 86;
const M: usize = 1100000;

// N = 86       Number of tags
// M = 1.1M     Number of words in English language
pub enum Tag {
    SentenceCloser,
    LeftParen,
    RightParen,
    Not,
    Dash,
    Comma,
    Colon,
    PreQualifier,
    PreQuantifierN,
    PreQuantifierX,
    PostDeterminer,
    Article,
    Be,
    Were,
    Was,
    Being,
    Am,
    Been,
    Are,
    Is,
    CoordinatingConjunction,
    CardinalNumeral,
    SubordinatingConjunction,
    Do,
    Did,
    Does,
    SingularDeterminer,
    DeterminerQualifier,
    PluralDeterminer,
    DoubleConjunction,
    Existential,
    ForeignWord,
    WordInHeadline,
    Have,
    HadPast,
    Having,
    HadPastParticiple,
    Has,
    Preposition,
    Adjective,
    ComparativeAdjective,
    SemSuperlativeAdjective,
    MorphSuperlativeAdjective,
    ModalAuxiliary,
    Cited,
    Noun,
    PossessiveSingularNoun,
    PluralNoun,
    PossessivePluralNoun,
    ProperNoun,
    PossessiveProperNoun,
    PluralProperNoun,
    PossessivePluralProperNoun,
    AdverbialNoun,
    PluralAdverbialNoun,
    OrdinalNumeral,
    NominalPronoun,
    PossessiveNominalPronoun,
    PossessivePersonalPronoun,
    SecondPossessivePronoun,
    SingularReflexivePersonalPronoun,
    PluralReflexivePersonalPronoun,
    ObjectivePersonalPronoun,
    SingularNominativePronoun,
    OtherNominativePersonalPronoun,
    Qualifier,
    PostQualifier,
    Adverb,
    ComparativeAdverb,
    SuperlativeAdverb,
    NominalAdverb,
    AdverbParticle,
    WordInTitle,
    InfinitiveMarker,
    Interjection,
    VerbBaseForm,
    VerbPastTense,
    VerbPresentParticiple,
    VerbPastPariciple,
    VerbThirdSingularPresent,
    DeterminerWH,
    PossessiveWHPronoun,
    ObjectiveWHPronoun,
    NominativeWHPronoun,
    QualifierWH,
    AdverbWH,
    None,
}

pub struct TagPair<'a> {
    word: &'a str,
    tag: Tag,
}

/*
A function for parsing a word in a tagged
corpus. These come in the form "word/tag" 
*/
fn parse_word<'a>(input: &'a str) -> TagPair<'a> {
    // Split the input string
    let mut parts = input.split("/");

    // Get the word
    let new_word = parts.next().unwrap();

    let new_tag = match parts.next().unwrap() {
        "." => Tag::SentenceCloser,
        "(" => Tag::LeftParen,
        ")" => Tag::RightParen,
        "*" => Tag::Not,
        "--" => Tag::Dash,
        "," => Tag::Comma,
        ":" => Tag::Colon,
        "abl" => Tag::PreQualifier,
        "abn" => Tag::PreQuantifierN,
        "abx" => Tag::PreQuantifierX,
        "ap" => Tag::PostDeterminer,
        "at" => Tag::Article,
        "be" => Tag::Be,
        "bed" => Tag::Were,
        "bedz" => Tag::Was,
        "beg" => Tag::Being,
        "bem" => Tag::Am,
        "ben" => Tag::Been,
        "ber" => Tag::Are,
        "bez" => Tag::Is,
        "cc" => Tag::CoordinatingConjunction,
        "cd" => Tag::CardinalNumeral,
        "cs" => Tag::SubordinatingConjunction,
        "do" => Tag::Do,
        "dod" => Tag::Did,
        "doz" => Tag::Does,
        "dt" => Tag::SingularDeterminer,
        "dti" => Tag::DeterminerQualifier,
        "dts" => Tag::PluralDeterminer,
        "dtx" => Tag::DoubleConjunction,
        "ex" => Tag::Existential,
        "fw" => Tag::ForeignWord,
        "hl" => Tag::WordInHeadline,
        "hv" => Tag::Have,
        "hvd" => Tag::HadPast,
        "hvg" => Tag::Having,
        "hvn" => Tag::HadPastParticiple,
        "hvz" => Tag::Has,
        "in" => Tag::Preposition,
        "jj" => Tag::Adjective,
        "jjr" => Tag::ComparativeAdjective,
        "jjs" => Tag::SemSuperlativeAdjective,
        "jjt" => Tag::MorphSuperlativeAdjective,
        "md" => Tag::ModalAuxiliary,
        "nc" => Tag::Cited,
        "nn" => Tag::Noun,
        "nn$" => Tag::PossessiveSingularNoun,
        "nns" => Tag::PluralNoun,
        "nns$" => Tag::PossessivePluralProperNoun,
        "np" => Tag::ProperNoun,
        "np$" => Tag::PossessiveProperNoun,
        "nps" => Tag::PluralProperNoun,
        "nps$" => Tag::PossessivePluralProperNoun,
        "nr" => Tag::AdverbialNoun,
        "nrs" => Tag::PluralAdverbialNoun,
        "od" => Tag::OrdinalNumeral,
        "pn" => Tag::NominalPronoun,
        "pn$" => Tag::PossessiveNominalPronoun,
        "pp$" => Tag::PossessivePersonalPronoun,
        "pp$$" => Tag::SecondPossessivePronoun,
        "ppl" => Tag::SingularReflexivePersonalPronoun,
        "ppls" => Tag::PluralReflexivePersonalPronoun,
        "ppo" => Tag::ObjectivePersonalPronoun,
        "pps" => Tag::SingularNominativePronoun,
        "ppss" => Tag::OtherNominativePersonalPronoun,
        "ql" => Tag::Qualifier,
        "qlp" => Tag::PostQualifier,
        "rb" => Tag::Adverb,
        "rbr" => Tag::ComparativeAdverb,
        "rbt" => Tag::SuperlativeAdverb,
        "rn" => Tag::NominalAdverb,
        "rp" => Tag::AdverbParticle,
        "tl" => Tag::WordInTitle,
        "to" => Tag::InfinitiveMarker,
        "uh" => Tag::Interjection,
        "vb" => Tag::VerbBaseForm,
        "vbd" => Tag::VerbPastTense,
        "vbg" => Tag::VerbPresentParticiple,
        "vbn" => Tag::VerbPastPariciple,
        "vbz" => Tag::VerbThirdSingularPresent,
        "wdt" => Tag::DeterminerWH,
        "wp$" => Tag::PossessiveWHPronoun,
        "wpo" => Tag::ObjectiveWHPronoun,
        "wps" => Tag::NominativeWHPronoun,
        "wql" => Tag::QualifierWH,
        "wrb" => Tag::AdverbWH,
        _ => Tag::None,
    };

    TagPair{word: new_word, tag: new_tag}
}

pub struct PoSTagger {
    pi: na::DVec<f32>,
    A: na::DMat<f32>,
    B: na::DMat<f32>,
}

impl PoSTagger {
    pub fn create_new() -> PoSTagger {
        let maxIter: u16 = 10000;
        let mut iters: u16 = 0;

        let mut newTagger = PoSTagger{
            pi: na::DVec::<f32>::from_elem(N, (1 as f32/N as f32)),
            A: na::DMat::<f32>::from_elem(N, N, (1 as f32/N as f32)),
            B: na::DMat::<f32>::from_elem(N, M, (1 as f32/M as f32)),
        };

        newTagger
    }

    pub fn load() {

    }
}
