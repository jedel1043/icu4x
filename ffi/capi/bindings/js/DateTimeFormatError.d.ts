// generated by diplomat-tool
import type { pointer, codepoint } from "./diplomat-runtime.d.ts";

// Base enumerator definition
/** Additional information: [1](https://docs.rs/icu/latest/icu/datetime/enum.DateTimeWriteError.html)
*/
export class DateTimeFormatError {
    constructor(value : DateTimeFormatError | string);

    get value() : string;

    get ffiValue() : number;

    static Unknown : DateTimeFormatError;
    static MissingInputField : DateTimeFormatError;
    static ZoneInfoMissingFields : DateTimeFormatError;
    static InvalidEra : DateTimeFormatError;
    static InvalidMonthCode : DateTimeFormatError;
    static InvalidCyclicYear : DateTimeFormatError;
    static NamesNotLoaded : DateTimeFormatError;
    static FixedDecimalFormatterNotLoaded : DateTimeFormatError;
    static UnsupportedField : DateTimeFormatError;
}