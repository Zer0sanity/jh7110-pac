#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    gpo_doen: GpoDoen,
    gpo_dout: GpoDout,
    gpi: Gpi,
    ioirq: Ioirq,
    padcfg: Padcfg,
    func_sel: FuncSel,
}
impl RegisterBlock {
    #[doc = "0x00..0x40 - The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub const fn gpo_doen(&self) -> &GpoDoen {
        &self.gpo_doen
    }
    #[doc = "0x40..0x80 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO DOUT - The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub const fn gpo_dout(&self) -> &GpoDout {
        &self.gpo_dout
    }
    #[doc = "0x80..0xdc - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
    #[inline(always)]
    pub const fn gpi(&self) -> &Gpi {
        &self.gpi
    }
    #[doc = "0xdc..0x120 - GPIO Interrupt Request configuration"]
    #[inline(always)]
    pub const fn ioirq(&self) -> &Ioirq {
        &self.ioirq
    }
    #[doc = "0x120..0x29c - SYS IOMUX CFG SAIF SYSCFG PADCFG: GPIO pad configuration"]
    #[inline(always)]
    pub const fn padcfg(&self) -> &Padcfg {
        &self.padcfg
    }
    #[doc = "0x29c..0x2b8 - Registers used to configure the function selector of the system signal indicated by the register name."]
    #[inline(always)]
    pub const fn func_sel(&self) -> &FuncSel {
        &self.func_sel
    }
}
#[doc = "The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub use self::gpo_doen::GpoDoen;
#[doc = r"Cluster"]
#[doc = "The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub mod gpo_doen;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO DOUT - The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub use self::gpo_dout::GpoDout;
#[doc = r"Cluster"]
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO DOUT - The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub mod gpo_dout;
#[doc = "The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
pub use self::gpi::Gpi;
#[doc = r"Cluster"]
#[doc = "The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
pub mod gpi;
#[doc = "GPIO Interrupt Request configuration"]
pub use self::ioirq::Ioirq;
#[doc = r"Cluster"]
#[doc = "GPIO Interrupt Request configuration"]
pub mod ioirq;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG: GPIO pad configuration"]
pub use self::padcfg::Padcfg;
#[doc = r"Cluster"]
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG: GPIO pad configuration"]
pub mod padcfg;
#[doc = "Registers used to configure the function selector of the system signal indicated by the register name."]
pub use self::func_sel::FuncSel;
#[doc = r"Cluster"]
#[doc = "Registers used to configure the function selector of the system signal indicated by the register name."]
pub mod func_sel;
