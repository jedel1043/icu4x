// generated by diplomat-tool
import { CalendarError } from "./CalendarError.mjs"
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";


/** An ICU4X Time object representing a time in terms of hour, minute, second, nanosecond
*
*See the [Rust documentation for `Time`](https://docs.rs/icu/latest/icu/calendar/struct.Time.html) for more information.
*/

const Time_box_destroy_registry = new FinalizationRegistry((ptr) => {
    wasm.icu4x_Time_destroy_mv1(ptr);
});
export class Time {
    // Internal ptr reference:
    #ptr = null;

    // Lifetimes are only to keep dependencies alive.
    // Since JS won't garbage collect until there are no incoming edges.
    #selfEdge = [];
    
    
    constructor(ptr, selfEdge) {
        
        this.#ptr = ptr;
        this.#selfEdge = selfEdge;
        // Unconditionally register to destroy when this object is ready to garbage collect.
        Time_box_destroy_registry.register(this, this.#ptr);
    }

    get ffiValue() {
        return this.#ptr;
    }


    static create(hour, minute, second, nanosecond) {
        
        const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
        const result = wasm.icu4x_Time_create_mv1(diplomat_receive_buffer, hour, minute, second, nanosecond);
    
        try {
    
            if (!diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4)) {
                const cause = CalendarError[Array.from(CalendarError.values.keys())[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)]];
                throw new Error('CalendarError: ' + cause.value, { cause });
            }
            return new Time(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), []);
        } finally {
        
            wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        
        }
    }

    static midnight() {
        
        const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
        const result = wasm.icu4x_Time_midnight_mv1(diplomat_receive_buffer);
    
        try {
    
            if (!diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4)) {
                const cause = CalendarError[Array.from(CalendarError.values.keys())[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)]];
                throw new Error('CalendarError: ' + cause.value, { cause });
            }
            return new Time(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), []);
        } finally {
        
            wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        
        }
    }

    get hour() {
        const result = wasm.icu4x_Time_hour_mv1(this.ffiValue);
    
        try {
    
            return result;
        } finally {
        
        }
    }

    get minute() {
        const result = wasm.icu4x_Time_minute_mv1(this.ffiValue);
    
        try {
    
            return result;
        } finally {
        
        }
    }

    get second() {
        const result = wasm.icu4x_Time_second_mv1(this.ffiValue);
    
        try {
    
            return result;
        } finally {
        
        }
    }

    get nanosecond() {
        const result = wasm.icu4x_Time_nanosecond_mv1(this.ffiValue);
    
        try {
    
            return result;
        } finally {
        
        }
    }

    

}