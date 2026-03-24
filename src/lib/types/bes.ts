// ==========================================================
// BES Book Formatter — Tipos BES Integration (module-6)
// ==========================================================

// ---------------------
// Cache / Document types
// ---------------------

export type BesDocumentType =
  | 'bdd'
  | 'book_architecture'
  | 'metadata'
  | 'editorial_progress';

export interface CacheEntry {
  id: string;
  projectId: string;
  documentType: BesDocumentType;
  content: string;
  parsedJson?: Record<string, unknown>;
  filePath: string;
  fileHash: string; // SHA-256
  cachedAt: string; // ISO datetime
}

// ---------------------
// BES Document bundle
// ---------------------

export interface BesDocuments {
  bdd: string;               // Conteúdo bruto BDD.md
  bookArchitecture: string;  // Conteúdo BOOK-ARCHITECTURE.md
  metadata: BesMetadata;     // Parsed METADATA.yaml
  editorialProgress: string; // Conteúdo bruto EDITORIAL-PROGRESS.md
}

export interface BesMetadata {
  title: string;
  author: string;
  genre: string;
  isbn?: string;
  description?: string;
  keywords?: string[];
  language?: string;
  publisher?: string;
  publicationDate?: string;
}

export interface BesWorkspaceInfo {
  projectId: string;
  workspacePath: string;
  isValid: boolean;
  missingFiles: string[];
  detectedFiles: string[];
}

// ---------------------
// Editorial Progress (F1-F12)
// ---------------------

export type EditorialStatusType =
  | 'done'
  | 'in_progress'
  | 'pending'
  | 'blocked'
  | 'skipped';

export interface PhaseStatus {
  phaseId: string;      // 'F1' ... 'F12'
  phaseName: string;    // Nome descritivo
  status: EditorialStatusType;
  date?: string;        // Data de conclusão
  responsible?: string;
  notes?: string;
}

export interface EditorialProgress {
  projectName: string;
  phases: PhaseStatus[]; // 12 fases F1-F12
  lastUpdated: string;   // ISO datetime
}
