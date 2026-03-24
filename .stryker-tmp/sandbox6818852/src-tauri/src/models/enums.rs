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
    Poetry,
}

impl Genre {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Nonfiction => "nonfiction",
            Self::SelfHelp => "self_help",
            Self::Technical => "technical",
            Self::Academic => "academic",
            Self::Fiction => "fiction",
            Self::Romance => "romance",
            Self::Business => "business",
            Self::Management => "management",
            Self::Children => "children",
            Self::Ya => "ya",
            Self::Poetry => "poetry",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "nonfiction" => Some(Self::Nonfiction),
            "self_help" => Some(Self::SelfHelp),
            "technical" => Some(Self::Technical),
            "academic" => Some(Self::Academic),
            "fiction" => Some(Self::Fiction),
            "romance" => Some(Self::Romance),
            "business" => Some(Self::Business),
            "management" => Some(Self::Management),
            "children" => Some(Self::Children),
            "ya" => Some(Self::Ya),
            "poetry" => Some(Self::Poetry),
            _ => None,
        }
    }
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
    Error,
}

impl IllustrationState {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Imported => "imported",
            Self::Linked => "linked",
            Self::Error => "error",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "pending" => Some(Self::Pending),
            "imported" => Some(Self::Imported),
            "linked" => Some(Self::Linked),
            "error" => Some(Self::Error),
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

impl BookLanguage {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::PtBr => "pt-BR",
            Self::EnUs => "en-US",
            Self::ItIt => "it-IT",
            Self::EsEs => "es-ES",
        }
    }
}

impl ChapterStartPage {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Odd => "odd",
            Self::Even => "even",
            Self::Continuous => "continuous",
        }
    }
}

impl DropCapStyle {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::FirstLetter => "first_letter",
            Self::FirstWordSmallCaps => "first_word_small_caps",
        }
    }
}

impl OrnamentStyle {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Line => "line",
            Self::Vignette => "vignette",
            Self::Asterisks => "asterisks",
        }
    }
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

// ── ST001 TASK-4 — Front/Back Matter types ────────────────────────────────

/// Type of front-matter section, detected from filename.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FrontMatterType {
    HalfTitle,      // 00-ante-rosto.md
    TitlePage,      // 01-frontispicio.md
    CopyrightPage,  // 02-creditos.md
    Dedication,     // 03-dedicatoria.md
    Epigraph,       // 04-epigrafe.md
    Toc,            // toc.md (manual TOC, skips auto-generation)
    Foreword,       // prefacio.md
    Preface,        // introducao.md
    Acknowledgments, // agradecimentos.md
    Unknown,        // Unrecognized filename
}

impl FrontMatterType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::HalfTitle => "half_title",
            Self::TitlePage => "title_page",
            Self::CopyrightPage => "copyright_page",
            Self::Dedication => "dedication",
            Self::Epigraph => "epigraph",
            Self::Toc => "toc",
            Self::Foreword => "foreword",
            Self::Preface => "preface",
            Self::Acknowledgments => "acknowledgments",
            Self::Unknown => "unknown",
        }
    }
}

/// Type of back-matter section, detected from filename.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BackMatterType {
    Appendix,     // apendice.md / appendix.md
    References,   // referencias.md / references.md
    Bibliography, // bibliografia.md
    Glossary,     // glossario.md
    Index,        // indice.md (remissive index)
    AboutAuthor,  // sobre-o-autor.md / about.md
    Colophon,     // colofao.md
    Unknown,      // Unrecognized filename
}

impl BackMatterType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Appendix => "appendix",
            Self::References => "references",
            Self::Bibliography => "bibliography",
            Self::Glossary => "glossary",
            Self::Index => "index",
            Self::AboutAuthor => "about_author",
            Self::Colophon => "colophon",
            Self::Unknown => "unknown",
        }
    }
}
