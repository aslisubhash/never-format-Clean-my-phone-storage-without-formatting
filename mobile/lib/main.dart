import 'package:flutter/material.dart';
import 'companion_protocol.dart';

void main() {
  WidgetsFlutterBinding.ensureInitialized();
  runApp(const NeverFormatCompanionApp());
}

class NeverFormatCompanionApp extends StatelessWidget {
  const NeverFormatCompanionApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Never Format',
      debugShowCheckedModeBanner: false,
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(
          seedColor: const Color(0xFF1F6B4A),
          brightness: Brightness.light,
        ),
        useMaterial3: true,
      ),
      home: const CompanionHomePage(),
    );
  }
}

class CompanionHomePage extends StatefulWidget {
  const CompanionHomePage({super.key});

  @override
  State<CompanionHomePage> createState() => _CompanionHomePageState();
}

class _CompanionHomePageState extends State<CompanionHomePage> {
  final CompanionProtocol _protocol = CompanionProtocol();
  String _status = 'Waiting for PC connection…';
  List<String> _capabilities = const [];
  bool _storageGranted = false;

  Future<void> _requestStorage() async {
    setState(() {
      _storageGranted = true;
      _status = 'Storage access granted locally. PC remains the control center.';
    });
  }

  Future<void> _simulateHandshake() async {
    final isIos = Theme.of(context).platform == TargetPlatform.iOS;
    final response = _protocol.handshake(ios: isIos);
    setState(() {
      _capabilities = _protocol.capabilitiesForPlatform(ios: isIos);
      _status =
          'Handshake ready · protocol v${response['protocol_version']} · ${_capabilities.length} capabilities';
    });
  }

  @override
  Widget build(BuildContext context) {
    final isIos = Theme.of(context).platform == TargetPlatform.iOS;
    final caps = _capabilities.isEmpty
        ? _protocol.capabilitiesForPlatform(ios: isIos)
        : _capabilities;

    return Scaffold(
      backgroundColor: const Color(0xFFF3F6F1),
      body: SafeArea(
        child: ListView(
          padding: const EdgeInsets.all(24),
          children: [
            Text(
              'NEVER FORMAT',
              style: Theme.of(context).textTheme.labelSmall?.copyWith(
                    letterSpacing: 2.4,
                    color: const Color(0xFF1F6B4A),
                    fontWeight: FontWeight.w600,
                  ),
            ),
            const SizedBox(height: 8),
            Text(
              'Companion',
              style: Theme.of(context).textTheme.headlineLarge?.copyWith(
                    fontWeight: FontWeight.w600,
                    color: const Color(0xFF0F1C14),
                  ),
            ),
            const SizedBox(height: 12),
            Text(
              isIos
                  ? 'Photos & Files subset. The PC verifies before any delete.'
                  : 'Permissions & approved ops only. The PC is the control center.',
              style: Theme.of(context).textTheme.bodyLarge?.copyWith(
                    color: const Color(0xFF5A6B60),
                  ),
            ),
            const SizedBox(height: 28),
            Container(
              width: double.infinity,
              padding: const EdgeInsets.all(16),
              decoration: BoxDecoration(
                border: Border(
                  top: BorderSide(color: const Color(0xFF1F6B4A).withValues(alpha: 0.2)),
                ),
              ),
              child: Text(_status),
            ),
            const SizedBox(height: 16),
            Text('Capabilities', style: Theme.of(context).textTheme.titleMedium),
            const SizedBox(height: 8),
            Wrap(
              spacing: 8,
              runSpacing: 8,
              children: caps
                  .map(
                    (c) => Chip(
                      label: Text(c, style: const TextStyle(fontSize: 12)),
                      backgroundColor: const Color(0xFFE2EBE3),
                      side: BorderSide.none,
                    ),
                  )
                  .toList(),
            ),
            const SizedBox(height: 32),
            if (!_storageGranted)
              FilledButton(
                onPressed: _requestStorage,
                child: Text(isIos ? 'Allow Photos access' : 'Allow storage access'),
              ),
            const SizedBox(height: 12),
            OutlinedButton(
              onPressed: _simulateHandshake,
              child: const Text('Prepare PC handshake'),
            ),
            const SizedBox(height: 16),
            Text(
              'No internet permission. No analytics. No cloud.',
              style: Theme.of(context).textTheme.bodySmall?.copyWith(
                    color: const Color(0xFF5A6B60),
                  ),
            ),
          ],
        ),
      ),
    );
  }
}
