/* Build alef component comilper to WASM package */

import { encode } from 'https://deno.land/std@0.83.0/encoding/base64.ts'
import { ensureDir } from 'https://deno.land/std@0.83.0/fs/ensure_dir.ts'

if (import.meta.main) {
    await ensureDir('./dist')
    Deno.chdir('./wasm')
    const p = Deno.run({
        cmd: ['wasm-pack', 'build', '--target', 'web'],
        stdout: 'inherit',
        stderr: 'inherit'
    })
    await p.status()
    p.close()
    const wasmData = await Deno.readFile('./pkg/alef_compiler_wasm_bg.wasm')
    const data64 = encode(wasmData)
    await Deno.writeTextFile(
        '../dist/wasm.js',
        [
            `const data = "${data64}";`,
            `export default () => Uint8Array.from(atob(data), v => v.charCodeAt(0))`
        ].join('\n')
    )
    await Deno.copyFile('./pkg/alef_compiler_wasm.js', '../dist/wasm-pack.js')
}
