// Astro integration that generates part of the API reference from source.
//
// It runs once when the site starts or builds, and again in `astro dev`
// whenever a watched source changes, so the reference grows with the code.
// A generator whose source does not exist yet is skipped, which lets the
// reference fill in phase by phase as each adapter lands.

import { spawn } from 'node:child_process';
import { existsSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

export const repoRoot = fileURLToPath(new URL('../../', import.meta.url));
export const docsRoot = fileURLToPath(new URL('../', import.meta.url));

/**
 * @param {object} options
 * @param {string} options.name      Label used in logs.
 * @param {string} options.source    Repo-relative path that must exist for the generator to run.
 * @param {string[]} options.watch   Repo-relative paths that trigger regeneration in dev.
 * @param {() => Promise<void>} options.generate
 */
export function apiReference({ name, source, watch, generate }) {
  const enabled = existsSync(path.join(repoRoot, source));
  const watched = watch.map((p) => path.join(repoRoot, p));

  const run = async (logger) => {
    const started = Date.now();
    try {
      await generate();
      logger.info(`generated in ${Date.now() - started}ms`);
    } catch (error) {
      logger.error(`generation failed: ${error.message}`);
      throw error;
    }
  };

  return {
    name: `api-reference:${name}`,
    hooks: {
      'astro:config:setup': async ({ command, logger }) => {
        if (command === 'preview') return;
        if (!enabled) {
          logger.info(`skipped: ${source} does not exist yet`);
          return;
        }
        await run(logger);
      },
      'astro:server:setup': ({ server, logger }) => {
        if (!enabled) return;
        let timer;
        server.watcher.add(watched);
        server.watcher.on('all', (_event, file) => {
          if (!watched.some((dir) => file.startsWith(dir))) return;
          clearTimeout(timer);
          timer = setTimeout(() => run(logger).catch(() => {}), 300);
        });
      },
    },
  };
}

/** Run a command from the repo root, rejecting on a non-zero exit. */
export function exec(command, args, env = {}) {
  return new Promise((resolve, reject) => {
    const child = spawn(command, args, {
      cwd: repoRoot,
      stdio: ['ignore', 'inherit', 'inherit'],
      env: { ...process.env, ...env },
    });
    child.on('error', reject);
    child.on('close', (code) =>
      code === 0 ? resolve() : reject(new Error(`${command} exited with code ${code}`)),
    );
  });
}
