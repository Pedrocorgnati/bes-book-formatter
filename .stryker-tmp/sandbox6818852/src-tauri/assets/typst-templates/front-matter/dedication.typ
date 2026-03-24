// BES Book Formatter — Dedication Page Typst Template
// Placeholder: {dedication_text}

#import "../base.typ": *

#set page(numbering: none)

// Dedication: centered horizontally, positioned 1/3 from top
#v(1fr)
#align(center)[
  #text(size: 1.1em, style: "italic", {dedication_text})
]
#v(2fr)
