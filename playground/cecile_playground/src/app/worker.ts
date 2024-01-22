import init, { run_cecile } from 'cecile_wasm/cecile_wasm';

onmessage = async (event) => {
  await init();
  run_cecile(event.data as string);
};
