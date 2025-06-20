// Minimal ESLint v9+ config for React + TypeScript (flat config, plugins as object)
import js from '@eslint/js';
import tseslint from 'typescript-eslint';
import react from 'eslint-plugin-react';

export default [
  js.configs.recommended,
  {
    ignores: ['**/*.d.ts'],
    files: ['**/*.ts', '**/*.tsx'],
    languageOptions: {
      parser: tseslint.parser,
      parserOptions: {
        project: './tsconfig.json',
      },
    },
    plugins: {
      '@typescript-eslint': tseslint.plugin,
      react,
    },
    settings: {
      react: {
        version: 'detect',
      },
    },
    rules: {
      ...tseslint.configs.recommended[1].rules, // Use recommended TS rules
      ...react.configs.recommended.rules,      // Use recommended React rules
      // Add custom rules here if needed
    },
  },
]; 