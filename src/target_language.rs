use serde::{Serialize, Deserialize};
use std::fmt;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TargetLanguage {
    BG, //Bulgaria
    CS, //Czech
    DA, //Danish
    DE, //German
    EL, //Greek
    EN, //English
    ES, //Spanish
    ET, //Estonian
    FI, //Finnish
    FR, //French
    HU, //Hungaria
    IT, //Italian
    JA, //Japanese
    LT, //Lithuani
    LV, //Latvian
    NL, //Dutch
    PL, //Polish
    PT, //Portugue
    RO, //Romanian
    RU, //Russian
    SK, //Slovak
    SL, //Slovenia
    SV, //Swedish
    ZH, //Chinese
    Undefined,
}

impl fmt::Display for TargetLanguage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TargetLanguage::BG => write!(f, "BG"),
            TargetLanguage::CS => write!(f, "CS"),
            TargetLanguage::DA => write!(f, "DA"),
            TargetLanguage::DE => write!(f, "DE"),
            TargetLanguage::EL => write!(f, "EL"),
            TargetLanguage::EN => write!(f, "EN"),
            TargetLanguage::ES => write!(f, "ES"),
            TargetLanguage::ET => write!(f, "ET"),
            TargetLanguage::FI => write!(f, "FI"),
            TargetLanguage::FR => write!(f, "FR"),
            TargetLanguage::HU => write!(f, "HU"),
            TargetLanguage::IT => write!(f, "IT"),
            TargetLanguage::JA => write!(f, "JA"),
            TargetLanguage::LT => write!(f, "LT"),
            TargetLanguage::LV => write!(f, "LV"),
            TargetLanguage::NL => write!(f, "NL"),
            TargetLanguage::PL => write!(f, "PL"),
            TargetLanguage::PT => write!(f, "PT"),
            TargetLanguage::RO => write!(f, "RO"),
            TargetLanguage::RU => write!(f, "RU"),
            TargetLanguage::SK => write!(f, "SK"),
            TargetLanguage::SL => write!(f, "SL"),
            TargetLanguage::SV => write!(f, "SV"),
            TargetLanguage::ZH => write!(f, "ZH"),
            TargetLanguage::Undefined => write!(f, "undefined"),
        }
    }
}