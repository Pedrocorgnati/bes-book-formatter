import { describe, it, expect, beforeEach, vi } from 'vitest';
import { t, setLocale, locale, initLocale } from './engine';
import { get } from 'svelte/store';

describe('i18n/engine', () => {
  beforeEach(() => {
    // Reset locale to pt-BR
    setLocale('pt-BR');
  });

  describe('t()', () => {
    it('should return a translation key in Portuguese', () => {
      const result = t('nav.dashboard');
      expect(result).toBeTruthy();
      expect(typeof result).toBe('string');
    });

    it('should support dot notation for nested keys', () => {
      const result = t('nav.editor');
      expect(result).toBeTruthy();
    });

    it('should return the key itself if translation not found (fallback)', () => {
      const result = t('non.existent.key');
      expect(result).toBe('non.existent.key');
    });

    it('should interpolate values in translations', () => {
      const result = t('some.key.with.{name}', { name: 'value' });
      expect(typeof result).toBe('string');
    });
  });

  describe('setLocale()', () => {
    it('should change the current locale', () => {
      setLocale('en-US');
      expect(get(locale)).toBe('en-US');
    });

    it('should support switching between locales', () => {
      setLocale('pt-BR');
      expect(get(locale)).toBe('pt-BR');

      setLocale('es-ES');
      expect(get(locale)).toBe('es-ES');

      setLocale('en-US');
      expect(get(locale)).toBe('en-US');
    });

    it('should persist locale to localStorage', () => {
      setLocale('en-US');
      expect(localStorage.setItem).toHaveBeenCalledWith('bes_language', 'en-US');
    });
  });

  describe('locale store', () => {
    it('should be a reactive store', () => {
      expect(locale.subscribe).toBeDefined();
    });

    it('should initialize with pt-BR', () => {
      expect(get(locale)).toBe('pt-BR');
    });
  });

  describe('initLocale()', () => {
    it('should initialize locale from localStorage if set', () => {
      vi.mocked(localStorage.getItem).mockReturnValue('en-US');
      initLocale();
      expect(get(locale)).toBe('en-US');
    });

    it('should fall back to pt-BR if localStorage value is invalid', () => {
      vi.mocked(localStorage.getItem).mockReturnValue('invalid-locale');
      initLocale();
      expect(get(locale)).toBe('pt-BR');
    });
  });
});
