// ignore_for_file: avoid_print

import 'dart:convert';
import 'dart:io';

import 'package:args/command_runner.dart';

// We can import this internal implementation, since frb_internal itself is only an internal utility
// ignore: implementation_imports
import 'package:flutter_rust_bridge/src/cli/run_command.dart';
import 'package:puppeteer/puppeteer.dart';
import 'package:shelf/shelf.dart';
import 'package:shelf/shelf_io.dart';
import 'package:shelf_static/shelf_static.dart';
import 'package:shelf_web_socket/shelf_web_socket.dart';
import 'package:web_socket_channel/web_socket_channel.dart';

class ServeWebCommand extends Command<void> {
  @override
  String get name => 'serve-web';

  @override
  String get description => '';

  @override
  Future<void> run() async {
    await _runServer();
  }
}

final _kOpen = const {
      'linux': 'xdg-open',
      'macos': 'open',
      'windows': 'start',
    }[Platform.operatingSystem] ??
    'open';

/// {@macro flutter_rust_bridge.internal}
Future<void> _runServer() async {
  final ip = InternetAddress.anyIPv4;

  final staticFilesHandler = createStaticHandler(config.root, defaultDocument: 'index.html');
  Browser? browser;

  // TODO
  // Test helper.
  // final socketHandler = webSocketHandler((WebSocketChannel channel) async {
  //   await for (final mes in channel.stream) {
  //     try {
  //       final data = jsonDecode(mes);
  //       if (data is Map && data.containsKey('__result__')) {
  //         await browser?.close();
  //         exit(data['__result__'] ? 0 : 1);
  //       } else {
  //         print(data);
  //       }
  //     } catch (err, st) {
  //       print('$err\nStacktrace:\n$st');
  //     }
  //   }
  // });

  final shouldRelaxCoep = config.cliOpts.shouldRelaxCoep;
  final handler = const Pipeline().addMiddleware((handler) {
    return (req) async {
      final res = await handler(req);
      return res.change(headers: {
        'Cross-Origin-Opener-Policy': 'same-origin',
        'Cross-Origin-Embedder-Policy': shouldRelaxCoep ? 'credentialless' : 'require-corp',
      });
    };
  }).addHandler(Cascade()
      // .add(socketHandler) // TODO
      .add(staticFilesHandler)
      .handler);

  final portEnv = Platform.environment['PORT'];
  final port = portEnv == null ? config.cliOpts.port : int.parse(portEnv);
  final addr = 'http://localhost:$port';
  await serve(handler, ip, port);
  print('🦀 Server listening on $addr 🎯');

  if (config.cliOpts.runTests) {
    browser = await puppeteer.launch(
      headless: true,
      timeout: const Duration(minutes: 5),
    );
    final page = await browser.newPage();
    await page.goto(addr);
  } else if (config.cliOpts.open) {
    runCommand(_kOpen, [addr]);
  }
}
