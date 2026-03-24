// BES Book Formatter — Technical / Academic Preset
// Default values:
//   font_body = "Source Serif 4", font_code = "JetBrains Mono"
//   size = 10pt, leading = 1.5em, page = 7×10in

#import "../base.typ": *

#set page(
  width:  {page_width}in,
  height: {page_height}in,
  margin: (
    top:     {margin_top}in,
    bottom:  {margin_bottom}in,
    inside:  {margin_inner}in,
    outside: {margin_outer}in,
  ),
  // Technical books often have page numbers with section
  footer: context {
    set text(size: 9pt)
    grid(
      columns: (1fr, 1fr),
      [],
      align(right, counter(page).display("1"))
    )
  },
)

#set text(
  font:     "{font_body}",
  size:     {font_size_body}pt,
  lang:     "{lang}",
  hyphenate: {hyphenation},
)

#set par(
  justify:           {justification},
  leading:           {leading}em,
  first-line-indent: {paragraph_indent}em,
)

// ST003: Code blocks use monospace font (technical preset: explicit lang support)
#show raw: it => {
  set text(font: ("{font_code}", "JetBrains Mono", "Courier New", "monospace"), size: 0.9em)
  block(
    fill: luma(240),
    radius: 4pt,
    inset: (x: 0.625em, y: 0.5em),
    width: 100%,
    it
  )
}

// Headings — Technical books: no drop cap (chapter-break to any page)
#show heading.where(level: 1): it => {
  chapter-break(recto: false)  // Technical books allow any page
  bes-heading(1, it.body)
}
#show heading.where(level: 2): it => bes-heading(2, it.body)
#show heading.where(level: 3): it => bes-heading(3, it.body)
#show heading.where(level: 4): it => bes-heading(4, it.body)

{content}
