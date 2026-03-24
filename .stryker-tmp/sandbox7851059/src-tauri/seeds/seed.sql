-- =========================================================================
-- BES Book Formatter — Seed de Desenvolvimento
-- Gerado por /seed-data-create em 2026-03-22
-- =========================================================================
-- IDEMPOTENTE: INSERT OR IGNORE / INSERT OR REPLACE — executar N vezes é seguro
-- ORDEM: Nível 0 → Nível 1 (FK-safe)
-- Sem usuários: app desktop single-user, sem tabela de autenticação
-- =========================================================================

-- Ativar foreign keys (necessário no SQLite)
PRAGMA foreign_keys = ON;

-- =========================================================================
-- NÍVEL 0 — projects (sem FK externas)
-- Cobre: todos os completeness_level (blocking/warning/normal/NULL),
--        todas as BookLanguage (pt-BR/en-US/es-ES/it-IT),
--        géneros variados, config_version variadas
-- =========================================================================

-- [1] Nonfiction PT-BR — completamente processado, todos os campos preenchidos
INSERT OR IGNORE INTO projects (
    id, name, bes_root_path, book_config_path,
    genre, language, config_version,
    last_opened, format_file_path,
    completeness_score, completeness_level,
    chapter_count, illustration_count,
    manuscript_root, output_dir,
    created_at, updated_at
) VALUES (
    'proj-0001-nonfic-ptbr',
    'A Arte da Produtividade Profunda',
    '/home/dev/books/produtividade-profunda',
    '/home/dev/books/produtividade-profunda/book-config.json',
    'nonfiction', 'pt-BR', 'v3',
    datetime('now', '-2 hours'),
    '/home/dev/books/produtividade-profunda/.bes-format',
    0.92, 'normal',
    12, 4,
    '/home/dev/books/produtividade-profunda/manuscript',
    '/home/dev/books/produtividade-profunda/output',
    datetime('now', '-30 days'), datetime('now', '-2 hours')
);

-- [2] Fiction EN-US — completude warning (ilustrações pendentes)
INSERT OR IGNORE INTO projects (
    id, name, bes_root_path, book_config_path,
    genre, language, config_version,
    last_opened, format_file_path,
    completeness_score, completeness_level,
    chapter_count, illustration_count,
    manuscript_root, output_dir,
    created_at, updated_at
) VALUES (
    'proj-0002-fiction-enus',
    'The Last Algorithm',
    '/home/dev/books/the-last-algorithm',
    '/home/dev/books/the-last-algorithm/book-config.json',
    'fiction', 'en-US', 'v2',
    datetime('now', '-1 day'),
    '/home/dev/books/the-last-algorithm/.bes-format',
    0.65, 'warning',
    18, 2,
    '/home/dev/books/the-last-algorithm/manuscript',
    '/home/dev/books/the-last-algorithm/output',
    datetime('now', '-15 days'), datetime('now', '-1 day')
);

-- [3] Technical PT-BR — completude BLOCKING (capítulos críticos ausentes)
-- Edge case: sem output_dir configurado, sem format_file_path
INSERT OR IGNORE INTO projects (
    id, name, bes_root_path, book_config_path,
    genre, language, config_version,
    last_opened, format_file_path,
    completeness_score, completeness_level,
    chapter_count, illustration_count,
    manuscript_root, output_dir,
    created_at, updated_at
) VALUES (
    'proj-0003-technical-ptbr',
    'Arquitetura de Software Distribuído',
    '/home/dev/books/arquitetura-distribuida',
    '/home/dev/books/arquitetura-distribuida/book-config.json',
    'technical', 'pt-BR', 'v3',
    datetime('now', '-3 days'),
    NULL,
    0.20, 'blocking',
    5, 8,
    '/home/dev/books/arquitetura-distribuida/manuscript',
    NULL,
    datetime('now', '-7 days'), datetime('now', '-3 days')
);

-- [4] Academic ES-ES — quase completo, normal
INSERT OR IGNORE INTO projects (
    id, name, bes_root_path, book_config_path,
    genre, language, config_version,
    last_opened, format_file_path,
    completeness_score, completeness_level,
    chapter_count, illustration_count,
    manuscript_root, output_dir,
    created_at, updated_at
) VALUES (
    'proj-0004-academic-eses',
    'Lingüística Computacional Moderna',
    '/home/dev/books/linguistica-computacional',
    '/home/dev/books/linguistica-computacional/book-config.json',
    'academic', 'es-ES', 'v3',
    datetime('now', '-5 hours'),
    '/home/dev/books/linguistica-computacional/.bes-format',
    0.95, 'normal',
    8, 3,
    '/home/dev/books/linguistica-computacional/manuscript',
    '/home/dev/books/linguistica-computacional/output',
    datetime('now', '-20 days'), datetime('now', '-5 hours')
);

-- [5] Romance IT-IT — recém importado, sem completeness (NULL)
-- Edge case: projeto sem parse executado, sem campos de completude
INSERT OR IGNORE INTO projects (
    id, name, bes_root_path, book_config_path,
    genre, language, config_version,
    last_opened, format_file_path,
    completeness_score, completeness_level,
    chapter_count, illustration_count,
    manuscript_root, output_dir,
    created_at, updated_at
) VALUES (
    'proj-0005-romance-itit',
    'Il Cuore della Sera',
    '/home/dev/books/il-cuore-della-sera',
    '/home/dev/books/il-cuore-della-sera/book-config.json',
    'romance', 'it-IT', 'v1',
    datetime('now'),
    NULL,
    NULL, NULL,
    NULL, NULL,
    NULL, NULL,
    datetime('now'), datetime('now')
);

-- [6] Children EN-US — warning, muitas ilustrações, sem output_dir
INSERT OR IGNORE INTO projects (
    id, name, bes_root_path, book_config_path,
    genre, language, config_version,
    last_opened, format_file_path,
    completeness_score, completeness_level,
    chapter_count, illustration_count,
    manuscript_root, output_dir,
    created_at, updated_at
) VALUES (
    'proj-0006-children-enus',
    'The Adventures of Little Bug',
    '/home/dev/books/little-bug',
    '/home/dev/books/little-bug/book-config.json',
    'children', 'en-US', 'v2',
    datetime('now', '-12 hours'),
    NULL,
    0.75, 'warning',
    6, 15,
    '/home/dev/books/little-bug/manuscript',
    NULL,
    datetime('now', '-10 days'), datetime('now', '-12 hours')
);

-- [7] Business PT-BR — nunca aberto (edge case: last_opened NULL)
-- Edge case: sem book_config_path, sem config_version
INSERT OR IGNORE INTO projects (
    id, name, bes_root_path, book_config_path,
    genre, language, config_version,
    last_opened, format_file_path,
    completeness_score, completeness_level,
    chapter_count, illustration_count,
    manuscript_root, output_dir,
    created_at, updated_at
) VALUES (
    'proj-0007-business-ptbr',
    'Gestão de Equipes Remotas',
    '/home/dev/books/gestao-equipes',
    NULL,
    'business', 'pt-BR', NULL,
    NULL, NULL,
    NULL, NULL,
    NULL, NULL,
    NULL, NULL,
    datetime('now', '-1 day'), datetime('now', '-1 day')
);

-- [8] Self-help PT-BR — normal, score alto
INSERT OR IGNORE INTO projects (
    id, name, bes_root_path, book_config_path,
    genre, language, config_version,
    last_opened,
    completeness_score, completeness_level,
    chapter_count, illustration_count,
    manuscript_root, output_dir,
    created_at, updated_at
) VALUES (
    'proj-0008-selfhelp-ptbr',
    'Desperte o Líder Interior',
    '/home/dev/books/lider-interior',
    '/home/dev/books/lider-interior/book-config.json',
    'self_help', 'pt-BR', 'v2',
    datetime('now', '-2 days'),
    0.88, 'normal',
    10, 2,
    '/home/dev/books/lider-interior/manuscript',
    '/home/dev/books/lider-interior/output',
    datetime('now', '-45 days'), datetime('now', '-2 days')
);

-- [9] Poetry PT-BR — completo, 0 ilustrações
-- Edge case: illustration_count = 0 (sem placeholder no manuscrito)
INSERT OR IGNORE INTO projects (
    id, name, bes_root_path, book_config_path,
    genre, language, config_version,
    last_opened,
    completeness_score, completeness_level,
    chapter_count, illustration_count,
    manuscript_root, output_dir,
    created_at, updated_at
) VALUES (
    'proj-0009-poetry-ptbr',
    'Fragmentos de Luz',
    '/home/dev/books/fragmentos-luz',
    '/home/dev/books/fragmentos-luz/book-config.json',
    'poetry', 'pt-BR', 'v3',
    datetime('now', '-6 hours'),
    1.0, 'normal',
    3, 0,
    '/home/dev/books/fragmentos-luz/manuscript',
    '/home/dev/books/fragmentos-luz/output',
    datetime('now', '-5 days'), datetime('now', '-6 hours')
);

-- =========================================================================
-- NÍVEL 0 — user_preferences (singleton, idempotente)
-- Cobrir: theme (light/dark), ui_language (3 locales)
-- =========================================================================

INSERT OR REPLACE INTO user_preferences (key, value, updated_at) VALUES
    ('theme',            'dark',  datetime('now')),
    ('ui_language',      'pt-BR', datetime('now')),
    ('analytics_opt_in', 'false', datetime('now'));

-- =========================================================================
-- NÍVEL 1 — illustrations (FK → projects)
-- Cobre todos os estados: pending, imported, linked, error
-- Cobre color_space: srgb, cmyk, NULL
-- =========================================================================

-- Projeto 1 (nonfiction): todos os 4 estados
INSERT OR IGNORE INTO illustrations (
    id, project_id, placeholder_name, description,
    state, image_path, validated_dpi, alt_text,
    width_px, height_px, color_space,
    created_at, updated_at
) VALUES
-- linked sRGB: imagem validada, todos os metadados presentes
(
    'ill-0001', 'proj-0001-nonfic-ptbr',
    'fig-01-cover',
    'Capa do livro com fundo escuro e tipografia bold',
    'linked',
    '/home/dev/books/produtividade-profunda/assets/cover-final.jpg',
    300, 'Capa do livro A Arte da Produtividade Profunda',
    1800, 2700, 'srgb',
    datetime('now', '-25 days'), datetime('now', '-2 days')
),
-- linked CMYK: imagem para impressão profissional
(
    'ill-0002', 'proj-0001-nonfic-ptbr',
    'fig-02-grafico-focus',
    'Gráfico de distribuição de tempo de foco por semana',
    'linked',
    '/home/dev/books/produtividade-profunda/assets/grafico-focus.png',
    300, 'Gráfico de distribuição de tempo de foco semanal',
    2100, 1400, 'cmyk',
    datetime('now', '-20 days'), datetime('now', '-5 days')
),
-- imported: importada mas sem alt_text (validação incompleta)
(
    'ill-0003', 'proj-0001-nonfic-ptbr',
    'fig-03-metodo-pomodoro',
    'Diagrama do método Pomodoro adaptado',
    'imported',
    '/home/dev/books/produtividade-profunda/assets/pomodoro.png',
    150, NULL,
    800, 600, 'srgb',
    datetime('now', '-10 days'), datetime('now', '-1 day')
),
-- error: DPI abaixo do mínimo (72 dpi — erro de validação)
-- Cobre ERROR-CATALOG: ILLUSTRATION_003 (DPI insuficiente)
(
    'ill-0004', 'proj-0001-nonfic-ptbr',
    'fig-04-autor-foto',
    'Foto do autor para contracapa',
    'error',
    '/home/dev/books/produtividade-profunda/assets/autor-baixa-res.jpg',
    72, NULL,
    400, 300, 'srgb',
    datetime('now', '-5 days'), datetime('now', '-1 day')
);

-- Projeto 2 (fiction): todas pending — cobre cenário "geração bloqueada por ilustrações"
INSERT OR IGNORE INTO illustrations (
    id, project_id, placeholder_name, description,
    state, created_at, updated_at
) VALUES
(
    'ill-0005', 'proj-0002-fiction-enus',
    'fig-01-hero-portrait',
    'Portrait of protagonist Alex Chen in neural interface lab',
    'pending',
    datetime('now', '-14 days'), datetime('now', '-14 days')
),
(
    'ill-0006', 'proj-0002-fiction-enus',
    'fig-02-city-map',
    'Futuristic city map showing key locations in the story',
    'pending',
    datetime('now', '-14 days'), datetime('now', '-14 days')
);

-- Projeto 3 (technical/blocking): error + múltiplas pending
-- Cobre cenário VAL_003: DPI inválido + estado de erro
INSERT OR IGNORE INTO illustrations (
    id, project_id, placeholder_name, description,
    state, image_path, validated_dpi, color_space,
    created_at, updated_at
) VALUES
(
    'ill-0007', 'proj-0003-technical-ptbr',
    'fig-01-arquitetura-geral',
    'Diagrama de arquitetura do sistema distribuído',
    'error',
    '/home/dev/books/arquitetura-distribuida/assets/arq-geral-BAIXA.png',
    72, 'srgb',
    datetime('now', '-6 days'), datetime('now', '-3 days')
),
(
    'ill-0008', 'proj-0003-technical-ptbr',
    'fig-02-fluxo-mensagens',
    'Diagrama de fluxo de mensagens entre microserviços',
    'pending',
    NULL, NULL, NULL,
    datetime('now', '-6 days'), datetime('now', '-6 days')
),
(
    'ill-0009', 'proj-0003-technical-ptbr',
    'fig-03-benchmark-latencia',
    'Gráfico de benchmark de latência p99',
    'pending',
    NULL, NULL, NULL,
    datetime('now', '-6 days'), datetime('now', '-6 days')
);

-- Projeto 4 (academic): linked + imported
INSERT OR IGNORE INTO illustrations (
    id, project_id, placeholder_name, description,
    state, image_path, validated_dpi, alt_text,
    width_px, height_px, color_space,
    created_at, updated_at
) VALUES
(
    'ill-0010', 'proj-0004-academic-eses',
    'fig-01-arvore-dependencias',
    'Árbol de dependencias sintácticas',
    'linked',
    '/home/dev/books/linguistica-computacional/assets/arvore-dep.png',
    300, 'Árbol de análisis de dependencias sintácticas del corpus',
    1600, 1200, 'srgb',
    datetime('now', '-18 days'), datetime('now', '-8 days')
),
(
    'ill-0011', 'proj-0004-academic-eses',
    'fig-02-tabela-resultados',
    'Tabla comparativa de modelos de embeddings',
    'imported',
    '/home/dev/books/linguistica-computacional/assets/tabla-resultados.png',
    200, NULL,
    1200, 800, 'srgb',
    datetime('now', '-12 days'), datetime('now', '-5 days')
);

-- Projeto 6 (children): mix de pending e imported — edge case: alta contagem de ilustrações
INSERT OR IGNORE INTO illustrations (
    id, project_id, placeholder_name, description,
    state, image_path, validated_dpi, width_px, height_px, color_space,
    created_at, updated_at
) VALUES
(
    'ill-0012', 'proj-0006-children-enus',
    'fig-01-little-bug',
    'Personagem principal Little Bug na introdução',
    'pending',
    NULL, NULL, NULL, NULL, NULL,
    datetime('now', '-9 days'), datetime('now', '-9 days')
),
(
    'ill-0013', 'proj-0006-children-enus',
    'fig-02-garden',
    'Jardim mágico onde Bug mora',
    'pending',
    NULL, NULL, NULL, NULL, NULL,
    datetime('now', '-9 days'), datetime('now', '-9 days')
),
(
    'ill-0014', 'proj-0006-children-enus',
    'fig-03-raindrop',
    'Gota de chuva caindo na flor',
    'imported',
    '/home/dev/books/little-bug/assets/raindrop.png',
    96, 1024, 768, 'srgb',
    datetime('now', '-9 days'), datetime('now', '-8 days')
),
(
    'ill-0015', 'proj-0006-children-enus',
    'fig-04-friends',
    'Grupo de insetos amigos reunidos',
    'pending',
    NULL, NULL, NULL, NULL, NULL,
    datetime('now', '-9 days'), datetime('now', '-9 days')
);

-- =========================================================================
-- NÍVEL 1 — typography_configs (FK → projects, UNIQUE project_id)
-- Cobre: diferentes genre_presets, drop_cap_style, ornament_style,
--        chapter_start (odd/even/continuous), fonts variadas
-- =========================================================================

-- Projeto 1 (nonfiction): configuração editorial profissional
INSERT OR IGNORE INTO typography_configs (
    id, project_id,
    font_body, font_heading, font_code,
    font_size_body, font_size_h1, font_size_h2, font_size_h3,
    leading, paragraph_indent, tracking,
    kerning, justification, hyphenation, hyphenation_language,
    orphan_control, widow_control,
    drop_cap_style, ornament_style,
    baseline_grid, genre_preset,
    page_width, page_height,
    margin_top, margin_bottom, margin_inner, margin_outer,
    chapter_start,
    created_at, updated_at
) VALUES (
    'typo-0001', 'proj-0001-nonfic-ptbr',
    'EB Garamond', 'EB Garamond', NULL,
    11.0, 22.0, 18.0, 14.0,
    1.4, 1.5, 0.0,
    1, 1, 1, 'pt-BR',
    2, 2,
    'first_letter', 'vignette',
    12.0, 'nonfiction',
    6.0, 9.0,
    0.75, 0.75, 1.0, 0.75,
    'odd',
    datetime('now', '-25 days'), datetime('now', '-2 days')
);

-- Projeto 2 (fiction): capítulo em página ímpar, drop cap, ornamentos asteriscos
INSERT OR IGNORE INTO typography_configs (
    id, project_id,
    font_body, font_heading,
    font_size_body, leading, paragraph_indent,
    kerning, justification, hyphenation, hyphenation_language,
    drop_cap_style, ornament_style,
    genre_preset, chapter_start,
    page_width, page_height,
    created_at, updated_at
) VALUES (
    'typo-0002', 'proj-0002-fiction-enus',
    'Lora', 'Cinzel',
    12.0, 1.5, 1.8,
    1, 1, 1, 'en-US',
    'first_letter', 'asterisks',
    'fiction', 'odd',
    6.0, 9.0,
    datetime('now', '-14 days'), datetime('now', '-1 day')
);

-- Projeto 3 (technical): fonte de código, sem recuo, sem drop cap, capítulo contínuo
INSERT OR IGNORE INTO typography_configs (
    id, project_id,
    font_body, font_heading, font_code,
    font_size_body, leading, paragraph_indent,
    justification, hyphenation, hyphenation_language,
    drop_cap_style, ornament_style,
    genre_preset, chapter_start,
    created_at, updated_at
) VALUES (
    'typo-0003', 'proj-0003-technical-ptbr',
    'Source Serif 4', 'Source Sans 3', 'JetBrains Mono',
    10.5, 1.3, 0.0,
    0, 0, 'pt-BR',
    'none', 'none',
    'technical', 'continuous',
    datetime('now', '-6 days'), datetime('now', '-3 days')
);

-- Projeto 4 (academic): margens maiores, ornamento linha, capítulo em página par
INSERT OR IGNORE INTO typography_configs (
    id, project_id,
    font_body, font_heading,
    font_size_body, leading,
    margin_inner, margin_outer,
    drop_cap_style, ornament_style,
    genre_preset, chapter_start,
    hyphenation_language,
    created_at, updated_at
) VALUES (
    'typo-0004', 'proj-0004-academic-eses',
    'Palatino Linotype', 'Palatino Linotype',
    11.5, 1.45,
    1.25, 1.0,
    'none', 'line',
    'academic', 'even',
    'es-ES',
    datetime('now', '-18 days'), datetime('now', '-5 hours')
);

-- =========================================================================
-- NÍVEL 1 — generation_results (FK → projects)
-- Cobre todos os status: success, error, pending
-- Cobre formatos: epub3, pdf_print, pdf_ebook, docx
-- Cobre plataformas: kdp, kdp_print, apple_books, kobo, generic
-- =========================================================================

INSERT OR IGNORE INTO generation_results (
    id, project_id, format, platform,
    output_path, file_size_bytes, duration_ms,
    status, errors, warnings,
    created_at
) VALUES
-- success: EPUB para KDP
(
    'gen-0001', 'proj-0001-nonfic-ptbr',
    'epub3', 'kdp',
    '/home/dev/books/produtividade-profunda/output/produtividade-kdp.epub',
    524288, 8420,
    'success', '[]', '[]',
    datetime('now', '-2 hours')
),
-- success: PDF print para KDP Print (com warning de fonte)
(
    'gen-0002', 'proj-0001-nonfic-ptbr',
    'pdf_print', 'kdp_print',
    '/home/dev/books/produtividade-profunda/output/produtividade-print.pdf',
    3145728, 25600,
    'success', '[]',
    '["Fonte ''EB Garamond'' embedded com subconjunto de 234 glifos"]',
    datetime('now', '-2 hours')
),
-- error: EPUB Apple Books bloqueado por ilustrações pending
-- Cobre ERROR-CATALOG: ILLUSTRATION_001 (ilustração pendente na geração)
(
    'gen-0003', 'proj-0002-fiction-enus',
    'epub3', 'apple_books',
    NULL, NULL, 3200,
    'error',
    '["ILLUSTRATION_001: fig-01-hero-portrait em estado pending","ILLUSTRATION_001: fig-02-city-map em estado pending"]',
    '[]',
    datetime('now', '-1 day')
),
-- pending: geração em fila (projeto com completude blocking)
-- Cobre edge case: geração iniciada mas não concluída
(
    'gen-0004', 'proj-0003-technical-ptbr',
    'epub3', 'generic',
    NULL, NULL, NULL,
    'pending', '[]', '[]',
    datetime('now', '-3 days')
),
-- success: DOCX exportado (formato alternativo)
(
    'gen-0005', 'proj-0001-nonfic-ptbr',
    'docx', 'generic',
    '/home/dev/books/produtividade-profunda/output/produtividade.docx',
    892416, 4100,
    'success', '[]', '[]',
    datetime('now', '-10 days')
),
-- success: PDF ebook para Kobo (com warning de DPI)
(
    'gen-0006', 'proj-0004-academic-eses',
    'pdf_ebook', 'kobo',
    '/home/dev/books/linguistica-computacional/output/linguistica-kobo.pdf',
    2097152, 18000,
    'success', '[]',
    '["fig-02-tabela-resultados: DPI 200 abaixo do recomendado 300 para impressão"]',
    datetime('now', '-5 hours')
);

-- =========================================================================
-- NÍVEL 1 — annotations (FK → projects)
-- Cobre todos os tipos: comment, highlight, flag
-- =========================================================================

INSERT OR IGNORE INTO annotations (
    id, project_id, page_number, x_percent, y_percent,
    annotation_type, color, content,
    created_at, updated_at
) VALUES
-- comment: revisão editorial
(
    'ann-0001', 'proj-0001-nonfic-ptbr',
    1, 50.0, 15.0,
    'comment', '#FFC107',
    'Verificar se o título da capa está alinhado com a versão final aprovada pelo editor',
    datetime('now', '-1 day'), datetime('now', '-1 day')
),
-- highlight: referência positiva de tipografia
(
    'ann-0002', 'proj-0001-nonfic-ptbr',
    5, 30.0, 60.0,
    'highlight', '#4CAF50',
    'Este parágrafo tem espaçamento excelente — manter como referência',
    datetime('now', '-1 day'), datetime('now', '-1 day')
),
-- flag: órfão detectado — cobre US de detecção de problemas tipográficos
(
    'ann-0003', 'proj-0001-nonfic-ptbr',
    10, 75.0, 45.0,
    'flag', '#F44336',
    'Órfão detectado — ajustar espaçamento do capítulo anterior',
    datetime('now', '-20 hours'), datetime('now', '-20 hours')
),
-- comment: projeto em EN-US
(
    'ann-0004', 'proj-0002-fiction-enus',
    3, 20.0, 80.0,
    'comment', '#FFC107',
    'Check if chapter break matches the manuscript scene transition',
    datetime('now', '-12 hours'), datetime('now', '-12 hours')
);

-- =========================================================================
-- NÍVEL 1 — bes_document_cache (FK → projects, UNIQUE project_id+document_type)
-- Cobre todos os document_type: bdd, book_architecture, metadata, editorial_progress
-- =========================================================================

INSERT OR IGNORE INTO bes_document_cache (
    id, project_id, document_type,
    content, parsed_json,
    file_path, file_hash, cached_at
) VALUES
-- metadata: book-config.json cacheado
(
    'bdc-0001', 'proj-0001-nonfic-ptbr',
    'metadata',
    '{"title":"A Arte da Produtividade Profunda","author":"Pedro Corgnati","isbn":"978-85-99999-01-1","language":"pt-BR"}',
    '{"title":"A Arte da Produtividade Profunda","author":"Pedro Corgnati","isbn":"978-85-99999-01-1","language":"pt-BR"}',
    '/home/dev/books/produtividade-profunda/book-config.json',
    'sha256-aaaaaa1111111111111111111111111111111111111111111111111111111111',
    datetime('now', '-2 hours')
),
-- book_architecture: estrutura do manuscrito
(
    'bdc-0002', 'proj-0001-nonfic-ptbr',
    'book_architecture',
    '{"chapters":12,"parts":3,"wordCount":52000,"hasIndex":true,"hasToc":true}',
    '{"chapters":12,"parts":3,"wordCount":52000,"hasIndex":true,"hasToc":true}',
    '/home/dev/books/produtividade-profunda/manuscript/ARCHITECTURE.md',
    'sha256-bbbbbb2222222222222222222222222222222222222222222222222222222222',
    datetime('now', '-2 hours')
),
-- editorial_progress: progresso editorial
(
    'bdc-0003', 'proj-0001-nonfic-ptbr',
    'editorial_progress',
    '{"completedChapters":10,"pendingChapters":2,"reviewStatus":"in_review","lastEditDate":"2026-03-20"}',
    '{"completedChapters":10,"pendingChapters":2,"reviewStatus":"in_review","lastEditDate":"2026-03-20"}',
    '/home/dev/books/produtividade-profunda/manuscript/PROGRESS.md',
    'sha256-cccccc3333333333333333333333333333333333333333333333333333333333',
    datetime('now', '-3 days')
),
-- bdd: cenários de qualidade
(
    'bdc-0004', 'proj-0001-nonfic-ptbr',
    'bdd',
    '{"scenarios":45,"passed":42,"failed":3,"pending":0,"lastRun":"2026-03-21"}',
    '{"scenarios":45,"passed":42,"failed":3,"pending":0,"lastRun":"2026-03-21"}',
    '/home/dev/books/produtividade-profunda/manuscript/BDD.md',
    'sha256-dddddd4444444444444444444444444444444444444444444444444444444444',
    datetime('now', '-1 day')
),
-- metadata de projeto 2
(
    'bdc-0005', 'proj-0002-fiction-enus',
    'metadata',
    '{"title":"The Last Algorithm","author":"Pedro Corgnati","genre":"fiction","language":"en-US"}',
    '{"title":"The Last Algorithm","author":"Pedro Corgnati","genre":"fiction","language":"en-US"}',
    '/home/dev/books/the-last-algorithm/book-config.json',
    'sha256-eeeeee5555555555555555555555555555555555555555555555555555555555',
    datetime('now', '-1 day')
);

-- =========================================================================
-- NÍVEL 1 — cover_configs (FK → projects, UNIQUE project_id)
-- Cobre platform: amazon-kdp, ingram, generic
-- Cobre paper_type: white, cream
-- =========================================================================

INSERT OR IGNORE INTO cover_configs (
    id, project_id,
    template_id, genre, platform,
    title_override, subtitle, author_override, back_cover_text,
    primary_color, secondary_color, font_title, font_author,
    cover_image_path, cover_image_dpi,
    page_count, spine_width_mm, paper_type,
    created_at, updated_at
) VALUES
-- Projeto 1: capa profissional para KDP, papel branco
(
    'cov-0001', 'proj-0001-nonfic-ptbr',
    'professional', 'nonfiction', 'amazon-kdp',
    NULL,
    'Como Recuperar o Foco na Era das Distrações',
    NULL,
    'Um guia prático para profissionais de tecnologia que desejam recuperar o foco e a criatividade em meio às distrações do mundo digital.',
    '#1A2C3D', '#F5F0E8', 'Playfair Display', 'Lato',
    '/home/dev/books/produtividade-profunda/assets/cover-final.jpg', 300,
    248, 15.6, 'white',
    datetime('now', '-20 days'), datetime('now', '-2 days')
),
-- Projeto 4: capa acadêmica para Ingram, papel creme
(
    'cov-0002', 'proj-0004-academic-eses',
    'academic', 'academic', 'ingram',
    NULL, NULL, NULL,
    'Una introducción exhaustiva a los fundamentos de la lingüística computacional moderna.',
    '#2C3E50', '#FAFAF8', 'EB Garamond', 'EB Garamond',
    NULL, NULL,
    186, 11.8, 'cream',
    datetime('now', '-15 days'), datetime('now', '-5 hours')
),
-- Projeto 5: capa de romance para plataforma genérica
-- Edge case: page_count=0, spine_width=0.0 (projeto sem parse)
(
    'cov-0003', 'proj-0005-romance-itit',
    'elegant', 'romance', 'generic',
    NULL, NULL, NULL,
    'Una storia d''amore che attraversa il tempo e la distanza.',
    '#8B1A4A', '#FFF5F7', 'Cormorant Garamond', 'Lato',
    NULL, NULL,
    0, 0.0, 'cream',
    datetime('now'), datetime('now')
);

-- =========================================================================
-- FIM DO SEED
-- =========================================================================

SELECT 'Seed concluído.' AS status,
    (SELECT COUNT(*) FROM projects) AS total_projects,
    (SELECT COUNT(*) FROM illustrations) AS total_illustrations,
    (SELECT COUNT(*) FROM user_preferences) AS total_preferences,
    (SELECT COUNT(*) FROM typography_configs) AS total_typography_configs,
    (SELECT COUNT(*) FROM generation_results) AS total_generation_results,
    (SELECT COUNT(*) FROM annotations) AS total_annotations,
    (SELECT COUNT(*) FROM bes_document_cache) AS total_cache_entries,
    (SELECT COUNT(*) FROM cover_configs) AS total_cover_configs;
