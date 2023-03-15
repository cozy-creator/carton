import { CartonProvider } from "./provider";
import { Ed25519Keypair, fromB64 } from "@mysten/sui.js";

export * from "./provider";

// TODO: properly get the private key and node url
const keypair = Ed25519Keypair.fromSeed(fromB64(process.env.PRIVATE_KEY!).slice(1));
export const provider = new CartonProvider(process.env.NODE_URL!, keypair);
