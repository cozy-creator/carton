import {
  JsonRpcProvider,
  RawSigner,
  Keypair,
  SuiAddress,
  normalizeSuiAddress,
  MoveCallTransaction,
} from "@mysten/sui.js";

export class CartonProvider {
  private _provider: JsonRpcProvider;
  private _signer: RawSigner;
  private _address: SuiAddress;

  constructor(url: string, keypair: Keypair) {
    this._provider = new JsonRpcProvider(url);
    this._signer = new RawSigner(keypair, this._provider);

    this._address = keypair.getPublicKey().toSuiAddress();
  }

  public getSuiProvider() {
    return this._provider;
  }

  public getSigner() {
    return this._signer;
  }

  public get address() {
    return normalizeSuiAddress(this._address);
  }

  async moveCall(transaction: MoveCallTransaction) {
    return await this._signer.executeMoveCall(transaction);
  }
}
