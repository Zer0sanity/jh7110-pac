#[doc = "Register `blksiz` reader"]
pub type R = crate::R<BlksizSpec>;
#[doc = "Register `blksiz` writer"]
pub type W = crate::W<BlksizSpec>;
#[doc = "Field `blksiz` reader - MMC block size"]
pub type BlksizR = crate::FieldReader<u32>;
#[doc = "Field `blksiz` writer - MMC block size"]
pub type BlksizW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - MMC block size"]
    #[inline(always)]
    pub fn blksiz(&self) -> BlksizR {
        BlksizR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - MMC block size"]
    #[inline(always)]
    #[must_use]
    pub fn blksiz(&mut self) -> BlksizW<BlksizSpec> {
        BlksizW::new(self, 0)
    }
}
#[doc = "MMC block size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blksiz::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blksiz::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlksizSpec;
impl crate::RegisterSpec for BlksizSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blksiz::R`](R) reader structure"]
impl crate::Readable for BlksizSpec {}
#[doc = "`write(|w| ..)` method takes [`blksiz::W`](W) writer structure"]
impl crate::Writable for BlksizSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets blksiz to value 0"]
impl crate::Resettable for BlksizSpec {
    const RESET_VALUE: u32 = 0;
}
