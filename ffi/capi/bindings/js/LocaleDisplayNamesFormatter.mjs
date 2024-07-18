// generated by diplomat-tool
import { DataError } from "./DataError.mjs"
import { DataProvider } from "./DataProvider.mjs"
import { DisplayNamesOptions } from "./DisplayNamesOptions.mjs"
import { Locale } from "./Locale.mjs"
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";


/** See the [Rust documentation for `LocaleDisplayNamesFormatter`](https://docs.rs/icu/latest/icu/displaynames/struct.LocaleDisplayNamesFormatter.html) for more information.
*/

const LocaleDisplayNamesFormatter_box_destroy_registry = new FinalizationRegistry((ptr) => {
    wasm.icu4x_LocaleDisplayNamesFormatter_destroy_mv1(ptr);
});
export class LocaleDisplayNamesFormatter {
    // Internal ptr reference:
    #ptr = null;

    // Lifetimes are only to keep dependencies alive.
    // Since JS won't garbage collect until there are no incoming edges.
    #selfEdge = [];
    
    
    constructor(ptr, selfEdge) {
        
        this.#ptr = ptr;
        this.#selfEdge = selfEdge;
        // Unconditionally register to destroy when this object is ready to garbage collect.
        LocaleDisplayNamesFormatter_box_destroy_registry.register(this, this.#ptr);
    }

    get ffiValue() {
        return this.#ptr;
    }


    static create(provider, locale, options) {
        
        let slice_cleanup_callbacks = [];
        
        const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
        const result = wasm.icu4x_LocaleDisplayNamesFormatter_create_v1_mv1(diplomat_receive_buffer, provider.ffiValue, locale.ffiValue, ...options._intoFFI(slice_cleanup_callbacks, {}));
    
        try {
    
            if (!diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4)) {
                const cause = DataError[Array.from(DataError.values.keys())[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)]];
                throw new Error('DataError: ' + cause.value, { cause });
            }
            return new LocaleDisplayNamesFormatter(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), []);
        } finally {
        
            for (let cleanup of slice_cleanup_callbacks) {
                cleanup();
            }
        
            wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        
        }
    }

    of(locale) {
        
        const write = wasm.diplomat_buffer_write_create(0);
        wasm.icu4x_LocaleDisplayNamesFormatter_of_mv1(this.ffiValue, locale.ffiValue, write);
    
        try {
    
            return diplomatRuntime.readString8(wasm, wasm.diplomat_buffer_write_get_bytes(write), wasm.diplomat_buffer_write_len(write));
        } finally {
        
            wasm.diplomat_buffer_write_destroy(write);
        
        }
    }

    

}