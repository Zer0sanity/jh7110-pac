#[doc = "Register `periph_id3` reader"]
pub type R = crate::R<PeriphId3Spec>;
#[doc = "Field `num_chan` reader - Indicates the number of channels - 0b000: 2 channels, 0b001: 4 channels, 0b010: 8 channels, 0b011: 16 channels, 0b100: 32 channels. This peripheral is set to 0b010."]
pub type NumChanR = crate::FieldReader;
#[doc = "Field `num_ahb` reader - Indicates the number of AHB masters - 0: one AHB interface, 1: two AHB interface"]
pub type NumAhbR = crate::BitReader;
#[doc = "Field `ahb_bus_width` reader - Indicates the AHB bus width - 0b000: 32-bit, 0b001: 64-bit, 0b010: 128-bit, 0b011: 256-bit, 0b100: 512-bit, 0b101: 1024-bit. This peripheral is set to 0b000."]
pub type AhbBusWidthR = crate::FieldReader;
#[doc = "Field `num_src_req` reader - Indicates the number of DMA source requestors for the DMAC configuration - 0: 16 DMA requestors, 1: 32 DMA requestors. This peripheral is set to 0."]
pub type NumSrcReqR = crate::BitReader;
impl R {
    #[doc = "Bits 0:2 - Indicates the number of channels - 0b000: 2 channels, 0b001: 4 channels, 0b010: 8 channels, 0b011: 16 channels, 0b100: 32 channels. This peripheral is set to 0b010."]
    #[inline(always)]
    pub fn num_chan(&self) -> NumChanR {
        NumChanR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Indicates the number of AHB masters - 0: one AHB interface, 1: two AHB interface"]
    #[inline(always)]
    pub fn num_ahb(&self) -> NumAhbR {
        NumAhbR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Indicates the AHB bus width - 0b000: 32-bit, 0b001: 64-bit, 0b010: 128-bit, 0b011: 256-bit, 0b100: 512-bit, 0b101: 1024-bit. This peripheral is set to 0b000."]
    #[inline(always)]
    pub fn ahb_bus_width(&self) -> AhbBusWidthR {
        AhbBusWidthR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Indicates the number of DMA source requestors for the DMAC configuration - 0: 16 DMA requestors, 1: 32 DMA requestors. This peripheral is set to 0."]
    #[inline(always)]
    pub fn num_src_req(&self) -> NumSrcReqR {
        NumSrcReqR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "DMA Peripheral ID 3 register - is hard-coded and the fields in the register determine the reset value.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`periph_id3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeriphId3Spec;
impl crate::RegisterSpec for PeriphId3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`periph_id3::R`](R) reader structure"]
impl crate::Readable for PeriphId3Spec {}
#[doc = "`reset()` method sets periph_id3 to value 0x06"]
impl crate::Resettable for PeriphId3Spec {
    const RESET_VALUE: u32 = 0x06;
}
