#[doc = "Register `iir` reader"]
pub type R = crate::R<IirSpec>;
#[doc = "Field `iid` reader - Interrupt ID. This indicates the highest priority pending interrupt which can be one of the following types: 0000 = modem status 0001 = no interrupt pending 0010 = THR empty 0100 = received data available 0110 = receiver line status 0111 = busy detect 1100 = character timeout The interrupt priorities are split into four levels that are detailed in Table 8 on page 97. Bit 3 indicates an interrupt can only occur when the FIFOs are enabled and used to distinguish a Character Timeout condition interrupt."]
pub type IidR = crate::FieldReader;
#[doc = "Field `fifose` reader - FIFOs Enabled. This is used to indicate whether the FIFOs are enabled or disabled. 00 = disabled 11 = enabled"]
pub type FifoseR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Interrupt ID. This indicates the highest priority pending interrupt which can be one of the following types: 0000 = modem status 0001 = no interrupt pending 0010 = THR empty 0100 = received data available 0110 = receiver line status 0111 = busy detect 1100 = character timeout The interrupt priorities are split into four levels that are detailed in Table 8 on page 97. Bit 3 indicates an interrupt can only occur when the FIFOs are enabled and used to distinguish a Character Timeout condition interrupt."]
    #[inline(always)]
    pub fn iid(&self) -> IidR {
        IidR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 6:7 - FIFOs Enabled. This is used to indicate whether the FIFOs are enabled or disabled. 00 = disabled 11 = enabled"]
    #[inline(always)]
    pub fn fifose(&self) -> FifoseR {
        FifoseR::new(((self.bits >> 6) & 3) as u8)
    }
}
#[doc = "Interrupt Identity Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iir::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IirSpec;
impl crate::RegisterSpec for IirSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iir::R`](R) reader structure"]
impl crate::Readable for IirSpec {}
#[doc = "`reset()` method sets iir to value 0x01"]
impl crate::Resettable for IirSpec {
    const RESET_VALUE: u32 = 0x01;
}
