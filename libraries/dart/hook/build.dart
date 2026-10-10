import 'dart:convert';
import 'dart:io';
import 'package:code_assets/code_assets.dart';
import 'package:crypto/crypto.dart';
import 'package:hooks/hooks.dart';

Future<void> main(List<String> args) => build(args, (input, output) async {
  if (!input.config.buildCodeAssets) return;
  final config = input.config.code;
  final pair = '${config.targetOS}_${config.targetArchitecture}';
  final definition = input.packageRoot.resolve('native-assets.json');
  output.dependencies.add(definition);
  final manifest =
      jsonDecode(await File.fromUri(definition).readAsString()) as Map;
  final asset = (manifest['assets'] as Map)[pair] as Map?;
  if (asset == null)
    throw UnsupportedError('ThinkThen has no native asset for $pair');
  final digest = asset['sha256'] as String?;
  final filename = asset['file'] as String;
  if (digest == null || !RegExp(r'^[a-f0-9]{64}$').hasMatch(digest)) {
    throw StateError('ThinkThen native asset has no approved checksum');
  }
  final owned = input.outputDirectoryShared.resolve('$digest/$filename');
  final file = File.fromUri(owned);
  final cache = input.userDefines.path('asset_cache');
  final source = cache == null
      ? file
      : File.fromUri(cache.resolve('$digest/$filename'));
  if (cache != null) output.dependencies.add(source.uri);
  List<int> bytes;
  if (await source.exists()) {
    bytes = await source.readAsBytes();
  } else {
    if (input.userDefines['offline'] == true) {
      throw StateError('ThinkThen native asset cache miss in offline build');
    }
    final location = asset['url'] as String?;
    if (location == null)
      throw StateError(
        'ThinkThen native asset has no approved download URL; use the matching development cache',
      );
    final uri = Uri.parse(location);
    if (uri.scheme != 'https' &&
        !(manifest['distribution'] == 'development-only' &&
            uri.scheme == 'http' &&
            uri.host == '127.0.0.1')) {
      throw StateError('ThinkThen native asset requires HTTPS');
    }
    final client = HttpClient();
    try {
      final request = await client.getUrl(uri);
      request.followRedirects = false;
      final response = await request.close();
      if (response.statusCode != HttpStatus.ok)
        throw HttpException(
          'ThinkThen native asset HTTP ${response.statusCode}',
          uri: uri,
        );
      bytes = await response.fold<List<int>>(
        [],
        (result, chunk) => result..addAll(chunk),
      );
    } finally {
      client.close(force: true);
    }
  }
  if (sha256.convert(bytes).toString() != digest) {
    throw StateError('ThinkThen native asset checksum mismatch');
  }
  await file.parent.create(recursive: true);
  await file.writeAsBytes(bytes, flush: true);
  output.assets.code.add(
    CodeAsset(
      package: input.packageName,
      name: 'thinkthen',
      linkMode: DynamicLoadingBundled(),
      file: owned,
    ),
  );
});
