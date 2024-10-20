#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    pwren: Pwren,
    clk_ctrl: ClkCtrl,
    tmout: Tmout,
    ctype: Ctype,
    blksiz: Blksiz,
    bytcnt: Bytcnt,
    intmask: Intmask,
    cmdarg: Cmdarg,
    cmd: Cmd,
    resp: [Resp; 4],
    mintsts: Mintsts,
    rintsts: Rintsts,
    status: Status,
    fifoth: Fifoth,
    cdetect: Cdetect,
    wrtprt: Wrtprt,
    gpio: Gpio,
    cnt: [Cnt; 2],
    debnce: Debnce,
    id: Id,
    hcon: Hcon,
    uhs_reg: UhsReg,
    rst_n: RstN,
    _reserved24: [u8; 0x04],
    bmod: Bmod,
    pldmnd: Pldmnd,
    idmac_addr: IdmacAddr,
    _reserved27: [u8; 0x58],
    cdthrctl: Cdthrctl,
    _reserved28: [u8; 0x04],
    uhs_reg_ext: UhsRegExt,
    _reserved29: [u8; 0xf4],
    data: [Data; 1024],
}
impl RegisterBlock {
    #[doc = "0x00 - MMC Control"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x04 - MMC Power Enable"]
    #[inline(always)]
    pub const fn pwren(&self) -> &Pwren {
        &self.pwren
    }
    #[doc = "0x08..0x14 - MMC Clock Control registers"]
    #[inline(always)]
    pub const fn clk_ctrl(&self) -> &ClkCtrl {
        &self.clk_ctrl
    }
    #[doc = "0x14 - MMC Timeout"]
    #[inline(always)]
    pub const fn tmout(&self) -> &Tmout {
        &self.tmout
    }
    #[doc = "0x18 - MMC card type"]
    #[inline(always)]
    pub const fn ctype(&self) -> &Ctype {
        &self.ctype
    }
    #[doc = "0x1c - MMC block size"]
    #[inline(always)]
    pub const fn blksiz(&self) -> &Blksiz {
        &self.blksiz
    }
    #[doc = "0x20 - MMC byte count"]
    #[inline(always)]
    pub const fn bytcnt(&self) -> &Bytcnt {
        &self.bytcnt
    }
    #[doc = "0x24 - MMC interrupt mask"]
    #[inline(always)]
    pub const fn intmask(&self) -> &Intmask {
        &self.intmask
    }
    #[doc = "0x28 - MMC command argument"]
    #[inline(always)]
    pub const fn cmdarg(&self) -> &Cmdarg {
        &self.cmdarg
    }
    #[doc = "0x2c - MMC command"]
    #[inline(always)]
    pub const fn cmd(&self) -> &Cmd {
        &self.cmd
    }
    #[doc = "0x30..0x40 - MMC response"]
    #[inline(always)]
    pub const fn resp(&self, n: usize) -> &Resp {
        &self.resp[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x30..0x40 - MMC response"]
    #[inline(always)]
    pub fn resp_iter(&self) -> impl Iterator<Item = &Resp> {
        self.resp.iter()
    }
    #[doc = "0x40 - MMC MINT status"]
    #[inline(always)]
    pub const fn mintsts(&self) -> &Mintsts {
        &self.mintsts
    }
    #[doc = "0x44 - MMC RINT status"]
    #[inline(always)]
    pub const fn rintsts(&self) -> &Rintsts {
        &self.rintsts
    }
    #[doc = "0x48 - MMC status"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x4c - MMC FIFOTH"]
    #[inline(always)]
    pub const fn fifoth(&self) -> &Fifoth {
        &self.fifoth
    }
    #[doc = "0x50 - MMC card detect"]
    #[inline(always)]
    pub const fn cdetect(&self) -> &Cdetect {
        &self.cdetect
    }
    #[doc = "0x54 - MMC write protect"]
    #[inline(always)]
    pub const fn wrtprt(&self) -> &Wrtprt {
        &self.wrtprt
    }
    #[doc = "0x58 - MMC GPIO"]
    #[inline(always)]
    pub const fn gpio(&self) -> &Gpio {
        &self.gpio
    }
    #[doc = "0x5c..0x64 - MMC count"]
    #[inline(always)]
    pub const fn cnt(&self, n: usize) -> &Cnt {
        &self.cnt[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x5c..0x64 - MMC count"]
    #[inline(always)]
    pub fn cnt_iter(&self) -> impl Iterator<Item = &Cnt> {
        self.cnt.iter()
    }
    #[doc = "0x5c - MMC count"]
    #[inline(always)]
    pub const fn cnttcb(&self) -> &Cnt {
        self.cnt(0)
    }
    #[doc = "0x60 - MMC count"]
    #[inline(always)]
    pub const fn cnttbb(&self) -> &Cnt {
        self.cnt(1)
    }
    #[doc = "0x64 - MMC debounce"]
    #[inline(always)]
    pub const fn debnce(&self) -> &Debnce {
        &self.debnce
    }
    #[doc = "0x68..0x70 - MMC ID registers"]
    #[inline(always)]
    pub const fn id(&self) -> &Id {
        &self.id
    }
    #[doc = "0x70 - MMC HCON"]
    #[inline(always)]
    pub const fn hcon(&self) -> &Hcon {
        &self.hcon
    }
    #[doc = "0x74 - MMC UHS-1 regulator"]
    #[inline(always)]
    pub const fn uhs_reg(&self) -> &UhsReg {
        &self.uhs_reg
    }
    #[doc = "0x78 - MMC Reset"]
    #[inline(always)]
    pub const fn rst_n(&self) -> &RstN {
        &self.rst_n
    }
    #[doc = "0x80 - MMC DMAC bus mode"]
    #[inline(always)]
    pub const fn bmod(&self) -> &Bmod {
        &self.bmod
    }
    #[doc = "0x84 - MMC PLDMND"]
    #[inline(always)]
    pub const fn pldmnd(&self) -> &Pldmnd {
        &self.pldmnd
    }
    #[doc = "0x88..0xa8 - MMC Internal DMAC Address registers"]
    #[inline(always)]
    pub const fn idmac_addr(&self) -> &IdmacAddr {
        &self.idmac_addr
    }
    #[doc = "0x100 - MMC card detect threshold control"]
    #[inline(always)]
    pub const fn cdthrctl(&self) -> &Cdthrctl {
        &self.cdthrctl
    }
    #[doc = "0x108 - MMC UHS regulator extended"]
    #[inline(always)]
    pub const fn uhs_reg_ext(&self) -> &UhsRegExt {
        &self.uhs_reg_ext
    }
    #[doc = "0x200..0x1200 - MMC FIFO data"]
    #[inline(always)]
    pub const fn data(&self, n: usize) -> &Data {
        &self.data[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x200..0x1200 - MMC FIFO data"]
    #[inline(always)]
    pub fn data_iter(&self) -> impl Iterator<Item = &Data> {
        self.data.iter()
    }
}
#[doc = "ctrl (rw) register accessor: MMC Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "ctrl")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "MMC Control"]
pub mod ctrl;
#[doc = "pwren (rw) register accessor: MMC Power Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwren::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwren::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwren`]
module"]
#[doc(alias = "pwren")]
pub type Pwren = crate::Reg<pwren::PwrenSpec>;
#[doc = "MMC Power Enable"]
pub mod pwren;
#[doc = "MMC Clock Control registers"]
pub use self::clk_ctrl::ClkCtrl;
#[doc = r"Cluster"]
#[doc = "MMC Clock Control registers"]
pub mod clk_ctrl;
#[doc = "tmout (rw) register accessor: MMC Timeout\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmout::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmout::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmout`]
module"]
#[doc(alias = "tmout")]
pub type Tmout = crate::Reg<tmout::TmoutSpec>;
#[doc = "MMC Timeout"]
pub mod tmout;
#[doc = "ctype (rw) register accessor: MMC card type\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctype::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctype::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctype`]
module"]
#[doc(alias = "ctype")]
pub type Ctype = crate::Reg<ctype::CtypeSpec>;
#[doc = "MMC card type"]
pub mod ctype;
#[doc = "blksiz (rw) register accessor: MMC block size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blksiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blksiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blksiz`]
module"]
#[doc(alias = "blksiz")]
pub type Blksiz = crate::Reg<blksiz::BlksizSpec>;
#[doc = "MMC block size"]
pub mod blksiz;
#[doc = "bytcnt (rw) register accessor: MMC byte count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bytcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bytcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bytcnt`]
module"]
#[doc(alias = "bytcnt")]
pub type Bytcnt = crate::Reg<bytcnt::BytcntSpec>;
#[doc = "MMC byte count"]
pub mod bytcnt;
#[doc = "intmask (rw) register accessor: MMC interrupt mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intmask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intmask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intmask`]
module"]
#[doc(alias = "intmask")]
pub type Intmask = crate::Reg<intmask::IntmaskSpec>;
#[doc = "MMC interrupt mask"]
pub mod intmask;
#[doc = "cmdarg (rw) register accessor: MMC command argument\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdarg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdarg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdarg`]
module"]
#[doc(alias = "cmdarg")]
pub type Cmdarg = crate::Reg<cmdarg::CmdargSpec>;
#[doc = "MMC command argument"]
pub mod cmdarg;
#[doc = "cmd (rw) register accessor: MMC command\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`]
module"]
#[doc(alias = "cmd")]
pub type Cmd = crate::Reg<cmd::CmdSpec>;
#[doc = "MMC command"]
pub mod cmd;
#[doc = "resp (rw) register accessor: MMC response\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`resp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resp`]
module"]
#[doc(alias = "resp")]
pub type Resp = crate::Reg<resp::RespSpec>;
#[doc = "MMC response"]
pub mod resp;
#[doc = "mintsts (rw) register accessor: MMC MINT status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mintsts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mintsts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mintsts`]
module"]
#[doc(alias = "mintsts")]
pub type Mintsts = crate::Reg<mintsts::MintstsSpec>;
#[doc = "MMC MINT status"]
pub mod mintsts;
#[doc = "rintsts (rw) register accessor: MMC RINT status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rintsts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rintsts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rintsts`]
module"]
#[doc(alias = "rintsts")]
pub type Rintsts = crate::Reg<rintsts::RintstsSpec>;
#[doc = "MMC RINT status"]
pub mod rintsts;
#[doc = "status (rw) register accessor: MMC status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "status")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "MMC status"]
pub mod status;
#[doc = "fifoth (rw) register accessor: MMC FIFOTH\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifoth::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifoth::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifoth`]
module"]
#[doc(alias = "fifoth")]
pub type Fifoth = crate::Reg<fifoth::FifothSpec>;
#[doc = "MMC FIFOTH"]
pub mod fifoth;
#[doc = "cdetect (rw) register accessor: MMC card detect\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdetect::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cdetect::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cdetect`]
module"]
#[doc(alias = "cdetect")]
pub type Cdetect = crate::Reg<cdetect::CdetectSpec>;
#[doc = "MMC card detect"]
pub mod cdetect;
#[doc = "wrtprt (rw) register accessor: MMC write protect\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wrtprt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wrtprt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wrtprt`]
module"]
#[doc(alias = "wrtprt")]
pub type Wrtprt = crate::Reg<wrtprt::WrtprtSpec>;
#[doc = "MMC write protect"]
pub mod wrtprt;
#[doc = "gpio (rw) register accessor: MMC GPIO\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio`]
module"]
#[doc(alias = "gpio")]
pub type Gpio = crate::Reg<gpio::GpioSpec>;
#[doc = "MMC GPIO"]
pub mod gpio;
#[doc = "cnt (rw) register accessor: MMC count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnt`]
module"]
#[doc(alias = "cnt")]
pub type Cnt = crate::Reg<cnt::CntSpec>;
#[doc = "MMC count"]
pub mod cnt;
#[doc = "debnce (rw) register accessor: MMC debounce\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`debnce::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`debnce::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debnce`]
module"]
#[doc(alias = "debnce")]
pub type Debnce = crate::Reg<debnce::DebnceSpec>;
#[doc = "MMC debounce"]
pub mod debnce;
#[doc = "MMC ID registers"]
pub use self::id::Id;
#[doc = r"Cluster"]
#[doc = "MMC ID registers"]
pub mod id;
#[doc = "hcon (rw) register accessor: MMC HCON\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcon::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcon::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcon`]
module"]
#[doc(alias = "hcon")]
pub type Hcon = crate::Reg<hcon::HconSpec>;
#[doc = "MMC HCON"]
pub mod hcon;
#[doc = "uhs_reg (rw) register accessor: MMC UHS-1 regulator\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uhs_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uhs_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uhs_reg`]
module"]
#[doc(alias = "uhs_reg")]
pub type UhsReg = crate::Reg<uhs_reg::UhsRegSpec>;
#[doc = "MMC UHS-1 regulator"]
pub mod uhs_reg;
#[doc = "rst_n (rw) register accessor: MMC Reset\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rst_n::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rst_n::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rst_n`]
module"]
#[doc(alias = "rst_n")]
pub type RstN = crate::Reg<rst_n::RstNSpec>;
#[doc = "MMC Reset"]
pub mod rst_n;
#[doc = "bmod (rw) register accessor: MMC DMAC bus mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bmod::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bmod::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bmod`]
module"]
#[doc(alias = "bmod")]
pub type Bmod = crate::Reg<bmod::BmodSpec>;
#[doc = "MMC DMAC bus mode"]
pub mod bmod;
#[doc = "pldmnd (rw) register accessor: MMC PLDMND\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pldmnd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pldmnd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pldmnd`]
module"]
#[doc(alias = "pldmnd")]
pub type Pldmnd = crate::Reg<pldmnd::PldmndSpec>;
#[doc = "MMC PLDMND"]
pub mod pldmnd;
#[doc = "MMC Internal DMAC Address registers"]
pub use self::idmac_addr::IdmacAddr;
#[doc = r"Cluster"]
#[doc = "MMC Internal DMAC Address registers"]
pub mod idmac_addr;
#[doc = "cdthrctl (rw) register accessor: MMC card detect threshold control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdthrctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cdthrctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cdthrctl`]
module"]
#[doc(alias = "cdthrctl")]
pub type Cdthrctl = crate::Reg<cdthrctl::CdthrctlSpec>;
#[doc = "MMC card detect threshold control"]
pub mod cdthrctl;
#[doc = "uhs_reg_ext (rw) register accessor: MMC UHS regulator extended\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uhs_reg_ext::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uhs_reg_ext::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uhs_reg_ext`]
module"]
#[doc(alias = "uhs_reg_ext")]
pub type UhsRegExt = crate::Reg<uhs_reg_ext::UhsRegExtSpec>;
#[doc = "MMC UHS regulator extended"]
pub mod uhs_reg_ext;
#[doc = "data (rw) register accessor: MMC FIFO data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data`]
module"]
#[doc(alias = "data")]
pub type Data = crate::Reg<data::DataSpec>;
#[doc = "MMC FIFO data"]
pub mod data;
