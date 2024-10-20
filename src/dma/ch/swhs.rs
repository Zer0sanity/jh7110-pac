#[doc = "Register `swhs[%s]` reader"]
pub type R = crate::R<SwhsSpec>;
#[doc = "Register `swhs[%s]` writer"]
pub type W = crate::W<SwhsSpec>;
#[doc = "Field `req(_src,_src_we,_sgl_src,_sgl_src_we,_lst_src,_lst_src_we)` reader - Software Handshake Request signal configuration. **NOTE** write-enable fields are write-only."]
pub type ReqR = crate::BitReader;
#[doc = "Field `req(_src,_src_we,_sgl_src,_sgl_src_we,_lst_src,_lst_src_we)` writer - Software Handshake Request signal configuration. **NOTE** write-enable fields are write-only."]
pub type ReqW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Software Handshake Request signal configuration. **NOTE** write-enable fields are write-only."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `req_src` field"]
    #[inline(always)]
    pub fn req(&self, n: u8) -> ReqR {
        #[allow(clippy::no_effect)]
        [(); 6][n as usize];
        ReqR::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Software Handshake Request signal configuration. **NOTE** write-enable fields are write-only."]
    #[inline(always)]
    pub fn req_iter(&self) -> impl Iterator<Item = ReqR> + '_ {
        (0..6).map(move |n| ReqR::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - Software Handshake Request signal configuration. **NOTE** write-enable fields are write-only."]
    #[inline(always)]
    pub fn req_src(&self) -> ReqR {
        ReqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Software Handshake Request signal configuration. **NOTE** write-enable fields are write-only."]
    #[inline(always)]
    pub fn req_src_we(&self) -> ReqR {
        ReqR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Software Handshake Request signal configuration. **NOTE** write-enable fields are write-only."]
    #[inline(always)]
    pub fn req_sgl_src(&self) -> ReqR {
        ReqR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Software Handshake Request signal configuration. **NOTE** write-enable fields are write-only."]
    #[inline(always)]
    pub fn req_sgl_src_we(&self) -> ReqR {
        ReqR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Software Handshake Request signal configuration. **NOTE** write-enable fields are write-only."]
    #[inline(always)]
    pub fn req_lst_src(&self) -> ReqR {
        ReqR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Software Handshake Request signal configuration. **NOTE** write-enable fields are write-only."]
    #[inline(always)]
    pub fn req_lst_src_we(&self) -> ReqR {
        ReqR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Software Handshake Request signal configuration. **NOTE** write-enable fields are write-only."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `req_src` field"]
    #[inline(always)]
    #[must_use]
    pub fn req(&mut self, n: u8) -> ReqW<SwhsSpec> {
        #[allow(clippy::no_effect)]
        [(); 6][n as usize];
        ReqW::new(self, n)
    }
    #[doc = "Bit 0 - Software Handshake Request signal configuration. **NOTE** write-enable fields are write-only."]
    #[inline(always)]
    #[must_use]
    pub fn req_src(&mut self) -> ReqW<SwhsSpec> {
        ReqW::new(self, 0)
    }
    #[doc = "Bit 1 - Software Handshake Request signal configuration. **NOTE** write-enable fields are write-only."]
    #[inline(always)]
    #[must_use]
    pub fn req_src_we(&mut self) -> ReqW<SwhsSpec> {
        ReqW::new(self, 1)
    }
    #[doc = "Bit 2 - Software Handshake Request signal configuration. **NOTE** write-enable fields are write-only."]
    #[inline(always)]
    #[must_use]
    pub fn req_sgl_src(&mut self) -> ReqW<SwhsSpec> {
        ReqW::new(self, 2)
    }
    #[doc = "Bit 3 - Software Handshake Request signal configuration. **NOTE** write-enable fields are write-only."]
    #[inline(always)]
    #[must_use]
    pub fn req_sgl_src_we(&mut self) -> ReqW<SwhsSpec> {
        ReqW::new(self, 3)
    }
    #[doc = "Bit 4 - Software Handshake Request signal configuration. **NOTE** write-enable fields are write-only."]
    #[inline(always)]
    #[must_use]
    pub fn req_lst_src(&mut self) -> ReqW<SwhsSpec> {
        ReqW::new(self, 4)
    }
    #[doc = "Bit 5 - Software Handshake Request signal configuration. **NOTE** write-enable fields are write-only."]
    #[inline(always)]
    #[must_use]
    pub fn req_lst_src_we(&mut self) -> ReqW<SwhsSpec> {
        ReqW::new(self, 5)
    }
}
#[doc = "Software Handshake register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swhs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swhs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwhsSpec;
impl crate::RegisterSpec for SwhsSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`swhs::R`](R) reader structure"]
impl crate::Readable for SwhsSpec {}
#[doc = "`write(|w| ..)` method takes [`swhs::W`](W) writer structure"]
impl crate::Writable for SwhsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets swhs[%s]
to value 0"]
impl crate::Resettable for SwhsSpec {
    const RESET_VALUE: u64 = 0;
}
