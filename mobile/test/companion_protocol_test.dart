import 'package:flutter_test/flutter_test.dart';
import 'package:never_format_mobile/companion_protocol.dart';

void main() {
  final protocol = CompanionProtocol();

  test('rejects factory reset', () {
    expect(protocol.isForbidden('FACTORY_RESET'), isTrue);
    expect(protocol.isAllowed('FACTORY_RESET', ios: false), isFalse);
  });

  test('allows android scan', () {
    expect(protocol.isAllowed('SCAN_STORAGE', ios: false), isTrue);
  });

  test('ios does not expose cache cleanup', () {
    expect(protocol.isAllowed('REQUEST_CACHE_CLEANUP', ios: true), isFalse);
    expect(protocol.isAllowed('LIST_PHOTOS', ios: true), isTrue);
  });

  test('handshake returns capabilities', () {
    final hs = protocol.handshake(ios: false);
    expect(hs['protocol_version'], 1);
    expect((hs['capabilities'] as List).isNotEmpty, isTrue);
  });
}
