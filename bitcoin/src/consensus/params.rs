// SPDX-License-Identifier: CC0-1.0

//! Bitcoin consensus parameters.
//!
//! This module provides a predefined set of parameters for different Bitcoin
//! chains (such as mainnet, testnet).
//!

use crate::network::Network;
#[cfg(doc)]
use crate::pow::CompactTarget;
use crate::pow::Target;

/// Parameters that influence chain consensus.
#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct Params {
    /// Network for which parameters are valid.
    pub network: Network,
    /// Time when BIP16 becomes active.
    pub bip16_time: u32,
    /// Block height at which BIP34 becomes active.
    pub bip34_height: u32,
    /// Block height at which BIP65 becomes active.
    pub bip65_height: u32,
    /// Block height at which BIP66 becomes active.
    pub bip66_height: u32,
    /// Minimum blocks including miner confirmation of the total of 2016 blocks in a retargeting period,
    /// (nPowTargetTimespan / nPowTargetSpacing) which is also used for BIP9 deployments.
    /// Examples: 1916 for 95%, 1512 for testchains.
    pub rule_change_activation_threshold: u32,
    /// Number of blocks with the same set of rules.
    pub miner_confirmation_window: u32,
    /// Proof of work limit value. It contains the lowest possible difficulty.
    #[deprecated(since = "0.32.0", note = "field renamed to max_attainable_target")]
    pub pow_limit: Target,
    /// The maximum **attainable** target value for these params.
    ///
    /// Not all target values are attainable because consensus code uses the compact format to
    /// represent targets (see [`CompactTarget`]).
    ///
    /// Note that this value differs from Bitcoin Core's powLimit field in that this value is
    /// attainable, but Bitcoin Core's is not. Specifically, because targets in Bitcoin are always
    /// rounded to the nearest float expressible in "compact form", not all targets are attainable.
    /// Still, this should not affect consensus as the only place where the non-compact form of
    /// this is used in Bitcoin Core's consensus algorithm is in comparison and there are no
    /// compact-expressible values between Bitcoin Core's and the limit expressed here.
    pub max_attainable_target: Target,
    /// Expected amount of time to mine one block.
    pub pow_target_spacing: u64,
    /// Difficulty recalculation interval.
    pub pow_target_timespan: u64,
    /// Determines whether minimal difficulty may be used for blocks or not.
    pub allow_min_difficulty_blocks: bool,
    /// Determines whether retargeting is disabled for this network or not.
    pub no_pow_retargeting: bool,
}

/// The mainnet parameters.
///
/// Use this for a static reference e.g., `&params::MAINNET`.
///
/// For more on static vs const see The Rust Reference [using-statics-or-consts] section.
///
/// [using-statics-or-consts]: <https://doc.rust-lang.org/reference/items/static-items.html#using-statics-or-consts>
pub static MAINNET: Params = Params::MAINNET;
/// The testnet parameters.
pub static TESTNET: Params = Params::TESTNET;
/// The signet parameters.
pub static SIGNET: Params = Params::SIGNET;
/// The regtest parameters.
pub static REGTEST: Params = Params::REGTEST;

#[allow(deprecated)]            // For `pow_limit`.
impl Params {
    /// The mainnet parameters (alias for `Params::MAINNET`).
    pub const BITCOIN: Params = Params::MAINNET;

    /// The mainnet parameters (Briskcoin).
    pub const MAINNET: Params = Params {
        network: Network::Bitcoin,
        bip16_time: 1333238400,                 // Apr 1 2012
        bip34_height: 227931, // 000000000000024b89b42a942fe0d9fea3bb44ab7bd1b19115dd6a759c0808b8
        bip65_height: 388381, // 000000000000000004c2b624ed5d7756c508d90fd0da2c7c679febfa6c4735f0
        bip66_height: 363725, // 00000000000000000379eaa19dce8c9b722d46ae6a57c2f1a988119488b50931
        rule_change_activation_threshold: 95, // 95% of 120 blocks = 114 blocks
        miner_confirmation_window: 120, // 1 hour at 30 second blocks
        pow_limit: Target::MAX_ATTAINABLE_MAINNET,
        max_attainable_target: Target::MAX_ATTAINABLE_MAINNET,
        pow_target_spacing: 30,                 // 30 seconds.
        pow_target_timespan: 60 * 60,           // 1 hour.
        allow_min_difficulty_blocks: false,
        no_pow_retargeting: false,
    };

    /// The testnet parameters.
    pub const TESTNET: Params = Params {
        network: Network::Testnet,
        bip16_time: 1333238400,                 // Apr 1 2012
        bip34_height: 21111, // 0000000023b3a96d3484e5abb3755c413e7d41500f8e2a5c3f0dd01299cd8ef8
        bip65_height: 581885, // 00000000007f6655f22f98e72ed80d8b06dc761d5da09df0fa1dc4be4f861eb6
        bip66_height: 330776, // 000000002104c8c45e99a8853285a3b592602a3ccde2b832481da85e9e4ba182
        rule_change_activation_threshold: 1512, // 75%
        miner_confirmation_window: 2016,
        pow_limit: Target::MAX_ATTAINABLE_TESTNET,
        max_attainable_target: Target::MAX_ATTAINABLE_TESTNET,
        pow_target_spacing: 10 * 60,            // 10 minutes.
        pow_target_timespan: 14 * 24 * 60 * 60, // 2 weeks.
        allow_min_difficulty_blocks: true,
        no_pow_retargeting: false,
    };

    /// The signet parameters.
    pub const SIGNET: Params = Params {
        network: Network::Signet,
        bip16_time: 1333238400, // Apr 1 2012
        bip34_height: 1,
        bip65_height: 1,
        bip66_height: 1,
        rule_change_activation_threshold: 1916, // 95%
        miner_confirmation_window: 2016,
        pow_limit: Target::MAX_ATTAINABLE_SIGNET,
        max_attainable_target: Target::MAX_ATTAINABLE_SIGNET,
        pow_target_spacing: 10 * 60,            // 10 minutes.
        pow_target_timespan: 14 * 24 * 60 * 60, // 2 weeks.
        allow_min_difficulty_blocks: false,
        no_pow_retargeting: false,
    };

    /// The regtest parameters.
    pub const REGTEST: Params = Params {
        network: Network::Regtest,
        bip16_time: 1333238400,  // Apr 1 2012
        bip34_height: 100000000, // not activated on regtest
        bip65_height: 1351,
        bip66_height: 1251,                    // used only in rpc tests
        rule_change_activation_threshold: 108, // 75%
        miner_confirmation_window: 144,
        pow_limit: Target::MAX_ATTAINABLE_REGTEST,
        max_attainable_target: Target::MAX_ATTAINABLE_REGTEST,
        pow_target_spacing: 10 * 60,            // 10 minutes.
        pow_target_timespan: 14 * 24 * 60 * 60, // 2 weeks.
        allow_min_difficulty_blocks: true,
        no_pow_retargeting: true,
    };

    /// Creates parameters set for the given network.    /// Creates parameters set for the given network.
    pub const fn new(network: Network) -> Self {
        match network {
            Network::Bitcoin => Params::MAINNET,
            Network::Testnet => Params::TESTNET,
            Network::Signet => Params::SIGNET,
            Network::Regtest => Params::REGTEST,
        }
    }

    /// Calculates the number of blocks between difficulty adjustments.
    pub fn difficulty_adjustment_interval(&self) -> u64 {
        self.pow_target_timespan / self.pow_target_spacing
    }
}

impl From<Network> for Params {
    fn from(value: Network) -> Self { Self::new(value) }
}

impl From<&Network> for Params {
    fn from(value: &Network) -> Self { Self::new(*value) }
}

impl From<Network> for &'static Params {
    fn from(value: Network) -> Self { value.params() }
}

impl From<&Network> for &'static Params {
    fn from(value: &Network) -> Self { value.params() }
}

impl AsRef<Params> for Params {
    fn as_ref(&self) -> &Params { self }
}

impl AsRef<Params> for Network {
    fn as_ref(&self) -> &Params {
        match *self {
            Network::Bitcoin => &MAINNET,
            Network::Testnet => &TESTNET,
            Network::Signet => &SIGNET,
            Network::Regtest => &REGTEST,
        }
    }
}
