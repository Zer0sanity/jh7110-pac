#[doc = "Register `enbld_chns` reader"]
pub type R = crate::R<EnbldChnsSpec>;
#[doc = "Field `enabled_channels` reader - Channel enable status."]
pub type EnabledChannelsR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Channel enable status."]
    #[inline(always)]
    pub fn enabled_channels(&self) -> EnabledChannelsR {
        EnabledChannelsR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "DMA Enabled Channels register - indicates the DMA channels that are enabled, as indicated by the Enable bit in the DMACCxConfiguration Register. A HIGH bit indicates that a DMA channel is enabled. A bit is cleared on completion of the DMA transfer.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enbld_chns::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EnbldChnsSpec;
impl crate::RegisterSpec for EnbldChnsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`enbld_chns::R`](R) reader structure"]
impl crate::Readable for EnbldChnsSpec {}
#[doc = "`reset()` method sets enbld_chns to value 0"]
impl crate::Resettable for EnbldChnsSpec {
    const RESET_VALUE: u32 = 0;
}
