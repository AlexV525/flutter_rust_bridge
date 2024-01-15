import 'dart:io';

import 'package:args/command_runner.dart';
import 'package:build_cli_annotations/build_cli_annotations.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/consts.dart';
import 'package:flutter_rust_bridge_internal/src/makefile_dart/release.dart';
import 'package:flutter_rust_bridge_internal/src/utils/makefile_dart_infra.dart';

part 'lint.g.dart';

List<Command<void>> createCommands() {
  return [
    SimpleConfigCommand(
        'lint', lint, _$populateLintConfigParser, _$parseLintConfigResult),
    SimpleConfigCommand('lint-rust', lintRust, _$populateLintConfigParser,
        _$parseLintConfigResult),
    SimpleConfigCommand('lint-dart', lintDart, _$populateLintConfigParser,
        _$parseLintConfigResult),
    SimpleCommand('lint-rust-feature-flag', lintRustFeatureFlag),
  ];
}

@CliOptions()
class LintConfig {
  @CliOption(defaultsTo: false)
  final bool fix;

  const LintConfig({
    required this.fix,
  });
}

Future<void> lint(LintConfig config) async {
  await lintRust(config);
  await lintDart(config);
}

Future<void> lintRust(LintConfig config) async {
  await lintRustFormat(config);
  await lintRustClippy(config);
}

Future<void> lintRustFormat(LintConfig config) async {
  for (final package in kRustPackages) {
    await exec('cargo +nightly fmt ${config.fix ? "" : "--check"}',
        relativePwd: package);
  }
}

Future<void> lintRustClippy(LintConfig config) async {
  for (final package in kRustPackages) {
    if (config.fix) {
      await exec('cargo fix --allow-dirty --allow-staged',
          relativePwd: package);
    }
    await exec(
        'cargo clippy ${config.fix ? "--fix --allow-dirty --allow-staged" : ""} -- -D warnings',
        relativePwd: package);
  }

  for (final package in kRustPackagesAllowWeb) {
    await exec(
        'cargo clippy --target wasm32-unknown-unknown ${config.fix ? "--fix --allow-dirty --allow-staged" : ""} -- -D warnings',
        relativePwd: package);
  }
}

Future<void> lintDart(LintConfig config) async {
  await lintDartVersion();
  await lintDartFormat(config);
  await lintDartAnalyze(config);
  await lintDartPana(config);
}

Future<void> lintDartVersion() async {
  final path = FrbDartCodeVersionInfo.kPath;
  final actualText = File(path).readAsStringSync();
  final matcher =
      FrbDartCodeVersionInfo.createCode(computeVersionInfo().oldVersion);
  if (!actualText.contains(matcher)) {
    throw Exception('$path should contain $matcher');
  }
}

Future<void> lintDartFormat(LintConfig config) async {
  for (final package in kDartPackages) {
    await exec('dart format ${config.fix ? "" : "--set-exit-if-changed"} .',
        relativePwd: package);
  }
}

Future<void> lintDartAnalyze(LintConfig config) async {
  for (final package in kDartPackages) {
    await runPubGetIfNotRunYet(package);
    await exec('dart ${config.fix ? "fix --apply" : "analyze --fatal-infos"}',
        relativePwd: package);
  }
}

Future<void> lintDartPana(LintConfig config) async {
  final pana = Platform.isWindows ? 'pana.bat' : 'pana';
  await exec('flutter pub global activate pana');
  await exec('$pana --no-warning --line-length 80 --exit-code-threshold 0',
      relativePwd: 'frb_dart');
}

Future<void> lintRustFeatureFlag() async {
  const package = 'frb_rust';
  for (final extra in [
    '',
    '--target wasm32-unknown-unknown',
  ]) {
    await exec('cargo hack check --each-feature --no-dev-deps $extra',
        relativePwd: package);
  }
}
