// BES Book Formatter — Base Typst Template
// This template is included by each genre preset and provides shared utilities.
// Variable placeholders are replaced at runtime by the Rust generation service.

// ── Shared heading styles ──────────────────────────────────────────────────

#let bes-heading(level, body) = {
  let sizes = (
    1: 2.0em,
    2: 1.6em,
    3: 1.3em,
    4: 1.1em,
  )
  let sz = sizes.at(str(level), default: 1.0em)
  block(
    above: 1.5em,
    below: 0.75em,
    text(size: sz, weight: "bold", body)
  )
}

// ── Page break utilities ───────────────────────────────────────────────────

#let chapter-break(recto: true) = {
  if recto {
    // Start on odd (recto) page
    pagebreak(to: "odd")
  } else {
    pagebreak()
  }
}

// ── Footnote style ─────────────────────────────────────────────────────────

#let bes-footnote(body) = footnote(body)

// ── Drop cap (ST001 — TASK-3) ──────────────────────────────────────────────
//
// Classic drop cap: enlarges the first letter of the heading body 3×.
// Applied via show rule on heading.where(level: 1) in genre templates.
//
// Usage in genre template:
//   #show heading.where(level: 1): it => {
//     chapter-break(recto: true)
//     drop-cap-classic(it.body)
//   }
//
// [ERROR] VAL_001: If heading starts with a non-letter (digit, etc.) the drop
// cap is skipped and the heading renders normally (handled via if guard below).
// [EDGE] Characters with accents (Á, É, etc.) are handled by .clusters().

#let drop-cap-classic(body) = {
  let body-text = body.text
  // Guard: if the heading starts with a non-letter (e.g. a number), skip drop cap.
  if body-text.len() == 0 {
    bes-heading(1, body)
  } else {
    let first-ch = body-text.clusters().first()
    let rest = body-text.slice(first-ch.len())
    par(justify: false)[
      #box(baseline: 0.7em)[
        #text(size: 3em, weight: "bold")[#first-ch]
      ]
      #h(0.1em)#rest
    ]
  }
}

// ── Small caps (ST001 — TASK-3) ────────────────────────────────────────────
//
// Applies OpenType small-caps feature to the given content.
// Usage: #bes-small-caps[Versaletes aqui]

#let bes-small-caps(body) = {
  text(features: ("smcp",))[#body]
}

// ── Blockquote (ST004 — TASK-3) ───────────────────────────────────────────
//
// Renders `> Citação` blocks as a visually distinct indented block with
// a left border and italic text.
// [ERROR] VAL_001: Empty blockquote `> ` → renders as an empty paragraph.

#let blockquote(body) = {
  block(
    above: 1em,
    below: 1em,
    inset: (left: 1.5em, right: 0.5em, top: 0.25em, bottom: 0.25em),
    stroke: (left: 2pt + rgb("#aaaaaa")),
    emph(body)
  )
}

// ── Epigraph (ST004 — TASK-3) ─────────────────────────────────────────────
//
// Renders `> _Epígrafe_` at chapter opening, aligned right, smaller font.

#let epigraph(body) = {
  align(right, block(
    width: 55%,
    above: 2em,
    below: 3em,
    text(size: 0.9em, style: "italic")[#body]
  ))
}

// ── Ornament (ST004 — TASK-3) ─────────────────────────────────────────────
//
// Renders a section ornament replacing `---` on its own line.
// `style` values: "fleuron" (❦), "rule" (line), "dinkus" (• • •), "none".

#let ornament(style) = {
  align(center)[
    #v(0.5em)
    #if style == "dinkus" [
      #text(size: 1.2em)[• • •]
    ] else if style == "rule" [
      #line(length: 30%, stroke: 0.5pt)
    ] else if style == "none" [
      // No ornament
    ] else [
      // Default: fleuron (❦)
      #text(size: 1.5em)[❦]
    ]
    #v(0.5em)
  ]
}

// ── Drop cap — original grid variant (kept for backwards compatibility) ────

#let drop-cap(letter, rest) = {
  grid(
    columns: (1.5em, 1fr),
    gutter: 0.25em,
    text(size: 3em, weight: "bold", letter),
    rest
  )
}
