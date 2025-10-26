const bip39 = require('bip39');
const { derivePath } = require('ed25519-hd-key');
const { Keypair } = require('stellar-sdk');

async function main() {
  const mnemonic = process.argv.slice(2).join(' ');
  if (!mnemonic) {
    console.error('Usage: node derive-stellar-key.js "<your mnemonic phrase here>"');
    process.exit(2);
  }
  if (!bip39.validateMnemonic(mnemonic)) {
    console.error('Invalid mnemonic!');
    process.exit(3);
  }

  // BIP39 -> seed
  const seed = bip39.mnemonicToSeedSync(mnemonic); // Buffer

  // Stellar standard derivation path
  const path = "m/44'/148'/0'";
  const { key } = derivePath(path, seed.toString('hex')); // key is Buffer (32 bytes)

  // Use stellar-sdk to create keypair from raw ed25519 seed
  const kp = Keypair.fromRawEd25519Seed(key);

  console.log('PUBLIC_KEY=' + kp.publicKey());
  console.log('SECRET_KEY=' + kp.secret());
  // optional: print derivation path used
  console.log('DERIVATION_PATH=' + path);
}

main().catch(e => { console.error(e); process.exit(1); });
