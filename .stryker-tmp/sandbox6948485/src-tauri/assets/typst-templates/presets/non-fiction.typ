// BES Book Formatter — Non-Fiction / Self-Help / Business Preset
// Default values:
//   font_body = "Source Serif 4", size = 11pt, leading = 1.45em
//   page = 6×9in, margins = 0.75/0.75/1.0/0.75in

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
  // Page numbers in footer
  footer: context {
    set text(size: 9pt)
    align(center, counter(page).display("1"))
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

// Headings — ST001: drop cap optional on H1, larger chapter title
#show heading.where(level: 1): it => {
  chapter-break(recto: true)
  if "{drop_cap_style}" == "none" {
    block(
      above: 2em,
      below: 1em,
      text(size: 2.2em, weight: "bold", it.body)
    )
  } else {
    drop-cap-classic(it.body)
  }
}
#show heading.where(level: 2): it => bes-heading(2, it.body)
#show heading.where(level: 3): it => bes-heading(3, it.body)
#show heading.where(level: 4): it => bes-heading(4, it.body)

// ST003: Code blocks
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

{content}
