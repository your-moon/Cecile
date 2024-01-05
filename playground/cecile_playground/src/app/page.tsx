import Image from 'next/image'
import wasm, { greet } from 'cecile_wasm';

export default async function Home() {
  return (<h1>a</h1>)
}

const ce_wasm = async () => {
    await wasm();
    return greet();
}

