#ifndef Script_D_H
#define Script_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum Script {
  Script_Adlam = 167,
  Script_Ahom = 161,
  Script_AnatolianHieroglyphs = 156,
  Script_Arabic = 2,
  Script_Armenian = 3,
  Script_Avestan = 117,
  Script_Balinese = 62,
  Script_Bamum = 130,
  Script_BassaVah = 134,
  Script_Batak = 63,
  Script_Bengali = 4,
  Script_Bhaiksuki = 168,
  Script_Bopomofo = 5,
  Script_Brahmi = 65,
  Script_Braille = 46,
  Script_Buginese = 55,
  Script_Buhid = 44,
  Script_CanadianAboriginal = 40,
  Script_Carian = 104,
  Script_CaucasianAlbanian = 159,
  Script_Chakma = 118,
  Script_Cham = 66,
  Script_Cherokee = 6,
  Script_Chorasmian = 189,
  Script_Common = 0,
  Script_Coptic = 7,
  Script_Cuneiform = 101,
  Script_Cypriot = 47,
  Script_CyproMinoan = 193,
  Script_Cyrillic = 8,
  Script_Deseret = 9,
  Script_Devanagari = 10,
  Script_DivesAkuru = 190,
  Script_Dogra = 178,
  Script_Duployan = 135,
  Script_EgyptianHieroglyphs = 71,
  Script_Elbasan = 136,
  Script_Elymaic = 185,
  Script_Ethiopian = 11,
  Script_Georgian = 12,
  Script_Glagolitic = 56,
  Script_Gothic = 13,
  Script_Grantha = 137,
  Script_Greek = 14,
  Script_Gujarati = 15,
  Script_GunjalaGondi = 179,
  Script_Gurmukhi = 16,
  Script_Han = 17,
  Script_Hangul = 18,
  Script_HanifiRohingya = 182,
  Script_Hanunoo = 43,
  Script_Hatran = 162,
  Script_Hebrew = 19,
  Script_Hiragana = 20,
  Script_ImperialAramaic = 116,
  Script_Inherited = 1,
  Script_InscriptionalPahlavi = 122,
  Script_InscriptionalParthian = 125,
  Script_Javanese = 78,
  Script_Kaithi = 120,
  Script_Kannada = 21,
  Script_Katakana = 22,
  Script_Kawi = 198,
  Script_KayahLi = 79,
  Script_Kharoshthi = 57,
  Script_KhitanSmallScript = 191,
  Script_Khmer = 23,
  Script_Khojki = 157,
  Script_Khudawadi = 145,
  Script_Lao = 24,
  Script_Latin = 25,
  Script_Lepcha = 82,
  Script_Limbu = 48,
  Script_LinearA = 83,
  Script_LinearB = 49,
  Script_Lisu = 131,
  Script_Lycian = 107,
  Script_Lydian = 108,
  Script_Mahajani = 160,
  Script_Makasar = 180,
  Script_Malayalam = 26,
  Script_Mandaic = 84,
  Script_Manichaean = 121,
  Script_Marchen = 169,
  Script_MasaramGondi = 175,
  Script_Medefaidrin = 181,
  Script_MeeteiMayek = 115,
  Script_MendeKikakui = 140,
  Script_MeroiticCursive = 141,
  Script_MeroiticHieroglyphs = 86,
  Script_Miao = 92,
  Script_Modi = 163,
  Script_Mongolian = 27,
  Script_Mro = 149,
  Script_Multani = 164,
  Script_Myanmar = 28,
  Script_Nabataean = 143,
  Script_NagMundari = 199,
  Script_Nandinagari = 187,
  Script_Nastaliq = 200,
  Script_NewTaiLue = 59,
  Script_Newa = 170,
  Script_Nko = 87,
  Script_Nushu = 150,
  Script_NyiakengPuachueHmong = 186,
  Script_Ogham = 29,
  Script_OlChiki = 109,
  Script_OldHungarian = 76,
  Script_OldItalic = 30,
  Script_OldNorthArabian = 142,
  Script_OldPermic = 89,
  Script_OldPersian = 61,
  Script_OldSogdian = 184,
  Script_OldSouthArabian = 133,
  Script_OldTurkic = 88,
  Script_OldUyghur = 194,
  Script_Oriya = 31,
  Script_Osage = 171,
  Script_Osmanya = 50,
  Script_PahawhHmong = 75,
  Script_Palmyrene = 144,
  Script_PauCinHau = 165,
  Script_PhagsPa = 90,
  Script_Phoenician = 91,
  Script_PsalterPahlavi = 123,
  Script_Rejang = 110,
  Script_Runic = 32,
  Script_Samaritan = 126,
  Script_Saurashtra = 111,
  Script_Sharada = 151,
  Script_Shavian = 51,
  Script_Siddham = 166,
  Script_SignWriting = 112,
  Script_Sinhala = 33,
  Script_Sogdian = 183,
  Script_SoraSompeng = 152,
  Script_Soyombo = 176,
  Script_Sundanese = 113,
  Script_SylotiNagri = 58,
  Script_Syriac = 34,
  Script_Tagalog = 42,
  Script_Tagbanwa = 45,
  Script_TaiLe = 52,
  Script_TaiTham = 106,
  Script_TaiViet = 127,
  Script_Takri = 153,
  Script_Tamil = 35,
  Script_Tangsa = 195,
  Script_Tangut = 154,
  Script_Telugu = 36,
  Script_Thaana = 37,
  Script_Thai = 38,
  Script_Tibetan = 39,
  Script_Tifinagh = 60,
  Script_Tirhuta = 158,
  Script_Toto = 196,
  Script_Ugaritic = 53,
  Script_Unknown = 103,
  Script_Vai = 99,
  Script_Vithkuqi = 197,
  Script_Wancho = 188,
  Script_WarangCiti = 146,
  Script_Yezidi = 192,
  Script_Yi = 41,
  Script_ZanabazarSquare = 177,
} Script;

typedef struct Script_option {union { Script ok; }; bool is_ok; } Script_option;



#endif // Script_D_H
