// BES Book Formatter — Fiction / Romance Preset
// Compiled by the Rust generation service. Placeholders ({var}) are replaced
// with actual values from TypographyConfig before passing to Typst CLI.
//
// Default values (used when no config override is provided):
//   font_body   = "EB Garamond"
//   font_heading = "EB Garamond"
//   size_body   = 11pt
//   leading     = 1.4em
//   page_width  = 5.5in
//   page_height = 8.5in
//   margin_top  = 0.75in
//   margin_bottom = 0.75in
//   margin_inner  = 1.0in
//   margin_outer  = 0.75in
//   lang        = "pt"  (or "en" / "es")

#import "../base.typ": *

#set page(
  width: {page_width}in,
  height: {page_height}in,
  margin: (
    top:     {margin_top}in,
    bottom:  {margin_bottom}in,
    inside:  {margin_inner}in,
    outside: {margin_outer}in,
  ),
)

#set text(
  font:   "{font_body}",
  size:   {font_size_body}pt,
  lang:   "{lang}",
  hyphenate: {hyphenation},
)

#set par(
  justify:           {justification},
  leading:           {leading}em,
  first-line-indent: {paragraph_indent}em,
)

// Headings (H1 merged with chapter-break + optional drop cap)
// ST001: Drop cap applied when {drop_cap_style} != "none"
#show heading.where(level: 1): it => {
  chapter-break(recto: true)
  if "{drop_cap_style}" == "none" {
    bes-heading(1, it.body)
  } else {
    drop-cap-classic(it.body)
  }
}
#show heading.where(level: 2): it => bes-heading(2, it.body)
#show heading.where(level: 3): it => bes-heading(3, it.body)
#show heading.where(level: 4): it => bes-heading(4, it.body)

// ST003: Code blocks — monospace, light background
#show raw: it => {
  set text(font: ("JetBrains Mono", "Courier New", "monospace"), size: 0.85em)
  block(
    fill: luma(245),
    radius: 3pt,
    inset: (x: 0.625em, y: 0.5em),
    width: 100%,
    it
  )
}

// Content placeholder — replaced by actual manuscript body
{content}
