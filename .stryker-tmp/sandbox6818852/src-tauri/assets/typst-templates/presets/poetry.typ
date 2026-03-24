// BES Book Formatter — Poetry Preset
// Default values:
//   font_body = "EB Garamond", size = 12pt, leading = 1.6em
//   page = 5.5×8.5in, no paragraph indent, no justification

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
  hyphenate: false,  // Poetry should never be hyphenated
)

#set par(
  justify:           false,  // Poetry is always left-aligned
  leading:           {leading}em,
  first-line-indent: 0em,
)

// Poem titles — centered, italic (no drop cap for poetry)
#show heading.where(level: 1): it => {
  chapter-break(recto: true)
  block(
    above: 2em,
    below: 1.5em,
    align(center, text(size: 1.5em, style: "italic", it.body))
  )
}

#show heading.where(level: 2): it => block(
  above: 2em,
  below: 1em,
  align(center, text(size: 1.25em, style: "italic", it.body))
)

// ST004: Section ornament between poems (---) — uses {ornament_style} config
#show line: _ => ornament("{ornament_style}")

// ST003: Code blocks (rare in poetry but supported)
#show raw: it => {
  set text(font: ("JetBrains Mono", "Courier New", "monospace"), size: 0.85em)
  block(
    fill: luma(248),
    radius: 3pt,
    inset: (x: 0.5em, y: 0.4em),
    width: 100%,
    it
  )
}

{content}
