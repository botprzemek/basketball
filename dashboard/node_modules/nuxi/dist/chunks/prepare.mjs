import { d as defineCommand, c as consola } from '../shared/nuxi.6aad497e.mjs';
import 'node:util';
import 'node:path';
import 'node:process';
import 'node:tty';
import { l as loadKit, w as writeTypes } from '../shared/nuxi.a5b1f4d1.mjs';
import { a as clearBuildDir } from '../shared/nuxi.7cace88e.mjs';
import { s as sharedArgs, l as legacyRootDirArgs, r as resolve, a as relative } from '../shared/nuxi.610c92ff.mjs';
import 'node:url';
import 'node:fs';
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

const prepare = defineCommand({
  meta: {
    name: "prepare",
    description: "Prepare Nuxt for development/build"
  },
  args: {
    dotenv: {
      type: "string",
      description: "Path to .env file"
    },
    ...sharedArgs,
    ...legacyRootDirArgs
  },
  async run(ctx) {
    process.env.NODE_ENV = process.env.NODE_ENV || "production";
    const cwd = resolve(ctx.args.cwd || ctx.args.rootDir || ".");
    const {
      loadNuxt,
      buildNuxt,
      writeTypes: writeTypes$1 = writeTypes
    } = await loadKit(cwd);
    const nuxt = await loadNuxt({
      cwd,
      dotenv: {
        cwd,
        fileName: ctx.args.dotenv
      },
      overrides: {
        _prepare: true,
        logLevel: ctx.args.logLevel,
        ...ctx.data?.overrides
      }
    });
    await clearBuildDir(nuxt.options.buildDir);
    await buildNuxt(nuxt);
    await writeTypes$1(nuxt);
    consola.success(
      "Types generated in",
      relative(process.cwd(), nuxt.options.buildDir)
    );
  }
});

export { prepare as default };
