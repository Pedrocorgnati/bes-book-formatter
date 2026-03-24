# Anti-Hacking Tasks — BES Book Formatter
Data: 2026-03-22

---

## P0-BLOCKER (Identificados pelo Codex — Fix imediato antes de qualquer release)

### T000-A — Usar caminho absoluto do bundle para sidecars (Sidecar PATH Hijacking)

**Vulnerabilidade:** M1 — Sidecar binary hijacking via PATH
**Arquivo:** `src-tauri/src/services/sidecar_manager.rs:77-99`

**Contexto:**
`Command::new("typst")` resolve pelo PATH do sistema. Um atacante com escrita no PATH pode substituir o binário. As capabilities Tauri (`shell:allow-execute` com sidecar scope) **não protegem o backend Rust** — apenas o frontend IPC.

**ANTES (vulnerável):**
```rust
pub async fn spawn_typst(args: &[String], timeout_ms: u64) -> Result<(String, String), AppError> {
    Self::spawn_process("typst", args, timeout_ms).await  // ← PATH lookup
}
```

**DEPOIS (seguro — via AppHandle resource_dir):**
```rust
// Adicionar AppHandle como parâmetro ou usar estado global
// Tauri bundled sidecars ficam em: {resource_dir}/binaries/typst

pub async fn spawn_typst(
    resource_dir: &std::path::Path,
    args: &[String],
    timeout_ms: u64
) -> Result<(String, String), AppError> {
    let typst_path = resource_dir.join("binaries").join(
        if cfg!(target_os = "windows") { "typst.exe" } else { "typst" }
    );
    if !typst_path.exists() {
        return Err(AppError::sidecar_not_found("typst"));
    }
    Self::spawn_process_at_path(&typst_path, args, timeout_ms).await
}
```

**Estimativa:** 3-4 horas (refactor SidecarManager para usar paths absolutos)

---

### T000-B — Expandir fix Typst para pdf_ebook_service e cover_service

**Vulnerabilidade:** M2 — Typst injection em outros fluxos
**Arquivos:**
- `src-tauri/src/services/pdf_ebook_service.rs:175,185`
- `src-tauri/src/services/cover_service.rs:121,123,138,140`

**Contexto:**
Corrigir apenas `pdf_print_service.rs` (T001) deixa 2 vetores idênticos ativos.

**pdf_ebook_service.rs — mesmo padrão do V001:**
```rust
// ANTES (linha ~185):
chapters_typ.push_str(line);  // ← SEM escape

// DEPOIS:
chapters_typ.push_str(&escape_typst_text(line));  // ← com escape
// E para titles (linha ~175):
chapters_typ.push_str(&format!("\n= {}\n\n", escape_typst_text(&chapter.title)));
```

**cover_service.rs — title/author sem escape:**
```rust
// ANTES (linhas ~138,140):
let typst = format!(r#"
  #text(...)[{title}]
  #text(...)[{author}]
"#, title = title, author = author);  // ← SEM escape

// DEPOIS:
let typst = format!(r#"
  #text(...)[{title}]
  #text(...)[{author}]
"#,
    title = escape_typst_text(title),
    author = escape_typst_text(author),
);
```

**Nota:** `escape_typst_text` deve ser extraído para `common.rs` no mesmo PR que T001.

**Estimativa:** 2 horas (após T001 já ter extraído a função para common.rs)

---

## P1-CRITICO (Fix antes do próximo release)

### T001 — Escapar conteúdo do manuscrito antes de embeddar em .typ (PDF Print)

**Vulnerabilidade:** V001 — Typst Code Injection
**Arquivo:** `src-tauri/src/services/pdf_print_service.rs:288-305, 339`
**Attack Chain:** Chain 1 (Manuscrito malicioso → leitura de arquivo local via Typst)

**Contexto:**
O `pdf_print_service.rs` tem sua própria lógica de conversão Markdown→Typst que NÃO usa as funções de escape que existem em `preview_service.rs`. O caractere `#` não é escapado, permitindo injeção de comandos Typst como `#read("/path")`.

**ANTES (vulnerável):**
```rust
// pdf_print_service.rs ~290-301
for line in chapter.content.lines() {
    let line = line.trim();
    if line.starts_with("## ") {
        chapters_typ.push_str(&format!("== {}\n\n", &line[3..]));  // SEM escape
    } else if line.starts_with("### ") {
        chapters_typ.push_str(&format!("=== {}\n\n", &line[4..]));  // SEM escape
    } else {
        chapters_typ.push_str(line);  // SEM escape ← VULNERÁVEL
        chapters_typ.push('\n');
    }
}
```

**DEPOIS (seguro):**
```rust
// Passo 1: Mover escape_typst_text para services/common.rs
// (já existe em preview_service.rs como função privada — extrair para reutilização)

// services/common.rs — adicionar:
pub fn escape_typst_text(text: &str) -> String {
    text.replace('\\', "\\\\")
        .replace('#', "\\#")
        .replace('@', "\\@")
        .replace('$', "\\$")
        .replace('<', "\\<")
        .replace('>', "\\>")
}

// pdf_print_service.rs — usar escape:
use crate::services::common::escape_typst_text;

for line in chapter.content.lines() {
    let line = line.trim();
    if line.starts_with("## ") {
        chapters_typ.push_str(&format!("== {}\n\n", escape_typst_text(&line[3..])));
    } else if line.starts_with("### ") {
        chapters_typ.push_str(&format!("=== {}\n\n", escape_typst_text(&line[4..])));
    } else if line.is_empty() {
        chapters_typ.push('\n');
    } else {
        chapters_typ.push_str(&escape_typst_text(line));  // ← SEGURO
        chapters_typ.push('\n');
    }
}

// Também escapar chapter.title (linha ~290):
chapters_typ.push_str(&format!("\n= {}\n\n", escape_typst_text(&chapter.title)));
```

**Nota:** O mesmo padrão de fix deve ser aplicado em `pdf_ebook_service.rs` e `cover_service.rs` se eles tiverem lógica similar de template building.

**Teste de validação:**
```rust
#[test]
fn test_typst_injection_blocked_in_pdf_generation() {
    // Criar manuscrito com payload Typst
    let malicious_content = "#read(\"/etc/hostname\")";
    let typ_file = generate_typ_file_with_content(malicious_content);
    // Verificar que o .typ resultante contém "\\#read" e não "#read"
    assert!(typ_file.contains("\\#read"));
    assert!(!typ_file.contains("#read(\"/etc/hostname\")"));
}
```

**Estimativa:** 2-3 horas (extração da função + fix em todos os serviços + testes)

---

## P2-ALTO (Fix em 1 semana)

### T002 — Validar parâmetro `platform` contra allowlist

**Vulnerabilidade:** V002 — Path Traversal via `platform`
**Arquivos:**
- `src-tauri/src/commands/generation.rs` (ponto de entrada IPC)
- `src-tauri/src/services/pdf_print_service.rs:116-120`

**ANTES (vulnerável):**
```rust
// generation.rs
pub async fn generate_pdf_print(
    pool: State<'_, SqlitePool>,
    project_id: String,
    platform: String,  // ← sem validação
    pdfx_profile: Option<String>,
) -> Result<ApiResponse<GenerationResult>, String> {
    // platform vai direto para o path de saída
```

**DEPOIS (seguro):**
```rust
// services/common.rs — adicionar:
pub fn validate_platform(platform: &str) -> Result<(), AppError> {
    const ALLOWED: &[&str] = &[
        "kdp", "kdp_print", "ingram_spark", "kobo", "generic",
        "amazon-kdp", "amazon-kdp-print", "amazon-hardcover",
        "ingram-spark", "draft2digital", "smashwords",
    ];
    if ALLOWED.iter().any(|&p| p == platform) {
        Ok(())
    } else {
        Err(AppError::new("VAL_010", format!("Plataforma inválida: {}", platform)))
    }
}

// generation.rs — adicionar validação no início de cada command:
pub async fn generate_pdf_print(
    pool: State<'_, SqlitePool>,
    project_id: String,
    platform: String,
    pdfx_profile: Option<String>,
) -> Result<ApiResponse<GenerationResult>, String> {
    validate_platform(&platform).map_err(AppError::into)?;
    // ...
```

**Comandos afetados:** `generate_pdf_print`, `generate_pdf_ebook`, `generate_epub`, `generate_docx`, `generate_html`

**Estimativa:** 1-2 horas

---

### T003 — Validar `project_id` como UUID em operações de filesystem

**Vulnerabilidade:** V003 — Path Traversal via `project_id`
**Arquivos:**
- `src-tauri/src/services/font_service.rs:68,113,148`
- `src-tauri/src/services/preview_service.rs:322,419,437`
- `src-tauri/src/services/illustration_service.rs:343`

**ANTES (vulnerável):**
```rust
// font_service.rs:66-68
let dest_dir = app_data_dir.join("fonts").join(project_id);
```

**DEPOIS (seguro):**
```rust
// services/common.rs — adicionar:
pub fn validate_project_id(id: &str) -> Result<(), String> {
    // UUID v4: 8-4-4-4-12 hex lowercase
    let re = once_cell::sync::Lazy::new(|| {
        regex::Regex::new(
            r"^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$"
        ).unwrap()
    });
    if re.is_match(id) {
        Ok(())
    } else {
        Err(format!("VAL_011: project_id inválido: {}", id))
    }
}

// font_service.rs, preview_service.rs, illustration_service.rs — adicionar no início:
validate_project_id(project_id).map_err(|e| e)?;
let dest_dir = app_data_dir.join("fonts").join(project_id);
```

**Estimativa:** 2 horas

---

## P3-MEDIO (Próximo sprint)

### T004 — Usar caminho absoluto para output de PDF (não relativo ao CWD)

**Vulnerabilidade:** V004 — Relative output path
**Arquivo:** `src-tauri/src/services/pdf_print_service.rs:116-120`

**Fix:** Substituir `format!("output/books/{}", ...)` por um caminho absoluto baseado em `bes_root_path` do projeto ou em um diretório de output configurável via `AppHandle`.

```rust
// Exemplo: usar bes_root_path como base
let output_dir = Path::new(&project.bes_root_path)
    .join(".bes-output")
    .join("books")
    .join(sanitize_slug(&project.name));
```

**Estimativa:** 1 hora

---

### T005 — Commitar Cargo.lock e implementar cargo audit no CI

**Vulnerabilidade:** V005 — Supply chain sem lock file

**Ações:**
1. Executar `cargo build` para gerar `src-tauri/Cargo.lock`
2. Remover `Cargo.lock` do `.gitignore` (se estiver lá)
3. Commitar `Cargo.lock`
4. Adicionar ao CI:
```yaml
- name: Rust security audit
  run: |
    cargo install cargo-audit
    cd src-tauri && cargo audit
```

**Estimativa:** 30 minutos

---

### T006 — Validar magic bytes em upload de fontes

**Vulnerabilidade:** V006 — Sem validação de magic bytes
**Arquivo:** `src-tauri/src/services/font_service.rs:56-63`

**Fix:**
```rust
// Adicionar ANTES da validação de tamanho:
fn validate_font_magic_bytes(source: &Path) -> Result<(), String> {
    use std::io::Read;
    let mut buf = [0u8; 4];
    let mut file = std::fs::File::open(source)
        .map_err(|e| format!("FS_001: Não foi possível ler o arquivo: {}", e))?;
    file.read_exact(&mut buf)
        .map_err(|_| "VAL_004: Arquivo muito pequeno para ser uma fonte válida".to_string())?;
    // OTF: "OTTO", TTF: \0\1\0\0 ou "true"
    let is_otf = &buf == b"OTTO";
    let is_ttf = buf == [0x00, 0x01, 0x00, 0x00] || &buf == b"true";
    if is_otf || is_ttf {
        Ok(())
    } else {
        Err("VAL_004: O arquivo não possui assinatura OTF/TTF válida".to_string())
    }
}
```

**Estimativa:** 1 hora (incluindo testes)

---

### T007 — Truncar stdout do sidecar para evitar leakage de conteúdo

**Vulnerabilidade:** V007 — stdout não truncado
**Arquivo:** `src-tauri/src/services/sidecar_manager.rs:124-134`

**Fix:**
```rust
// sidecar_manager.rs — aplicar truncamento simétrico em stdout
let stdout_full = String::from_utf8_lossy(&output.stdout).to_string();
// SEC-010: Truncar stdout assim como stderr para evitar leakage de conteúdo do manuscrito
let stdout = if stdout_full.len() > 2048 {
    format!("{}…[truncated {} chars]", &stdout_full[..2048], stdout_full.len() - 2048)
} else {
    stdout_full
};
```

**Nota:** 2048 chars é razoável para mensagens de status do Typst sem truncar output legítimo de diagnóstico.

**Estimativa:** 30 minutos

---

## P4-BAIXO (Backlog)

### T008 — Atualizar devDependencies do Stryker

**Vulnerabilidade:** V009 — npm devDep vulnerabilities

```bash
npm audit fix --force
# Instala @stryker-mutator/core 9.6.0 (breaking change)
# Verificar se configuração do Stryker precisa atualização
```

**Nota:** Verificar `stryker.config.json` para compatibilidade com v9.6.0 antes de atualizar.

**Estimativa:** 1-2 horas (incluindo ajuste de config)

---

### T009 — Remover `'unsafe-inline'` da CSP de estilos

**Vulnerabilidade:** V008 — CSP unsafe-inline
**Arquivo:** `src-tauri/tauri.conf.json`

```json
// ANTES
"csp": "... style-src 'self' 'unsafe-inline' ..."

// DEPOIS (testar se o app funciona sem unsafe-inline)
"csp": "... style-src 'self' ..."
```

**Nota:** SvelteKit com estilos scoped pode precisar de `unsafe-inline` para estilos injetados. Testar exaustivamente antes de remover.

**Estimativa:** 2-4 horas (teste + possível refactor de estilos)

---

## Notas do Pair Programming (Codex)

### Regressões nos fixes propostos:
- **T002 (platform allowlist):** Verificar aliases aceitos em `platform_presets.rs` antes de implementar allowlist — alguns aliases podem diferir do nome exato
- **T003 (UUID v4 estrito):** Regex v4-only pode rejeitar IDs legados. Preferir `uuid::Uuid::parse_str()` que aceita v1-v5
- **T007 (stdout truncation):** Truncar globalmente quebra parsing de EPUBCheck. Truncar apenas para logging/UI, preservar output completo para parsing interno

### Testes mínimos obrigatórios pós-fix:
1. **TST1:** Payload `#read("/etc/hostname")` em pdf_print + pdf_ebook + cover → verificar que NÃO aparece no PDF
2. **TST2:** PATH poisoning com `typst` falso → app deve usar binário bundled por caminho absoluto
3. **TST3:** `project_id = "../../../tmp"` em todos os commands → verificar que path fica dentro de `$APPDATA`
4. **TST4:** Symlinks em `manuscript_root` → parser não deve seguir links fora do workspace
5. **TST5:** EPUBCheck output parsing após truncation — não quebrar
6. **TST6:** Manuscrito com 10.000 capítulos → timeouts e limites efetivos

## Ordem de Execução Recomendada

```
T000-A (sidecar PATH hijack — P0-BLOCKER)
→ T000-B (typst injection em ebook+cover — P0-BLOCKER, junto com T001)
→ T001 (typst injection em pdf_print — P1, extrair escape_typst_text para common.rs)
→ T002 (platform allowlist — P2, verificar aliases primeiro)
→ T003 (project_id UUID — P2, usar uuid::Uuid::parse_str em vez de regex)
→ T005 (Cargo.lock — P3, não requer código)
→ T004 (output path — P3)
→ T006 (magic bytes — P3)
→ T007 (stdout truncation — P3, truncar só para UI/logs)
→ T008 (npm update — P4)
→ T009 (CSP — P4)
```
