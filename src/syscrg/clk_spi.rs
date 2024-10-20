#[repr(C)]
#[doc = "Clock SPI registers"]
#[doc(alias = "clk_spi")]
pub struct ClkSpi {
    apb: [Apb; 7],
}
impl ClkSpi {
    #[doc = "0x00..0x1c - Clock SPI APB"]
    #[inline(always)]
    pub const fn apb(&self, n: usize) -> &Apb {
        &self.apb[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x1c - Clock SPI APB"]
    #[inline(always)]
    pub fn apb_iter(&self) -> impl Iterator<Item = &Apb> {
        self.apb.iter()
    }
    #[doc = "0x00 - Clock SPI APB"]
    #[inline(always)]
    pub const fn apb_u0(&self) -> &Apb {
        self.apb(0)
    }
    #[doc = "0x04 - Clock SPI APB"]
    #[inline(always)]
    pub const fn apb_u1(&self) -> &Apb {
        self.apb(1)
    }
    #[doc = "0x08 - Clock SPI APB"]
    #[inline(always)]
    pub const fn apb_u2(&self) -> &Apb {
        self.apb(2)
    }
    #[doc = "0x0c - Clock SPI APB"]
    #[inline(always)]
    pub const fn apb_u3(&self) -> &Apb {
        self.apb(3)
    }
    #[doc = "0x10 - Clock SPI APB"]
    #[inline(always)]
    pub const fn apb_u4(&self) -> &Apb {
        self.apb(4)
    }
    #[doc = "0x14 - Clock SPI APB"]
    #[inline(always)]
    pub const fn apb_u5(&self) -> &Apb {
        self.apb(5)
    }
    #[doc = "0x18 - Clock SPI APB"]
    #[inline(always)]
    pub const fn apb_u6(&self) -> &Apb {
        self.apb(6)
    }
}
#[doc = "apb (rw) register accessor: Clock SPI APB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb`]
module"]
#[doc(alias = "apb")]
pub type Apb = crate::Reg<apb::ApbSpec>;
#[doc = "Clock SPI APB"]
pub mod apb;
