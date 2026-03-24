// BES Book Formatter — Colophon Typst Template
// Placeholders: {colophon_text}, {generation_date}
// The {colophon_text} block typically contains typographic notes about
// fonts, paper, and printing details. {generation_date} is auto-populated.

#import "../base.typ": *

// Last page — no page number
#pagebreak(to: "odd")
#set page(numbering: none)

#v(1fr)

#align(center)[
  #set text(size: 8pt)
  #set par(justify: false, leading: 1.4em)

  {colophon_text}

  #v(1em)
  #text(style: "italic")[{generation_date}]
]
