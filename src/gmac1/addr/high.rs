#[doc = "Register `high` reader"]
pub type R = crate::R<HighSpec>;
#[doc = "Register `high` writer"]
pub type W = crate::W<HighSpec>;
#[doc = "Field `addr` reader - Hardware Address High"]
pub type AddrR = crate::FieldReader<u16>;
#[doc = "Field `addr` writer - Hardware Address High"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `dcs` reader - Hardware Address High DCS"]
pub type DcsR = crate::FieldReader;
#[doc = "Field `dcs` writer - Hardware Address High DCS"]
pub type DcsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ae` reader - Hardware Address High Address Enable"]
pub type AeR = crate::BitReader;
#[doc = "Field `ae` writer - Hardware Address High Address Enable"]
pub type AeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Hardware Address High"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - Hardware Address High DCS"]
    #[inline(always)]
    pub fn dcs(&self) -> DcsR {
        DcsR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 31 - Hardware Address High Address Enable"]
    #[inline(always)]
    pub fn ae(&self) -> AeR {
        AeR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Hardware Address High"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> AddrW<HighSpec> {
        AddrW::new(self, 0)
    }
    #[doc = "Bits 16:18 - Hardware Address High DCS"]
    #[inline(always)]
    #[must_use]
    pub fn dcs(&mut self) -> DcsW<HighSpec> {
        DcsW::new(self, 16)
    }
    #[doc = "Bit 31 - Hardware Address High Address Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ae(&mut self) -> AeW<HighSpec> {
        AeW::new(self, 31)
    }
}
#[doc = "Hardware Address High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`high::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`high::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HighSpec;
impl crate::RegisterSpec for HighSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`high::R`](R) reader structure"]
impl crate::Readable for HighSpec {}
#[doc = "`write(|w| ..)` method takes [`high::W`](W) writer structure"]
impl crate::Writable for HighSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets high to value 0"]
impl crate::Resettable for HighSpec {
    const RESET_VALUE: u32 = 0;
}
