import fs from 'node:fs';

fs.mkdirSync('dist', { recursive: true });
fs.writeFileSync(
  'dist/index.html',
  '<!doctype html><meta charset="utf-8"><title>BlinkHost GRAFT canary</title><main><h1>GRAFT provider canary</h1><p>Trusted source, build, preview and release verification.</p></main>',
);

// BlinkHost GRAFT branch-preview certification
