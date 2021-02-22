// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2019-2021 by the author(s)
//
// Author(s):
//   - Berkus Decker <berkus+github@metta.systems>

//! Secure Configuration Register - EL3, page D12.2.99 of armv8arm.
//! Defines the configuration of the current Security state. It specifies:
//! • The Security state of EL0, EL1, and EL2. The Security state is either Secure or Non-secure.
//! • The Execution state at lower Exception levels.
//! • Whether IRQ, FIQ, SError interrupts, and External abort exceptions are taken to EL3.
//! • Whether various operations are trapped to EL3.
use register::{cpu::RegisterReadWrite, register_bitfields};

register_bitfields! {u64,
    SCR_EL3 [
        /// Execution state control for lower Exception levels:
        ///
        /// 0 Lower levels are all AArch32.
        /// 1 The next lower level is AArch64.
        ///   If EL2 is present:
        ///     The Execution state for EL2 is AArch64.
        ///     EL2 controls EL1 and EL0 behaviors.
        ///   If EL2 is not present:
        ///     The Execution state for EL1 is AArch64.
        ///     The Execution state for EL0 is determined by the current value of PSTATE.nRW when
        ///     executing at EL0.
        ///
        /// If all lower Exception levels cannot use AArch32 then this bit is RAO/WI.
        ///
        /// When SCR_EL3.{EEL2,NS}=={1,0}, this bit is treated as 1 for all purposes other than
        /// reading or writing the register.
        ///
        /// The RW bit is permitted to be cached in a TLB.
        RW   OFFSET(10) NUMBITS(1) [
            AllLowerELsAreAarch32 = 0,
            NextELIsAarch64 = 1
        ],

        /// Non-secure bit.
        /// * 0b0 Indicates that EL0 and EL1 are in Secure state.
        /// * 0b1 Indicates that Exception levels lower than EL3 are in Non-secure state,
        ///       and so memory accesses from those Exception levels cannot access Secure memory.
        ///
        /// When SCR_EL3.{EEL2, NS} == {1, 0}, then EL2 is using AArch64 and in Secure state.
        NS   OFFSET(0) NUMBITS(1) [
            Secure = 0,
            NonSecure = 1
        ]
    ]
}

pub struct Reg;

impl RegisterReadWrite<u64, SCR_EL3::Register> for Reg {
    sys_coproc_read_raw!(u64, "SCR_EL3", "x");
    sys_coproc_write_raw!(u64, "SCR_EL3", "x");
}

pub static SCR_EL3: Reg = Reg {};
