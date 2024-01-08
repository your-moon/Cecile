'use client';
import init, { greet}  from 'cecile_wasm';
import { run_cecile } from 'cecile_wasm/cecile_wasm';
import { useEffect } from 'react';

async function init_cecile() {
  await init();
console.log(run_cecile())
}

export default async function Playground() {
  useEffect(() => {
        init_cecile()
    })
  return (<h1>a</h1>)
}


