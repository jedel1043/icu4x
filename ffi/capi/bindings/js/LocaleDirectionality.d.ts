// generated by diplomat-tool
import type { DataError } from "./DataError"
import type { DataProvider } from "./DataProvider"
import type { Locale } from "./Locale"
import type { LocaleDirection } from "./LocaleDirection"
import type { LocaleExpander } from "./LocaleExpander"
import type { pointer, codepoint } from "./diplomat-runtime.d.ts";


/** See the [Rust documentation for `LocaleDirectionality`](https://docs.rs/icu/latest/icu/locale/struct.LocaleDirectionality.html) for more information.
*/
export class LocaleDirectionality {
    

    get ffiValue(): pointer;

    static create(): LocaleDirectionality;

    static createWithProvider(provider: DataProvider): LocaleDirectionality;

    static createWithExpander(expander: LocaleExpander): LocaleDirectionality;

    static createWithExpanderAndProvider(provider: DataProvider, expander: LocaleExpander): LocaleDirectionality;

    get(locale: Locale): LocaleDirection;

    isLeftToRight(locale: Locale): boolean;

    isRightToLeft(locale: Locale): boolean;
}