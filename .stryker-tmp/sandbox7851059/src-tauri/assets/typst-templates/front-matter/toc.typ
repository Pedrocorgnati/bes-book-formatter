// BES Book Formatter — Table of Contents (Sumário) Typst Template
// Uses Typst native two-pass outline() for accurate page numbers.
// Placeholder: {toc_title} (default "Sumário" for pt-BR)

#import "../base.typ": *

// Front matter pages use roman numerals
#set page(numbering: "i")

#heading(level: 1, bookmarked: false)[{toc_title}]

#outline(
  title: none,
  indent: auto,
  depth: 2,
  fill: repeat[.],  // Leader dots: "Capítulo 1 .......... 7"
)
