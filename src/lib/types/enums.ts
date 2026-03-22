// ==========================================================
// BES Book Formatter — Enums Compartilhados (14 enums)
// Usados por todos os rocks via $lib/types
// ==========================================================

export enum Genre {
  NONFICTION = 'nonfiction',
  SELF_HELP = 'self_help',
  TECHNICAL = 'technical',
  ACADEMIC = 'academic',
  FICTION = 'fiction',
  ROMANCE = 'romance',
  BUSINESS = 'business',
  MANAGEMENT = 'management',
  CHILDREN = 'children',
  YA = 'ya'
}

export enum OutputFormat {
  EPUB3 = 'epub3',
  PDF_EBOOK = 'pdf_ebook',
  PDF_PRINT = 'pdf_print',
  DOCX = 'docx',
  HTML5 = 'html5',
  MARKDOWN_CLEAN = 'markdown_clean',
  TXT = 'txt',
  JSON_STRUCTURAL = 'json_structural'
}

export enum Platform {
  KDP = 'kdp',
  KDP_PRINT = 'kdp_print',
  INGRAM_SPARK = 'ingram_spark',
  APPLE_BOOKS = 'apple_books',
  KOBO = 'kobo',
  DRAFT2DIGITAL = 'draft2digital',
  GENERIC = 'generic'
}

export enum IllustrationState {
  PENDING = 'pending',
  IMPORTED = 'imported',
  LINKED = 'linked'
}

export enum PageFormat {
  TRADE_6X9 = 'trade_6x9',
  DIGEST_5_5X8_5 = 'digest_5_5x8_5',
  POCKET_4_25X6_87 = 'pocket_4_25x6_87',
  A5 = 'a5',
  A4 = 'a4',
  LETTER = 'letter',
  CUSTOM = 'custom'
}

export enum BookLanguage {
  PT_BR = 'pt-BR',
  EN_US = 'en-US',
  IT_IT = 'it-IT',
  ES_ES = 'es-ES'
}

export enum UILanguage {
  PT_BR = 'pt-BR',
  EN_US = 'en-US',
  ES_ES = 'es-ES'
}

export enum PaperColor {
  WHITE_70LB = 'white_70lb',
  CREAM_60LB = 'cream_60lb',
  CUSTOM = 'custom'
}

export enum ChapterStartPage {
  ODD = 'odd',
  EVEN = 'even',
  CONTINUOUS = 'continuous'
}

export enum DropCapStyle {
  NONE = 'none',
  FIRST_LETTER = 'first_letter',
  FIRST_WORD_SMALL_CAPS = 'first_word_small_caps'
}

export enum OrnamentStyle {
  NONE = 'none',
  LINE = 'line',
  VIGNETTE = 'vignette',
  ASTERISKS = 'asterisks'
}

export enum PDFXProfile {
  PDF_X1A = 'pdf_x1a',
  PDF_X4 = 'pdf_x4'
}

export enum BookConfigVersion {
  V1 = 'v1',
  V2 = 'v2',
  V3 = 'v3'
}

export enum ManuscriptCompleteness {
  BLOCKING = 'blocking',
  WARNING = 'warning',
  NORMAL = 'normal'
}
