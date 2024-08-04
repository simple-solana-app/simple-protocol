import 'dart:convert';
import 'dart:io';
import 'dart:typed_data';
import 'package:solana/solana.dart';

final programId = Ed25519HDPublicKey.fromBase58(
    '24x6XDgxxZgSzuAefWmx7WAppzBfgCSHtxAkDtpALbq1');
final rpcClient = RpcClient('https://api.devnet.solana.com');

void main() async {
  try {
    Ed25519HDKeyPair simple = await getSimpleAccount();
    print('simple pubkey: ${simple.publicKey.toBase58()}');
  } catch (e) {
    print('Error: $e');
  }
}

Future<Ed25519HDKeyPair> getSimpleAccount() async {
  final secretKeyString = await File('../../KEYS/simple.json').readAsString();
  final List<dynamic> secretKeyJson = jsonDecode(secretKeyString);

  final Uint8List fullKey = Uint8List.fromList(secretKeyJson.cast<int>());

  // Extract only the private key (first 32 bytes)
  final Uint8List privateKey = fullKey.sublist(0, 32);

  final account = await Ed25519HDKeyPair.fromPrivateKeyBytes(
    privateKey: privateKey,
  );

  return account;
}
