#[doc = "Register `ssp_cr0` reader"]
pub type R = crate::R<SspCr0Spec>;
#[doc = "Register `ssp_cr0` writer"]
pub type W = crate::W<SspCr0Spec>;
#[doc = "Field `dss` reader - Data Size Select - 0000, 0001, 0010: Reserved, 0011: 4-bit data, 0100: 5-bit data, 6-bit data, 0110: 7-bit data, 0111: 8-bit data, 1000: 9-bit data, 1001: 10-bit data, 1010: 11-bit data, 1011: 12-bit data, 1100: 13-bit data, 1101: 14-bit data, 1110: 15-bit data, 1111: 16-bit data"]
pub type DssR = crate::FieldReader;
#[doc = "Field `dss` writer - Data Size Select - 0000, 0001, 0010: Reserved, 0011: 4-bit data, 0100: 5-bit data, 6-bit data, 0110: 7-bit data, 0111: 8-bit data, 1000: 9-bit data, 1001: 10-bit data, 1010: 11-bit data, 1011: 12-bit data, 1100: 13-bit data, 1101: 14-bit data, 1110: 15-bit data, 1111: 16-bit data"]
pub type DssW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `frf` reader - Frame format - 00: Motorola SPI frame format, 01: TI synchronous serial frame format, 10: National Microwire frame format, 11: Reserved"]
pub type FrfR = crate::FieldReader;
#[doc = "Field `frf` writer - Frame format - 00: Motorola SPI frame format, 01: TI synchronous serial frame format, 10: National Microwire frame format, 11: Reserved"]
pub type FrfW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `spo` reader - SSPCLKOUT polarity, applicable to Motorola SPI frame format only."]
pub type SpoR = crate::BitReader;
#[doc = "Field `spo` writer - SSPCLKOUT polarity, applicable to Motorola SPI frame format only."]
pub type SpoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sph` reader - SSPCLKOUT phase, applicable to Motorola SPI frame format only."]
pub type SphR = crate::BitReader;
#[doc = "Field `sph` writer - SSPCLKOUT phase, applicable to Motorola SPI frame format only."]
pub type SphW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `scr` reader - Serial clock rate. The value SCR is used to generate the transmit and receive bit rate of the PrimeCell SSP. The bit rate is: (F\\[sspclk\\]
/ (CPSDVR * (1 + SCR))), where CPSDVSR is an even value from \\[2:254\\], programmed through the SSPCPSR register and SCR is a value from \\[0:255\\]"]
pub type ScrR = crate::FieldReader;
#[doc = "Field `scr` writer - Serial clock rate. The value SCR is used to generate the transmit and receive bit rate of the PrimeCell SSP. The bit rate is: (F\\[sspclk\\]
/ (CPSDVR * (1 + SCR))), where CPSDVSR is an even value from \\[2:254\\], programmed through the SSPCPSR register and SCR is a value from \\[0:255\\]"]
pub type ScrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:3 - Data Size Select - 0000, 0001, 0010: Reserved, 0011: 4-bit data, 0100: 5-bit data, 6-bit data, 0110: 7-bit data, 0111: 8-bit data, 1000: 9-bit data, 1001: 10-bit data, 1010: 11-bit data, 1011: 12-bit data, 1100: 13-bit data, 1101: 14-bit data, 1110: 15-bit data, 1111: 16-bit data"]
    #[inline(always)]
    pub fn dss(&self) -> DssR {
        DssR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Frame format - 00: Motorola SPI frame format, 01: TI synchronous serial frame format, 10: National Microwire frame format, 11: Reserved"]
    #[inline(always)]
    pub fn frf(&self) -> FrfR {
        FrfR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - SSPCLKOUT polarity, applicable to Motorola SPI frame format only."]
    #[inline(always)]
    pub fn spo(&self) -> SpoR {
        SpoR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SSPCLKOUT phase, applicable to Motorola SPI frame format only."]
    #[inline(always)]
    pub fn sph(&self) -> SphR {
        SphR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Serial clock rate. The value SCR is used to generate the transmit and receive bit rate of the PrimeCell SSP. The bit rate is: (F\\[sspclk\\]
/ (CPSDVR * (1 + SCR))), where CPSDVSR is an even value from \\[2:254\\], programmed through the SSPCPSR register and SCR is a value from \\[0:255\\]"]
    #[inline(always)]
    pub fn scr(&self) -> ScrR {
        ScrR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Data Size Select - 0000, 0001, 0010: Reserved, 0011: 4-bit data, 0100: 5-bit data, 6-bit data, 0110: 7-bit data, 0111: 8-bit data, 1000: 9-bit data, 1001: 10-bit data, 1010: 11-bit data, 1011: 12-bit data, 1100: 13-bit data, 1101: 14-bit data, 1110: 15-bit data, 1111: 16-bit data"]
    #[inline(always)]
    #[must_use]
    pub fn dss(&mut self) -> DssW<SspCr0Spec> {
        DssW::new(self, 0)
    }
    #[doc = "Bits 4:5 - Frame format - 00: Motorola SPI frame format, 01: TI synchronous serial frame format, 10: National Microwire frame format, 11: Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn frf(&mut self) -> FrfW<SspCr0Spec> {
        FrfW::new(self, 4)
    }
    #[doc = "Bit 6 - SSPCLKOUT polarity, applicable to Motorola SPI frame format only."]
    #[inline(always)]
    #[must_use]
    pub fn spo(&mut self) -> SpoW<SspCr0Spec> {
        SpoW::new(self, 6)
    }
    #[doc = "Bit 7 - SSPCLKOUT phase, applicable to Motorola SPI frame format only."]
    #[inline(always)]
    #[must_use]
    pub fn sph(&mut self) -> SphW<SspCr0Spec> {
        SphW::new(self, 7)
    }
    #[doc = "Bits 8:15 - Serial clock rate. The value SCR is used to generate the transmit and receive bit rate of the PrimeCell SSP. The bit rate is: (F\\[sspclk\\]
/ (CPSDVR * (1 + SCR))), where CPSDVSR is an even value from \\[2:254\\], programmed through the SSPCPSR register and SCR is a value from \\[0:255\\]"]
    #[inline(always)]
    #[must_use]
    pub fn scr(&mut self) -> ScrW<SspCr0Spec> {
        ScrW::new(self, 8)
    }
}
#[doc = "SSPCR0 is control register 0 and contains five bit fields that control various functions within the PrimeCell SSP.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssp_cr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssp_cr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SspCr0Spec;
impl crate::RegisterSpec for SspCr0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ssp_cr0::R`](R) reader structure"]
impl crate::Readable for SspCr0Spec {}
#[doc = "`write(|w| ..)` method takes [`ssp_cr0::W`](W) writer structure"]
impl crate::Writable for SspCr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets ssp_cr0 to value 0"]
impl crate::Resettable for SspCr0Spec {
    const RESET_VALUE: u16 = 0;
}
