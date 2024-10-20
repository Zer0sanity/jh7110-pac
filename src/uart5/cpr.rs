#[doc = "Register `cpr` reader"]
pub type R = crate::R<CprSpec>;
#[doc = "Field `apb_data_width` reader - 00 = 8 bits 01 = 16 bits 10 = 32 bits 11 = reserved"]
pub type ApbDataWidthR = crate::FieldReader;
#[doc = "Field `afce_mode` reader - 0 = false 1 = true"]
pub type AfceModeR = crate::BitReader;
#[doc = "Field `thre_mode` reader - 0 = false 1 = true"]
pub type ThreModeR = crate::BitReader;
#[doc = "Field `sir_mode` reader - 0 = false 1 = true"]
pub type SirModeR = crate::BitReader;
#[doc = "Field `sir_lp_mode` reader - 0 = false 1 = true"]
pub type SirLpModeR = crate::BitReader;
#[doc = "Field `additional_feat` reader - 0 = false 1 = true"]
pub type AdditionalFeatR = crate::BitReader;
#[doc = "Field `fifo_access` reader - 0 = false 1 = true"]
pub type FifoAccessR = crate::BitReader;
#[doc = "Field `fifo_stat` reader - 0 = false 1 = true"]
pub type FifoStatR = crate::BitReader;
#[doc = "Field `shadow` reader - 0 = false 1 = true"]
pub type ShadowR = crate::BitReader;
#[doc = "Field `uart_add_encoded_params` reader - 0 = false 1 = true"]
pub type UartAddEncodedParamsR = crate::BitReader;
#[doc = "Field `dma_extra` reader - 0 = false 1 = true"]
pub type DmaExtraR = crate::BitReader;
#[doc = "Field `fifo_mode` reader - 0x00 = 0 0x01 = 16 0x02 = 32 to 0x80 = 2048 0x81 - 0xff = reserved"]
pub type FifoModeR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - 00 = 8 bits 01 = 16 bits 10 = 32 bits 11 = reserved"]
    #[inline(always)]
    pub fn apb_data_width(&self) -> ApbDataWidthR {
        ApbDataWidthR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - 0 = false 1 = true"]
    #[inline(always)]
    pub fn afce_mode(&self) -> AfceModeR {
        AfceModeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 0 = false 1 = true"]
    #[inline(always)]
    pub fn thre_mode(&self) -> ThreModeR {
        ThreModeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 0 = false 1 = true"]
    #[inline(always)]
    pub fn sir_mode(&self) -> SirModeR {
        SirModeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 0 = false 1 = true"]
    #[inline(always)]
    pub fn sir_lp_mode(&self) -> SirLpModeR {
        SirLpModeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 0 = false 1 = true"]
    #[inline(always)]
    pub fn additional_feat(&self) -> AdditionalFeatR {
        AdditionalFeatR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 0 = false 1 = true"]
    #[inline(always)]
    pub fn fifo_access(&self) -> FifoAccessR {
        FifoAccessR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 0 = false 1 = true"]
    #[inline(always)]
    pub fn fifo_stat(&self) -> FifoStatR {
        FifoStatR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 0 = false 1 = true"]
    #[inline(always)]
    pub fn shadow(&self) -> ShadowR {
        ShadowR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 0 = false 1 = true"]
    #[inline(always)]
    pub fn uart_add_encoded_params(&self) -> UartAddEncodedParamsR {
        UartAddEncodedParamsR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 0 = false 1 = true"]
    #[inline(always)]
    pub fn dma_extra(&self) -> DmaExtraR {
        DmaExtraR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:23 - 0x00 = 0 0x01 = 16 0x02 = 32 to 0x80 = 2048 0x81 - 0xff = reserved"]
    #[inline(always)]
    pub fn fifo_mode(&self) -> FifoModeR {
        FifoModeR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "Component Parameter Register: This register is only valid when the DW_apb_uart is configured to have the Component Parameter register implemented (UART_ADD_ENCODED_PARAMS == YES). If the Component Parameter register is not implemented, this register does not exist and reading from this register address returns zero.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CprSpec;
impl crate::RegisterSpec for CprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpr::R`](R) reader structure"]
impl crate::Readable for CprSpec {}
#[doc = "`reset()` method sets cpr to value 0"]
impl crate::Resettable for CprSpec {
    const RESET_VALUE: u32 = 0;
}
