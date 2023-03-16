import {
  JsonRpcProvider,
  RawSigner,
  Keypair,
  SuiAddress,
  normalizeSuiAddress,
  Ed25519Keypair,
  fromB64,
} from "@mysten/sui.js";

export class Carton {
  #provider: JsonRpcProvider;
  #signer: RawSigner;
  #address: SuiAddress;

  constructor(url: string, keypair: Keypair) {
    this.#provider = new JsonRpcProvider(url);
    this.#signer = new RawSigner(keypair, this.#provider);

    this.#address = keypair.getPublicKey().toSuiAddress();
  }

  public get provider() {
    return this.#provider;
  }

  public get signer() {
    return this.#signer;
  }

  public get address() {
    return normalizeSuiAddress(this.#address);
  }
}

let carton: Carton;
const { NODE_URL, PRIVATE_KEY } = process.env;

if (NODE_URL && PRIVATE_KEY) {
  const keypair = Ed25519Keypair.fromSeed(fromB64(PRIVATE_KEY));
  carton = new Carton(NODE_URL, keypair);
}

export { carton };
