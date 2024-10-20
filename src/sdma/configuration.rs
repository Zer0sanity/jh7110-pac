#[doc = "Register `configuration` reader"]
pub type R = crate::R<ConfigurationSpec>;
#[doc = "Register `configuration` writer"]
pub type W = crate::W<ConfigurationSpec>;
#[doc = "Field `e` reader - DMAC enable - 0: Diable, 1: Enable. This bit is reset to 0. Disabling the DMAC reduces power consumption."]
pub type ER = crate::BitReader;
#[doc = "Field `e` writer - DMAC enable - 0: Diable, 1: Enable. This bit is reset to 0. Disabling the DMAC reduces power consumption."]
pub type EW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `m(1-2)` reader - DMAC AHB Master - 0: little-endian mode, 1: big-endian mode. This bit is reset to 0."]
pub type MR = crate::BitReader;
#[doc = "Field `m(1-2)` writer - DMAC AHB Master - 0: little-endian mode, 1: big-endian mode. This bit is reset to 0."]
pub type MW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DMAC enable - 0: Diable, 1: Enable. This bit is reset to 0. Disabling the DMAC reduces power consumption."]
    #[inline(always)]
    pub fn e(&self) -> ER {
        ER::new((self.bits & 1) != 0)
    }
    #[doc = "DMAC AHB Master - 0: little-endian mode, 1: big-endian mode. This bit is reset to 0."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `m1` field"]
    #[inline(always)]
    pub fn m(&self, n: u8) -> MR {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        MR::new(((self.bits >> (n + 1)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "DMAC AHB Master - 0: little-endian mode, 1: big-endian mode. This bit is reset to 0."]
    #[inline(always)]
    pub fn m_iter(&self) -> impl Iterator<Item = MR> + '_ {
        (0..2).map(move |n| MR::new(((self.bits >> (n + 1)) & 1) != 0))
    }
    #[doc = "Bit 1 - DMAC AHB Master - 0: little-endian mode, 1: big-endian mode. This bit is reset to 0."]
    #[inline(always)]
    pub fn m1(&self) -> MR {
        MR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMAC AHB Master - 0: little-endian mode, 1: big-endian mode. This bit is reset to 0."]
    #[inline(always)]
    pub fn m2(&self) -> MR {
        MR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMAC enable - 0: Diable, 1: Enable. This bit is reset to 0. Disabling the DMAC reduces power consumption."]
    #[inline(always)]
    #[must_use]
    pub fn e(&mut self) -> EW<ConfigurationSpec> {
        EW::new(self, 0)
    }
    #[doc = "DMAC AHB Master - 0: little-endian mode, 1: big-endian mode. This bit is reset to 0."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `m1` field"]
    #[inline(always)]
    #[must_use]
    pub fn m(&mut self, n: u8) -> MW<ConfigurationSpec> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        MW::new(self, n + 1)
    }
    #[doc = "Bit 1 - DMAC AHB Master - 0: little-endian mode, 1: big-endian mode. This bit is reset to 0."]
    #[inline(always)]
    #[must_use]
    pub fn m1(&mut self) -> MW<ConfigurationSpec> {
        MW::new(self, 1)
    }
    #[doc = "Bit 2 - DMAC AHB Master - 0: little-endian mode, 1: big-endian mode. This bit is reset to 0."]
    #[inline(always)]
    #[must_use]
    pub fn m2(&mut self) -> MW<ConfigurationSpec> {
        MW::new(self, 2)
    }
}
#[doc = "DMA Configuration register - configures the operation of the DMAC. You can alter the endianness of the individual AHB master interfaces by writing to the M1 and M2 bits of this register. The M1 bit enables you to alter the endianness of AHB master interface 1. The M2 bit enables you to alter the endianness of AHB master interface 2. The AHB master interfaces are set to little-endian mode on reset.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`configuration::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`configuration::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConfigurationSpec;
impl crate::RegisterSpec for ConfigurationSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`configuration::R`](R) reader structure"]
impl crate::Readable for ConfigurationSpec {}
#[doc = "`write(|w| ..)` method takes [`configuration::W`](W) writer structure"]
impl crate::Writable for ConfigurationSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets configuration to value 0"]
impl crate::Resettable for ConfigurationSpec {
    const RESET_VALUE: u32 = 0;
}
