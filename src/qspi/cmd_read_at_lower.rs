#[doc = "Register `cmd_read_at_lower` reader"]
pub type R = crate::R<CmdReadAtLowerSpec>;
#[doc = "Register `cmd_read_at_lower` writer"]
pub type W = crate::W<CmdReadAtLowerSpec>;
#[doc = "Field `read_at_lower` reader - "]
pub type ReadAtLowerR = crate::FieldReader<u32>;
#[doc = "Field `read_at_lower` writer - "]
pub type ReadAtLowerW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn read_at_lower(&self) -> ReadAtLowerR {
        ReadAtLowerR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn read_at_lower(&mut self) -> ReadAtLowerW<CmdReadAtLowerSpec> {
        ReadAtLowerW::new(self, 0)
    }
}
#[doc = "Cadence QSPI Command Read at Lower\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd_read_at_lower::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd_read_at_lower::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdReadAtLowerSpec;
impl crate::RegisterSpec for CmdReadAtLowerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd_read_at_lower::R`](R) reader structure"]
impl crate::Readable for CmdReadAtLowerSpec {}
#[doc = "`write(|w| ..)` method takes [`cmd_read_at_lower::W`](W) writer structure"]
impl crate::Writable for CmdReadAtLowerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets cmd_read_at_lower to value 0"]
impl crate::Resettable for CmdReadAtLowerSpec {
    const RESET_VALUE: u32 = 0;
}
