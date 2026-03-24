# Anti-Hacking Review Report — BES Book Formatter
Data: 2026-03-22
Projeto: BES Book Formatter (Tauri 2 + SvelteKit + Rust/SQLite)
Fingerprint: `ai-forge/anti-hacking-fingerprint.md`

> **NOTA IMPORTANTE:** Este não é um app Next.js. A auditoria foi adaptada ao stack real:
> Tauri 2.x (desktop), SvelteKit frontend, Rust backend, SQLite, sidecars externos.
> O modelo de ameaça é fundamentalmente diferente de uma web app: sem acesso remoto direto,
> sem autenticação multi-usuário, sem PCI. A ameaça principal são **arquivos de entrada maliciosos**.

---

## Resumo Executivo

| Severidade | Total | Corrigidas |
|------------|-------|------------|
| P0-BLOCKER | 2 | 0 |
| P1-CRITICO | 4 | 0 |
| P2-ALTO | 2 | 0 |
| P3-MEDIO | 3 | 0 |
| P4-BAIXO | 2 | 0 |
| **Total** | **13** | **0** |

> ⚠️ **Pair Programming Codex:** BLOCKED — 2 blockers adicionais identificados não cobertos pela auditoria inicial.

- CVEs aplicáveis ao projeto: **2** (RUSTSEC informacionais, sem impacto direto)
- CVEs verificados e NÃO aplicáveis: 5 (PostgreSQL-only, versões já corrigidas)
- Attack chains identificadas: **3**
- Risco geral: **CRÍTICO**

---

## CVEs Verificados

| ID | Componente | Afeta este projeto? | Motivo |
|----|------------|-------------------|--------|
| CVE-2025-32388 | @sveltejs/kit ≤ 2.20.5 | **NÃO** | App usa `adapter-static` (sem SSR), uso de `.get('tab')` não itera params como array |
| RUSTSEC-2024-0363 | sqlx ≤ 0.8.0 | **NÃO** | Advisory afeta PostgreSQL; projeto usa SQLite exclusivamente |
| RUSTSEC-2021-0026/63 | comrak XSS | **NÃO** | Corrigido em comrak 0.13+; projeto usa 0.26 |
| RUSTSEC-2024-0320 | syntect/yaml-rust | **INFORMACIONAL** | Dependência indireta não mantida; sem CVE ativo |
| CVE-2024-24576 | Rust stdlib | **NÃO** | Afeta Windows `Command::new(cmd)` com argumentos shell. Projeto usa argumentos tipados via array — SEC-009 anotado no código |

---

## Vulnerabilidades Detalhadas

---

### V001 — Typst Code Injection na Geração de PDF (pdf_print_service)
**Severidade:** P1-CRITICO | **CVSS Estimado:** 7.8 (Local, High Impact)

**Arquivo:** `src-tauri/src/services/pdf_print_service.rs:288-305, 339`

**Descrição:**
A função `generate_typ_file` em `PdfPrintService` embute o conteúdo bruto dos capítulos do manuscrito diretamente em um arquivo `.typ` (Typst) **sem escaping** dos caracteres especiais da linguagem Typst (`#`, `@`, `$`, `<`, `>`).

O serviço de preview (`preview_service.rs:87-94`) possui as funções `escape_typst_text()` e `escape_typst_content()` que realizam o escaping correto. Porém, o caminho de geração de PDF (`pdf_print_service.rs`) não utiliza essas funções — é uma duplicação insegura do código de conversão Markdown→Typst.

**Código vulnerável:**
```rust
// pdf_print_service.rs:301
chapters_typ.push_str(line);  // ← linha bruta SEM escape de # @ $ < >

// pdf_print_service.rs:339
{chapters_typ}  // ← embeddado diretamente no .typ sem escaping
```

**Código seguro (preview_service.rs, que escapa corretamente):**
```rust
// preview_service.rs:87-94
fn escape_typst_text(text: &str) -> String {
    text.replace('\\', "\\\\")
        .replace('#', "\\#")
        .replace('@', "\\@")
        .replace('$', "\\$")
        .replace('<', "\\<")
        .replace('>', "\\>")
}
```

**Vetor de ataque:**
O Typst suporta leitura de arquivos locais via `#read("caminho")` e inclusão de outros scripts via `#import`. Um livro com o seguinte conteúdo em qualquer capítulo:

```
#read("/home/usuario/.ssh/id_rsa")
```

Fará com que o compilador Typst embutа o conteúdo da chave privada SSH **dentro do PDF gerado**, que é então salvo em disco e potencialmente compartilhado.

Outros payloads Typst relevantes:
- `#import "/etc/passwd": *` — inclui conteúdo de system files
- `#read("/home/usuario/.env")` — segredos locais
- `#sys.inputs` — variáveis de ambiente (Typst 0.10+)

**Impacto:** Leitura de arquivos locais arbitrários (dentro das permissões do processo) via manuscrito malicioso. O arquivo lido é embeddado no PDF de saída.

**Fix proposto:**
```rust
// Em pdf_print_service.rs — importar ou duplicar as funções de escape do preview_service
// Mover para services/common.rs e reutilizar em ambos:

fn escape_typst_text(text: &str) -> String {
    text.replace('\\', "\\\\")
        .replace('#', "\\#")
        .replace('@', "\\@")
        .replace('$', "\\$")
        .replace('<', "\\<")
        .replace('>', "\\>")
}

// Aplicar na linha 290:
chapters_typ.push_str(&format!("\n= {}\n\n", escape_typst_text(&chapter.title)));

// Aplicar na linha 301:
chapters_typ.push_str(&escape_typst_text(line));
```

**Teste de validação:** Criar manuscrito com `#read("/etc/hostname")` em um capítulo, gerar PDF, verificar que o nome do host NÃO aparece no PDF.

---

### V002 — Parâmetro `platform` Não Sanitizado no Caminho de Saída
**Severidade:** P2-ALTO | **CVSS Estimado:** 6.5 (Local, Path Traversal)

**Arquivo:** `src-tauri/src/services/pdf_print_service.rs:116-120`

**Descrição:**
O parâmetro `platform` recebido via IPC é usado diretamente na construção do caminho do arquivo de saída sem sanitização:

```rust
// pdf_print_service.rs:120
let output_path = format!(
    "{}/{}-print-{}.pdf",
    output_dir,
    sanitize_slug(&project.name),  // ← sanitizado ✓
    platform                         // ← NÃO sanitizado ✗
);
```

**Vetor de ataque:** Chamada IPC com `platform = "../../etc/cron.d/malicious"` resultaria em:
```
output/books/meu-livro/../../etc/cron.d/malicious.pdf
```
O arquivo PDF seria escrito em `/etc/cron.d/malicious.pdf` (se o processo tiver permissão, improvável em desktop normal, mas possível com sudo ou em ambientes CI/CD).

**Fix proposto:**
```rust
// Validar platform contra allowlist ANTES de usar
fn validate_platform(platform: &str) -> Result<&str, AppError> {
    const ALLOWED: &[&str] = &["kdp", "kdp_print", "ingram_spark", "kobo", "generic"];
    if ALLOWED.contains(&platform) {
        Ok(platform)
    } else {
        Err(AppError::new("VAL_010", format!("Plataforma inválida: {}", platform)))
    }
}
```

**Também afeta:** `generate_pdf_ebook`, `generate_epub`, `generate_docx` (mesmos padrões de construção de path com platform não validado).

---

### V003 — `project_id` Não Validado como UUID em Operações de Path
**Severidade:** P2-ALTO | **CVSS Estimado:** 6.3 (Local, Path Traversal)

**Arquivos:**
- `src-tauri/src/services/font_service.rs:68,113,148`
- `src-tauri/src/services/preview_service.rs:322,419,437`
- `src-tauri/src/services/illustration_service.rs:343`

**Descrição:**
`project_id: String` recebido via IPC é usado diretamente em `Path::join()` sem validação UUID:

```rust
// font_service.rs:66-68
let dest_dir = app_data_dir
    .join("fonts")
    .join(project_id);  // ← sem validação de formato
```

Se `project_id = "../../../tmp"`, o path resultante escapa do diretório `fonts/`.

**Nota:** Em uso normal, `project_id` é um UUID gerado pelo app. Mas o IPC aceita qualquer string, e a validação só acontece no banco (se a query falhar, o path já pode ter sido construído).

**Fix proposto:**
```rust
fn validate_uuid(id: &str) -> Result<&str, String> {
    // UUID v4 format: 8-4-4-4-12 hex chars
    let uuid_re = regex::Regex::new(
        r"^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$"
    ).unwrap();
    if uuid_re.is_match(id) {
        Ok(id)
    } else {
        Err(format!("VAL_011: project_id inválido: {}", id))
    }
}
```

---

### V004 — Caminho de Saída Relativo em Geração de PDF
**Severidade:** P3-MEDIO

**Arquivo:** `src-tauri/src/services/pdf_print_service.rs:116`

**Descrição:**
```rust
let output_dir = format!("output/books/{}", sanitize_slug(&project.name));
```

O caminho `output/books/` é **relativo ao CWD do processo**, não ao diretório de dados do app (`$APPDATA`). Dependendo de como o app é iniciado, os PDFs podem ser gerados em locais inesperados.

**Fix proposto:** Usar o caminho absoluto do projeto armazenado no banco de dados (`bes_root_path`) ou o `$APPDATA` do Tauri como base.

---

### V005 — Cargo.lock Não Commitado
**Severidade:** P3-MEDIO

**Descrição:**
O arquivo `src-tauri/Cargo.lock` não existe no workspace. Isso impede:
1. Auditoria de dependências transitivas (`cargo audit`)
2. Builds reproduzíveis (versões transitive podem mudar entre builds)
3. Detecção de supply chain attacks em dependências indiretas

**Risco identificado:** Sem `Cargo.lock`, não é possível verificar se crates como `yaml-rust` (usado indiretamente por `syntect`) ou outras dependências transitivas possuem vulnerabilidades ativas.

**Fix proposto:** Commitar `Cargo.lock`. Para aplicações (não bibliotecas), o lock file **deve** ser versionado.

---

### V006 — Sem Validação de Magic Bytes em Upload de Fonte
**Severidade:** P3-MEDIO

**Arquivo:** `src-tauri/src/services/font_service.rs:43-63`

**Descrição:**
O upload de fonte valida apenas extensão (`.otf`/`.ttf`) e tamanho (≤ 10MB), mas não verifica os magic bytes do arquivo:
- OTF: `4F 54 54 4F`
- TTF: `00 01 00 00` ou `74 72 75 65`

Um arquivo com extensão `.otf` mas conteúdo malicioso (ex: ZIP com path traversal entries) poderia ser aceito. O limite de 10MB mitiga zip bombs básicos, mas não bomba poliglota.

**Fix proposto:**
```rust
fn validate_font_magic_bytes(source: &Path) -> Result<(), String> {
    let mut buf = [0u8; 4];
    let mut file = std::fs::File::open(source)
        .map_err(|e| format!("FS_001: {}", e))?;
    std::io::Read::read_exact(&mut file, &mut buf)
        .map_err(|_| "VAL_004: Arquivo muito pequeno para ser uma fonte válida".to_string())?;
    // OTF: OTTO, TTF: \0\1\0\0 or true
    let is_otf = &buf == b"OTTO";
    let is_ttf = buf == [0, 1, 0, 0] || &buf == b"true";
    if is_otf || is_ttf {
        Ok(())
    } else {
        Err("VAL_004: Magic bytes inválidos — arquivo não é OTF/TTF".to_string())
    }
}
```

---

### V007 — stdout do Sidecar Não Truncado
**Severidade:** P3-MEDIO

**Arquivo:** `src-tauri/src/services/sidecar_manager.rs:126-134`

**Descrição:**
O código trunca `stderr` a 100 chars (comentário `SEC-008`) para evitar vazamento de conteúdo do manuscrito nos logs. Porém, `stdout` não é truncado e é retornado completo para o chamador.

Se um sidecar (ex: Typst compilando com `#read()`) retornar conteúdo do arquivo lido em stdout (no caso de erros de compilação que ecoam o conteúdo), esse conteúdo seria propagado ao frontend sem filtro.

**Impacto combinado com V001:** Agrava o vetor de exfiltração — o conteúdo injetado poderia aparecer tanto no PDF quanto em mensagens de erro exibidas na UI.

---

### V008 — CSP com `'unsafe-inline'` para Estilos
**Severidade:** P4-BAIXO

**Arquivo:** `src-tauri/tauri.conf.json`

**Descrição:**
```json
"csp": "style-src 'self' 'unsafe-inline'"
```

`'unsafe-inline'` permite injeção de estilos via atributo `style=` em elementos HTML. Para um app Tauri local, o vetor é limitado (não há origem remota de CSS), mas é uma prática de hardening.

**Fix proposto:** Usar `nonce` ou `hash` se o SvelteKit inserir styles inline críticos; caso contrário, remover `'unsafe-inline'` e verificar que o app ainda funciona.

---

### V009 — Vulnerabilidades npm em DevDependencies (Impacto Nulo em Produção)
**Severidade:** P4-BAIXO

**Descrição:**
`npm audit` reporta 19 vulnerabilidades (6 high) **todas em devDependencies**:

| Pacote | CVE | Severity | Prod? |
|--------|-----|----------|-------|
| @stryker-mutator/util | GHSA-9j5q-479x-43g2 | HIGH (Prototype Pollution, CVSS 7.5) | NÃO |
| glob ≥ 10.2.0 < 10.5.0 | GHSA-5j98-mcp5-4vw2 | HIGH (Command Injection CLI, CVSS 7.5) | NÃO |
| tmp ≤ 0.2.3 | GHSA-52f5-9888-hmc6 | MODERATE (Symlink write) | NÃO |

**Impacto em produção:** ZERO — esses pacotes são ferramentas de teste/mutation testing (Stryker). O bundle Tauri gerado não os inclui.

**Ação:** Atualizar `@stryker-mutator/core` para 9.6.0+ (breaking change) quando conveniente.

---

---

### M1 — Sidecar Hijacking via PATH (BLOCKER — Codex)
**Severidade:** P0-BLOCKER (identificado por Codex)

**Arquivo:** `src-tauri/src/services/sidecar_manager.rs:29,77,82,94,98`

**Descrição:**
Os sidecars (`typst`, `gs`, `java`, `pandoc`) são invocados pelo **nome simples** via `Command::new("typst")`, o que resolve pelo `PATH` do sistema. Um atacante com escrita no PATH (ou CWD do processo) pode colocar um binário malicioso chamado `typst` que será executado com os mesmos privilégios do app.

Embora `tauri.conf.json` declare `shell:allow-execute` com escopo restrito aos sidecars bundled, o backend Rust usa `tokio::process::Command` **diretamente**, bypassando completamente as capabilities Tauri (que só se aplicam ao frontend JS/IPC).

**Attack vector:** Colocar `typst` malicioso em `~/bin/` ou CWD (se PATH inclui `.` no POSIX) → toda geração de PDF/Preview executa binário malicioso.

**Fix:** Usar caminho absoluto do bundle Tauri:
```rust
// Em vez de Command::new("typst")
// Usar o recurso bundled do Tauri:
let typst_path = app_handle.path().resource_dir()?.join("binaries/typst");
Command::new(typst_path).args(args)...
```

---

### M2 — Typst Injection Ativa em pdf_ebook e cover_service (BLOCKER — Codex)
**Severidade:** P0-BLOCKER (identificado por Codex)

**Arquivos:**
- `src-tauri/src/services/pdf_ebook_service.rs:175,185`
- `src-tauri/src/services/cover_service.rs:121,123,138,140,152-153`

**Descrição:**
A vulnerabilidade V001 (Typst injection) foi identificada apenas em `pdf_print_service.rs`, mas os **mesmos padrões inseguros** existem em outros dois serviços:

**pdf_ebook_service.rs:185 (idêntico ao V001):**
```rust
chapters_typ.push_str(line);  // ← SEM escape, mesmo payload de #read() funciona
```

**cover_service.rs:138,140 (campos title/author sem escape):**
```rust
#text(fill: rgb("{primary_color}"), size: 32pt, weight: "bold")[{title}]
#text(fill: rgb("{primary_color}"), size: 14pt)[{author}]
```
Um title como `#read("/home/user/.ssh/id_rsa")]` fecha o bloco Typst e executa o comando.

Corrigir apenas T001 deixa 2 vetores de Typst injection ativos → **o fix precisa cobrir os 3 serviços**.

---

### M3 — Capabilities Tauri Não Restringem o Backend Rust
**Severidade:** P1-CRITICO (identificado por Codex)

**Arquivo:** `src-tauri/capabilities/default.json`

**Descrição:**
As capabilities Tauri (`fs:allow-read-file`, `fs:allow-write-file` restritas a `$APPDATA/**`) se aplicam **apenas às chamadas IPC do frontend JavaScript**. O backend Rust usa `std::fs` e `tokio::process::Command` diretamente, sem restrições do sistema de capabilities.

Isso significa que o modelo de segurança documentado em `tauri.conf.json` cria uma **falsa sensação de confinamento**: o backend pode ler/escrever qualquer arquivo acessível ao processo, independente do que as capabilities declaram.

**Impacto:** Agrava todos os vetores de path traversal (V002, V003, M1, M2) — não há sandbox Tauri protegendo contra abuso no backend Rust.

**Mitigação:** Documentar explicitamente que as restrições de fs são frontend-only; adicionar validações `canonicalize` + `starts_with(base_dir)` em todo acesso a arquivo no backend.

---

## Attack Chains

### Chain 1: Manuscrito Malicioso → Typst Injection → Local File Read

```
1. Attacker cria/entrega manuscrito BES com payload:
   Conteúdo de capítulo: "#read(\"/home/usuario/.ssh/id_rsa\")"

2. Usuário importa o manuscrito e dispara `generate_pdf_print`

3. PdfPrintService.generate_typ_file() embute o payload SEM escape no .typ

4. Typst compiler executa o payload → lê ~/.ssh/id_rsa

5. O conteúdo da chave SSH é embeddado no PDF gerado

6. PDF é salvo em output/books/ — se compartilhado ou enviado, chave vaza
```

**Complexidade:** Baixa (1 arquivo de manuscrito malicioso)
**Impacto:** Leitura de arquivos locais dentro das permissões do processo

---

### Chain 2: PATH Poisoning → Sidecar Hijack → Execução Arbitrária

```
1. Atacante coloca binário malicioso "typst" em ~/bin/ (mais cedo no PATH)

2. Usuário abre o app e dispara qualquer geração (PDF/EPUB/Preview)

3. SidecarManager.spawn_process("typst", ...) executa ~/bin/typst malicioso

4. Binário malicioso tem acesso completo ao filesystem com permissões do usuário
```

**Complexidade:** Média (requer acesso ao filesystem do usuário — common em apps maliciosos de terceiros)
**Impacto:** RCE local com privilégios do usuário

---

### Chain 3: Manuscrito Malicioso → Typst Injection (3 vetores) → Exfiltração

```
1. Atacante entrega manuscrito com "#read('/home/user/.aws/credentials')"

2. Vítima pode acionar via: pdf_print OU pdf_ebook OU cover title/author
   (qualquer um dos 3 caminhos — fix parcial não resolve)

3. Dados sensíveis embeddados no PDF de saída

4. PDF compartilhado (upload KDP/IngramSpark) → dados exfiltrados
```

---

### Chain 4: IPC Malicioso → platform Injection → Path Traversal de Saída

```
1. Frontend modificado ou chamada IPC direta:
   invoke('generate_pdf_print', { projectId: '...', platform: '../../tmp/evil' })

2. PdfPrintService constrói: "output/books/{slug}/../../tmp/evil.pdf"

3. PDF é gerado no diretório /tmp/evil.pdf
```

**Complexidade:** Alta (requer acesso ao processo Tauri/frontend modificado)
**Impacto:** Escrita de PDF em locais inesperados — baixo impacto direto, mas relevante em pipelines CI/CD

---

## Headers de Segurança (Tauri CSP)

| Diretiva | Status | Nota |
|----------|--------|------|
| `default-src 'self'` | ✅ OK | Seguro |
| `script-src 'self'` | ✅ OK | Sem inline scripts |
| `style-src 'self' 'unsafe-inline'` | ⚠️ ATENÇÃO | unsafe-inline presente |
| `img-src 'self' asset: tauri: data:` | ✅ OK | Necessário para assets |
| `connect-src 'self' ipc: tauri:` | ✅ OK | Sem conexões externas |
| `frame-ancestors` | ℹ️ N/A | Desktop app |

## Dependências Vulneráveis

### JavaScript (devDependencies — sem impacto em produção)
```
19 vulnerabilities (6 low, 7 moderate, 6 high)
Todos em @stryker-mutator/* e dependências do glob/tmp
Fix: npm audit fix --force (breaking change para stryker 9.6.0)
```

### Rust (Cargo.lock ausente — auditoria incompleta)
```
cargo audit: NÃO EXECUTÁVEL (Cargo.lock não existe)
AÇÃO NECESSÁRIA: commitar Cargo.lock e executar cargo audit
```

## Fontes Pesquisadas

- https://github.com/advisories/GHSA-6q87-84jw-cjhp (CVE-2025-32388 SvelteKit)
- https://rustsec.org/advisories/RUSTSEC-2024-0363 (sqlx PostgreSQL)
- https://rustsec.org/advisories/RUSTSEC-2021-0026 (comrak)
- https://github.com/tauri-apps/tauri/security/advisories (Tauri CVEs)
- https://typst.app/docs/reference/foundations/sys/ (Typst capabilities)
- npm audit output (19 vulns, devDeps only)
