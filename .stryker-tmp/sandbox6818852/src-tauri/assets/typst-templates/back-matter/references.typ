// BES Book Formatter — References / Bibliography Typst Template
// Format: ABNT NBR 6023:2018 (pt-BR) or generic for other languages.
// Placeholder: {references_title}, {references_content}, {lang}

#import "../base.typ": *

#pagebreak()

#heading(level: 1)[{references_title}]

// ABNT style: hanging indent, 12pt between entries, 1.5 leading
#set par(
  hanging-indent: 2em,
  justify: false,
  leading: 1.5em,
)

#set block(spacing: 0.75em)

{references_content}
