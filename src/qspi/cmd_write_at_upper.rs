#[doc = "Register `cmd_write_at_upper` reader"]
pub type R = crate::R<CmdWriteAtUpperSpec>;
#[doc = "Register `cmd_write_at_upper` writer"]
pub type W = crate::W<CmdWriteAtUpperSpec>;
#[doc = "Field `write_at_upper` reader - "]
pub type WriteAtUpperR = crate::FieldReader<u32>;
#[doc = "Field `write_at_upper` writer - "]
pub type WriteAtUpperW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn write_at_upper(&self) -> WriteAtUpperR {
        WriteAtUpperR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn write_at_upper(&mut self) -> WriteAtUpperW<CmdWriteAtUpperSpec> {
        WriteAtUpperW::new(self, 0)
    }
}
#[doc = "Cadence QSPI Command Write at Upper\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd_write_at_upper::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd_write_at_upper::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdWriteAtUpperSpec;
impl crate::RegisterSpec for CmdWriteAtUpperSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd_write_at_upper::R`](R) reader structure"]
impl crate::Readable for CmdWriteAtUpperSpec {}
#[doc = "`write(|w| ..)` method takes [`cmd_write_at_upper::W`](W) writer structure"]
impl crate::Writable for CmdWriteAtUpperSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets cmd_write_at_upper to value 0"]
impl crate::Resettable for CmdWriteAtUpperSpec {
    const RESET_VALUE: u32 = 0;
}
