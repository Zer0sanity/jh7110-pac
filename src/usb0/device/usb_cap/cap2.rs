#[doc = "Register `cap2` reader"]
pub type R = crate::R<Cap2Spec>;
#[doc = "Register `cap2` writer"]
pub type W = crate::W<Cap2Spec>;
#[doc = "The actual size of the connnected on-chip RAM memory in kB - 0: 256kB, 1-255: 1-255kB.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ActualMemSize {
    #[doc = "0: Actual supported memory size: 256kB"]
    Mem256kb = 0,
    #[doc = "1: Actual supported memory size: 1kB"]
    Mem1kb = 1,
    #[doc = "2: Actual supported memory size: 2kB"]
    Mem2kb = 2,
    #[doc = "3: Actual supported memory size: 3kB"]
    Mem3kb = 3,
    #[doc = "4: Actual supported memory size: 4kB"]
    Mem4kb = 4,
    #[doc = "5: Actual supported memory size: 5kB"]
    Mem5kb = 5,
    #[doc = "6: Actual supported memory size: 6kB"]
    Mem6kb = 6,
    #[doc = "7: Actual supported memory size: 7kB"]
    Mem7kb = 7,
    #[doc = "8: Actual supported memory size: 8kB"]
    Mem8kb = 8,
    #[doc = "9: Actual supported memory size: 9kB"]
    Mem9kb = 9,
    #[doc = "10: Actual supported memory size: 10kB"]
    Mem10kb = 10,
    #[doc = "11: Actual supported memory size: 11kB"]
    Mem11kb = 11,
    #[doc = "12: Actual supported memory size: 12kB"]
    Mem12kb = 12,
    #[doc = "13: Actual supported memory size: 13kB"]
    Mem13kb = 13,
    #[doc = "14: Actual supported memory size: 14kB"]
    Mem14kb = 14,
    #[doc = "15: Actual supported memory size: 15kB"]
    Mem15kb = 15,
    #[doc = "16: Actual supported memory size: 16kB"]
    Mem16kb = 16,
    #[doc = "17: Actual supported memory size: 17kB"]
    Mem17kb = 17,
    #[doc = "18: Actual supported memory size: 18kB"]
    Mem18kb = 18,
    #[doc = "19: Actual supported memory size: 19kB"]
    Mem19kb = 19,
    #[doc = "20: Actual supported memory size: 20kB"]
    Mem20kb = 20,
    #[doc = "21: Actual supported memory size: 21kB"]
    Mem21kb = 21,
    #[doc = "22: Actual supported memory size: 22kB"]
    Mem22kb = 22,
    #[doc = "23: Actual supported memory size: 23kB"]
    Mem23kb = 23,
    #[doc = "24: Actual supported memory size: 24kB"]
    Mem24kb = 24,
    #[doc = "25: Actual supported memory size: 25kB"]
    Mem25kb = 25,
    #[doc = "26: Actual supported memory size: 26kB"]
    Mem26kb = 26,
    #[doc = "27: Actual supported memory size: 27kB"]
    Mem27kb = 27,
    #[doc = "28: Actual supported memory size: 28kB"]
    Mem28kb = 28,
    #[doc = "29: Actual supported memory size: 29kB"]
    Mem29kb = 29,
    #[doc = "30: Actual supported memory size: 30kB"]
    Mem30kb = 30,
    #[doc = "31: Actual supported memory size: 31kB"]
    Mem31kb = 31,
    #[doc = "32: Actual supported memory size: 32kB"]
    Mem32kb = 32,
    #[doc = "33: Actual supported memory size: 33kB"]
    Mem33kb = 33,
    #[doc = "34: Actual supported memory size: 34kB"]
    Mem34kb = 34,
    #[doc = "35: Actual supported memory size: 35kB"]
    Mem35kb = 35,
    #[doc = "36: Actual supported memory size: 36kB"]
    Mem36kb = 36,
    #[doc = "37: Actual supported memory size: 37kB"]
    Mem37kb = 37,
    #[doc = "38: Actual supported memory size: 38kB"]
    Mem38kb = 38,
    #[doc = "39: Actual supported memory size: 39kB"]
    Mem39kb = 39,
    #[doc = "40: Actual supported memory size: 40kB"]
    Mem40kb = 40,
    #[doc = "41: Actual supported memory size: 41kB"]
    Mem41kb = 41,
    #[doc = "42: Actual supported memory size: 42kB"]
    Mem42kb = 42,
    #[doc = "43: Actual supported memory size: 43kB"]
    Mem43kb = 43,
    #[doc = "44: Actual supported memory size: 44kB"]
    Mem44kb = 44,
    #[doc = "45: Actual supported memory size: 45kB"]
    Mem45kb = 45,
    #[doc = "46: Actual supported memory size: 46kB"]
    Mem46kb = 46,
    #[doc = "47: Actual supported memory size: 47kB"]
    Mem47kb = 47,
    #[doc = "48: Actual supported memory size: 48kB"]
    Mem48kb = 48,
    #[doc = "49: Actual supported memory size: 49kB"]
    Mem49kb = 49,
    #[doc = "50: Actual supported memory size: 50kB"]
    Mem50kb = 50,
    #[doc = "51: Actual supported memory size: 51kB"]
    Mem51kb = 51,
    #[doc = "52: Actual supported memory size: 52kB"]
    Mem52kb = 52,
    #[doc = "53: Actual supported memory size: 53kB"]
    Mem53kb = 53,
    #[doc = "54: Actual supported memory size: 54kB"]
    Mem54kb = 54,
    #[doc = "55: Actual supported memory size: 55kB"]
    Mem55kb = 55,
    #[doc = "56: Actual supported memory size: 56kB"]
    Mem56kb = 56,
    #[doc = "57: Actual supported memory size: 57kB"]
    Mem57kb = 57,
    #[doc = "58: Actual supported memory size: 58kB"]
    Mem58kb = 58,
    #[doc = "59: Actual supported memory size: 59kB"]
    Mem59kb = 59,
    #[doc = "60: Actual supported memory size: 60kB"]
    Mem60kb = 60,
    #[doc = "61: Actual supported memory size: 61kB"]
    Mem61kb = 61,
    #[doc = "62: Actual supported memory size: 62kB"]
    Mem62kb = 62,
    #[doc = "63: Actual supported memory size: 63kB"]
    Mem63kb = 63,
    #[doc = "64: Actual supported memory size: 64kB"]
    Mem64kb = 64,
    #[doc = "65: Actual supported memory size: 65kB"]
    Mem65kb = 65,
    #[doc = "66: Actual supported memory size: 66kB"]
    Mem66kb = 66,
    #[doc = "67: Actual supported memory size: 67kB"]
    Mem67kb = 67,
    #[doc = "68: Actual supported memory size: 68kB"]
    Mem68kb = 68,
    #[doc = "69: Actual supported memory size: 69kB"]
    Mem69kb = 69,
    #[doc = "70: Actual supported memory size: 70kB"]
    Mem70kb = 70,
    #[doc = "71: Actual supported memory size: 71kB"]
    Mem71kb = 71,
    #[doc = "72: Actual supported memory size: 72kB"]
    Mem72kb = 72,
    #[doc = "73: Actual supported memory size: 73kB"]
    Mem73kb = 73,
    #[doc = "74: Actual supported memory size: 74kB"]
    Mem74kb = 74,
    #[doc = "75: Actual supported memory size: 75kB"]
    Mem75kb = 75,
    #[doc = "76: Actual supported memory size: 76kB"]
    Mem76kb = 76,
    #[doc = "77: Actual supported memory size: 77kB"]
    Mem77kb = 77,
    #[doc = "78: Actual supported memory size: 78kB"]
    Mem78kb = 78,
    #[doc = "79: Actual supported memory size: 79kB"]
    Mem79kb = 79,
    #[doc = "80: Actual supported memory size: 80kB"]
    Mem80kb = 80,
    #[doc = "81: Actual supported memory size: 81kB"]
    Mem81kb = 81,
    #[doc = "82: Actual supported memory size: 82kB"]
    Mem82kb = 82,
    #[doc = "83: Actual supported memory size: 83kB"]
    Mem83kb = 83,
    #[doc = "84: Actual supported memory size: 84kB"]
    Mem84kb = 84,
    #[doc = "85: Actual supported memory size: 85kB"]
    Mem85kb = 85,
    #[doc = "86: Actual supported memory size: 86kB"]
    Mem86kb = 86,
    #[doc = "87: Actual supported memory size: 87kB"]
    Mem87kb = 87,
    #[doc = "88: Actual supported memory size: 88kB"]
    Mem88kb = 88,
    #[doc = "89: Actual supported memory size: 89kB"]
    Mem89kb = 89,
    #[doc = "90: Actual supported memory size: 90kB"]
    Mem90kb = 90,
    #[doc = "91: Actual supported memory size: 91kB"]
    Mem91kb = 91,
    #[doc = "92: Actual supported memory size: 92kB"]
    Mem92kb = 92,
    #[doc = "93: Actual supported memory size: 93kB"]
    Mem93kb = 93,
    #[doc = "94: Actual supported memory size: 94kB"]
    Mem94kb = 94,
    #[doc = "95: Actual supported memory size: 95kB"]
    Mem95kb = 95,
    #[doc = "96: Actual supported memory size: 96kB"]
    Mem96kb = 96,
    #[doc = "97: Actual supported memory size: 97kB"]
    Mem97kb = 97,
    #[doc = "98: Actual supported memory size: 98kB"]
    Mem98kb = 98,
    #[doc = "99: Actual supported memory size: 99kB"]
    Mem99kb = 99,
    #[doc = "100: Actual supported memory size: 100kB"]
    Mem100kb = 100,
    #[doc = "101: Actual supported memory size: 101kB"]
    Mem101kb = 101,
    #[doc = "102: Actual supported memory size: 102kB"]
    Mem102kb = 102,
    #[doc = "103: Actual supported memory size: 103kB"]
    Mem103kb = 103,
    #[doc = "104: Actual supported memory size: 104kB"]
    Mem104kb = 104,
    #[doc = "105: Actual supported memory size: 105kB"]
    Mem105kb = 105,
    #[doc = "106: Actual supported memory size: 106kB"]
    Mem106kb = 106,
    #[doc = "107: Actual supported memory size: 107kB"]
    Mem107kb = 107,
    #[doc = "108: Actual supported memory size: 108kB"]
    Mem108kb = 108,
    #[doc = "109: Actual supported memory size: 109kB"]
    Mem109kb = 109,
    #[doc = "110: Actual supported memory size: 110kB"]
    Mem110kb = 110,
    #[doc = "111: Actual supported memory size: 111kB"]
    Mem111kb = 111,
    #[doc = "112: Actual supported memory size: 112kB"]
    Mem112kb = 112,
    #[doc = "113: Actual supported memory size: 113kB"]
    Mem113kb = 113,
    #[doc = "114: Actual supported memory size: 114kB"]
    Mem114kb = 114,
    #[doc = "115: Actual supported memory size: 115kB"]
    Mem115kb = 115,
    #[doc = "116: Actual supported memory size: 116kB"]
    Mem116kb = 116,
    #[doc = "117: Actual supported memory size: 117kB"]
    Mem117kb = 117,
    #[doc = "118: Actual supported memory size: 118kB"]
    Mem118kb = 118,
    #[doc = "119: Actual supported memory size: 119kB"]
    Mem119kb = 119,
    #[doc = "120: Actual supported memory size: 120kB"]
    Mem120kb = 120,
    #[doc = "121: Actual supported memory size: 121kB"]
    Mem121kb = 121,
    #[doc = "122: Actual supported memory size: 122kB"]
    Mem122kb = 122,
    #[doc = "123: Actual supported memory size: 123kB"]
    Mem123kb = 123,
    #[doc = "124: Actual supported memory size: 124kB"]
    Mem124kb = 124,
    #[doc = "125: Actual supported memory size: 125kB"]
    Mem125kb = 125,
    #[doc = "126: Actual supported memory size: 126kB"]
    Mem126kb = 126,
    #[doc = "127: Actual supported memory size: 127kB"]
    Mem127kb = 127,
    #[doc = "128: Actual supported memory size: 128kB"]
    Mem128kb = 128,
    #[doc = "129: Actual supported memory size: 129kB"]
    Mem129kb = 129,
    #[doc = "130: Actual supported memory size: 130kB"]
    Mem130kb = 130,
    #[doc = "131: Actual supported memory size: 131kB"]
    Mem131kb = 131,
    #[doc = "132: Actual supported memory size: 132kB"]
    Mem132kb = 132,
    #[doc = "133: Actual supported memory size: 133kB"]
    Mem133kb = 133,
    #[doc = "134: Actual supported memory size: 134kB"]
    Mem134kb = 134,
    #[doc = "135: Actual supported memory size: 135kB"]
    Mem135kb = 135,
    #[doc = "136: Actual supported memory size: 136kB"]
    Mem136kb = 136,
    #[doc = "137: Actual supported memory size: 137kB"]
    Mem137kb = 137,
    #[doc = "138: Actual supported memory size: 138kB"]
    Mem138kb = 138,
    #[doc = "139: Actual supported memory size: 139kB"]
    Mem139kb = 139,
    #[doc = "140: Actual supported memory size: 140kB"]
    Mem140kb = 140,
    #[doc = "141: Actual supported memory size: 141kB"]
    Mem141kb = 141,
    #[doc = "142: Actual supported memory size: 142kB"]
    Mem142kb = 142,
    #[doc = "143: Actual supported memory size: 143kB"]
    Mem143kb = 143,
    #[doc = "144: Actual supported memory size: 144kB"]
    Mem144kb = 144,
    #[doc = "145: Actual supported memory size: 145kB"]
    Mem145kb = 145,
    #[doc = "146: Actual supported memory size: 146kB"]
    Mem146kb = 146,
    #[doc = "147: Actual supported memory size: 147kB"]
    Mem147kb = 147,
    #[doc = "148: Actual supported memory size: 148kB"]
    Mem148kb = 148,
    #[doc = "149: Actual supported memory size: 149kB"]
    Mem149kb = 149,
    #[doc = "150: Actual supported memory size: 150kB"]
    Mem150kb = 150,
    #[doc = "151: Actual supported memory size: 151kB"]
    Mem151kb = 151,
    #[doc = "152: Actual supported memory size: 152kB"]
    Mem152kb = 152,
    #[doc = "153: Actual supported memory size: 153kB"]
    Mem153kb = 153,
    #[doc = "154: Actual supported memory size: 154kB"]
    Mem154kb = 154,
    #[doc = "155: Actual supported memory size: 155kB"]
    Mem155kb = 155,
    #[doc = "156: Actual supported memory size: 156kB"]
    Mem156kb = 156,
    #[doc = "157: Actual supported memory size: 157kB"]
    Mem157kb = 157,
    #[doc = "158: Actual supported memory size: 158kB"]
    Mem158kb = 158,
    #[doc = "159: Actual supported memory size: 159kB"]
    Mem159kb = 159,
    #[doc = "160: Actual supported memory size: 160kB"]
    Mem160kb = 160,
    #[doc = "161: Actual supported memory size: 161kB"]
    Mem161kb = 161,
    #[doc = "162: Actual supported memory size: 162kB"]
    Mem162kb = 162,
    #[doc = "163: Actual supported memory size: 163kB"]
    Mem163kb = 163,
    #[doc = "164: Actual supported memory size: 164kB"]
    Mem164kb = 164,
    #[doc = "165: Actual supported memory size: 165kB"]
    Mem165kb = 165,
    #[doc = "166: Actual supported memory size: 166kB"]
    Mem166kb = 166,
    #[doc = "167: Actual supported memory size: 167kB"]
    Mem167kb = 167,
    #[doc = "168: Actual supported memory size: 168kB"]
    Mem168kb = 168,
    #[doc = "169: Actual supported memory size: 169kB"]
    Mem169kb = 169,
    #[doc = "170: Actual supported memory size: 170kB"]
    Mem170kb = 170,
    #[doc = "171: Actual supported memory size: 171kB"]
    Mem171kb = 171,
    #[doc = "172: Actual supported memory size: 172kB"]
    Mem172kb = 172,
    #[doc = "173: Actual supported memory size: 173kB"]
    Mem173kb = 173,
    #[doc = "174: Actual supported memory size: 174kB"]
    Mem174kb = 174,
    #[doc = "175: Actual supported memory size: 175kB"]
    Mem175kb = 175,
    #[doc = "176: Actual supported memory size: 176kB"]
    Mem176kb = 176,
    #[doc = "177: Actual supported memory size: 177kB"]
    Mem177kb = 177,
    #[doc = "178: Actual supported memory size: 178kB"]
    Mem178kb = 178,
    #[doc = "179: Actual supported memory size: 179kB"]
    Mem179kb = 179,
    #[doc = "180: Actual supported memory size: 180kB"]
    Mem180kb = 180,
    #[doc = "181: Actual supported memory size: 181kB"]
    Mem181kb = 181,
    #[doc = "182: Actual supported memory size: 182kB"]
    Mem182kb = 182,
    #[doc = "183: Actual supported memory size: 183kB"]
    Mem183kb = 183,
    #[doc = "184: Actual supported memory size: 184kB"]
    Mem184kb = 184,
    #[doc = "185: Actual supported memory size: 185kB"]
    Mem185kb = 185,
    #[doc = "186: Actual supported memory size: 186kB"]
    Mem186kb = 186,
    #[doc = "187: Actual supported memory size: 187kB"]
    Mem187kb = 187,
    #[doc = "188: Actual supported memory size: 188kB"]
    Mem188kb = 188,
    #[doc = "189: Actual supported memory size: 189kB"]
    Mem189kb = 189,
    #[doc = "190: Actual supported memory size: 190kB"]
    Mem190kb = 190,
    #[doc = "191: Actual supported memory size: 191kB"]
    Mem191kb = 191,
    #[doc = "192: Actual supported memory size: 192kB"]
    Mem192kb = 192,
    #[doc = "193: Actual supported memory size: 193kB"]
    Mem193kb = 193,
    #[doc = "194: Actual supported memory size: 194kB"]
    Mem194kb = 194,
    #[doc = "195: Actual supported memory size: 195kB"]
    Mem195kb = 195,
    #[doc = "196: Actual supported memory size: 196kB"]
    Mem196kb = 196,
    #[doc = "197: Actual supported memory size: 197kB"]
    Mem197kb = 197,
    #[doc = "198: Actual supported memory size: 198kB"]
    Mem198kb = 198,
    #[doc = "199: Actual supported memory size: 199kB"]
    Mem199kb = 199,
    #[doc = "200: Actual supported memory size: 200kB"]
    Mem200kb = 200,
    #[doc = "201: Actual supported memory size: 201kB"]
    Mem201kb = 201,
    #[doc = "202: Actual supported memory size: 202kB"]
    Mem202kb = 202,
    #[doc = "203: Actual supported memory size: 203kB"]
    Mem203kb = 203,
    #[doc = "204: Actual supported memory size: 204kB"]
    Mem204kb = 204,
    #[doc = "205: Actual supported memory size: 205kB"]
    Mem205kb = 205,
    #[doc = "206: Actual supported memory size: 206kB"]
    Mem206kb = 206,
    #[doc = "207: Actual supported memory size: 207kB"]
    Mem207kb = 207,
    #[doc = "208: Actual supported memory size: 208kB"]
    Mem208kb = 208,
    #[doc = "209: Actual supported memory size: 209kB"]
    Mem209kb = 209,
    #[doc = "210: Actual supported memory size: 210kB"]
    Mem210kb = 210,
    #[doc = "211: Actual supported memory size: 211kB"]
    Mem211kb = 211,
    #[doc = "212: Actual supported memory size: 212kB"]
    Mem212kb = 212,
    #[doc = "213: Actual supported memory size: 213kB"]
    Mem213kb = 213,
    #[doc = "214: Actual supported memory size: 214kB"]
    Mem214kb = 214,
    #[doc = "215: Actual supported memory size: 215kB"]
    Mem215kb = 215,
    #[doc = "216: Actual supported memory size: 216kB"]
    Mem216kb = 216,
    #[doc = "217: Actual supported memory size: 217kB"]
    Mem217kb = 217,
    #[doc = "218: Actual supported memory size: 218kB"]
    Mem218kb = 218,
    #[doc = "219: Actual supported memory size: 219kB"]
    Mem219kb = 219,
    #[doc = "220: Actual supported memory size: 220kB"]
    Mem220kb = 220,
    #[doc = "221: Actual supported memory size: 221kB"]
    Mem221kb = 221,
    #[doc = "222: Actual supported memory size: 222kB"]
    Mem222kb = 222,
    #[doc = "223: Actual supported memory size: 223kB"]
    Mem223kb = 223,
    #[doc = "224: Actual supported memory size: 224kB"]
    Mem224kb = 224,
    #[doc = "225: Actual supported memory size: 225kB"]
    Mem225kb = 225,
    #[doc = "226: Actual supported memory size: 226kB"]
    Mem226kb = 226,
    #[doc = "227: Actual supported memory size: 227kB"]
    Mem227kb = 227,
    #[doc = "228: Actual supported memory size: 228kB"]
    Mem228kb = 228,
    #[doc = "229: Actual supported memory size: 229kB"]
    Mem229kb = 229,
    #[doc = "230: Actual supported memory size: 230kB"]
    Mem230kb = 230,
    #[doc = "231: Actual supported memory size: 231kB"]
    Mem231kb = 231,
    #[doc = "232: Actual supported memory size: 232kB"]
    Mem232kb = 232,
    #[doc = "233: Actual supported memory size: 233kB"]
    Mem233kb = 233,
    #[doc = "234: Actual supported memory size: 234kB"]
    Mem234kb = 234,
    #[doc = "235: Actual supported memory size: 235kB"]
    Mem235kb = 235,
    #[doc = "236: Actual supported memory size: 236kB"]
    Mem236kb = 236,
    #[doc = "237: Actual supported memory size: 237kB"]
    Mem237kb = 237,
    #[doc = "238: Actual supported memory size: 238kB"]
    Mem238kb = 238,
    #[doc = "239: Actual supported memory size: 239kB"]
    Mem239kb = 239,
    #[doc = "240: Actual supported memory size: 240kB"]
    Mem240kb = 240,
    #[doc = "241: Actual supported memory size: 241kB"]
    Mem241kb = 241,
    #[doc = "242: Actual supported memory size: 242kB"]
    Mem242kb = 242,
    #[doc = "243: Actual supported memory size: 243kB"]
    Mem243kb = 243,
    #[doc = "244: Actual supported memory size: 244kB"]
    Mem244kb = 244,
    #[doc = "245: Actual supported memory size: 245kB"]
    Mem245kb = 245,
    #[doc = "246: Actual supported memory size: 246kB"]
    Mem246kb = 246,
    #[doc = "247: Actual supported memory size: 247kB"]
    Mem247kb = 247,
    #[doc = "248: Actual supported memory size: 248kB"]
    Mem248kb = 248,
    #[doc = "249: Actual supported memory size: 249kB"]
    Mem249kb = 249,
    #[doc = "250: Actual supported memory size: 250kB"]
    Mem250kb = 250,
    #[doc = "251: Actual supported memory size: 251kB"]
    Mem251kb = 251,
    #[doc = "252: Actual supported memory size: 252kB"]
    Mem252kb = 252,
    #[doc = "253: Actual supported memory size: 253kB"]
    Mem253kb = 253,
    #[doc = "254: Actual supported memory size: 254kB"]
    Mem254kb = 254,
    #[doc = "255: Actual supported memory size: 255kB"]
    Mem255kb = 255,
}
impl From<ActualMemSize> for u8 {
    #[inline(always)]
    fn from(variant: ActualMemSize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ActualMemSize {
    type Ux = u8;
}
#[doc = "Field `actual_mem_size` reader - The actual size of the connnected on-chip RAM memory in kB - 0: 256kB, 1-255: 1-255kB."]
pub type ActualMemSizeR = crate::FieldReader<ActualMemSize>;
impl ActualMemSizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ActualMemSize {
        match self.bits {
            0 => ActualMemSize::Mem256kb,
            1 => ActualMemSize::Mem1kb,
            2 => ActualMemSize::Mem2kb,
            3 => ActualMemSize::Mem3kb,
            4 => ActualMemSize::Mem4kb,
            5 => ActualMemSize::Mem5kb,
            6 => ActualMemSize::Mem6kb,
            7 => ActualMemSize::Mem7kb,
            8 => ActualMemSize::Mem8kb,
            9 => ActualMemSize::Mem9kb,
            10 => ActualMemSize::Mem10kb,
            11 => ActualMemSize::Mem11kb,
            12 => ActualMemSize::Mem12kb,
            13 => ActualMemSize::Mem13kb,
            14 => ActualMemSize::Mem14kb,
            15 => ActualMemSize::Mem15kb,
            16 => ActualMemSize::Mem16kb,
            17 => ActualMemSize::Mem17kb,
            18 => ActualMemSize::Mem18kb,
            19 => ActualMemSize::Mem19kb,
            20 => ActualMemSize::Mem20kb,
            21 => ActualMemSize::Mem21kb,
            22 => ActualMemSize::Mem22kb,
            23 => ActualMemSize::Mem23kb,
            24 => ActualMemSize::Mem24kb,
            25 => ActualMemSize::Mem25kb,
            26 => ActualMemSize::Mem26kb,
            27 => ActualMemSize::Mem27kb,
            28 => ActualMemSize::Mem28kb,
            29 => ActualMemSize::Mem29kb,
            30 => ActualMemSize::Mem30kb,
            31 => ActualMemSize::Mem31kb,
            32 => ActualMemSize::Mem32kb,
            33 => ActualMemSize::Mem33kb,
            34 => ActualMemSize::Mem34kb,
            35 => ActualMemSize::Mem35kb,
            36 => ActualMemSize::Mem36kb,
            37 => ActualMemSize::Mem37kb,
            38 => ActualMemSize::Mem38kb,
            39 => ActualMemSize::Mem39kb,
            40 => ActualMemSize::Mem40kb,
            41 => ActualMemSize::Mem41kb,
            42 => ActualMemSize::Mem42kb,
            43 => ActualMemSize::Mem43kb,
            44 => ActualMemSize::Mem44kb,
            45 => ActualMemSize::Mem45kb,
            46 => ActualMemSize::Mem46kb,
            47 => ActualMemSize::Mem47kb,
            48 => ActualMemSize::Mem48kb,
            49 => ActualMemSize::Mem49kb,
            50 => ActualMemSize::Mem50kb,
            51 => ActualMemSize::Mem51kb,
            52 => ActualMemSize::Mem52kb,
            53 => ActualMemSize::Mem53kb,
            54 => ActualMemSize::Mem54kb,
            55 => ActualMemSize::Mem55kb,
            56 => ActualMemSize::Mem56kb,
            57 => ActualMemSize::Mem57kb,
            58 => ActualMemSize::Mem58kb,
            59 => ActualMemSize::Mem59kb,
            60 => ActualMemSize::Mem60kb,
            61 => ActualMemSize::Mem61kb,
            62 => ActualMemSize::Mem62kb,
            63 => ActualMemSize::Mem63kb,
            64 => ActualMemSize::Mem64kb,
            65 => ActualMemSize::Mem65kb,
            66 => ActualMemSize::Mem66kb,
            67 => ActualMemSize::Mem67kb,
            68 => ActualMemSize::Mem68kb,
            69 => ActualMemSize::Mem69kb,
            70 => ActualMemSize::Mem70kb,
            71 => ActualMemSize::Mem71kb,
            72 => ActualMemSize::Mem72kb,
            73 => ActualMemSize::Mem73kb,
            74 => ActualMemSize::Mem74kb,
            75 => ActualMemSize::Mem75kb,
            76 => ActualMemSize::Mem76kb,
            77 => ActualMemSize::Mem77kb,
            78 => ActualMemSize::Mem78kb,
            79 => ActualMemSize::Mem79kb,
            80 => ActualMemSize::Mem80kb,
            81 => ActualMemSize::Mem81kb,
            82 => ActualMemSize::Mem82kb,
            83 => ActualMemSize::Mem83kb,
            84 => ActualMemSize::Mem84kb,
            85 => ActualMemSize::Mem85kb,
            86 => ActualMemSize::Mem86kb,
            87 => ActualMemSize::Mem87kb,
            88 => ActualMemSize::Mem88kb,
            89 => ActualMemSize::Mem89kb,
            90 => ActualMemSize::Mem90kb,
            91 => ActualMemSize::Mem91kb,
            92 => ActualMemSize::Mem92kb,
            93 => ActualMemSize::Mem93kb,
            94 => ActualMemSize::Mem94kb,
            95 => ActualMemSize::Mem95kb,
            96 => ActualMemSize::Mem96kb,
            97 => ActualMemSize::Mem97kb,
            98 => ActualMemSize::Mem98kb,
            99 => ActualMemSize::Mem99kb,
            100 => ActualMemSize::Mem100kb,
            101 => ActualMemSize::Mem101kb,
            102 => ActualMemSize::Mem102kb,
            103 => ActualMemSize::Mem103kb,
            104 => ActualMemSize::Mem104kb,
            105 => ActualMemSize::Mem105kb,
            106 => ActualMemSize::Mem106kb,
            107 => ActualMemSize::Mem107kb,
            108 => ActualMemSize::Mem108kb,
            109 => ActualMemSize::Mem109kb,
            110 => ActualMemSize::Mem110kb,
            111 => ActualMemSize::Mem111kb,
            112 => ActualMemSize::Mem112kb,
            113 => ActualMemSize::Mem113kb,
            114 => ActualMemSize::Mem114kb,
            115 => ActualMemSize::Mem115kb,
            116 => ActualMemSize::Mem116kb,
            117 => ActualMemSize::Mem117kb,
            118 => ActualMemSize::Mem118kb,
            119 => ActualMemSize::Mem119kb,
            120 => ActualMemSize::Mem120kb,
            121 => ActualMemSize::Mem121kb,
            122 => ActualMemSize::Mem122kb,
            123 => ActualMemSize::Mem123kb,
            124 => ActualMemSize::Mem124kb,
            125 => ActualMemSize::Mem125kb,
            126 => ActualMemSize::Mem126kb,
            127 => ActualMemSize::Mem127kb,
            128 => ActualMemSize::Mem128kb,
            129 => ActualMemSize::Mem129kb,
            130 => ActualMemSize::Mem130kb,
            131 => ActualMemSize::Mem131kb,
            132 => ActualMemSize::Mem132kb,
            133 => ActualMemSize::Mem133kb,
            134 => ActualMemSize::Mem134kb,
            135 => ActualMemSize::Mem135kb,
            136 => ActualMemSize::Mem136kb,
            137 => ActualMemSize::Mem137kb,
            138 => ActualMemSize::Mem138kb,
            139 => ActualMemSize::Mem139kb,
            140 => ActualMemSize::Mem140kb,
            141 => ActualMemSize::Mem141kb,
            142 => ActualMemSize::Mem142kb,
            143 => ActualMemSize::Mem143kb,
            144 => ActualMemSize::Mem144kb,
            145 => ActualMemSize::Mem145kb,
            146 => ActualMemSize::Mem146kb,
            147 => ActualMemSize::Mem147kb,
            148 => ActualMemSize::Mem148kb,
            149 => ActualMemSize::Mem149kb,
            150 => ActualMemSize::Mem150kb,
            151 => ActualMemSize::Mem151kb,
            152 => ActualMemSize::Mem152kb,
            153 => ActualMemSize::Mem153kb,
            154 => ActualMemSize::Mem154kb,
            155 => ActualMemSize::Mem155kb,
            156 => ActualMemSize::Mem156kb,
            157 => ActualMemSize::Mem157kb,
            158 => ActualMemSize::Mem158kb,
            159 => ActualMemSize::Mem159kb,
            160 => ActualMemSize::Mem160kb,
            161 => ActualMemSize::Mem161kb,
            162 => ActualMemSize::Mem162kb,
            163 => ActualMemSize::Mem163kb,
            164 => ActualMemSize::Mem164kb,
            165 => ActualMemSize::Mem165kb,
            166 => ActualMemSize::Mem166kb,
            167 => ActualMemSize::Mem167kb,
            168 => ActualMemSize::Mem168kb,
            169 => ActualMemSize::Mem169kb,
            170 => ActualMemSize::Mem170kb,
            171 => ActualMemSize::Mem171kb,
            172 => ActualMemSize::Mem172kb,
            173 => ActualMemSize::Mem173kb,
            174 => ActualMemSize::Mem174kb,
            175 => ActualMemSize::Mem175kb,
            176 => ActualMemSize::Mem176kb,
            177 => ActualMemSize::Mem177kb,
            178 => ActualMemSize::Mem178kb,
            179 => ActualMemSize::Mem179kb,
            180 => ActualMemSize::Mem180kb,
            181 => ActualMemSize::Mem181kb,
            182 => ActualMemSize::Mem182kb,
            183 => ActualMemSize::Mem183kb,
            184 => ActualMemSize::Mem184kb,
            185 => ActualMemSize::Mem185kb,
            186 => ActualMemSize::Mem186kb,
            187 => ActualMemSize::Mem187kb,
            188 => ActualMemSize::Mem188kb,
            189 => ActualMemSize::Mem189kb,
            190 => ActualMemSize::Mem190kb,
            191 => ActualMemSize::Mem191kb,
            192 => ActualMemSize::Mem192kb,
            193 => ActualMemSize::Mem193kb,
            194 => ActualMemSize::Mem194kb,
            195 => ActualMemSize::Mem195kb,
            196 => ActualMemSize::Mem196kb,
            197 => ActualMemSize::Mem197kb,
            198 => ActualMemSize::Mem198kb,
            199 => ActualMemSize::Mem199kb,
            200 => ActualMemSize::Mem200kb,
            201 => ActualMemSize::Mem201kb,
            202 => ActualMemSize::Mem202kb,
            203 => ActualMemSize::Mem203kb,
            204 => ActualMemSize::Mem204kb,
            205 => ActualMemSize::Mem205kb,
            206 => ActualMemSize::Mem206kb,
            207 => ActualMemSize::Mem207kb,
            208 => ActualMemSize::Mem208kb,
            209 => ActualMemSize::Mem209kb,
            210 => ActualMemSize::Mem210kb,
            211 => ActualMemSize::Mem211kb,
            212 => ActualMemSize::Mem212kb,
            213 => ActualMemSize::Mem213kb,
            214 => ActualMemSize::Mem214kb,
            215 => ActualMemSize::Mem215kb,
            216 => ActualMemSize::Mem216kb,
            217 => ActualMemSize::Mem217kb,
            218 => ActualMemSize::Mem218kb,
            219 => ActualMemSize::Mem219kb,
            220 => ActualMemSize::Mem220kb,
            221 => ActualMemSize::Mem221kb,
            222 => ActualMemSize::Mem222kb,
            223 => ActualMemSize::Mem223kb,
            224 => ActualMemSize::Mem224kb,
            225 => ActualMemSize::Mem225kb,
            226 => ActualMemSize::Mem226kb,
            227 => ActualMemSize::Mem227kb,
            228 => ActualMemSize::Mem228kb,
            229 => ActualMemSize::Mem229kb,
            230 => ActualMemSize::Mem230kb,
            231 => ActualMemSize::Mem231kb,
            232 => ActualMemSize::Mem232kb,
            233 => ActualMemSize::Mem233kb,
            234 => ActualMemSize::Mem234kb,
            235 => ActualMemSize::Mem235kb,
            236 => ActualMemSize::Mem236kb,
            237 => ActualMemSize::Mem237kb,
            238 => ActualMemSize::Mem238kb,
            239 => ActualMemSize::Mem239kb,
            240 => ActualMemSize::Mem240kb,
            241 => ActualMemSize::Mem241kb,
            242 => ActualMemSize::Mem242kb,
            243 => ActualMemSize::Mem243kb,
            244 => ActualMemSize::Mem244kb,
            245 => ActualMemSize::Mem245kb,
            246 => ActualMemSize::Mem246kb,
            247 => ActualMemSize::Mem247kb,
            248 => ActualMemSize::Mem248kb,
            249 => ActualMemSize::Mem249kb,
            250 => ActualMemSize::Mem250kb,
            251 => ActualMemSize::Mem251kb,
            252 => ActualMemSize::Mem252kb,
            253 => ActualMemSize::Mem253kb,
            254 => ActualMemSize::Mem254kb,
            255 => ActualMemSize::Mem255kb,
        }
    }
    #[doc = "Actual supported memory size: 256kB"]
    #[inline(always)]
    pub fn is_mem256kb(&self) -> bool {
        *self == ActualMemSize::Mem256kb
    }
    #[doc = "Actual supported memory size: 1kB"]
    #[inline(always)]
    pub fn is_mem1kb(&self) -> bool {
        *self == ActualMemSize::Mem1kb
    }
    #[doc = "Actual supported memory size: 2kB"]
    #[inline(always)]
    pub fn is_mem2kb(&self) -> bool {
        *self == ActualMemSize::Mem2kb
    }
    #[doc = "Actual supported memory size: 3kB"]
    #[inline(always)]
    pub fn is_mem3kb(&self) -> bool {
        *self == ActualMemSize::Mem3kb
    }
    #[doc = "Actual supported memory size: 4kB"]
    #[inline(always)]
    pub fn is_mem4kb(&self) -> bool {
        *self == ActualMemSize::Mem4kb
    }
    #[doc = "Actual supported memory size: 5kB"]
    #[inline(always)]
    pub fn is_mem5kb(&self) -> bool {
        *self == ActualMemSize::Mem5kb
    }
    #[doc = "Actual supported memory size: 6kB"]
    #[inline(always)]
    pub fn is_mem6kb(&self) -> bool {
        *self == ActualMemSize::Mem6kb
    }
    #[doc = "Actual supported memory size: 7kB"]
    #[inline(always)]
    pub fn is_mem7kb(&self) -> bool {
        *self == ActualMemSize::Mem7kb
    }
    #[doc = "Actual supported memory size: 8kB"]
    #[inline(always)]
    pub fn is_mem8kb(&self) -> bool {
        *self == ActualMemSize::Mem8kb
    }
    #[doc = "Actual supported memory size: 9kB"]
    #[inline(always)]
    pub fn is_mem9kb(&self) -> bool {
        *self == ActualMemSize::Mem9kb
    }
    #[doc = "Actual supported memory size: 10kB"]
    #[inline(always)]
    pub fn is_mem10kb(&self) -> bool {
        *self == ActualMemSize::Mem10kb
    }
    #[doc = "Actual supported memory size: 11kB"]
    #[inline(always)]
    pub fn is_mem11kb(&self) -> bool {
        *self == ActualMemSize::Mem11kb
    }
    #[doc = "Actual supported memory size: 12kB"]
    #[inline(always)]
    pub fn is_mem12kb(&self) -> bool {
        *self == ActualMemSize::Mem12kb
    }
    #[doc = "Actual supported memory size: 13kB"]
    #[inline(always)]
    pub fn is_mem13kb(&self) -> bool {
        *self == ActualMemSize::Mem13kb
    }
    #[doc = "Actual supported memory size: 14kB"]
    #[inline(always)]
    pub fn is_mem14kb(&self) -> bool {
        *self == ActualMemSize::Mem14kb
    }
    #[doc = "Actual supported memory size: 15kB"]
    #[inline(always)]
    pub fn is_mem15kb(&self) -> bool {
        *self == ActualMemSize::Mem15kb
    }
    #[doc = "Actual supported memory size: 16kB"]
    #[inline(always)]
    pub fn is_mem16kb(&self) -> bool {
        *self == ActualMemSize::Mem16kb
    }
    #[doc = "Actual supported memory size: 17kB"]
    #[inline(always)]
    pub fn is_mem17kb(&self) -> bool {
        *self == ActualMemSize::Mem17kb
    }
    #[doc = "Actual supported memory size: 18kB"]
    #[inline(always)]
    pub fn is_mem18kb(&self) -> bool {
        *self == ActualMemSize::Mem18kb
    }
    #[doc = "Actual supported memory size: 19kB"]
    #[inline(always)]
    pub fn is_mem19kb(&self) -> bool {
        *self == ActualMemSize::Mem19kb
    }
    #[doc = "Actual supported memory size: 20kB"]
    #[inline(always)]
    pub fn is_mem20kb(&self) -> bool {
        *self == ActualMemSize::Mem20kb
    }
    #[doc = "Actual supported memory size: 21kB"]
    #[inline(always)]
    pub fn is_mem21kb(&self) -> bool {
        *self == ActualMemSize::Mem21kb
    }
    #[doc = "Actual supported memory size: 22kB"]
    #[inline(always)]
    pub fn is_mem22kb(&self) -> bool {
        *self == ActualMemSize::Mem22kb
    }
    #[doc = "Actual supported memory size: 23kB"]
    #[inline(always)]
    pub fn is_mem23kb(&self) -> bool {
        *self == ActualMemSize::Mem23kb
    }
    #[doc = "Actual supported memory size: 24kB"]
    #[inline(always)]
    pub fn is_mem24kb(&self) -> bool {
        *self == ActualMemSize::Mem24kb
    }
    #[doc = "Actual supported memory size: 25kB"]
    #[inline(always)]
    pub fn is_mem25kb(&self) -> bool {
        *self == ActualMemSize::Mem25kb
    }
    #[doc = "Actual supported memory size: 26kB"]
    #[inline(always)]
    pub fn is_mem26kb(&self) -> bool {
        *self == ActualMemSize::Mem26kb
    }
    #[doc = "Actual supported memory size: 27kB"]
    #[inline(always)]
    pub fn is_mem27kb(&self) -> bool {
        *self == ActualMemSize::Mem27kb
    }
    #[doc = "Actual supported memory size: 28kB"]
    #[inline(always)]
    pub fn is_mem28kb(&self) -> bool {
        *self == ActualMemSize::Mem28kb
    }
    #[doc = "Actual supported memory size: 29kB"]
    #[inline(always)]
    pub fn is_mem29kb(&self) -> bool {
        *self == ActualMemSize::Mem29kb
    }
    #[doc = "Actual supported memory size: 30kB"]
    #[inline(always)]
    pub fn is_mem30kb(&self) -> bool {
        *self == ActualMemSize::Mem30kb
    }
    #[doc = "Actual supported memory size: 31kB"]
    #[inline(always)]
    pub fn is_mem31kb(&self) -> bool {
        *self == ActualMemSize::Mem31kb
    }
    #[doc = "Actual supported memory size: 32kB"]
    #[inline(always)]
    pub fn is_mem32kb(&self) -> bool {
        *self == ActualMemSize::Mem32kb
    }
    #[doc = "Actual supported memory size: 33kB"]
    #[inline(always)]
    pub fn is_mem33kb(&self) -> bool {
        *self == ActualMemSize::Mem33kb
    }
    #[doc = "Actual supported memory size: 34kB"]
    #[inline(always)]
    pub fn is_mem34kb(&self) -> bool {
        *self == ActualMemSize::Mem34kb
    }
    #[doc = "Actual supported memory size: 35kB"]
    #[inline(always)]
    pub fn is_mem35kb(&self) -> bool {
        *self == ActualMemSize::Mem35kb
    }
    #[doc = "Actual supported memory size: 36kB"]
    #[inline(always)]
    pub fn is_mem36kb(&self) -> bool {
        *self == ActualMemSize::Mem36kb
    }
    #[doc = "Actual supported memory size: 37kB"]
    #[inline(always)]
    pub fn is_mem37kb(&self) -> bool {
        *self == ActualMemSize::Mem37kb
    }
    #[doc = "Actual supported memory size: 38kB"]
    #[inline(always)]
    pub fn is_mem38kb(&self) -> bool {
        *self == ActualMemSize::Mem38kb
    }
    #[doc = "Actual supported memory size: 39kB"]
    #[inline(always)]
    pub fn is_mem39kb(&self) -> bool {
        *self == ActualMemSize::Mem39kb
    }
    #[doc = "Actual supported memory size: 40kB"]
    #[inline(always)]
    pub fn is_mem40kb(&self) -> bool {
        *self == ActualMemSize::Mem40kb
    }
    #[doc = "Actual supported memory size: 41kB"]
    #[inline(always)]
    pub fn is_mem41kb(&self) -> bool {
        *self == ActualMemSize::Mem41kb
    }
    #[doc = "Actual supported memory size: 42kB"]
    #[inline(always)]
    pub fn is_mem42kb(&self) -> bool {
        *self == ActualMemSize::Mem42kb
    }
    #[doc = "Actual supported memory size: 43kB"]
    #[inline(always)]
    pub fn is_mem43kb(&self) -> bool {
        *self == ActualMemSize::Mem43kb
    }
    #[doc = "Actual supported memory size: 44kB"]
    #[inline(always)]
    pub fn is_mem44kb(&self) -> bool {
        *self == ActualMemSize::Mem44kb
    }
    #[doc = "Actual supported memory size: 45kB"]
    #[inline(always)]
    pub fn is_mem45kb(&self) -> bool {
        *self == ActualMemSize::Mem45kb
    }
    #[doc = "Actual supported memory size: 46kB"]
    #[inline(always)]
    pub fn is_mem46kb(&self) -> bool {
        *self == ActualMemSize::Mem46kb
    }
    #[doc = "Actual supported memory size: 47kB"]
    #[inline(always)]
    pub fn is_mem47kb(&self) -> bool {
        *self == ActualMemSize::Mem47kb
    }
    #[doc = "Actual supported memory size: 48kB"]
    #[inline(always)]
    pub fn is_mem48kb(&self) -> bool {
        *self == ActualMemSize::Mem48kb
    }
    #[doc = "Actual supported memory size: 49kB"]
    #[inline(always)]
    pub fn is_mem49kb(&self) -> bool {
        *self == ActualMemSize::Mem49kb
    }
    #[doc = "Actual supported memory size: 50kB"]
    #[inline(always)]
    pub fn is_mem50kb(&self) -> bool {
        *self == ActualMemSize::Mem50kb
    }
    #[doc = "Actual supported memory size: 51kB"]
    #[inline(always)]
    pub fn is_mem51kb(&self) -> bool {
        *self == ActualMemSize::Mem51kb
    }
    #[doc = "Actual supported memory size: 52kB"]
    #[inline(always)]
    pub fn is_mem52kb(&self) -> bool {
        *self == ActualMemSize::Mem52kb
    }
    #[doc = "Actual supported memory size: 53kB"]
    #[inline(always)]
    pub fn is_mem53kb(&self) -> bool {
        *self == ActualMemSize::Mem53kb
    }
    #[doc = "Actual supported memory size: 54kB"]
    #[inline(always)]
    pub fn is_mem54kb(&self) -> bool {
        *self == ActualMemSize::Mem54kb
    }
    #[doc = "Actual supported memory size: 55kB"]
    #[inline(always)]
    pub fn is_mem55kb(&self) -> bool {
        *self == ActualMemSize::Mem55kb
    }
    #[doc = "Actual supported memory size: 56kB"]
    #[inline(always)]
    pub fn is_mem56kb(&self) -> bool {
        *self == ActualMemSize::Mem56kb
    }
    #[doc = "Actual supported memory size: 57kB"]
    #[inline(always)]
    pub fn is_mem57kb(&self) -> bool {
        *self == ActualMemSize::Mem57kb
    }
    #[doc = "Actual supported memory size: 58kB"]
    #[inline(always)]
    pub fn is_mem58kb(&self) -> bool {
        *self == ActualMemSize::Mem58kb
    }
    #[doc = "Actual supported memory size: 59kB"]
    #[inline(always)]
    pub fn is_mem59kb(&self) -> bool {
        *self == ActualMemSize::Mem59kb
    }
    #[doc = "Actual supported memory size: 60kB"]
    #[inline(always)]
    pub fn is_mem60kb(&self) -> bool {
        *self == ActualMemSize::Mem60kb
    }
    #[doc = "Actual supported memory size: 61kB"]
    #[inline(always)]
    pub fn is_mem61kb(&self) -> bool {
        *self == ActualMemSize::Mem61kb
    }
    #[doc = "Actual supported memory size: 62kB"]
    #[inline(always)]
    pub fn is_mem62kb(&self) -> bool {
        *self == ActualMemSize::Mem62kb
    }
    #[doc = "Actual supported memory size: 63kB"]
    #[inline(always)]
    pub fn is_mem63kb(&self) -> bool {
        *self == ActualMemSize::Mem63kb
    }
    #[doc = "Actual supported memory size: 64kB"]
    #[inline(always)]
    pub fn is_mem64kb(&self) -> bool {
        *self == ActualMemSize::Mem64kb
    }
    #[doc = "Actual supported memory size: 65kB"]
    #[inline(always)]
    pub fn is_mem65kb(&self) -> bool {
        *self == ActualMemSize::Mem65kb
    }
    #[doc = "Actual supported memory size: 66kB"]
    #[inline(always)]
    pub fn is_mem66kb(&self) -> bool {
        *self == ActualMemSize::Mem66kb
    }
    #[doc = "Actual supported memory size: 67kB"]
    #[inline(always)]
    pub fn is_mem67kb(&self) -> bool {
        *self == ActualMemSize::Mem67kb
    }
    #[doc = "Actual supported memory size: 68kB"]
    #[inline(always)]
    pub fn is_mem68kb(&self) -> bool {
        *self == ActualMemSize::Mem68kb
    }
    #[doc = "Actual supported memory size: 69kB"]
    #[inline(always)]
    pub fn is_mem69kb(&self) -> bool {
        *self == ActualMemSize::Mem69kb
    }
    #[doc = "Actual supported memory size: 70kB"]
    #[inline(always)]
    pub fn is_mem70kb(&self) -> bool {
        *self == ActualMemSize::Mem70kb
    }
    #[doc = "Actual supported memory size: 71kB"]
    #[inline(always)]
    pub fn is_mem71kb(&self) -> bool {
        *self == ActualMemSize::Mem71kb
    }
    #[doc = "Actual supported memory size: 72kB"]
    #[inline(always)]
    pub fn is_mem72kb(&self) -> bool {
        *self == ActualMemSize::Mem72kb
    }
    #[doc = "Actual supported memory size: 73kB"]
    #[inline(always)]
    pub fn is_mem73kb(&self) -> bool {
        *self == ActualMemSize::Mem73kb
    }
    #[doc = "Actual supported memory size: 74kB"]
    #[inline(always)]
    pub fn is_mem74kb(&self) -> bool {
        *self == ActualMemSize::Mem74kb
    }
    #[doc = "Actual supported memory size: 75kB"]
    #[inline(always)]
    pub fn is_mem75kb(&self) -> bool {
        *self == ActualMemSize::Mem75kb
    }
    #[doc = "Actual supported memory size: 76kB"]
    #[inline(always)]
    pub fn is_mem76kb(&self) -> bool {
        *self == ActualMemSize::Mem76kb
    }
    #[doc = "Actual supported memory size: 77kB"]
    #[inline(always)]
    pub fn is_mem77kb(&self) -> bool {
        *self == ActualMemSize::Mem77kb
    }
    #[doc = "Actual supported memory size: 78kB"]
    #[inline(always)]
    pub fn is_mem78kb(&self) -> bool {
        *self == ActualMemSize::Mem78kb
    }
    #[doc = "Actual supported memory size: 79kB"]
    #[inline(always)]
    pub fn is_mem79kb(&self) -> bool {
        *self == ActualMemSize::Mem79kb
    }
    #[doc = "Actual supported memory size: 80kB"]
    #[inline(always)]
    pub fn is_mem80kb(&self) -> bool {
        *self == ActualMemSize::Mem80kb
    }
    #[doc = "Actual supported memory size: 81kB"]
    #[inline(always)]
    pub fn is_mem81kb(&self) -> bool {
        *self == ActualMemSize::Mem81kb
    }
    #[doc = "Actual supported memory size: 82kB"]
    #[inline(always)]
    pub fn is_mem82kb(&self) -> bool {
        *self == ActualMemSize::Mem82kb
    }
    #[doc = "Actual supported memory size: 83kB"]
    #[inline(always)]
    pub fn is_mem83kb(&self) -> bool {
        *self == ActualMemSize::Mem83kb
    }
    #[doc = "Actual supported memory size: 84kB"]
    #[inline(always)]
    pub fn is_mem84kb(&self) -> bool {
        *self == ActualMemSize::Mem84kb
    }
    #[doc = "Actual supported memory size: 85kB"]
    #[inline(always)]
    pub fn is_mem85kb(&self) -> bool {
        *self == ActualMemSize::Mem85kb
    }
    #[doc = "Actual supported memory size: 86kB"]
    #[inline(always)]
    pub fn is_mem86kb(&self) -> bool {
        *self == ActualMemSize::Mem86kb
    }
    #[doc = "Actual supported memory size: 87kB"]
    #[inline(always)]
    pub fn is_mem87kb(&self) -> bool {
        *self == ActualMemSize::Mem87kb
    }
    #[doc = "Actual supported memory size: 88kB"]
    #[inline(always)]
    pub fn is_mem88kb(&self) -> bool {
        *self == ActualMemSize::Mem88kb
    }
    #[doc = "Actual supported memory size: 89kB"]
    #[inline(always)]
    pub fn is_mem89kb(&self) -> bool {
        *self == ActualMemSize::Mem89kb
    }
    #[doc = "Actual supported memory size: 90kB"]
    #[inline(always)]
    pub fn is_mem90kb(&self) -> bool {
        *self == ActualMemSize::Mem90kb
    }
    #[doc = "Actual supported memory size: 91kB"]
    #[inline(always)]
    pub fn is_mem91kb(&self) -> bool {
        *self == ActualMemSize::Mem91kb
    }
    #[doc = "Actual supported memory size: 92kB"]
    #[inline(always)]
    pub fn is_mem92kb(&self) -> bool {
        *self == ActualMemSize::Mem92kb
    }
    #[doc = "Actual supported memory size: 93kB"]
    #[inline(always)]
    pub fn is_mem93kb(&self) -> bool {
        *self == ActualMemSize::Mem93kb
    }
    #[doc = "Actual supported memory size: 94kB"]
    #[inline(always)]
    pub fn is_mem94kb(&self) -> bool {
        *self == ActualMemSize::Mem94kb
    }
    #[doc = "Actual supported memory size: 95kB"]
    #[inline(always)]
    pub fn is_mem95kb(&self) -> bool {
        *self == ActualMemSize::Mem95kb
    }
    #[doc = "Actual supported memory size: 96kB"]
    #[inline(always)]
    pub fn is_mem96kb(&self) -> bool {
        *self == ActualMemSize::Mem96kb
    }
    #[doc = "Actual supported memory size: 97kB"]
    #[inline(always)]
    pub fn is_mem97kb(&self) -> bool {
        *self == ActualMemSize::Mem97kb
    }
    #[doc = "Actual supported memory size: 98kB"]
    #[inline(always)]
    pub fn is_mem98kb(&self) -> bool {
        *self == ActualMemSize::Mem98kb
    }
    #[doc = "Actual supported memory size: 99kB"]
    #[inline(always)]
    pub fn is_mem99kb(&self) -> bool {
        *self == ActualMemSize::Mem99kb
    }
    #[doc = "Actual supported memory size: 100kB"]
    #[inline(always)]
    pub fn is_mem100kb(&self) -> bool {
        *self == ActualMemSize::Mem100kb
    }
    #[doc = "Actual supported memory size: 101kB"]
    #[inline(always)]
    pub fn is_mem101kb(&self) -> bool {
        *self == ActualMemSize::Mem101kb
    }
    #[doc = "Actual supported memory size: 102kB"]
    #[inline(always)]
    pub fn is_mem102kb(&self) -> bool {
        *self == ActualMemSize::Mem102kb
    }
    #[doc = "Actual supported memory size: 103kB"]
    #[inline(always)]
    pub fn is_mem103kb(&self) -> bool {
        *self == ActualMemSize::Mem103kb
    }
    #[doc = "Actual supported memory size: 104kB"]
    #[inline(always)]
    pub fn is_mem104kb(&self) -> bool {
        *self == ActualMemSize::Mem104kb
    }
    #[doc = "Actual supported memory size: 105kB"]
    #[inline(always)]
    pub fn is_mem105kb(&self) -> bool {
        *self == ActualMemSize::Mem105kb
    }
    #[doc = "Actual supported memory size: 106kB"]
    #[inline(always)]
    pub fn is_mem106kb(&self) -> bool {
        *self == ActualMemSize::Mem106kb
    }
    #[doc = "Actual supported memory size: 107kB"]
    #[inline(always)]
    pub fn is_mem107kb(&self) -> bool {
        *self == ActualMemSize::Mem107kb
    }
    #[doc = "Actual supported memory size: 108kB"]
    #[inline(always)]
    pub fn is_mem108kb(&self) -> bool {
        *self == ActualMemSize::Mem108kb
    }
    #[doc = "Actual supported memory size: 109kB"]
    #[inline(always)]
    pub fn is_mem109kb(&self) -> bool {
        *self == ActualMemSize::Mem109kb
    }
    #[doc = "Actual supported memory size: 110kB"]
    #[inline(always)]
    pub fn is_mem110kb(&self) -> bool {
        *self == ActualMemSize::Mem110kb
    }
    #[doc = "Actual supported memory size: 111kB"]
    #[inline(always)]
    pub fn is_mem111kb(&self) -> bool {
        *self == ActualMemSize::Mem111kb
    }
    #[doc = "Actual supported memory size: 112kB"]
    #[inline(always)]
    pub fn is_mem112kb(&self) -> bool {
        *self == ActualMemSize::Mem112kb
    }
    #[doc = "Actual supported memory size: 113kB"]
    #[inline(always)]
    pub fn is_mem113kb(&self) -> bool {
        *self == ActualMemSize::Mem113kb
    }
    #[doc = "Actual supported memory size: 114kB"]
    #[inline(always)]
    pub fn is_mem114kb(&self) -> bool {
        *self == ActualMemSize::Mem114kb
    }
    #[doc = "Actual supported memory size: 115kB"]
    #[inline(always)]
    pub fn is_mem115kb(&self) -> bool {
        *self == ActualMemSize::Mem115kb
    }
    #[doc = "Actual supported memory size: 116kB"]
    #[inline(always)]
    pub fn is_mem116kb(&self) -> bool {
        *self == ActualMemSize::Mem116kb
    }
    #[doc = "Actual supported memory size: 117kB"]
    #[inline(always)]
    pub fn is_mem117kb(&self) -> bool {
        *self == ActualMemSize::Mem117kb
    }
    #[doc = "Actual supported memory size: 118kB"]
    #[inline(always)]
    pub fn is_mem118kb(&self) -> bool {
        *self == ActualMemSize::Mem118kb
    }
    #[doc = "Actual supported memory size: 119kB"]
    #[inline(always)]
    pub fn is_mem119kb(&self) -> bool {
        *self == ActualMemSize::Mem119kb
    }
    #[doc = "Actual supported memory size: 120kB"]
    #[inline(always)]
    pub fn is_mem120kb(&self) -> bool {
        *self == ActualMemSize::Mem120kb
    }
    #[doc = "Actual supported memory size: 121kB"]
    #[inline(always)]
    pub fn is_mem121kb(&self) -> bool {
        *self == ActualMemSize::Mem121kb
    }
    #[doc = "Actual supported memory size: 122kB"]
    #[inline(always)]
    pub fn is_mem122kb(&self) -> bool {
        *self == ActualMemSize::Mem122kb
    }
    #[doc = "Actual supported memory size: 123kB"]
    #[inline(always)]
    pub fn is_mem123kb(&self) -> bool {
        *self == ActualMemSize::Mem123kb
    }
    #[doc = "Actual supported memory size: 124kB"]
    #[inline(always)]
    pub fn is_mem124kb(&self) -> bool {
        *self == ActualMemSize::Mem124kb
    }
    #[doc = "Actual supported memory size: 125kB"]
    #[inline(always)]
    pub fn is_mem125kb(&self) -> bool {
        *self == ActualMemSize::Mem125kb
    }
    #[doc = "Actual supported memory size: 126kB"]
    #[inline(always)]
    pub fn is_mem126kb(&self) -> bool {
        *self == ActualMemSize::Mem126kb
    }
    #[doc = "Actual supported memory size: 127kB"]
    #[inline(always)]
    pub fn is_mem127kb(&self) -> bool {
        *self == ActualMemSize::Mem127kb
    }
    #[doc = "Actual supported memory size: 128kB"]
    #[inline(always)]
    pub fn is_mem128kb(&self) -> bool {
        *self == ActualMemSize::Mem128kb
    }
    #[doc = "Actual supported memory size: 129kB"]
    #[inline(always)]
    pub fn is_mem129kb(&self) -> bool {
        *self == ActualMemSize::Mem129kb
    }
    #[doc = "Actual supported memory size: 130kB"]
    #[inline(always)]
    pub fn is_mem130kb(&self) -> bool {
        *self == ActualMemSize::Mem130kb
    }
    #[doc = "Actual supported memory size: 131kB"]
    #[inline(always)]
    pub fn is_mem131kb(&self) -> bool {
        *self == ActualMemSize::Mem131kb
    }
    #[doc = "Actual supported memory size: 132kB"]
    #[inline(always)]
    pub fn is_mem132kb(&self) -> bool {
        *self == ActualMemSize::Mem132kb
    }
    #[doc = "Actual supported memory size: 133kB"]
    #[inline(always)]
    pub fn is_mem133kb(&self) -> bool {
        *self == ActualMemSize::Mem133kb
    }
    #[doc = "Actual supported memory size: 134kB"]
    #[inline(always)]
    pub fn is_mem134kb(&self) -> bool {
        *self == ActualMemSize::Mem134kb
    }
    #[doc = "Actual supported memory size: 135kB"]
    #[inline(always)]
    pub fn is_mem135kb(&self) -> bool {
        *self == ActualMemSize::Mem135kb
    }
    #[doc = "Actual supported memory size: 136kB"]
    #[inline(always)]
    pub fn is_mem136kb(&self) -> bool {
        *self == ActualMemSize::Mem136kb
    }
    #[doc = "Actual supported memory size: 137kB"]
    #[inline(always)]
    pub fn is_mem137kb(&self) -> bool {
        *self == ActualMemSize::Mem137kb
    }
    #[doc = "Actual supported memory size: 138kB"]
    #[inline(always)]
    pub fn is_mem138kb(&self) -> bool {
        *self == ActualMemSize::Mem138kb
    }
    #[doc = "Actual supported memory size: 139kB"]
    #[inline(always)]
    pub fn is_mem139kb(&self) -> bool {
        *self == ActualMemSize::Mem139kb
    }
    #[doc = "Actual supported memory size: 140kB"]
    #[inline(always)]
    pub fn is_mem140kb(&self) -> bool {
        *self == ActualMemSize::Mem140kb
    }
    #[doc = "Actual supported memory size: 141kB"]
    #[inline(always)]
    pub fn is_mem141kb(&self) -> bool {
        *self == ActualMemSize::Mem141kb
    }
    #[doc = "Actual supported memory size: 142kB"]
    #[inline(always)]
    pub fn is_mem142kb(&self) -> bool {
        *self == ActualMemSize::Mem142kb
    }
    #[doc = "Actual supported memory size: 143kB"]
    #[inline(always)]
    pub fn is_mem143kb(&self) -> bool {
        *self == ActualMemSize::Mem143kb
    }
    #[doc = "Actual supported memory size: 144kB"]
    #[inline(always)]
    pub fn is_mem144kb(&self) -> bool {
        *self == ActualMemSize::Mem144kb
    }
    #[doc = "Actual supported memory size: 145kB"]
    #[inline(always)]
    pub fn is_mem145kb(&self) -> bool {
        *self == ActualMemSize::Mem145kb
    }
    #[doc = "Actual supported memory size: 146kB"]
    #[inline(always)]
    pub fn is_mem146kb(&self) -> bool {
        *self == ActualMemSize::Mem146kb
    }
    #[doc = "Actual supported memory size: 147kB"]
    #[inline(always)]
    pub fn is_mem147kb(&self) -> bool {
        *self == ActualMemSize::Mem147kb
    }
    #[doc = "Actual supported memory size: 148kB"]
    #[inline(always)]
    pub fn is_mem148kb(&self) -> bool {
        *self == ActualMemSize::Mem148kb
    }
    #[doc = "Actual supported memory size: 149kB"]
    #[inline(always)]
    pub fn is_mem149kb(&self) -> bool {
        *self == ActualMemSize::Mem149kb
    }
    #[doc = "Actual supported memory size: 150kB"]
    #[inline(always)]
    pub fn is_mem150kb(&self) -> bool {
        *self == ActualMemSize::Mem150kb
    }
    #[doc = "Actual supported memory size: 151kB"]
    #[inline(always)]
    pub fn is_mem151kb(&self) -> bool {
        *self == ActualMemSize::Mem151kb
    }
    #[doc = "Actual supported memory size: 152kB"]
    #[inline(always)]
    pub fn is_mem152kb(&self) -> bool {
        *self == ActualMemSize::Mem152kb
    }
    #[doc = "Actual supported memory size: 153kB"]
    #[inline(always)]
    pub fn is_mem153kb(&self) -> bool {
        *self == ActualMemSize::Mem153kb
    }
    #[doc = "Actual supported memory size: 154kB"]
    #[inline(always)]
    pub fn is_mem154kb(&self) -> bool {
        *self == ActualMemSize::Mem154kb
    }
    #[doc = "Actual supported memory size: 155kB"]
    #[inline(always)]
    pub fn is_mem155kb(&self) -> bool {
        *self == ActualMemSize::Mem155kb
    }
    #[doc = "Actual supported memory size: 156kB"]
    #[inline(always)]
    pub fn is_mem156kb(&self) -> bool {
        *self == ActualMemSize::Mem156kb
    }
    #[doc = "Actual supported memory size: 157kB"]
    #[inline(always)]
    pub fn is_mem157kb(&self) -> bool {
        *self == ActualMemSize::Mem157kb
    }
    #[doc = "Actual supported memory size: 158kB"]
    #[inline(always)]
    pub fn is_mem158kb(&self) -> bool {
        *self == ActualMemSize::Mem158kb
    }
    #[doc = "Actual supported memory size: 159kB"]
    #[inline(always)]
    pub fn is_mem159kb(&self) -> bool {
        *self == ActualMemSize::Mem159kb
    }
    #[doc = "Actual supported memory size: 160kB"]
    #[inline(always)]
    pub fn is_mem160kb(&self) -> bool {
        *self == ActualMemSize::Mem160kb
    }
    #[doc = "Actual supported memory size: 161kB"]
    #[inline(always)]
    pub fn is_mem161kb(&self) -> bool {
        *self == ActualMemSize::Mem161kb
    }
    #[doc = "Actual supported memory size: 162kB"]
    #[inline(always)]
    pub fn is_mem162kb(&self) -> bool {
        *self == ActualMemSize::Mem162kb
    }
    #[doc = "Actual supported memory size: 163kB"]
    #[inline(always)]
    pub fn is_mem163kb(&self) -> bool {
        *self == ActualMemSize::Mem163kb
    }
    #[doc = "Actual supported memory size: 164kB"]
    #[inline(always)]
    pub fn is_mem164kb(&self) -> bool {
        *self == ActualMemSize::Mem164kb
    }
    #[doc = "Actual supported memory size: 165kB"]
    #[inline(always)]
    pub fn is_mem165kb(&self) -> bool {
        *self == ActualMemSize::Mem165kb
    }
    #[doc = "Actual supported memory size: 166kB"]
    #[inline(always)]
    pub fn is_mem166kb(&self) -> bool {
        *self == ActualMemSize::Mem166kb
    }
    #[doc = "Actual supported memory size: 167kB"]
    #[inline(always)]
    pub fn is_mem167kb(&self) -> bool {
        *self == ActualMemSize::Mem167kb
    }
    #[doc = "Actual supported memory size: 168kB"]
    #[inline(always)]
    pub fn is_mem168kb(&self) -> bool {
        *self == ActualMemSize::Mem168kb
    }
    #[doc = "Actual supported memory size: 169kB"]
    #[inline(always)]
    pub fn is_mem169kb(&self) -> bool {
        *self == ActualMemSize::Mem169kb
    }
    #[doc = "Actual supported memory size: 170kB"]
    #[inline(always)]
    pub fn is_mem170kb(&self) -> bool {
        *self == ActualMemSize::Mem170kb
    }
    #[doc = "Actual supported memory size: 171kB"]
    #[inline(always)]
    pub fn is_mem171kb(&self) -> bool {
        *self == ActualMemSize::Mem171kb
    }
    #[doc = "Actual supported memory size: 172kB"]
    #[inline(always)]
    pub fn is_mem172kb(&self) -> bool {
        *self == ActualMemSize::Mem172kb
    }
    #[doc = "Actual supported memory size: 173kB"]
    #[inline(always)]
    pub fn is_mem173kb(&self) -> bool {
        *self == ActualMemSize::Mem173kb
    }
    #[doc = "Actual supported memory size: 174kB"]
    #[inline(always)]
    pub fn is_mem174kb(&self) -> bool {
        *self == ActualMemSize::Mem174kb
    }
    #[doc = "Actual supported memory size: 175kB"]
    #[inline(always)]
    pub fn is_mem175kb(&self) -> bool {
        *self == ActualMemSize::Mem175kb
    }
    #[doc = "Actual supported memory size: 176kB"]
    #[inline(always)]
    pub fn is_mem176kb(&self) -> bool {
        *self == ActualMemSize::Mem176kb
    }
    #[doc = "Actual supported memory size: 177kB"]
    #[inline(always)]
    pub fn is_mem177kb(&self) -> bool {
        *self == ActualMemSize::Mem177kb
    }
    #[doc = "Actual supported memory size: 178kB"]
    #[inline(always)]
    pub fn is_mem178kb(&self) -> bool {
        *self == ActualMemSize::Mem178kb
    }
    #[doc = "Actual supported memory size: 179kB"]
    #[inline(always)]
    pub fn is_mem179kb(&self) -> bool {
        *self == ActualMemSize::Mem179kb
    }
    #[doc = "Actual supported memory size: 180kB"]
    #[inline(always)]
    pub fn is_mem180kb(&self) -> bool {
        *self == ActualMemSize::Mem180kb
    }
    #[doc = "Actual supported memory size: 181kB"]
    #[inline(always)]
    pub fn is_mem181kb(&self) -> bool {
        *self == ActualMemSize::Mem181kb
    }
    #[doc = "Actual supported memory size: 182kB"]
    #[inline(always)]
    pub fn is_mem182kb(&self) -> bool {
        *self == ActualMemSize::Mem182kb
    }
    #[doc = "Actual supported memory size: 183kB"]
    #[inline(always)]
    pub fn is_mem183kb(&self) -> bool {
        *self == ActualMemSize::Mem183kb
    }
    #[doc = "Actual supported memory size: 184kB"]
    #[inline(always)]
    pub fn is_mem184kb(&self) -> bool {
        *self == ActualMemSize::Mem184kb
    }
    #[doc = "Actual supported memory size: 185kB"]
    #[inline(always)]
    pub fn is_mem185kb(&self) -> bool {
        *self == ActualMemSize::Mem185kb
    }
    #[doc = "Actual supported memory size: 186kB"]
    #[inline(always)]
    pub fn is_mem186kb(&self) -> bool {
        *self == ActualMemSize::Mem186kb
    }
    #[doc = "Actual supported memory size: 187kB"]
    #[inline(always)]
    pub fn is_mem187kb(&self) -> bool {
        *self == ActualMemSize::Mem187kb
    }
    #[doc = "Actual supported memory size: 188kB"]
    #[inline(always)]
    pub fn is_mem188kb(&self) -> bool {
        *self == ActualMemSize::Mem188kb
    }
    #[doc = "Actual supported memory size: 189kB"]
    #[inline(always)]
    pub fn is_mem189kb(&self) -> bool {
        *self == ActualMemSize::Mem189kb
    }
    #[doc = "Actual supported memory size: 190kB"]
    #[inline(always)]
    pub fn is_mem190kb(&self) -> bool {
        *self == ActualMemSize::Mem190kb
    }
    #[doc = "Actual supported memory size: 191kB"]
    #[inline(always)]
    pub fn is_mem191kb(&self) -> bool {
        *self == ActualMemSize::Mem191kb
    }
    #[doc = "Actual supported memory size: 192kB"]
    #[inline(always)]
    pub fn is_mem192kb(&self) -> bool {
        *self == ActualMemSize::Mem192kb
    }
    #[doc = "Actual supported memory size: 193kB"]
    #[inline(always)]
    pub fn is_mem193kb(&self) -> bool {
        *self == ActualMemSize::Mem193kb
    }
    #[doc = "Actual supported memory size: 194kB"]
    #[inline(always)]
    pub fn is_mem194kb(&self) -> bool {
        *self == ActualMemSize::Mem194kb
    }
    #[doc = "Actual supported memory size: 195kB"]
    #[inline(always)]
    pub fn is_mem195kb(&self) -> bool {
        *self == ActualMemSize::Mem195kb
    }
    #[doc = "Actual supported memory size: 196kB"]
    #[inline(always)]
    pub fn is_mem196kb(&self) -> bool {
        *self == ActualMemSize::Mem196kb
    }
    #[doc = "Actual supported memory size: 197kB"]
    #[inline(always)]
    pub fn is_mem197kb(&self) -> bool {
        *self == ActualMemSize::Mem197kb
    }
    #[doc = "Actual supported memory size: 198kB"]
    #[inline(always)]
    pub fn is_mem198kb(&self) -> bool {
        *self == ActualMemSize::Mem198kb
    }
    #[doc = "Actual supported memory size: 199kB"]
    #[inline(always)]
    pub fn is_mem199kb(&self) -> bool {
        *self == ActualMemSize::Mem199kb
    }
    #[doc = "Actual supported memory size: 200kB"]
    #[inline(always)]
    pub fn is_mem200kb(&self) -> bool {
        *self == ActualMemSize::Mem200kb
    }
    #[doc = "Actual supported memory size: 201kB"]
    #[inline(always)]
    pub fn is_mem201kb(&self) -> bool {
        *self == ActualMemSize::Mem201kb
    }
    #[doc = "Actual supported memory size: 202kB"]
    #[inline(always)]
    pub fn is_mem202kb(&self) -> bool {
        *self == ActualMemSize::Mem202kb
    }
    #[doc = "Actual supported memory size: 203kB"]
    #[inline(always)]
    pub fn is_mem203kb(&self) -> bool {
        *self == ActualMemSize::Mem203kb
    }
    #[doc = "Actual supported memory size: 204kB"]
    #[inline(always)]
    pub fn is_mem204kb(&self) -> bool {
        *self == ActualMemSize::Mem204kb
    }
    #[doc = "Actual supported memory size: 205kB"]
    #[inline(always)]
    pub fn is_mem205kb(&self) -> bool {
        *self == ActualMemSize::Mem205kb
    }
    #[doc = "Actual supported memory size: 206kB"]
    #[inline(always)]
    pub fn is_mem206kb(&self) -> bool {
        *self == ActualMemSize::Mem206kb
    }
    #[doc = "Actual supported memory size: 207kB"]
    #[inline(always)]
    pub fn is_mem207kb(&self) -> bool {
        *self == ActualMemSize::Mem207kb
    }
    #[doc = "Actual supported memory size: 208kB"]
    #[inline(always)]
    pub fn is_mem208kb(&self) -> bool {
        *self == ActualMemSize::Mem208kb
    }
    #[doc = "Actual supported memory size: 209kB"]
    #[inline(always)]
    pub fn is_mem209kb(&self) -> bool {
        *self == ActualMemSize::Mem209kb
    }
    #[doc = "Actual supported memory size: 210kB"]
    #[inline(always)]
    pub fn is_mem210kb(&self) -> bool {
        *self == ActualMemSize::Mem210kb
    }
    #[doc = "Actual supported memory size: 211kB"]
    #[inline(always)]
    pub fn is_mem211kb(&self) -> bool {
        *self == ActualMemSize::Mem211kb
    }
    #[doc = "Actual supported memory size: 212kB"]
    #[inline(always)]
    pub fn is_mem212kb(&self) -> bool {
        *self == ActualMemSize::Mem212kb
    }
    #[doc = "Actual supported memory size: 213kB"]
    #[inline(always)]
    pub fn is_mem213kb(&self) -> bool {
        *self == ActualMemSize::Mem213kb
    }
    #[doc = "Actual supported memory size: 214kB"]
    #[inline(always)]
    pub fn is_mem214kb(&self) -> bool {
        *self == ActualMemSize::Mem214kb
    }
    #[doc = "Actual supported memory size: 215kB"]
    #[inline(always)]
    pub fn is_mem215kb(&self) -> bool {
        *self == ActualMemSize::Mem215kb
    }
    #[doc = "Actual supported memory size: 216kB"]
    #[inline(always)]
    pub fn is_mem216kb(&self) -> bool {
        *self == ActualMemSize::Mem216kb
    }
    #[doc = "Actual supported memory size: 217kB"]
    #[inline(always)]
    pub fn is_mem217kb(&self) -> bool {
        *self == ActualMemSize::Mem217kb
    }
    #[doc = "Actual supported memory size: 218kB"]
    #[inline(always)]
    pub fn is_mem218kb(&self) -> bool {
        *self == ActualMemSize::Mem218kb
    }
    #[doc = "Actual supported memory size: 219kB"]
    #[inline(always)]
    pub fn is_mem219kb(&self) -> bool {
        *self == ActualMemSize::Mem219kb
    }
    #[doc = "Actual supported memory size: 220kB"]
    #[inline(always)]
    pub fn is_mem220kb(&self) -> bool {
        *self == ActualMemSize::Mem220kb
    }
    #[doc = "Actual supported memory size: 221kB"]
    #[inline(always)]
    pub fn is_mem221kb(&self) -> bool {
        *self == ActualMemSize::Mem221kb
    }
    #[doc = "Actual supported memory size: 222kB"]
    #[inline(always)]
    pub fn is_mem222kb(&self) -> bool {
        *self == ActualMemSize::Mem222kb
    }
    #[doc = "Actual supported memory size: 223kB"]
    #[inline(always)]
    pub fn is_mem223kb(&self) -> bool {
        *self == ActualMemSize::Mem223kb
    }
    #[doc = "Actual supported memory size: 224kB"]
    #[inline(always)]
    pub fn is_mem224kb(&self) -> bool {
        *self == ActualMemSize::Mem224kb
    }
    #[doc = "Actual supported memory size: 225kB"]
    #[inline(always)]
    pub fn is_mem225kb(&self) -> bool {
        *self == ActualMemSize::Mem225kb
    }
    #[doc = "Actual supported memory size: 226kB"]
    #[inline(always)]
    pub fn is_mem226kb(&self) -> bool {
        *self == ActualMemSize::Mem226kb
    }
    #[doc = "Actual supported memory size: 227kB"]
    #[inline(always)]
    pub fn is_mem227kb(&self) -> bool {
        *self == ActualMemSize::Mem227kb
    }
    #[doc = "Actual supported memory size: 228kB"]
    #[inline(always)]
    pub fn is_mem228kb(&self) -> bool {
        *self == ActualMemSize::Mem228kb
    }
    #[doc = "Actual supported memory size: 229kB"]
    #[inline(always)]
    pub fn is_mem229kb(&self) -> bool {
        *self == ActualMemSize::Mem229kb
    }
    #[doc = "Actual supported memory size: 230kB"]
    #[inline(always)]
    pub fn is_mem230kb(&self) -> bool {
        *self == ActualMemSize::Mem230kb
    }
    #[doc = "Actual supported memory size: 231kB"]
    #[inline(always)]
    pub fn is_mem231kb(&self) -> bool {
        *self == ActualMemSize::Mem231kb
    }
    #[doc = "Actual supported memory size: 232kB"]
    #[inline(always)]
    pub fn is_mem232kb(&self) -> bool {
        *self == ActualMemSize::Mem232kb
    }
    #[doc = "Actual supported memory size: 233kB"]
    #[inline(always)]
    pub fn is_mem233kb(&self) -> bool {
        *self == ActualMemSize::Mem233kb
    }
    #[doc = "Actual supported memory size: 234kB"]
    #[inline(always)]
    pub fn is_mem234kb(&self) -> bool {
        *self == ActualMemSize::Mem234kb
    }
    #[doc = "Actual supported memory size: 235kB"]
    #[inline(always)]
    pub fn is_mem235kb(&self) -> bool {
        *self == ActualMemSize::Mem235kb
    }
    #[doc = "Actual supported memory size: 236kB"]
    #[inline(always)]
    pub fn is_mem236kb(&self) -> bool {
        *self == ActualMemSize::Mem236kb
    }
    #[doc = "Actual supported memory size: 237kB"]
    #[inline(always)]
    pub fn is_mem237kb(&self) -> bool {
        *self == ActualMemSize::Mem237kb
    }
    #[doc = "Actual supported memory size: 238kB"]
    #[inline(always)]
    pub fn is_mem238kb(&self) -> bool {
        *self == ActualMemSize::Mem238kb
    }
    #[doc = "Actual supported memory size: 239kB"]
    #[inline(always)]
    pub fn is_mem239kb(&self) -> bool {
        *self == ActualMemSize::Mem239kb
    }
    #[doc = "Actual supported memory size: 240kB"]
    #[inline(always)]
    pub fn is_mem240kb(&self) -> bool {
        *self == ActualMemSize::Mem240kb
    }
    #[doc = "Actual supported memory size: 241kB"]
    #[inline(always)]
    pub fn is_mem241kb(&self) -> bool {
        *self == ActualMemSize::Mem241kb
    }
    #[doc = "Actual supported memory size: 242kB"]
    #[inline(always)]
    pub fn is_mem242kb(&self) -> bool {
        *self == ActualMemSize::Mem242kb
    }
    #[doc = "Actual supported memory size: 243kB"]
    #[inline(always)]
    pub fn is_mem243kb(&self) -> bool {
        *self == ActualMemSize::Mem243kb
    }
    #[doc = "Actual supported memory size: 244kB"]
    #[inline(always)]
    pub fn is_mem244kb(&self) -> bool {
        *self == ActualMemSize::Mem244kb
    }
    #[doc = "Actual supported memory size: 245kB"]
    #[inline(always)]
    pub fn is_mem245kb(&self) -> bool {
        *self == ActualMemSize::Mem245kb
    }
    #[doc = "Actual supported memory size: 246kB"]
    #[inline(always)]
    pub fn is_mem246kb(&self) -> bool {
        *self == ActualMemSize::Mem246kb
    }
    #[doc = "Actual supported memory size: 247kB"]
    #[inline(always)]
    pub fn is_mem247kb(&self) -> bool {
        *self == ActualMemSize::Mem247kb
    }
    #[doc = "Actual supported memory size: 248kB"]
    #[inline(always)]
    pub fn is_mem248kb(&self) -> bool {
        *self == ActualMemSize::Mem248kb
    }
    #[doc = "Actual supported memory size: 249kB"]
    #[inline(always)]
    pub fn is_mem249kb(&self) -> bool {
        *self == ActualMemSize::Mem249kb
    }
    #[doc = "Actual supported memory size: 250kB"]
    #[inline(always)]
    pub fn is_mem250kb(&self) -> bool {
        *self == ActualMemSize::Mem250kb
    }
    #[doc = "Actual supported memory size: 251kB"]
    #[inline(always)]
    pub fn is_mem251kb(&self) -> bool {
        *self == ActualMemSize::Mem251kb
    }
    #[doc = "Actual supported memory size: 252kB"]
    #[inline(always)]
    pub fn is_mem252kb(&self) -> bool {
        *self == ActualMemSize::Mem252kb
    }
    #[doc = "Actual supported memory size: 253kB"]
    #[inline(always)]
    pub fn is_mem253kb(&self) -> bool {
        *self == ActualMemSize::Mem253kb
    }
    #[doc = "Actual supported memory size: 254kB"]
    #[inline(always)]
    pub fn is_mem254kb(&self) -> bool {
        *self == ActualMemSize::Mem254kb
    }
    #[doc = "Actual supported memory size: 255kB"]
    #[inline(always)]
    pub fn is_mem255kb(&self) -> bool {
        *self == ActualMemSize::Mem255kb
    }
}
#[doc = "Field `actual_mem_size` writer - The actual size of the connnected on-chip RAM memory in kB - 0: 256kB, 1-255: 1-255kB."]
pub type ActualMemSizeW<'a, REG> = crate::FieldWriterSafe<'a, REG, 8, ActualMemSize>;
impl<'a, REG> ActualMemSizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Actual supported memory size: 256kB"]
    #[inline(always)]
    pub fn mem256kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem256kb)
    }
    #[doc = "Actual supported memory size: 1kB"]
    #[inline(always)]
    pub fn mem1kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem1kb)
    }
    #[doc = "Actual supported memory size: 2kB"]
    #[inline(always)]
    pub fn mem2kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem2kb)
    }
    #[doc = "Actual supported memory size: 3kB"]
    #[inline(always)]
    pub fn mem3kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem3kb)
    }
    #[doc = "Actual supported memory size: 4kB"]
    #[inline(always)]
    pub fn mem4kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem4kb)
    }
    #[doc = "Actual supported memory size: 5kB"]
    #[inline(always)]
    pub fn mem5kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem5kb)
    }
    #[doc = "Actual supported memory size: 6kB"]
    #[inline(always)]
    pub fn mem6kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem6kb)
    }
    #[doc = "Actual supported memory size: 7kB"]
    #[inline(always)]
    pub fn mem7kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem7kb)
    }
    #[doc = "Actual supported memory size: 8kB"]
    #[inline(always)]
    pub fn mem8kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem8kb)
    }
    #[doc = "Actual supported memory size: 9kB"]
    #[inline(always)]
    pub fn mem9kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem9kb)
    }
    #[doc = "Actual supported memory size: 10kB"]
    #[inline(always)]
    pub fn mem10kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem10kb)
    }
    #[doc = "Actual supported memory size: 11kB"]
    #[inline(always)]
    pub fn mem11kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem11kb)
    }
    #[doc = "Actual supported memory size: 12kB"]
    #[inline(always)]
    pub fn mem12kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem12kb)
    }
    #[doc = "Actual supported memory size: 13kB"]
    #[inline(always)]
    pub fn mem13kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem13kb)
    }
    #[doc = "Actual supported memory size: 14kB"]
    #[inline(always)]
    pub fn mem14kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem14kb)
    }
    #[doc = "Actual supported memory size: 15kB"]
    #[inline(always)]
    pub fn mem15kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem15kb)
    }
    #[doc = "Actual supported memory size: 16kB"]
    #[inline(always)]
    pub fn mem16kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem16kb)
    }
    #[doc = "Actual supported memory size: 17kB"]
    #[inline(always)]
    pub fn mem17kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem17kb)
    }
    #[doc = "Actual supported memory size: 18kB"]
    #[inline(always)]
    pub fn mem18kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem18kb)
    }
    #[doc = "Actual supported memory size: 19kB"]
    #[inline(always)]
    pub fn mem19kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem19kb)
    }
    #[doc = "Actual supported memory size: 20kB"]
    #[inline(always)]
    pub fn mem20kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem20kb)
    }
    #[doc = "Actual supported memory size: 21kB"]
    #[inline(always)]
    pub fn mem21kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem21kb)
    }
    #[doc = "Actual supported memory size: 22kB"]
    #[inline(always)]
    pub fn mem22kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem22kb)
    }
    #[doc = "Actual supported memory size: 23kB"]
    #[inline(always)]
    pub fn mem23kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem23kb)
    }
    #[doc = "Actual supported memory size: 24kB"]
    #[inline(always)]
    pub fn mem24kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem24kb)
    }
    #[doc = "Actual supported memory size: 25kB"]
    #[inline(always)]
    pub fn mem25kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem25kb)
    }
    #[doc = "Actual supported memory size: 26kB"]
    #[inline(always)]
    pub fn mem26kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem26kb)
    }
    #[doc = "Actual supported memory size: 27kB"]
    #[inline(always)]
    pub fn mem27kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem27kb)
    }
    #[doc = "Actual supported memory size: 28kB"]
    #[inline(always)]
    pub fn mem28kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem28kb)
    }
    #[doc = "Actual supported memory size: 29kB"]
    #[inline(always)]
    pub fn mem29kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem29kb)
    }
    #[doc = "Actual supported memory size: 30kB"]
    #[inline(always)]
    pub fn mem30kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem30kb)
    }
    #[doc = "Actual supported memory size: 31kB"]
    #[inline(always)]
    pub fn mem31kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem31kb)
    }
    #[doc = "Actual supported memory size: 32kB"]
    #[inline(always)]
    pub fn mem32kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem32kb)
    }
    #[doc = "Actual supported memory size: 33kB"]
    #[inline(always)]
    pub fn mem33kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem33kb)
    }
    #[doc = "Actual supported memory size: 34kB"]
    #[inline(always)]
    pub fn mem34kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem34kb)
    }
    #[doc = "Actual supported memory size: 35kB"]
    #[inline(always)]
    pub fn mem35kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem35kb)
    }
    #[doc = "Actual supported memory size: 36kB"]
    #[inline(always)]
    pub fn mem36kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem36kb)
    }
    #[doc = "Actual supported memory size: 37kB"]
    #[inline(always)]
    pub fn mem37kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem37kb)
    }
    #[doc = "Actual supported memory size: 38kB"]
    #[inline(always)]
    pub fn mem38kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem38kb)
    }
    #[doc = "Actual supported memory size: 39kB"]
    #[inline(always)]
    pub fn mem39kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem39kb)
    }
    #[doc = "Actual supported memory size: 40kB"]
    #[inline(always)]
    pub fn mem40kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem40kb)
    }
    #[doc = "Actual supported memory size: 41kB"]
    #[inline(always)]
    pub fn mem41kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem41kb)
    }
    #[doc = "Actual supported memory size: 42kB"]
    #[inline(always)]
    pub fn mem42kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem42kb)
    }
    #[doc = "Actual supported memory size: 43kB"]
    #[inline(always)]
    pub fn mem43kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem43kb)
    }
    #[doc = "Actual supported memory size: 44kB"]
    #[inline(always)]
    pub fn mem44kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem44kb)
    }
    #[doc = "Actual supported memory size: 45kB"]
    #[inline(always)]
    pub fn mem45kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem45kb)
    }
    #[doc = "Actual supported memory size: 46kB"]
    #[inline(always)]
    pub fn mem46kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem46kb)
    }
    #[doc = "Actual supported memory size: 47kB"]
    #[inline(always)]
    pub fn mem47kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem47kb)
    }
    #[doc = "Actual supported memory size: 48kB"]
    #[inline(always)]
    pub fn mem48kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem48kb)
    }
    #[doc = "Actual supported memory size: 49kB"]
    #[inline(always)]
    pub fn mem49kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem49kb)
    }
    #[doc = "Actual supported memory size: 50kB"]
    #[inline(always)]
    pub fn mem50kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem50kb)
    }
    #[doc = "Actual supported memory size: 51kB"]
    #[inline(always)]
    pub fn mem51kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem51kb)
    }
    #[doc = "Actual supported memory size: 52kB"]
    #[inline(always)]
    pub fn mem52kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem52kb)
    }
    #[doc = "Actual supported memory size: 53kB"]
    #[inline(always)]
    pub fn mem53kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem53kb)
    }
    #[doc = "Actual supported memory size: 54kB"]
    #[inline(always)]
    pub fn mem54kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem54kb)
    }
    #[doc = "Actual supported memory size: 55kB"]
    #[inline(always)]
    pub fn mem55kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem55kb)
    }
    #[doc = "Actual supported memory size: 56kB"]
    #[inline(always)]
    pub fn mem56kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem56kb)
    }
    #[doc = "Actual supported memory size: 57kB"]
    #[inline(always)]
    pub fn mem57kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem57kb)
    }
    #[doc = "Actual supported memory size: 58kB"]
    #[inline(always)]
    pub fn mem58kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem58kb)
    }
    #[doc = "Actual supported memory size: 59kB"]
    #[inline(always)]
    pub fn mem59kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem59kb)
    }
    #[doc = "Actual supported memory size: 60kB"]
    #[inline(always)]
    pub fn mem60kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem60kb)
    }
    #[doc = "Actual supported memory size: 61kB"]
    #[inline(always)]
    pub fn mem61kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem61kb)
    }
    #[doc = "Actual supported memory size: 62kB"]
    #[inline(always)]
    pub fn mem62kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem62kb)
    }
    #[doc = "Actual supported memory size: 63kB"]
    #[inline(always)]
    pub fn mem63kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem63kb)
    }
    #[doc = "Actual supported memory size: 64kB"]
    #[inline(always)]
    pub fn mem64kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem64kb)
    }
    #[doc = "Actual supported memory size: 65kB"]
    #[inline(always)]
    pub fn mem65kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem65kb)
    }
    #[doc = "Actual supported memory size: 66kB"]
    #[inline(always)]
    pub fn mem66kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem66kb)
    }
    #[doc = "Actual supported memory size: 67kB"]
    #[inline(always)]
    pub fn mem67kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem67kb)
    }
    #[doc = "Actual supported memory size: 68kB"]
    #[inline(always)]
    pub fn mem68kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem68kb)
    }
    #[doc = "Actual supported memory size: 69kB"]
    #[inline(always)]
    pub fn mem69kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem69kb)
    }
    #[doc = "Actual supported memory size: 70kB"]
    #[inline(always)]
    pub fn mem70kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem70kb)
    }
    #[doc = "Actual supported memory size: 71kB"]
    #[inline(always)]
    pub fn mem71kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem71kb)
    }
    #[doc = "Actual supported memory size: 72kB"]
    #[inline(always)]
    pub fn mem72kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem72kb)
    }
    #[doc = "Actual supported memory size: 73kB"]
    #[inline(always)]
    pub fn mem73kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem73kb)
    }
    #[doc = "Actual supported memory size: 74kB"]
    #[inline(always)]
    pub fn mem74kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem74kb)
    }
    #[doc = "Actual supported memory size: 75kB"]
    #[inline(always)]
    pub fn mem75kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem75kb)
    }
    #[doc = "Actual supported memory size: 76kB"]
    #[inline(always)]
    pub fn mem76kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem76kb)
    }
    #[doc = "Actual supported memory size: 77kB"]
    #[inline(always)]
    pub fn mem77kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem77kb)
    }
    #[doc = "Actual supported memory size: 78kB"]
    #[inline(always)]
    pub fn mem78kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem78kb)
    }
    #[doc = "Actual supported memory size: 79kB"]
    #[inline(always)]
    pub fn mem79kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem79kb)
    }
    #[doc = "Actual supported memory size: 80kB"]
    #[inline(always)]
    pub fn mem80kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem80kb)
    }
    #[doc = "Actual supported memory size: 81kB"]
    #[inline(always)]
    pub fn mem81kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem81kb)
    }
    #[doc = "Actual supported memory size: 82kB"]
    #[inline(always)]
    pub fn mem82kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem82kb)
    }
    #[doc = "Actual supported memory size: 83kB"]
    #[inline(always)]
    pub fn mem83kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem83kb)
    }
    #[doc = "Actual supported memory size: 84kB"]
    #[inline(always)]
    pub fn mem84kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem84kb)
    }
    #[doc = "Actual supported memory size: 85kB"]
    #[inline(always)]
    pub fn mem85kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem85kb)
    }
    #[doc = "Actual supported memory size: 86kB"]
    #[inline(always)]
    pub fn mem86kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem86kb)
    }
    #[doc = "Actual supported memory size: 87kB"]
    #[inline(always)]
    pub fn mem87kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem87kb)
    }
    #[doc = "Actual supported memory size: 88kB"]
    #[inline(always)]
    pub fn mem88kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem88kb)
    }
    #[doc = "Actual supported memory size: 89kB"]
    #[inline(always)]
    pub fn mem89kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem89kb)
    }
    #[doc = "Actual supported memory size: 90kB"]
    #[inline(always)]
    pub fn mem90kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem90kb)
    }
    #[doc = "Actual supported memory size: 91kB"]
    #[inline(always)]
    pub fn mem91kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem91kb)
    }
    #[doc = "Actual supported memory size: 92kB"]
    #[inline(always)]
    pub fn mem92kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem92kb)
    }
    #[doc = "Actual supported memory size: 93kB"]
    #[inline(always)]
    pub fn mem93kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem93kb)
    }
    #[doc = "Actual supported memory size: 94kB"]
    #[inline(always)]
    pub fn mem94kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem94kb)
    }
    #[doc = "Actual supported memory size: 95kB"]
    #[inline(always)]
    pub fn mem95kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem95kb)
    }
    #[doc = "Actual supported memory size: 96kB"]
    #[inline(always)]
    pub fn mem96kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem96kb)
    }
    #[doc = "Actual supported memory size: 97kB"]
    #[inline(always)]
    pub fn mem97kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem97kb)
    }
    #[doc = "Actual supported memory size: 98kB"]
    #[inline(always)]
    pub fn mem98kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem98kb)
    }
    #[doc = "Actual supported memory size: 99kB"]
    #[inline(always)]
    pub fn mem99kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem99kb)
    }
    #[doc = "Actual supported memory size: 100kB"]
    #[inline(always)]
    pub fn mem100kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem100kb)
    }
    #[doc = "Actual supported memory size: 101kB"]
    #[inline(always)]
    pub fn mem101kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem101kb)
    }
    #[doc = "Actual supported memory size: 102kB"]
    #[inline(always)]
    pub fn mem102kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem102kb)
    }
    #[doc = "Actual supported memory size: 103kB"]
    #[inline(always)]
    pub fn mem103kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem103kb)
    }
    #[doc = "Actual supported memory size: 104kB"]
    #[inline(always)]
    pub fn mem104kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem104kb)
    }
    #[doc = "Actual supported memory size: 105kB"]
    #[inline(always)]
    pub fn mem105kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem105kb)
    }
    #[doc = "Actual supported memory size: 106kB"]
    #[inline(always)]
    pub fn mem106kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem106kb)
    }
    #[doc = "Actual supported memory size: 107kB"]
    #[inline(always)]
    pub fn mem107kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem107kb)
    }
    #[doc = "Actual supported memory size: 108kB"]
    #[inline(always)]
    pub fn mem108kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem108kb)
    }
    #[doc = "Actual supported memory size: 109kB"]
    #[inline(always)]
    pub fn mem109kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem109kb)
    }
    #[doc = "Actual supported memory size: 110kB"]
    #[inline(always)]
    pub fn mem110kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem110kb)
    }
    #[doc = "Actual supported memory size: 111kB"]
    #[inline(always)]
    pub fn mem111kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem111kb)
    }
    #[doc = "Actual supported memory size: 112kB"]
    #[inline(always)]
    pub fn mem112kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem112kb)
    }
    #[doc = "Actual supported memory size: 113kB"]
    #[inline(always)]
    pub fn mem113kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem113kb)
    }
    #[doc = "Actual supported memory size: 114kB"]
    #[inline(always)]
    pub fn mem114kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem114kb)
    }
    #[doc = "Actual supported memory size: 115kB"]
    #[inline(always)]
    pub fn mem115kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem115kb)
    }
    #[doc = "Actual supported memory size: 116kB"]
    #[inline(always)]
    pub fn mem116kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem116kb)
    }
    #[doc = "Actual supported memory size: 117kB"]
    #[inline(always)]
    pub fn mem117kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem117kb)
    }
    #[doc = "Actual supported memory size: 118kB"]
    #[inline(always)]
    pub fn mem118kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem118kb)
    }
    #[doc = "Actual supported memory size: 119kB"]
    #[inline(always)]
    pub fn mem119kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem119kb)
    }
    #[doc = "Actual supported memory size: 120kB"]
    #[inline(always)]
    pub fn mem120kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem120kb)
    }
    #[doc = "Actual supported memory size: 121kB"]
    #[inline(always)]
    pub fn mem121kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem121kb)
    }
    #[doc = "Actual supported memory size: 122kB"]
    #[inline(always)]
    pub fn mem122kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem122kb)
    }
    #[doc = "Actual supported memory size: 123kB"]
    #[inline(always)]
    pub fn mem123kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem123kb)
    }
    #[doc = "Actual supported memory size: 124kB"]
    #[inline(always)]
    pub fn mem124kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem124kb)
    }
    #[doc = "Actual supported memory size: 125kB"]
    #[inline(always)]
    pub fn mem125kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem125kb)
    }
    #[doc = "Actual supported memory size: 126kB"]
    #[inline(always)]
    pub fn mem126kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem126kb)
    }
    #[doc = "Actual supported memory size: 127kB"]
    #[inline(always)]
    pub fn mem127kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem127kb)
    }
    #[doc = "Actual supported memory size: 128kB"]
    #[inline(always)]
    pub fn mem128kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem128kb)
    }
    #[doc = "Actual supported memory size: 129kB"]
    #[inline(always)]
    pub fn mem129kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem129kb)
    }
    #[doc = "Actual supported memory size: 130kB"]
    #[inline(always)]
    pub fn mem130kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem130kb)
    }
    #[doc = "Actual supported memory size: 131kB"]
    #[inline(always)]
    pub fn mem131kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem131kb)
    }
    #[doc = "Actual supported memory size: 132kB"]
    #[inline(always)]
    pub fn mem132kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem132kb)
    }
    #[doc = "Actual supported memory size: 133kB"]
    #[inline(always)]
    pub fn mem133kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem133kb)
    }
    #[doc = "Actual supported memory size: 134kB"]
    #[inline(always)]
    pub fn mem134kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem134kb)
    }
    #[doc = "Actual supported memory size: 135kB"]
    #[inline(always)]
    pub fn mem135kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem135kb)
    }
    #[doc = "Actual supported memory size: 136kB"]
    #[inline(always)]
    pub fn mem136kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem136kb)
    }
    #[doc = "Actual supported memory size: 137kB"]
    #[inline(always)]
    pub fn mem137kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem137kb)
    }
    #[doc = "Actual supported memory size: 138kB"]
    #[inline(always)]
    pub fn mem138kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem138kb)
    }
    #[doc = "Actual supported memory size: 139kB"]
    #[inline(always)]
    pub fn mem139kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem139kb)
    }
    #[doc = "Actual supported memory size: 140kB"]
    #[inline(always)]
    pub fn mem140kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem140kb)
    }
    #[doc = "Actual supported memory size: 141kB"]
    #[inline(always)]
    pub fn mem141kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem141kb)
    }
    #[doc = "Actual supported memory size: 142kB"]
    #[inline(always)]
    pub fn mem142kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem142kb)
    }
    #[doc = "Actual supported memory size: 143kB"]
    #[inline(always)]
    pub fn mem143kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem143kb)
    }
    #[doc = "Actual supported memory size: 144kB"]
    #[inline(always)]
    pub fn mem144kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem144kb)
    }
    #[doc = "Actual supported memory size: 145kB"]
    #[inline(always)]
    pub fn mem145kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem145kb)
    }
    #[doc = "Actual supported memory size: 146kB"]
    #[inline(always)]
    pub fn mem146kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem146kb)
    }
    #[doc = "Actual supported memory size: 147kB"]
    #[inline(always)]
    pub fn mem147kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem147kb)
    }
    #[doc = "Actual supported memory size: 148kB"]
    #[inline(always)]
    pub fn mem148kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem148kb)
    }
    #[doc = "Actual supported memory size: 149kB"]
    #[inline(always)]
    pub fn mem149kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem149kb)
    }
    #[doc = "Actual supported memory size: 150kB"]
    #[inline(always)]
    pub fn mem150kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem150kb)
    }
    #[doc = "Actual supported memory size: 151kB"]
    #[inline(always)]
    pub fn mem151kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem151kb)
    }
    #[doc = "Actual supported memory size: 152kB"]
    #[inline(always)]
    pub fn mem152kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem152kb)
    }
    #[doc = "Actual supported memory size: 153kB"]
    #[inline(always)]
    pub fn mem153kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem153kb)
    }
    #[doc = "Actual supported memory size: 154kB"]
    #[inline(always)]
    pub fn mem154kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem154kb)
    }
    #[doc = "Actual supported memory size: 155kB"]
    #[inline(always)]
    pub fn mem155kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem155kb)
    }
    #[doc = "Actual supported memory size: 156kB"]
    #[inline(always)]
    pub fn mem156kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem156kb)
    }
    #[doc = "Actual supported memory size: 157kB"]
    #[inline(always)]
    pub fn mem157kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem157kb)
    }
    #[doc = "Actual supported memory size: 158kB"]
    #[inline(always)]
    pub fn mem158kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem158kb)
    }
    #[doc = "Actual supported memory size: 159kB"]
    #[inline(always)]
    pub fn mem159kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem159kb)
    }
    #[doc = "Actual supported memory size: 160kB"]
    #[inline(always)]
    pub fn mem160kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem160kb)
    }
    #[doc = "Actual supported memory size: 161kB"]
    #[inline(always)]
    pub fn mem161kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem161kb)
    }
    #[doc = "Actual supported memory size: 162kB"]
    #[inline(always)]
    pub fn mem162kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem162kb)
    }
    #[doc = "Actual supported memory size: 163kB"]
    #[inline(always)]
    pub fn mem163kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem163kb)
    }
    #[doc = "Actual supported memory size: 164kB"]
    #[inline(always)]
    pub fn mem164kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem164kb)
    }
    #[doc = "Actual supported memory size: 165kB"]
    #[inline(always)]
    pub fn mem165kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem165kb)
    }
    #[doc = "Actual supported memory size: 166kB"]
    #[inline(always)]
    pub fn mem166kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem166kb)
    }
    #[doc = "Actual supported memory size: 167kB"]
    #[inline(always)]
    pub fn mem167kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem167kb)
    }
    #[doc = "Actual supported memory size: 168kB"]
    #[inline(always)]
    pub fn mem168kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem168kb)
    }
    #[doc = "Actual supported memory size: 169kB"]
    #[inline(always)]
    pub fn mem169kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem169kb)
    }
    #[doc = "Actual supported memory size: 170kB"]
    #[inline(always)]
    pub fn mem170kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem170kb)
    }
    #[doc = "Actual supported memory size: 171kB"]
    #[inline(always)]
    pub fn mem171kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem171kb)
    }
    #[doc = "Actual supported memory size: 172kB"]
    #[inline(always)]
    pub fn mem172kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem172kb)
    }
    #[doc = "Actual supported memory size: 173kB"]
    #[inline(always)]
    pub fn mem173kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem173kb)
    }
    #[doc = "Actual supported memory size: 174kB"]
    #[inline(always)]
    pub fn mem174kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem174kb)
    }
    #[doc = "Actual supported memory size: 175kB"]
    #[inline(always)]
    pub fn mem175kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem175kb)
    }
    #[doc = "Actual supported memory size: 176kB"]
    #[inline(always)]
    pub fn mem176kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem176kb)
    }
    #[doc = "Actual supported memory size: 177kB"]
    #[inline(always)]
    pub fn mem177kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem177kb)
    }
    #[doc = "Actual supported memory size: 178kB"]
    #[inline(always)]
    pub fn mem178kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem178kb)
    }
    #[doc = "Actual supported memory size: 179kB"]
    #[inline(always)]
    pub fn mem179kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem179kb)
    }
    #[doc = "Actual supported memory size: 180kB"]
    #[inline(always)]
    pub fn mem180kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem180kb)
    }
    #[doc = "Actual supported memory size: 181kB"]
    #[inline(always)]
    pub fn mem181kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem181kb)
    }
    #[doc = "Actual supported memory size: 182kB"]
    #[inline(always)]
    pub fn mem182kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem182kb)
    }
    #[doc = "Actual supported memory size: 183kB"]
    #[inline(always)]
    pub fn mem183kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem183kb)
    }
    #[doc = "Actual supported memory size: 184kB"]
    #[inline(always)]
    pub fn mem184kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem184kb)
    }
    #[doc = "Actual supported memory size: 185kB"]
    #[inline(always)]
    pub fn mem185kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem185kb)
    }
    #[doc = "Actual supported memory size: 186kB"]
    #[inline(always)]
    pub fn mem186kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem186kb)
    }
    #[doc = "Actual supported memory size: 187kB"]
    #[inline(always)]
    pub fn mem187kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem187kb)
    }
    #[doc = "Actual supported memory size: 188kB"]
    #[inline(always)]
    pub fn mem188kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem188kb)
    }
    #[doc = "Actual supported memory size: 189kB"]
    #[inline(always)]
    pub fn mem189kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem189kb)
    }
    #[doc = "Actual supported memory size: 190kB"]
    #[inline(always)]
    pub fn mem190kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem190kb)
    }
    #[doc = "Actual supported memory size: 191kB"]
    #[inline(always)]
    pub fn mem191kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem191kb)
    }
    #[doc = "Actual supported memory size: 192kB"]
    #[inline(always)]
    pub fn mem192kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem192kb)
    }
    #[doc = "Actual supported memory size: 193kB"]
    #[inline(always)]
    pub fn mem193kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem193kb)
    }
    #[doc = "Actual supported memory size: 194kB"]
    #[inline(always)]
    pub fn mem194kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem194kb)
    }
    #[doc = "Actual supported memory size: 195kB"]
    #[inline(always)]
    pub fn mem195kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem195kb)
    }
    #[doc = "Actual supported memory size: 196kB"]
    #[inline(always)]
    pub fn mem196kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem196kb)
    }
    #[doc = "Actual supported memory size: 197kB"]
    #[inline(always)]
    pub fn mem197kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem197kb)
    }
    #[doc = "Actual supported memory size: 198kB"]
    #[inline(always)]
    pub fn mem198kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem198kb)
    }
    #[doc = "Actual supported memory size: 199kB"]
    #[inline(always)]
    pub fn mem199kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem199kb)
    }
    #[doc = "Actual supported memory size: 200kB"]
    #[inline(always)]
    pub fn mem200kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem200kb)
    }
    #[doc = "Actual supported memory size: 201kB"]
    #[inline(always)]
    pub fn mem201kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem201kb)
    }
    #[doc = "Actual supported memory size: 202kB"]
    #[inline(always)]
    pub fn mem202kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem202kb)
    }
    #[doc = "Actual supported memory size: 203kB"]
    #[inline(always)]
    pub fn mem203kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem203kb)
    }
    #[doc = "Actual supported memory size: 204kB"]
    #[inline(always)]
    pub fn mem204kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem204kb)
    }
    #[doc = "Actual supported memory size: 205kB"]
    #[inline(always)]
    pub fn mem205kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem205kb)
    }
    #[doc = "Actual supported memory size: 206kB"]
    #[inline(always)]
    pub fn mem206kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem206kb)
    }
    #[doc = "Actual supported memory size: 207kB"]
    #[inline(always)]
    pub fn mem207kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem207kb)
    }
    #[doc = "Actual supported memory size: 208kB"]
    #[inline(always)]
    pub fn mem208kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem208kb)
    }
    #[doc = "Actual supported memory size: 209kB"]
    #[inline(always)]
    pub fn mem209kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem209kb)
    }
    #[doc = "Actual supported memory size: 210kB"]
    #[inline(always)]
    pub fn mem210kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem210kb)
    }
    #[doc = "Actual supported memory size: 211kB"]
    #[inline(always)]
    pub fn mem211kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem211kb)
    }
    #[doc = "Actual supported memory size: 212kB"]
    #[inline(always)]
    pub fn mem212kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem212kb)
    }
    #[doc = "Actual supported memory size: 213kB"]
    #[inline(always)]
    pub fn mem213kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem213kb)
    }
    #[doc = "Actual supported memory size: 214kB"]
    #[inline(always)]
    pub fn mem214kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem214kb)
    }
    #[doc = "Actual supported memory size: 215kB"]
    #[inline(always)]
    pub fn mem215kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem215kb)
    }
    #[doc = "Actual supported memory size: 216kB"]
    #[inline(always)]
    pub fn mem216kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem216kb)
    }
    #[doc = "Actual supported memory size: 217kB"]
    #[inline(always)]
    pub fn mem217kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem217kb)
    }
    #[doc = "Actual supported memory size: 218kB"]
    #[inline(always)]
    pub fn mem218kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem218kb)
    }
    #[doc = "Actual supported memory size: 219kB"]
    #[inline(always)]
    pub fn mem219kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem219kb)
    }
    #[doc = "Actual supported memory size: 220kB"]
    #[inline(always)]
    pub fn mem220kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem220kb)
    }
    #[doc = "Actual supported memory size: 221kB"]
    #[inline(always)]
    pub fn mem221kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem221kb)
    }
    #[doc = "Actual supported memory size: 222kB"]
    #[inline(always)]
    pub fn mem222kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem222kb)
    }
    #[doc = "Actual supported memory size: 223kB"]
    #[inline(always)]
    pub fn mem223kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem223kb)
    }
    #[doc = "Actual supported memory size: 224kB"]
    #[inline(always)]
    pub fn mem224kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem224kb)
    }
    #[doc = "Actual supported memory size: 225kB"]
    #[inline(always)]
    pub fn mem225kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem225kb)
    }
    #[doc = "Actual supported memory size: 226kB"]
    #[inline(always)]
    pub fn mem226kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem226kb)
    }
    #[doc = "Actual supported memory size: 227kB"]
    #[inline(always)]
    pub fn mem227kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem227kb)
    }
    #[doc = "Actual supported memory size: 228kB"]
    #[inline(always)]
    pub fn mem228kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem228kb)
    }
    #[doc = "Actual supported memory size: 229kB"]
    #[inline(always)]
    pub fn mem229kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem229kb)
    }
    #[doc = "Actual supported memory size: 230kB"]
    #[inline(always)]
    pub fn mem230kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem230kb)
    }
    #[doc = "Actual supported memory size: 231kB"]
    #[inline(always)]
    pub fn mem231kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem231kb)
    }
    #[doc = "Actual supported memory size: 232kB"]
    #[inline(always)]
    pub fn mem232kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem232kb)
    }
    #[doc = "Actual supported memory size: 233kB"]
    #[inline(always)]
    pub fn mem233kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem233kb)
    }
    #[doc = "Actual supported memory size: 234kB"]
    #[inline(always)]
    pub fn mem234kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem234kb)
    }
    #[doc = "Actual supported memory size: 235kB"]
    #[inline(always)]
    pub fn mem235kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem235kb)
    }
    #[doc = "Actual supported memory size: 236kB"]
    #[inline(always)]
    pub fn mem236kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem236kb)
    }
    #[doc = "Actual supported memory size: 237kB"]
    #[inline(always)]
    pub fn mem237kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem237kb)
    }
    #[doc = "Actual supported memory size: 238kB"]
    #[inline(always)]
    pub fn mem238kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem238kb)
    }
    #[doc = "Actual supported memory size: 239kB"]
    #[inline(always)]
    pub fn mem239kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem239kb)
    }
    #[doc = "Actual supported memory size: 240kB"]
    #[inline(always)]
    pub fn mem240kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem240kb)
    }
    #[doc = "Actual supported memory size: 241kB"]
    #[inline(always)]
    pub fn mem241kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem241kb)
    }
    #[doc = "Actual supported memory size: 242kB"]
    #[inline(always)]
    pub fn mem242kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem242kb)
    }
    #[doc = "Actual supported memory size: 243kB"]
    #[inline(always)]
    pub fn mem243kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem243kb)
    }
    #[doc = "Actual supported memory size: 244kB"]
    #[inline(always)]
    pub fn mem244kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem244kb)
    }
    #[doc = "Actual supported memory size: 245kB"]
    #[inline(always)]
    pub fn mem245kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem245kb)
    }
    #[doc = "Actual supported memory size: 246kB"]
    #[inline(always)]
    pub fn mem246kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem246kb)
    }
    #[doc = "Actual supported memory size: 247kB"]
    #[inline(always)]
    pub fn mem247kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem247kb)
    }
    #[doc = "Actual supported memory size: 248kB"]
    #[inline(always)]
    pub fn mem248kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem248kb)
    }
    #[doc = "Actual supported memory size: 249kB"]
    #[inline(always)]
    pub fn mem249kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem249kb)
    }
    #[doc = "Actual supported memory size: 250kB"]
    #[inline(always)]
    pub fn mem250kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem250kb)
    }
    #[doc = "Actual supported memory size: 251kB"]
    #[inline(always)]
    pub fn mem251kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem251kb)
    }
    #[doc = "Actual supported memory size: 252kB"]
    #[inline(always)]
    pub fn mem252kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem252kb)
    }
    #[doc = "Actual supported memory size: 253kB"]
    #[inline(always)]
    pub fn mem253kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem253kb)
    }
    #[doc = "Actual supported memory size: 254kB"]
    #[inline(always)]
    pub fn mem254kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem254kb)
    }
    #[doc = "Actual supported memory size: 255kB"]
    #[inline(always)]
    pub fn mem255kb(self) -> &'a mut crate::W<REG> {
        self.variant(ActualMemSize::Mem255kb)
    }
}
#[doc = "Max supported memory size.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MaxMemSize {
    #[doc = "8: Support for 4kB memory."]
    Mem4kb = 8,
    #[doc = "9: Support for 8kB memory."]
    Mem8kb = 9,
    #[doc = "10: Support for 16kB memory."]
    Mem16kb = 10,
    #[doc = "11: Support for 32kB memory."]
    Mem32kb = 11,
    #[doc = "12: Support for 64kB memory."]
    Mem64kb = 12,
    #[doc = "13: Support for 128kB memory."]
    Mem128kb = 13,
    #[doc = "14: Support for 256kB memory."]
    Mem256kb = 14,
}
impl From<MaxMemSize> for u8 {
    #[inline(always)]
    fn from(variant: MaxMemSize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MaxMemSize {
    type Ux = u8;
}
#[doc = "Field `max_mem_size` reader - Max supported memory size."]
pub type MaxMemSizeR = crate::FieldReader<MaxMemSize>;
impl MaxMemSizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MaxMemSize> {
        match self.bits {
            8 => Some(MaxMemSize::Mem4kb),
            9 => Some(MaxMemSize::Mem8kb),
            10 => Some(MaxMemSize::Mem16kb),
            11 => Some(MaxMemSize::Mem32kb),
            12 => Some(MaxMemSize::Mem64kb),
            13 => Some(MaxMemSize::Mem128kb),
            14 => Some(MaxMemSize::Mem256kb),
            _ => None,
        }
    }
    #[doc = "Support for 4kB memory."]
    #[inline(always)]
    pub fn is_mem4kb(&self) -> bool {
        *self == MaxMemSize::Mem4kb
    }
    #[doc = "Support for 8kB memory."]
    #[inline(always)]
    pub fn is_mem8kb(&self) -> bool {
        *self == MaxMemSize::Mem8kb
    }
    #[doc = "Support for 16kB memory."]
    #[inline(always)]
    pub fn is_mem16kb(&self) -> bool {
        *self == MaxMemSize::Mem16kb
    }
    #[doc = "Support for 32kB memory."]
    #[inline(always)]
    pub fn is_mem32kb(&self) -> bool {
        *self == MaxMemSize::Mem32kb
    }
    #[doc = "Support for 64kB memory."]
    #[inline(always)]
    pub fn is_mem64kb(&self) -> bool {
        *self == MaxMemSize::Mem64kb
    }
    #[doc = "Support for 128kB memory."]
    #[inline(always)]
    pub fn is_mem128kb(&self) -> bool {
        *self == MaxMemSize::Mem128kb
    }
    #[doc = "Support for 256kB memory."]
    #[inline(always)]
    pub fn is_mem256kb(&self) -> bool {
        *self == MaxMemSize::Mem256kb
    }
}
#[doc = "Field `max_mem_size` writer - Max supported memory size."]
pub type MaxMemSizeW<'a, REG> = crate::FieldWriter<'a, REG, 4, MaxMemSize>;
impl<'a, REG> MaxMemSizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Support for 4kB memory."]
    #[inline(always)]
    pub fn mem4kb(self) -> &'a mut crate::W<REG> {
        self.variant(MaxMemSize::Mem4kb)
    }
    #[doc = "Support for 8kB memory."]
    #[inline(always)]
    pub fn mem8kb(self) -> &'a mut crate::W<REG> {
        self.variant(MaxMemSize::Mem8kb)
    }
    #[doc = "Support for 16kB memory."]
    #[inline(always)]
    pub fn mem16kb(self) -> &'a mut crate::W<REG> {
        self.variant(MaxMemSize::Mem16kb)
    }
    #[doc = "Support for 32kB memory."]
    #[inline(always)]
    pub fn mem32kb(self) -> &'a mut crate::W<REG> {
        self.variant(MaxMemSize::Mem32kb)
    }
    #[doc = "Support for 64kB memory."]
    #[inline(always)]
    pub fn mem64kb(self) -> &'a mut crate::W<REG> {
        self.variant(MaxMemSize::Mem64kb)
    }
    #[doc = "Support for 128kB memory."]
    #[inline(always)]
    pub fn mem128kb(self) -> &'a mut crate::W<REG> {
        self.variant(MaxMemSize::Mem128kb)
    }
    #[doc = "Support for 256kB memory."]
    #[inline(always)]
    pub fn mem256kb(self) -> &'a mut crate::W<REG> {
        self.variant(MaxMemSize::Mem256kb)
    }
}
impl R {
    #[doc = "Bits 0:7 - The actual size of the connnected on-chip RAM memory in kB - 0: 256kB, 1-255: 1-255kB."]
    #[inline(always)]
    pub fn actual_mem_size(&self) -> ActualMemSizeR {
        ActualMemSizeR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - Max supported memory size."]
    #[inline(always)]
    pub fn max_mem_size(&self) -> MaxMemSizeR {
        MaxMemSizeR::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - The actual size of the connnected on-chip RAM memory in kB - 0: 256kB, 1-255: 1-255kB."]
    #[inline(always)]
    #[must_use]
    pub fn actual_mem_size(&mut self) -> ActualMemSizeW<Cap2Spec> {
        ActualMemSizeW::new(self, 0)
    }
    #[doc = "Bits 8:11 - Max supported memory size."]
    #[inline(always)]
    #[must_use]
    pub fn max_mem_size(&mut self) -> MaxMemSizeW<Cap2Spec> {
        MaxMemSizeW::new(self, 8)
    }
}
#[doc = "USB3 Global capability 2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cap2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cap2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cap2Spec;
impl crate::RegisterSpec for Cap2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cap2::R`](R) reader structure"]
impl crate::Readable for Cap2Spec {}
#[doc = "`write(|w| ..)` method takes [`cap2::W`](W) writer structure"]
impl crate::Writable for Cap2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets cap2 to value 0"]
impl crate::Resettable for Cap2Spec {
    const RESET_VALUE: u32 = 0;
}
