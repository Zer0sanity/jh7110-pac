#[doc = "Register `cmdarg` reader"]
pub type R = crate::R<CmdargSpec>;
#[doc = "Register `cmdarg` writer"]
pub type W = crate::W<CmdargSpec>;
#[doc = "Field `cmdarg` reader - MMC command argument"]
pub type CmdargR = crate::FieldReader<u32>;
#[doc = "Field `cmdarg` writer - MMC command argument"]
pub type CmdargW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - MMC command argument"]
    #[inline(always)]
    pub fn cmdarg(&self) -> CmdargR {
        CmdargR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - MMC command argument"]
    #[inline(always)]
    #[must_use]
    pub fn cmdarg(&mut self) -> CmdargW<CmdargSpec> {
        CmdargW::new(self, 0)
    }
}
#[doc = "MMC command argument\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdarg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdarg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdargSpec;
impl crate::RegisterSpec for CmdargSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmdarg::R`](R) reader structure"]
impl crate::Readable for CmdargSpec {}
#[doc = "`write(|w| ..)` method takes [`cmdarg::W`](W) writer structure"]
impl crate::Writable for CmdargSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets cmdarg to value 0"]
impl crate::Resettable for CmdargSpec {
    const RESET_VALUE: u32 = 0;
}
