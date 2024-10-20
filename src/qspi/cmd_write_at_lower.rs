#[doc = "Register `cmd_write_at_lower` reader"]
pub type R = crate::R<CmdWriteAtLowerSpec>;
#[doc = "Register `cmd_write_at_lower` writer"]
pub type W = crate::W<CmdWriteAtLowerSpec>;
#[doc = "Field `write_at_lower` reader - "]
pub type WriteAtLowerR = crate::FieldReader<u32>;
#[doc = "Field `write_at_lower` writer - "]
pub type WriteAtLowerW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn write_at_lower(&self) -> WriteAtLowerR {
        WriteAtLowerR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn write_at_lower(&mut self) -> WriteAtLowerW<CmdWriteAtLowerSpec> {
        WriteAtLowerW::new(self, 0)
    }
}
#[doc = "Cadence QSPI Command Write at Lower\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd_write_at_lower::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd_write_at_lower::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdWriteAtLowerSpec;
impl crate::RegisterSpec for CmdWriteAtLowerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd_write_at_lower::R`](R) reader structure"]
impl crate::Readable for CmdWriteAtLowerSpec {}
#[doc = "`write(|w| ..)` method takes [`cmd_write_at_lower::W`](W) writer structure"]
impl crate::Writable for CmdWriteAtLowerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets cmd_write_at_lower to value 0"]
impl crate::Resettable for CmdWriteAtLowerSpec {
    const RESET_VALUE: u32 = 0;
}
