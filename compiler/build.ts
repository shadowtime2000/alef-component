/* Build alef component comilper to WASM package */

import * as base64 from 'https://deno.land/std@0.80.0/encoding/base64.ts'

if (import.meta.main) {
    const p = Deno.run({
        cmd: ['wasm-pack', 'build', '--target', 'web'],
        stdout: 'inherit',
        stderr: 'inherit'
    })
    await p.status()
    p.close()
    const wasmData = await Deno.readFile('./pkg/alef_compiler_bg.wasm')
    const data64 = base64.encode(wasmData)
    await Deno.writeTextFile(
        './wasm.js',
        [
            `const data = "${data64}";`,
            `export default () => Uint8Array.from(atob(data), v => v.charCodeAt(0))`
        ].join('\n')
    )
    await Deno.copyFile('./pkg/alef_compiler.js', './wasm-pack.js')
}
