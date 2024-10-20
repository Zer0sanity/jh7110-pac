#[doc = "Register `hcon` reader"]
pub type R = crate::R<HconSpec>;
#[doc = "Register `hcon` writer"]
pub type W = crate::W<HconSpec>;
#[doc = "MMC slot number\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SlotNum {
    #[doc = "0: MMC slot 0"]
    Slot0 = 0,
    #[doc = "1: MMC slot 1"]
    Slot1 = 1,
    #[doc = "2: MMC slot 2"]
    Slot2 = 2,
    #[doc = "3: MMC slot 3"]
    Slot3 = 3,
    #[doc = "4: MMC slot 4"]
    Slot4 = 4,
    #[doc = "5: MMC slot 5"]
    Slot5 = 5,
    #[doc = "6: MMC slot 6"]
    Slot6 = 6,
    #[doc = "7: MMC slot 7"]
    Slot7 = 7,
    #[doc = "8: MMC slot 8"]
    Slot8 = 8,
    #[doc = "9: MMC slot 9"]
    Slot9 = 9,
    #[doc = "10: MMC slot 10"]
    Slot10 = 10,
    #[doc = "11: MMC slot 11"]
    Slot11 = 11,
    #[doc = "12: MMC slot 12"]
    Slot12 = 12,
    #[doc = "13: MMC slot 13"]
    Slot13 = 13,
    #[doc = "14: MMC slot 14"]
    Slot14 = 14,
    #[doc = "15: MMC slot 15"]
    Slot15 = 15,
    #[doc = "16: MMC slot 16"]
    Slot16 = 16,
    #[doc = "17: MMC slot 17"]
    Slot17 = 17,
    #[doc = "18: MMC slot 18"]
    Slot18 = 18,
    #[doc = "19: MMC slot 19"]
    Slot19 = 19,
    #[doc = "20: MMC slot 20"]
    Slot20 = 20,
    #[doc = "21: MMC slot 21"]
    Slot21 = 21,
    #[doc = "22: MMC slot 22"]
    Slot22 = 22,
    #[doc = "23: MMC slot 23"]
    Slot23 = 23,
    #[doc = "24: MMC slot 24"]
    Slot24 = 24,
    #[doc = "25: MMC slot 25"]
    Slot25 = 25,
    #[doc = "26: MMC slot 26"]
    Slot26 = 26,
    #[doc = "27: MMC slot 27"]
    Slot27 = 27,
    #[doc = "28: MMC slot 28"]
    Slot28 = 28,
    #[doc = "29: MMC slot 29"]
    Slot29 = 29,
    #[doc = "30: MMC slot 30"]
    Slot30 = 30,
    #[doc = "31: MMC slot 31"]
    Slot31 = 31,
}
impl From<SlotNum> for u8 {
    #[inline(always)]
    fn from(variant: SlotNum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SlotNum {
    type Ux = u8;
}
#[doc = "Field `slot_num` reader - MMC slot number"]
pub type SlotNumR = crate::FieldReader<SlotNum>;
impl SlotNumR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SlotNum {
        match self.bits {
            0 => SlotNum::Slot0,
            1 => SlotNum::Slot1,
            2 => SlotNum::Slot2,
            3 => SlotNum::Slot3,
            4 => SlotNum::Slot4,
            5 => SlotNum::Slot5,
            6 => SlotNum::Slot6,
            7 => SlotNum::Slot7,
            8 => SlotNum::Slot8,
            9 => SlotNum::Slot9,
            10 => SlotNum::Slot10,
            11 => SlotNum::Slot11,
            12 => SlotNum::Slot12,
            13 => SlotNum::Slot13,
            14 => SlotNum::Slot14,
            15 => SlotNum::Slot15,
            16 => SlotNum::Slot16,
            17 => SlotNum::Slot17,
            18 => SlotNum::Slot18,
            19 => SlotNum::Slot19,
            20 => SlotNum::Slot20,
            21 => SlotNum::Slot21,
            22 => SlotNum::Slot22,
            23 => SlotNum::Slot23,
            24 => SlotNum::Slot24,
            25 => SlotNum::Slot25,
            26 => SlotNum::Slot26,
            27 => SlotNum::Slot27,
            28 => SlotNum::Slot28,
            29 => SlotNum::Slot29,
            30 => SlotNum::Slot30,
            31 => SlotNum::Slot31,
            _ => unreachable!(),
        }
    }
    #[doc = "MMC slot 0"]
    #[inline(always)]
    pub fn is_slot0(&self) -> bool {
        *self == SlotNum::Slot0
    }
    #[doc = "MMC slot 1"]
    #[inline(always)]
    pub fn is_slot1(&self) -> bool {
        *self == SlotNum::Slot1
    }
    #[doc = "MMC slot 2"]
    #[inline(always)]
    pub fn is_slot2(&self) -> bool {
        *self == SlotNum::Slot2
    }
    #[doc = "MMC slot 3"]
    #[inline(always)]
    pub fn is_slot3(&self) -> bool {
        *self == SlotNum::Slot3
    }
    #[doc = "MMC slot 4"]
    #[inline(always)]
    pub fn is_slot4(&self) -> bool {
        *self == SlotNum::Slot4
    }
    #[doc = "MMC slot 5"]
    #[inline(always)]
    pub fn is_slot5(&self) -> bool {
        *self == SlotNum::Slot5
    }
    #[doc = "MMC slot 6"]
    #[inline(always)]
    pub fn is_slot6(&self) -> bool {
        *self == SlotNum::Slot6
    }
    #[doc = "MMC slot 7"]
    #[inline(always)]
    pub fn is_slot7(&self) -> bool {
        *self == SlotNum::Slot7
    }
    #[doc = "MMC slot 8"]
    #[inline(always)]
    pub fn is_slot8(&self) -> bool {
        *self == SlotNum::Slot8
    }
    #[doc = "MMC slot 9"]
    #[inline(always)]
    pub fn is_slot9(&self) -> bool {
        *self == SlotNum::Slot9
    }
    #[doc = "MMC slot 10"]
    #[inline(always)]
    pub fn is_slot10(&self) -> bool {
        *self == SlotNum::Slot10
    }
    #[doc = "MMC slot 11"]
    #[inline(always)]
    pub fn is_slot11(&self) -> bool {
        *self == SlotNum::Slot11
    }
    #[doc = "MMC slot 12"]
    #[inline(always)]
    pub fn is_slot12(&self) -> bool {
        *self == SlotNum::Slot12
    }
    #[doc = "MMC slot 13"]
    #[inline(always)]
    pub fn is_slot13(&self) -> bool {
        *self == SlotNum::Slot13
    }
    #[doc = "MMC slot 14"]
    #[inline(always)]
    pub fn is_slot14(&self) -> bool {
        *self == SlotNum::Slot14
    }
    #[doc = "MMC slot 15"]
    #[inline(always)]
    pub fn is_slot15(&self) -> bool {
        *self == SlotNum::Slot15
    }
    #[doc = "MMC slot 16"]
    #[inline(always)]
    pub fn is_slot16(&self) -> bool {
        *self == SlotNum::Slot16
    }
    #[doc = "MMC slot 17"]
    #[inline(always)]
    pub fn is_slot17(&self) -> bool {
        *self == SlotNum::Slot17
    }
    #[doc = "MMC slot 18"]
    #[inline(always)]
    pub fn is_slot18(&self) -> bool {
        *self == SlotNum::Slot18
    }
    #[doc = "MMC slot 19"]
    #[inline(always)]
    pub fn is_slot19(&self) -> bool {
        *self == SlotNum::Slot19
    }
    #[doc = "MMC slot 20"]
    #[inline(always)]
    pub fn is_slot20(&self) -> bool {
        *self == SlotNum::Slot20
    }
    #[doc = "MMC slot 21"]
    #[inline(always)]
    pub fn is_slot21(&self) -> bool {
        *self == SlotNum::Slot21
    }
    #[doc = "MMC slot 22"]
    #[inline(always)]
    pub fn is_slot22(&self) -> bool {
        *self == SlotNum::Slot22
    }
    #[doc = "MMC slot 23"]
    #[inline(always)]
    pub fn is_slot23(&self) -> bool {
        *self == SlotNum::Slot23
    }
    #[doc = "MMC slot 24"]
    #[inline(always)]
    pub fn is_slot24(&self) -> bool {
        *self == SlotNum::Slot24
    }
    #[doc = "MMC slot 25"]
    #[inline(always)]
    pub fn is_slot25(&self) -> bool {
        *self == SlotNum::Slot25
    }
    #[doc = "MMC slot 26"]
    #[inline(always)]
    pub fn is_slot26(&self) -> bool {
        *self == SlotNum::Slot26
    }
    #[doc = "MMC slot 27"]
    #[inline(always)]
    pub fn is_slot27(&self) -> bool {
        *self == SlotNum::Slot27
    }
    #[doc = "MMC slot 28"]
    #[inline(always)]
    pub fn is_slot28(&self) -> bool {
        *self == SlotNum::Slot28
    }
    #[doc = "MMC slot 29"]
    #[inline(always)]
    pub fn is_slot29(&self) -> bool {
        *self == SlotNum::Slot29
    }
    #[doc = "MMC slot 30"]
    #[inline(always)]
    pub fn is_slot30(&self) -> bool {
        *self == SlotNum::Slot30
    }
    #[doc = "MMC slot 31"]
    #[inline(always)]
    pub fn is_slot31(&self) -> bool {
        *self == SlotNum::Slot31
    }
}
#[doc = "Field `slot_num` writer - MMC slot number"]
pub type SlotNumW<'a, REG> = crate::FieldWriterSafe<'a, REG, 5, SlotNum>;
impl<'a, REG> SlotNumW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MMC slot 0"]
    #[inline(always)]
    pub fn slot0(self) -> &'a mut crate::W<REG> {
        self.variant(SlotNum::Slot0)
    }
    #[doc = "MMC slot 1"]
    #[inline(always)]
    pub fn slot1(self) -> &'a mut crate::W<REG> {
        self.variant(SlotNum::Slot1)
    }
    #[doc = "MMC slot 2"]
    #[inline(always)]
    pub fn slot2(self) -> &'a mut crate::W<REG> {
        self.variant(SlotNum::Slot2)
    }
    #[doc = "MMC slot 3"]
    #[inline(always)]
    pub fn slot3(self) -> &'a mut crate::W<REG> {
        self.variant(SlotNum::Slot3)
    }
    #[doc = "MMC slot 4"]
    #[inline(always)]
    pub fn slot4(self) -> &'a mut crate::W<REG> {
        self.variant(SlotNum::Slot4)
    }
    #[doc = "MMC slot 5"]
    #[inline(always)]
    pub fn slot5(self) -> &'a mut crate::W<REG> {
        self.variant(SlotNum::Slot5)
    }
    #[doc = "MMC slot 6"]
    #[inline(always)]
    pub fn slot6(self) -> &'a mut crate::W<REG> {
        self.variant(SlotNum::Slot6)
    }
    #[doc = "MMC slot 7"]
    #[inline(always)]
    pub fn slot7(self) -> &'a mut crate::W<REG> {
        self.variant(SlotNum::Slot7)
    }
    #[doc = "MMC slot 8"]
    #[inline(always)]
    pub fn slot8(self) -> &'a mut crate::W<REG> {
        self.variant(SlotNum::Slot8)
    }
    #[doc = "MMC slot 9"]
    #[inline(always)]
    pub fn slot9(self) -> &'a mut crate::W<REG> {
        self.variant(SlotNum::Slot9)
    }
    #[doc = "MMC slot 10"]
    #[inline(always)]
    pub fn slot10(self) -> &'a mut crate::W<REG> {
        self.variant(SlotNum::Slot10)
    }
    #[doc = "MMC slot 11"]
    #[inline(always)]
    pub fn slot11(self) -> &'a mut crate::W<REG> {
        self.variant(SlotNum::Slot11)
    }
    #[doc = "MMC slot 12"]
    #[inline(always)]
    pub fn slot12(self) -> &'a mut crate::W<REG> {
        self.variant(SlotNum::Slot12)
    }
    #[doc = "MMC slot 13"]
    #[inline(always)]
    pub fn slot13(self) -> &'a mut crate::W<REG> {
        self.variant(SlotNum::Slot13)
    }
    #[doc = "MMC slot 14"]
    #[inline(always)]
    pub fn slot14(self) -> &'a mut crate::W<REG> {
        self.variant(SlotNum::Slot14)
    }
    #[doc = "MMC slot 15"]
    #[inline(always)]
    pub fn slot15(self) -> &'a mut crate::W<REG> {
        self.variant(SlotNum::Slot15)
    }
    #[doc = "MMC slot 16"]
    #[inline(always)]
    pub fn slot16(self) -> &'a mut crate::W<REG> {
        self.variant(SlotNum::Slot16)
    }
    #[doc = "MMC slot 17"]
    #[inline(always)]
    pub fn slot17(self) -> &'a mut crate::W<REG> {
        self.variant(SlotNum::Slot17)
    }
    #[doc = "MMC slot 18"]
    #[inline(always)]
    pub fn slot18(self) -> &'a mut crate::W<REG> {
        self.variant(SlotNum::Slot18)
    }
    #[doc = "MMC slot 19"]
    #[inline(always)]
    pub fn slot19(self) -> &'a mut crate::W<REG> {
        self.variant(SlotNum::Slot19)
    }
    #[doc = "MMC slot 20"]
    #[inline(always)]
    pub fn slot20(self) -> &'a mut crate::W<REG> {
        self.variant(SlotNum::Slot20)
    }
    #[doc = "MMC slot 21"]
    #[inline(always)]
    pub fn slot21(self) -> &'a mut crate::W<REG> {
        self.variant(SlotNum::Slot21)
    }
    #[doc = "MMC slot 22"]
    #[inline(always)]
    pub fn slot22(self) -> &'a mut crate::W<REG> {
        self.variant(SlotNum::Slot22)
    }
    #[doc = "MMC slot 23"]
    #[inline(always)]
    pub fn slot23(self) -> &'a mut crate::W<REG> {
        self.variant(SlotNum::Slot23)
    }
    #[doc = "MMC slot 24"]
    #[inline(always)]
    pub fn slot24(self) -> &'a mut crate::W<REG> {
        self.variant(SlotNum::Slot24)
    }
    #[doc = "MMC slot 25"]
    #[inline(always)]
    pub fn slot25(self) -> &'a mut crate::W<REG> {
        self.variant(SlotNum::Slot25)
    }
    #[doc = "MMC slot 26"]
    #[inline(always)]
    pub fn slot26(self) -> &'a mut crate::W<REG> {
        self.variant(SlotNum::Slot26)
    }
    #[doc = "MMC slot 27"]
    #[inline(always)]
    pub fn slot27(self) -> &'a mut crate::W<REG> {
        self.variant(SlotNum::Slot27)
    }
    #[doc = "MMC slot 28"]
    #[inline(always)]
    pub fn slot28(self) -> &'a mut crate::W<REG> {
        self.variant(SlotNum::Slot28)
    }
    #[doc = "MMC slot 29"]
    #[inline(always)]
    pub fn slot29(self) -> &'a mut crate::W<REG> {
        self.variant(SlotNum::Slot29)
    }
    #[doc = "MMC slot 30"]
    #[inline(always)]
    pub fn slot30(self) -> &'a mut crate::W<REG> {
        self.variant(SlotNum::Slot30)
    }
    #[doc = "MMC slot 31"]
    #[inline(always)]
    pub fn slot31(self) -> &'a mut crate::W<REG> {
        self.variant(SlotNum::Slot31)
    }
}
#[doc = "Field `hdata_width` reader - MMC HDATA width"]
pub type HdataWidthR = crate::FieldReader;
#[doc = "Field `hdata_width` writer - MMC HDATA width"]
pub type HdataWidthW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "MMC transfer mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TransMode {
    #[doc = "0: No DMA interface - using internal DMA block"]
    Idma = 0,
    #[doc = "1: DesignWare DMA interface"]
    Dwdma = 1,
    #[doc = "2: Generic DMA interface"]
    Gdma = 2,
    #[doc = "3: Non-DesignWare DMA interface - pio only"]
    Nodma = 3,
}
impl From<TransMode> for u8 {
    #[inline(always)]
    fn from(variant: TransMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TransMode {
    type Ux = u8;
}
#[doc = "Field `trans_mode` reader - MMC transfer mode"]
pub type TransModeR = crate::FieldReader<TransMode>;
impl TransModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TransMode {
        match self.bits {
            0 => TransMode::Idma,
            1 => TransMode::Dwdma,
            2 => TransMode::Gdma,
            3 => TransMode::Nodma,
            _ => unreachable!(),
        }
    }
    #[doc = "No DMA interface - using internal DMA block"]
    #[inline(always)]
    pub fn is_idma(&self) -> bool {
        *self == TransMode::Idma
    }
    #[doc = "DesignWare DMA interface"]
    #[inline(always)]
    pub fn is_dwdma(&self) -> bool {
        *self == TransMode::Dwdma
    }
    #[doc = "Generic DMA interface"]
    #[inline(always)]
    pub fn is_gdma(&self) -> bool {
        *self == TransMode::Gdma
    }
    #[doc = "Non-DesignWare DMA interface - pio only"]
    #[inline(always)]
    pub fn is_nodma(&self) -> bool {
        *self == TransMode::Nodma
    }
}
#[doc = "Field `trans_mode` writer - MMC transfer mode"]
pub type TransModeW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, TransMode>;
impl<'a, REG> TransModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No DMA interface - using internal DMA block"]
    #[inline(always)]
    pub fn idma(self) -> &'a mut crate::W<REG> {
        self.variant(TransMode::Idma)
    }
    #[doc = "DesignWare DMA interface"]
    #[inline(always)]
    pub fn dwdma(self) -> &'a mut crate::W<REG> {
        self.variant(TransMode::Dwdma)
    }
    #[doc = "Generic DMA interface"]
    #[inline(always)]
    pub fn gdma(self) -> &'a mut crate::W<REG> {
        self.variant(TransMode::Gdma)
    }
    #[doc = "Non-DesignWare DMA interface - pio only"]
    #[inline(always)]
    pub fn nodma(self) -> &'a mut crate::W<REG> {
        self.variant(TransMode::Nodma)
    }
}
#[doc = "MMC address config\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AddrConfig {
    #[doc = "0: IDMAC 32-bit address"]
    Addr32 = 0,
    #[doc = "1: IDMAC 64-bit address"]
    Addr64 = 1,
}
impl From<AddrConfig> for bool {
    #[inline(always)]
    fn from(variant: AddrConfig) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `addr_config` reader - MMC address config"]
pub type AddrConfigR = crate::BitReader<AddrConfig>;
impl AddrConfigR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AddrConfig {
        match self.bits {
            false => AddrConfig::Addr32,
            true => AddrConfig::Addr64,
        }
    }
    #[doc = "IDMAC 32-bit address"]
    #[inline(always)]
    pub fn is_addr32(&self) -> bool {
        *self == AddrConfig::Addr32
    }
    #[doc = "IDMAC 64-bit address"]
    #[inline(always)]
    pub fn is_addr64(&self) -> bool {
        *self == AddrConfig::Addr64
    }
}
#[doc = "Field `addr_config` writer - MMC address config"]
pub type AddrConfigW<'a, REG> = crate::BitWriter<'a, REG, AddrConfig>;
impl<'a, REG> AddrConfigW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IDMAC 32-bit address"]
    #[inline(always)]
    pub fn addr32(self) -> &'a mut crate::W<REG> {
        self.variant(AddrConfig::Addr32)
    }
    #[doc = "IDMAC 64-bit address"]
    #[inline(always)]
    pub fn addr64(self) -> &'a mut crate::W<REG> {
        self.variant(AddrConfig::Addr64)
    }
}
impl R {
    #[doc = "Bits 1:5 - MMC slot number"]
    #[inline(always)]
    pub fn slot_num(&self) -> SlotNumR {
        SlotNumR::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bits 7:9 - MMC HDATA width"]
    #[inline(always)]
    pub fn hdata_width(&self) -> HdataWidthR {
        HdataWidthR::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bits 16:17 - MMC transfer mode"]
    #[inline(always)]
    pub fn trans_mode(&self) -> TransModeR {
        TransModeR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 27 - MMC address config"]
    #[inline(always)]
    pub fn addr_config(&self) -> AddrConfigR {
        AddrConfigR::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 1:5 - MMC slot number"]
    #[inline(always)]
    #[must_use]
    pub fn slot_num(&mut self) -> SlotNumW<HconSpec> {
        SlotNumW::new(self, 1)
    }
    #[doc = "Bits 7:9 - MMC HDATA width"]
    #[inline(always)]
    #[must_use]
    pub fn hdata_width(&mut self) -> HdataWidthW<HconSpec> {
        HdataWidthW::new(self, 7)
    }
    #[doc = "Bits 16:17 - MMC transfer mode"]
    #[inline(always)]
    #[must_use]
    pub fn trans_mode(&mut self) -> TransModeW<HconSpec> {
        TransModeW::new(self, 16)
    }
    #[doc = "Bit 27 - MMC address config"]
    #[inline(always)]
    #[must_use]
    pub fn addr_config(&mut self) -> AddrConfigW<HconSpec> {
        AddrConfigW::new(self, 27)
    }
}
#[doc = "MMC HCON\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcon::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcon::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HconSpec;
impl crate::RegisterSpec for HconSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcon::R`](R) reader structure"]
impl crate::Readable for HconSpec {}
#[doc = "`write(|w| ..)` method takes [`hcon::W`](W) writer structure"]
impl crate::Writable for HconSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets hcon to value 0"]
impl crate::Resettable for HconSpec {
    const RESET_VALUE: u32 = 0;
}
