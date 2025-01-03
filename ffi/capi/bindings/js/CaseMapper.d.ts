// generated by diplomat-tool
import type { CodePointSetBuilder } from "./CodePointSetBuilder"
import type { DataError } from "./DataError"
import type { DataProvider } from "./DataProvider"
import type { Locale } from "./Locale"
import type { TitlecaseOptions } from "./TitlecaseOptions"
import type { pointer, codepoint } from "./diplomat-runtime.d.ts";


/** See the [Rust documentation for `CaseMapper`](https://docs.rs/icu/latest/icu/casemap/struct.CaseMapper.html) for more information.
*/
export class CaseMapper {
    

    get ffiValue(): pointer;

    static create(): CaseMapper;

    static createWithProvider(provider: DataProvider): CaseMapper;

    lowercase(s: string, locale: Locale): string;

    uppercase(s: string, locale: Locale): string;

    titlecaseSegmentWithOnlyCaseData(s: string, locale: Locale, options: TitlecaseOptions): string;

    fold(s: string): string;

    foldTurkic(s: string): string;

    addCaseClosureTo(c: codepoint, builder: CodePointSetBuilder): void;

    simpleLowercase(ch: codepoint): codepoint;

    simpleUppercase(ch: codepoint): codepoint;

    simpleTitlecase(ch: codepoint): codepoint;

    simpleFold(ch: codepoint): codepoint;

    simpleFoldTurkic(ch: codepoint): codepoint;
}