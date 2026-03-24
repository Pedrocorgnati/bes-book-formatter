// BES Book Formatter — Title Page (Frontispício) Typst Template
// Placeholders replaced at runtime: {title}, {author}, {publisher}, {year}

#import "../base.typ": *

// No page number on title page
#set page(numbering: none)

// Title page content: vertically centered
#align(center + horizon)[
  #v(2fr)

  #block(below: 1.5em)[
    #text(size: 3em, weight: "bold", {title})
  ]

  #block(below: 0.5em)[
    #text(size: 1.5em, style: "italic", {author})
  ]

  #v(3fr)

  #block(above: 0em)[
    #text(size: 0.9em, {publisher})
    #linebreak()
    #text(size: 0.9em, str({year}))
  ]
]
