pub mod book_config;
pub mod cover;
pub mod enums;
pub mod generation;
pub mod illustration;
pub mod manuscript;
pub mod preference;
pub mod project;
pub mod responses;
pub mod typography;

pub use book_config::{genre_defaults, PageDimensions, TypographyDefaults};
pub use cover::{CoverConfig, CoverConfigInput, CoverTemplate, CoverGenerationResult, SpineWidthResult};
pub use enums::{BackMatterType, FrontMatterType};
pub use illustration::{Illustration, NewIllustration};
pub use manuscript::{Footnote, IllustrationRef, IndexEntry, ParseError, ParsedChapter, ParsedManuscript};
pub use generation::{GenOptions, StoredGenerationResult};
pub use preference::Preference;
pub use project::{NewProject, Project, UpdateProject};
pub use responses::{
    ApiResponse, SidecarStatus, InitResult, CompletenessResult, ChecklistResult, ChecklistItem,
    ManuscriptAst, ChapterNode, SectionNode, IllustrationPlaceholder, ManuscriptMetadata,
    GenerationResult, ValidationResult, PreflightResult,
    PreviewResult, LayoutIssue,
    PreviewPageResponse, PageImage,
    Annotation, TypoIssuePreview,
    StructureReport, PageDimensionsResponse, TypographyDefaultsResponse, BookConfig,
};
pub use typography::{DpiValidation, TypoIssue, TypographyConfig, UpdateTypographyConfig, ValidationError};
