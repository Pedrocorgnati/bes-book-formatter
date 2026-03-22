use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Genre {
    Nonfiction,
    SelfHelp,
    Technical,
    Academic,
    Fiction,
    Romance,
    Business,
    Management,
    Children,
    Ya,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum OutputFormat {
    Epub3,
    PdfEbook,
    PdfPrint,
    Docx,
    Html5,
    MarkdownClean,
    Txt,
    JsonStructural,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Platform {
    Kdp,
    KdpPrint,
    IngramSpark,
    AppleBooks,
    Kobo,
    Draft2digital,
    Generic,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IllustrationState {
    Pending,
    Imported,
    Linked,
}

impl IllustrationState {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Imported => "imported",
            Self::Linked => "linked",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "pending" => Some(Self::Pending),
            "imported" => Some(Self::Imported),
            "linked" => Some(Self::Linked),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PageFormat {
    Trade6x9,
    Digest5_5x8_5,
    Pocket4_25x6_87,
    A5,
    A4,
    Letter,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BookLanguage {
    #[serde(rename = "pt-BR")]
    PtBr,
    #[serde(rename = "en-US")]
    EnUs,
    #[serde(rename = "it-IT")]
    ItIt,
    #[serde(rename = "es-ES")]
    EsEs,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UILanguage {
    #[serde(rename = "pt-BR")]
    PtBr,
    #[serde(rename = "en-US")]
    EnUs,
    #[serde(rename = "es-ES")]
    EsEs,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaperColor {
    White70lb,
    Cream60lb,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ChapterStartPage {
    Odd,
    Even,
    Continuous,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DropCapStyle {
    None,
    FirstLetter,
    FirstWordSmallCaps,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum OrnamentStyle {
    None,
    Line,
    Vignette,
    Asterisks,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PdfxProfile {
    PdfX1a,
    PdfX4,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BookConfigVersion {
    V1,
    V2,
    V3,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ManuscriptCompleteness {
    Blocking,
    Warning,
    Normal,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ColorSpace {
    Srgb,
    Cmyk,
}
