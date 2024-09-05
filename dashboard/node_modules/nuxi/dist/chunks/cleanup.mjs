import { d as defineCommand } from '../shared/nuxi.6aad497e.mjs';
import { c as cleanupNuxtDirs } from '../shared/nuxi.aef5ff85.mjs';
import { l as loadKit } from '../shared/nuxi.a5b1f4d1.mjs';
import { s as sharedArgs, l as legacyRootDirArgs, r as resolve } from '../shared/nuxi.610c92ff.mjs';
import 'node:util';
import 'node:path';
import 'node:process';
import 'node:tty';
import 'node:url';
import 'node:fs';
import '../shared/nuxi.7cace88e.mjs';
import 'node:perf_hooks';
import '../shared/nuxi.eaa29140.mjs';
import './satisfies.mjs';
import '../shared/nuxi.2155838d.mjs';
import '../shared/nuxi.b4f7d829.mjs';
import 'node:module';
import '../shared/nuxi.a6273a0c.mjs';
import 'node:assert';
import 'node:v8';
import '../shared/nuxi.88c35b8d.mjs';
import 'crypto';
import 'fs';
import 'module';
import 'path';
import 'perf_hooks';
import 'os';
import 'vm';
import 'url';
import 'assert';
import 'process';
import 'v8';
import 'util';
import 'tty';

const cleanup = defineCommand({
  meta: {
    name: "cleanup",
    description: "Clean up generated Nuxt files and caches"
  },
  args: {
    ...sharedArgs,
    ...legacyRootDirArgs
  },
  async run(ctx) {
    const cwd = resolve(ctx.args.cwd || ctx.args.rootDir || ".");
    const { loadNuxtConfig } = await loadKit(cwd);
    const nuxtOptions = await loadNuxtConfig({ cwd });
    await cleanupNuxtDirs(nuxtOptions.rootDir, nuxtOptions.buildDir);
  }
});

export { cleanup as default };
