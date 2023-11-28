// ignore_for_file: avoid_print

import 'dart:io';

import 'package:args/command_runner.dart';
import 'package:build_cli_annotations/build_cli_annotations.dart';
import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/generator.dart'
    as frb_example_pure_dart_generator;
import 'package:flutter_rust_bridge_internal/src/makefile_dart/consts.dart';
import 'package:flutter_rust_bridge_internal/src/utils/makefile_dart_infra.dart';
import 'package:path/path.dart' as path;

part 'generate.g.dart';

List<Command<void>> createCommands() {
  return [
    SimpleConfigCommand('generate-internal', generateInternal,
        _$populateGenerateConfigParser, _$parseGenerateConfigResult),
    SimpleConfigCommand(
        'generate-run-frb-codegen-command-generate',
        generateRunFrbCodegenCommandGenerate,
        _$populateGeneratePackageConfigParser,
        _$parseGeneratePackageConfigResult),
    SimpleConfigCommand(
        'generate-run-frb-codegen-command-integrate',
        generateRunFrbCodegenCommandIntegrate,
        _$populateGeneratePackageConfigParser,
        _$parseGeneratePackageConfigResult),
    // more detailed command, can be used to execute just a portion of the main command
    SimpleConfigCommand(
        'generate-internal-frb-example-pure-dart',
        generateInternalFrbExamplePureDart,
        _$populateGenerateConfigParser,
        _$parseGenerateConfigResult),
    SimpleConfigCommand('generate-internal-rust', generateInternalRust,
        _$populateGenerateConfigParser, _$parseGenerateConfigResult),
    SimpleConfigCommand(
        'generate-internal-dart-source',
        generateInternalDartSource,
        _$populateGenerateConfigParser,
        _$parseGenerateConfigResult),
  ];
}

@CliOptions()
class GenerateConfig {
  @CliOption(defaultsTo: false)
  final bool setExitIfChanged;

  const GenerateConfig({
    required this.setExitIfChanged,
  });
}

@CliOptions()
class GeneratePackageConfig implements GenerateConfig {
  @override
  @CliOption(defaultsTo: false)
  final bool setExitIfChanged;
  final String package;

  const GeneratePackageConfig({
    required this.setExitIfChanged,
    required this.package,
  });
}

Future<void> generateInternal(GenerateConfig config) async {
  await generateInternalFrbExamplePureDart(config);
  await generateInternalRust(config);
  await generateInternalBookHelp(config);
  await generateInternalDartSource(config);
  await generateInternalBuildRunner(config);
}

Future<void> generateInternalFrbExamplePureDart(GenerateConfig config) async {
  await _wrapMaybeSetExitIfChanged(config, () async {
    await frb_example_pure_dart_generator.generate();
  });
}

Future<void> generateInternalDartSource(GenerateConfig config) async {
  await _wrapMaybeSetExitIfChanged(config, () async {
    final path = randomTempDir();
    await exec('''
    #!/usr/bin/env bash
    set -eux
    mkdir -p $path && cd $path

    git clone --depth 1 --filter=blob:none --sparse --branch stable https://github.com/dart-lang/sdk.git
    (cd sdk && git sparse-checkout set runtime/include)
    cp -rf ./sdk/runtime/include/* ${exec.pwd}frb_rust/src/dart_api/
    rm -rf sdk
  ''');
  });
}

Future<void> generateInternalRust(GenerateConfig config) async {
  await _wrapMaybeSetExitIfChanged(config, () async {
    for (final package in kDartPackages) {
      await runDartPubGetIfNotRunYet(package);
    }

    await exec('cargo run -- internal-generate', relativePwd: 'frb_codegen');
  });
}

Future<void> generateInternalBookHelp(GenerateConfig config) async {
  await _wrapMaybeSetExitIfChanged(config, () async {
    for (final (cmd, extraArgs) in [
      ('', ''),
      ('generate', ''),
      ('create', ''),
      ('integrate', ''),
      ('build-web', '--dart-root ${exec.pwd}frb_example/pure_dart'),
    ]) {
      await exec(
          'cargo run -- $cmd $extraArgs --help > ${exec.pwd}book/src/generated/${cmd.isEmpty ? "main" : cmd}.txt',
          relativePwd: 'frb_codegen');
    }
  });
}

Future<void> generateInternalBuildRunner(GenerateConfig config) async {
  await _wrapMaybeSetExitIfChanged(config, () async {
    for (final package in kDartNonExamplePackages) {
      await runDartPubGetIfNotRunYet(package);
      await exec('dart run build_runner build --delete-conflicting-outputs',
          relativePwd: package);
    }
  });
}

Future<void> generateRunFrbCodegenCommandGenerate(
    GeneratePackageConfig config) async {
  await _wrapMaybeSetExitIfChanged(config, () async {
    await runDartPubGetIfNotRunYet(config.package);
    await _executeFrbCodegen(
      relativePwd: config.package,
      cmd: 'generate',
    );
  });
}

Future<void> generateRunFrbCodegenCommandIntegrate(
    GeneratePackageConfig config) async {
  await _wrapMaybeSetExitIfChanged(config, () async {
    final dirPackage = '${exec.pwd}/${config.package}';
    final dirTemp = randomTempDir();
    print('Pick temporary directory: $dirTemp');
    await Directory(dirTemp).create(recursive: true);

    // We move instead of delete folder for extra safety of this script
    final dirTempOriginal = path.join(dirTemp, 'original');
    await Directory(dirPackage).rename(dirTempOriginal);

    switch (config.package) {
      case 'frb_example/flutter_via_create':
        await _executeFrbCodegen(relativePwd: config.package, cmd: 'create');

      case 'frb_example/flutter_via_integrate':
        await exec('flutter create flutter_via_integrate',
            relativePwd: 'frb_example');
        await _executeFrbCodegen(relativePwd: config.package, cmd: 'integrate');

      default:
        throw Exception('Do not know how to handle package ${config.package}');
    }

    // move back compilation cache to speed up future usage
    for (final subPath in ['build', 'rust/target']) {
      await _renameDirIfExists(
          path.join(dirTempOriginal, subPath), path.join(dirPackage, subPath));
    }
  });
}

Future<void> _executeFrbCodegen({
  required String relativePwd,
  required String cmd,
}) async {
  await exec(
    'cargo run --manifest-path ${exec.pwd}/frb_codegen/Cargo.toml -- $cmd',
    relativePwd: relativePwd,
    extraEnv: {'RUST_BACKTRACE': '1'},
  );
}

Future<void> _renameDirIfExists(String src, String dst) async {
  if (!await Directory(src).exists()) return;
  await Directory(src).rename(dst);
}

Future<void> _wrapMaybeSetExitIfChanged(
    GenerateConfig config, Future<void> Function() inner) async {
  // Before actually executing anything, check whether git repository is already dirty
  await _maybeSetExitIfChanged(config);
  await inner();
  // The real check
  await _maybeSetExitIfChanged(config);
}

Future<void> _maybeSetExitIfChanged(GenerateConfig config) async {
  if (config.setExitIfChanged) {
    await exec('git diff --exit-code');
  }
}
