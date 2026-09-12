import 'package:flutter_test/flutter_test.dart';
import 'package:never_format_mobile/main.dart';

void main() {
  testWidgets('companion home renders brand', (WidgetTester tester) async {
    await tester.pumpWidget(const NeverFormatCompanionApp());
    expect(find.text('NEVER FORMAT'), findsOneWidget);
    expect(find.text('Companion'), findsOneWidget);
  });
}
