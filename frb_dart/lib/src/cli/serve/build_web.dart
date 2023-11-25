import 'dart:convert';
import 'dart:io';

import 'package:flutter_rust_bridge/src/cli/cli_utils.dart';
import 'package:flutter_rust_bridge/src/cli/run_command.dart';
import 'package:flutter_rust_bridge/src/cli/serve/config.dart';
import 'package:path/path.dart' as p;

/// {@macro flutter_rust_bridge.internal}
Future<void> buildWeb(Config config) async {
  final crateDir = config.cliOpts.crate;

  final manifest = jsonDecode(await runCommand(
    'cargo',
    ['read-manifest'],
    pwd: crateDir,
    silent: true,
  ));

  final String crateName =
      (manifest['targets'] as List).firstWhere((target) => (target['kind'] as List).contains('cdylib'))['name'];
  if (crateName.isEmpty) bail('Crate name cannot be empty.');

  await runCommand('wasm-pack', [
    'build', '-t', 'no-modules', '-d', config.wasmOutput, '--no-typescript',
    '--out-name', crateName,
    if (!config.cliOpts.release) '--dev', crateDir,
    '--', // cargo build args
    '-Z', 'build-std=std,panic_abort',
    if (config.cliOpts.noDefaultFeatures) '--no-default-features',
    if (config.cliOpts.features != null) '--features=${config.cliOpts.features}'
  ], env: {
    'RUSTUP_TOOLCHAIN': 'nightly',
    'RUSTFLAGS': '-C target-feature=+atomics,+bulk-memory,+mutable-globals',
    if (stdout.supportsAnsiEscapes) 'CARGO_TERM_COLOR': 'always',
  });

  if (config.cliOpts.shouldRunBindgen) {
    await runCommand('wasm-bindgen', [
      '$crateDir/target/wasm32-unknown-unknown/${config.cliOpts.release ? 'release' : 'debug'}/$crateName.wasm',
      '--out-dir',
      config.wasmOutput,
      '--no-typescript',
      '--target',
      'no-modules',
      if (config.cliOpts.weakRefs) '--weak-refs',
      if (config.cliOpts.referenceTypes) '--reference-types',
    ]);
  }
 
  if (config.cliOpts.dartInput != null) {
    final output = p.basename(config.cliOpts.dartInput!);
    await runCommand('dart', [
      'compile',
      'js',
      '-o',
      '${config.root}/$output.js',
      if (config.cliOpts.release) '-O2',
      if (stdout.supportsAnsiEscapes) '--enable-diagnostic-colors',
      if (config.cliOpts.verbose) '--verbose',
      config.cliOpts.dartInput!,
    ]);
  } else {
    await runCommand(
      'flutter',
      ['build', 'web', if (!config.cliOpts.release) '--profile'] + config.restArgs,
    );
  }
}
