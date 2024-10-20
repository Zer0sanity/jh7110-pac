#[doc = "Register `cmd_read_at_upper` reader"]
pub type R = crate::R<CmdReadAtUpperSpec>;
#[doc = "Register `cmd_read_at_upper` writer"]
pub type W = crate::W<CmdReadAtUpperSpec>;
#[doc = "Field `read_at_upper` reader - "]
pub type ReadAtUpperR = crate::FieldReader<u32>;
#[doc = "Field `read_at_upper` writer - "]
pub type ReadAtUpperW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn read_at_upper(&self) -> ReadAtUpperR {
        ReadAtUpperR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn read_at_upper(&mut self) -> ReadAtUpperW<CmdReadAtUpperSpec> {
        ReadAtUpperW::new(self, 0)
    }
}
#[doc = "Cadence QSPI Command Read at Upper\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd_read_at_upper::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd_read_at_upper::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdReadAtUpperSpec;
impl crate::RegisterSpec for CmdReadAtUpperSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd_read_at_upper::R`](R) reader structure"]
impl crate::Readable for CmdReadAtUpperSpec {}
#[doc = "`write(|w| ..)` method takes [`cmd_read_at_upper::W`](W) writer structure"]
impl crate::Writable for CmdReadAtUpperSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets cmd_read_at_upper to value 0"]
impl crate::Resettable for CmdReadAtUpperSpec {
    const RESET_VALUE: u32 = 0;
}
