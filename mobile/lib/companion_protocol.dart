/// Allowlisted companion protocol — mirrors never-format-core::companion.
class CompanionProtocol {
  static const forbidden = <String>{
    'FORMAT_DEVICE',
    'WIPE_STORAGE',
    'FACTORY_RESET',
    'DELETE_ARBITRARY_PATH',
    'UNLOCK_BOOTLOADER',
    'CLEAR_APP_DATA',
    'SHELL',
    'EXEC',
  };

  static const androidOps = <String>[
    'SCAN_STORAGE',
    'LIST_FILES',
    'READ_FILE_METADATA',
    'COPY_FILE',
    'VERIFY_FILE',
    'REQUEST_CACHE_CLEANUP',
    'EXECUTE_APPROVED_CLEANUP',
    'GET_WHATSAPP_BACKUPS',
    'HANDSHAKE',
  ];

  static const iosOps = <String>[
    'LIST_PHOTOS',
    'EXPORT_PHOTO',
    'DELETE_PHOTO_AFTER_VERIFY',
    'HANDSHAKE',
  ];

  bool isForbidden(String op) {
    final upper = op.toUpperCase();
    return forbidden.any((f) => upper == f || upper.contains(f));
  }

  bool isAllowed(String op, {required bool ios}) {
    if (isForbidden(op)) return false;
    final list = ios ? iosOps : androidOps;
    return list.contains(op.toUpperCase());
  }

  Map<String, dynamic> handshake({bool ios = false}) {
    return {
      'protocol_version': 1,
      'platform': ios ? 'ios' : 'android',
      'capabilities': capabilitiesForPlatform(ios: ios),
      'app_version': '0.1.0',
    };
  }

  List<String> capabilitiesForPlatform({bool? ios}) {
    // Default to Android capability set; UI can pass TargetPlatform.
    if (ios == true) {
      return const [
        'list_photos',
        'export_photo',
        'delete_photo_after_verify',
      ];
    }
    return const [
      'scan_storage',
      'list_files',
      'read_file_metadata',
      'copy_file',
      'verify_file',
      'request_cache_cleanup',
      'execute_approved_cleanup',
      'get_whatsapp_backups',
    ];
  }

  Map<String, dynamic> dispatch(String op, Map<String, dynamic> args, {bool ios = false}) {
    if (!isAllowed(op, ios: ios)) {
      return {
        'ok': false,
        'error': 'rejected op: $op',
        'data': null,
      };
    }
    if (op.toUpperCase() == 'HANDSHAKE') {
      return {'ok': true, 'error': null, 'data': handshake(ios: ios)};
    }
    return {
      'ok': true,
      'error': null,
      'data': {'accepted': true, 'op': op, 'args': args},
    };
  }
}
