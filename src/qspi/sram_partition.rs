#[doc = "Register `sram_partition` reader"]
pub type R = crate::R<SramPartitionSpec>;
#[doc = "Register `sram_partition` writer"]
pub type W = crate::W<SramPartitionSpec>;
#[doc = "Field `size` reader - Partition size in bytes"]
pub type SizeR = crate::FieldReader<u32>;
#[doc = "Field `size` writer - Partition size in bytes"]
pub type SizeW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Partition size in bytes"]
    #[inline(always)]
    pub fn size(&self) -> SizeR {
        SizeR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Partition size in bytes"]
    #[inline(always)]
    #[must_use]
    pub fn size(&mut self) -> SizeW<SramPartitionSpec> {
        SizeW::new(self, 0)
    }
}
#[doc = "Cadence QSPI SRAM Partition Size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sram_partition::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sram_partition::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SramPartitionSpec;
impl crate::RegisterSpec for SramPartitionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sram_partition::R`](R) reader structure"]
impl crate::Readable for SramPartitionSpec {}
#[doc = "`write(|w| ..)` method takes [`sram_partition::W`](W) writer structure"]
impl crate::Writable for SramPartitionSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets sram_partition to value 0"]
impl crate::Resettable for SramPartitionSpec {
    const RESET_VALUE: u32 = 0;
}
