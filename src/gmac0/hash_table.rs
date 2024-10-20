#[doc = "Register `hash_table[%s]` reader"]
pub type R = crate::R<HashTableSpec>;
#[doc = "Register `hash_table[%s]` writer"]
pub type W = crate::W<HashTableSpec>;
#[doc = "Field `hash_table` reader - MAC Hash Table"]
pub type HashTableR = crate::FieldReader<u32>;
#[doc = "Field `hash_table` writer - MAC Hash Table"]
pub type HashTableW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - MAC Hash Table"]
    #[inline(always)]
    pub fn hash_table(&self) -> HashTableR {
        HashTableR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - MAC Hash Table"]
    #[inline(always)]
    #[must_use]
    pub fn hash_table(&mut self) -> HashTableW<HashTableSpec> {
        HashTableW::new(self, 0)
    }
}
#[doc = "MAC Hash Table\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_table::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_table::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HashTableSpec;
impl crate::RegisterSpec for HashTableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_table::R`](R) reader structure"]
impl crate::Readable for HashTableSpec {}
#[doc = "`write(|w| ..)` method takes [`hash_table::W`](W) writer structure"]
impl crate::Writable for HashTableSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets hash_table[%s]
to value 0"]
impl crate::Resettable for HashTableSpec {
    const RESET_VALUE: u32 = 0;
}
