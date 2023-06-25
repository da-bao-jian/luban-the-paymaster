pub use istakemanager::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types
)]
#[allow(clippy::all)]
pub mod istakemanager {
    const _: () = {
        ::core::include_bytes!(
            "/home/harry/misc/eth-waterloo/pseudo_bundler/src/abi/IStakeManager.json"
        );
    };
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"totalDeposit\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Deposited\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"totalStaked\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"unstakeDelaySec\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"StakeLocked\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"withdrawTime\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"StakeUnlocked\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"withdrawAddress\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"StakeWithdrawn\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"withdrawAddress\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Withdrawn\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"_unstakeDelaySec\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"addStake\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"balanceOf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"depositTo\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getDepositInfo\",\"outputs\":[{\"internalType\":\"struct IStakeManager.DepositInfo\",\"name\":\"info\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint112\",\"name\":\"deposit\",\"type\":\"uint112\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"staked\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"uint112\",\"name\":\"stake\",\"type\":\"uint112\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"unstakeDelaySec\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"uint48\",\"name\":\"withdrawTime\",\"type\":\"uint48\",\"components\":[]}]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"unlockStake\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address payable\",\"name\":\"withdrawAddress\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"withdrawStake\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address payable\",\"name\":\"withdrawAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"withdrawAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"withdrawTo\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static ISTAKEMANAGER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(|| {
            ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid")
        });
    pub struct istakemanager<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for istakemanager<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for istakemanager<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for istakemanager<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for istakemanager<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(istakemanager))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> istakemanager<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                ISTAKEMANAGER_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `addStake` (0x0396cb60) function
        pub fn add_stake(
            &self,
            unstake_delay_sec: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([3, 150, 203, 96], unstake_delay_sec)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `balanceOf` (0x70a08231) function
        pub fn balance_of(
            &self,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], account)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `depositTo` (0xb760faf9) function
        pub fn deposit_to(
            &self,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([183, 96, 250, 249], account)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getDepositInfo` (0x5287ce12) function
        pub fn get_deposit_info(
            &self,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, DepositInfo> {
            self.0
                .method_hash([82, 135, 206, 18], account)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `unlockStake` (0xbb9fe6bf) function
        pub fn unlock_stake(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([187, 159, 230, 191], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawStake` (0xc23a5cea) function
        pub fn withdraw_stake(
            &self,
            withdraw_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([194, 58, 92, 234], withdraw_address)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawTo` (0x205c2878) function
        pub fn withdraw_to(
            &self,
            withdraw_address: ::ethers::core::types::Address,
            withdraw_amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([32, 92, 40, 120], (withdraw_address, withdraw_amount))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Deposited` event
        pub fn deposited_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, DepositedFilter> {
            self.0.event()
        }
        ///Gets the contract's `StakeLocked` event
        pub fn stake_locked_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, StakeLockedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `StakeUnlocked` event
        pub fn stake_unlocked_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, StakeUnlockedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `StakeWithdrawn` event
        pub fn stake_withdrawn_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, StakeWithdrawnFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `Withdrawn` event
        pub fn withdrawn_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, WithdrawnFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, istakemanagerEvents>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for istakemanager<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "Deposited", abi = "Deposited(address,uint256)")]
    pub struct DepositedFilter {
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        pub total_deposit: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "StakeLocked", abi = "StakeLocked(address,uint256,uint256)")]
    pub struct StakeLockedFilter {
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        pub total_staked: ::ethers::core::types::U256,
        pub unstake_delay_sec: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "StakeUnlocked", abi = "StakeUnlocked(address,uint256)")]
    pub struct StakeUnlockedFilter {
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        pub withdraw_time: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "StakeWithdrawn",
        abi = "StakeWithdrawn(address,address,uint256)"
    )]
    pub struct StakeWithdrawnFilter {
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        pub withdraw_address: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "Withdrawn", abi = "Withdrawn(address,address,uint256)")]
    pub struct WithdrawnFilter {
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        pub withdraw_address: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum istakemanagerEvents {
        DepositedFilter(DepositedFilter),
        StakeLockedFilter(StakeLockedFilter),
        StakeUnlockedFilter(StakeUnlockedFilter),
        StakeWithdrawnFilter(StakeWithdrawnFilter),
        WithdrawnFilter(WithdrawnFilter),
    }
    impl ::ethers::contract::EthLogDecode for istakemanagerEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = DepositedFilter::decode_log(log) {
                return Ok(istakemanagerEvents::DepositedFilter(decoded));
            }
            if let Ok(decoded) = StakeLockedFilter::decode_log(log) {
                return Ok(istakemanagerEvents::StakeLockedFilter(decoded));
            }
            if let Ok(decoded) = StakeUnlockedFilter::decode_log(log) {
                return Ok(istakemanagerEvents::StakeUnlockedFilter(decoded));
            }
            if let Ok(decoded) = StakeWithdrawnFilter::decode_log(log) {
                return Ok(istakemanagerEvents::StakeWithdrawnFilter(decoded));
            }
            if let Ok(decoded) = WithdrawnFilter::decode_log(log) {
                return Ok(istakemanagerEvents::WithdrawnFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for istakemanagerEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DepositedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakeLockedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakeUnlockedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakeWithdrawnFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawnFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DepositedFilter> for istakemanagerEvents {
        fn from(value: DepositedFilter) -> Self {
            Self::DepositedFilter(value)
        }
    }
    impl ::core::convert::From<StakeLockedFilter> for istakemanagerEvents {
        fn from(value: StakeLockedFilter) -> Self {
            Self::StakeLockedFilter(value)
        }
    }
    impl ::core::convert::From<StakeUnlockedFilter> for istakemanagerEvents {
        fn from(value: StakeUnlockedFilter) -> Self {
            Self::StakeUnlockedFilter(value)
        }
    }
    impl ::core::convert::From<StakeWithdrawnFilter> for istakemanagerEvents {
        fn from(value: StakeWithdrawnFilter) -> Self {
            Self::StakeWithdrawnFilter(value)
        }
    }
    impl ::core::convert::From<WithdrawnFilter> for istakemanagerEvents {
        fn from(value: WithdrawnFilter) -> Self {
            Self::WithdrawnFilter(value)
        }
    }
    ///Container type for all input parameters for the `addStake` function with signature `addStake(uint32)` and selector `0x0396cb60`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "addStake", abi = "addStake(uint32)")]
    pub struct AddStakeCall {
        pub unstake_delay_sec: u32,
    }
    ///Container type for all input parameters for the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "balanceOf", abi = "balanceOf(address)")]
    pub struct BalanceOfCall {
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `depositTo` function with signature `depositTo(address)` and selector `0xb760faf9`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "depositTo", abi = "depositTo(address)")]
    pub struct DepositToCall {
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getDepositInfo` function with signature `getDepositInfo(address)` and selector `0x5287ce12`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getDepositInfo", abi = "getDepositInfo(address)")]
    pub struct GetDepositInfoCall {
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `unlockStake` function with signature `unlockStake()` and selector `0xbb9fe6bf`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "unlockStake", abi = "unlockStake()")]
    pub struct UnlockStakeCall;
    ///Container type for all input parameters for the `withdrawStake` function with signature `withdrawStake(address)` and selector `0xc23a5cea`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "withdrawStake", abi = "withdrawStake(address)")]
    pub struct WithdrawStakeCall {
        pub withdraw_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `withdrawTo` function with signature `withdrawTo(address,uint256)` and selector `0x205c2878`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "withdrawTo", abi = "withdrawTo(address,uint256)")]
    pub struct WithdrawToCall {
        pub withdraw_address: ::ethers::core::types::Address,
        pub withdraw_amount: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum istakemanagerCalls {
        AddStake(AddStakeCall),
        BalanceOf(BalanceOfCall),
        DepositTo(DepositToCall),
        GetDepositInfo(GetDepositInfoCall),
        UnlockStake(UnlockStakeCall),
        WithdrawStake(WithdrawStakeCall),
        WithdrawTo(WithdrawToCall),
    }
    impl ::ethers::core::abi::AbiDecode for istakemanagerCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AddStakeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AddStake(decoded));
            }
            if let Ok(decoded) = <BalanceOfCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BalanceOf(decoded));
            }
            if let Ok(decoded) = <DepositToCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DepositTo(decoded));
            }
            if let Ok(decoded) =
                <GetDepositInfoCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetDepositInfo(decoded));
            }
            if let Ok(decoded) = <UnlockStakeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::UnlockStake(decoded));
            }
            if let Ok(decoded) = <WithdrawStakeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::WithdrawStake(decoded));
            }
            if let Ok(decoded) = <WithdrawToCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::WithdrawTo(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for istakemanagerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AddStake(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BalanceOf(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DepositTo(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetDepositInfo(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UnlockStake(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::WithdrawStake(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::WithdrawTo(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for istakemanagerCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddStake(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::DepositTo(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetDepositInfo(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnlockStake(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawStake(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawTo(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AddStakeCall> for istakemanagerCalls {
        fn from(value: AddStakeCall) -> Self {
            Self::AddStake(value)
        }
    }
    impl ::core::convert::From<BalanceOfCall> for istakemanagerCalls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<DepositToCall> for istakemanagerCalls {
        fn from(value: DepositToCall) -> Self {
            Self::DepositTo(value)
        }
    }
    impl ::core::convert::From<GetDepositInfoCall> for istakemanagerCalls {
        fn from(value: GetDepositInfoCall) -> Self {
            Self::GetDepositInfo(value)
        }
    }
    impl ::core::convert::From<UnlockStakeCall> for istakemanagerCalls {
        fn from(value: UnlockStakeCall) -> Self {
            Self::UnlockStake(value)
        }
    }
    impl ::core::convert::From<WithdrawStakeCall> for istakemanagerCalls {
        fn from(value: WithdrawStakeCall) -> Self {
            Self::WithdrawStake(value)
        }
    }
    impl ::core::convert::From<WithdrawToCall> for istakemanagerCalls {
        fn from(value: WithdrawToCall) -> Self {
            Self::WithdrawTo(value)
        }
    }
    ///Container type for all return fields from the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct BalanceOfReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getDepositInfo` function with signature `getDepositInfo(address)` and selector `0x5287ce12`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetDepositInfoReturn {
        pub info: DepositInfo,
    }
    ///`DepositInfo(uint112,bool,uint112,uint32,uint48)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct DepositInfo {
        pub deposit: u128,
        pub staked: bool,
        pub stake: u128,
        pub unstake_delay_sec: u32,
        pub withdraw_time: u64,
    }
}
