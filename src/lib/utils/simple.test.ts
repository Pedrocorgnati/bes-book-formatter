import { describe, it, expect } from 'vitest';

// Testes simples para validar Stryker
describe('Simple Utils', () => {
  describe('add()', () => {
    function add(a: number, b: number): number {
      return a + b;
    }

    it('should add two positive numbers', () => {
      expect(add(2, 3)).toBe(5);
    });

    it('should handle zero', () => {
      expect(add(0, 5)).toBe(5);
    });

    it('should handle negative numbers', () => {
      expect(add(-2, 3)).toBe(1);
    });
  });

  describe('isEven()', () => {
    function isEven(n: number): boolean {
      return n % 2 === 0;
    }

    it('should return true for even numbers', () => {
      expect(isEven(2)).toBe(true);
      expect(isEven(4)).toBe(true);
    });

    it('should return false for odd numbers', () => {
      expect(isEven(1)).toBe(false);
      expect(isEven(3)).toBe(false);
    });

    it('should handle zero', () => {
      expect(isEven(0)).toBe(true);
    });
  });

  describe('clamp()', () => {
    function clamp(value: number, min: number, max: number): number {
      if (value < min) return min;
      if (value > max) return max;
      return value;
    }

    it('should clamp value below min', () => {
      expect(clamp(-1, 0, 10)).toBe(0);
    });

    it('should clamp value above max', () => {
      expect(clamp(11, 0, 10)).toBe(10);
    });

    it('should return value if within bounds', () => {
      expect(clamp(5, 0, 10)).toBe(5);
    });
  });
});
