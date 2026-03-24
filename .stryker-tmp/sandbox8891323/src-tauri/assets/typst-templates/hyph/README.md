# Hyphenation Dictionaries

Typst bundles hyphenation patterns via `hyph-lang` in `#set text()`.
No external dictionary files are needed — Typst's built-in engine handles:

- `lang: "pt"` → Portuguese (pt-BR / pt-PT)
- `lang: "en"` → English
- `lang: "es"` → Spanish

If you need custom exception lists, place `.tex` pattern files here
and reference them in the template:

```typst
#set text(lang: "pt", region: "BR")
```
