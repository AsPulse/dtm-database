import { defineConfig } from 'orval';

export default defineConfig({
  'dtm-database-backend': {
    input: './../../../openapi.json',
    output: {
      target: './lib/backend.ts',
      client: 'swr',
    },
    hooks: {
      afterAllFilesWrite: [
        'prettier --write lib/backend.ts',
        'eslint --fix lib/backend.ts',
      ],
    },
  },
});
