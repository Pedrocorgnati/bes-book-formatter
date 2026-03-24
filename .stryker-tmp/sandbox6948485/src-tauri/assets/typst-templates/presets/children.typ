// BES Book Formatter — Children's / Young Adult Preset
// Default values:
//   font_body = "Source Serif 4", size = 14pt, leading = 1.6em
//   page = 8.5×8.5in (square format)

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
)

#set text(
  font:     "{font_body}",
  size:     {font_size_body}pt,
  lang:     "{lang}",
  hyphenate: false,  // Children's books avoid hyphenation for readability
)

#set par(
  justify:           false,   // Left-aligned for children's books
  leading:           {leading}em,
  first-line-indent: 0em,     // No indent — spacing between paragraphs instead
  spacing:           0.75em,
)

// Headings — Children's: larger font, no drop cap (child-friendly layout)
#show heading.where(level: 1): it => {
  chapter-break(recto: true)
  block(
    above: 1em,
    below: 1em,
    text(size: 2.5em, weight: "bold", it.body)
  )
}
#show heading.where(level: 2): it => block(
  above: 1em,
  below: 0.5em,
  text(size: 1.8em, weight: "bold", it.body)
)

// ST003: Code blocks (minimal, suitable for activity/puzzle books)
#show raw: it => {
  set text(font: ("JetBrains Mono", "Courier New", "monospace"), size: 0.9em)
  block(
    fill: luma(250),
    radius: 4pt,
    inset: (x: 0.5em, y: 0.5em),
    width: 100%,
    it
  )
}

{content}
