pub use inoncemanager::*;
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
pub mod inoncemanager {
    const _: () = {
        ::core::include_bytes!(
            "/home/harry/qi/eth-waterloo/pseudo_bundler/src/abi/INonceManager.json"
        );
    };
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint192\",\"name\":\"key\",\"type\":\"uint192\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getNonce\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint192\",\"name\":\"key\",\"type\":\"uint192\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"incrementNonce\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static INONCEMANAGER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(|| {
            ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid")
        });
    pub struct inoncemanager<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for inoncemanager<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for inoncemanager<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for inoncemanager<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for inoncemanager<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(inoncemanager))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> inoncemanager<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                INONCEMANAGER_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `getNonce` (0x35567e1a) function
        pub fn get_nonce(
            &self,
            sender: ::ethers::core::types::Address,
            key: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([53, 86, 126, 26], (sender, key))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `incrementNonce` (0x0bd28e3b) function
        pub fn increment_nonce(
            &self,
            key: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([11, 210, 142, 59], key)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for inoncemanager<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `getNonce` function with signature `getNonce(address,uint192)` and selector `0x35567e1a`
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
    #[ethcall(name = "getNonce", abi = "getNonce(address,uint192)")]
    pub struct GetNonceCall {
        pub sender: ::ethers::core::types::Address,
        pub key: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `incrementNonce` function with signature `incrementNonce(uint192)` and selector `0x0bd28e3b`
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
    #[ethcall(name = "incrementNonce", abi = "incrementNonce(uint192)")]
    pub struct IncrementNonceCall {
        pub key: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum inoncemanagerCalls {
        GetNonce(GetNonceCall),
        IncrementNonce(IncrementNonceCall),
    }
    impl ::ethers::core::abi::AbiDecode for inoncemanagerCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <GetNonceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetNonce(decoded));
            }
            if let Ok(decoded) =
                <IncrementNonceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::IncrementNonce(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for inoncemanagerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::GetNonce(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IncrementNonce(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for inoncemanagerCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::GetNonce(element) => ::core::fmt::Display::fmt(element, f),
                Self::IncrementNonce(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<GetNonceCall> for inoncemanagerCalls {
        fn from(value: GetNonceCall) -> Self {
            Self::GetNonce(value)
        }
    }
    impl ::core::convert::From<IncrementNonceCall> for inoncemanagerCalls {
        fn from(value: IncrementNonceCall) -> Self {
            Self::IncrementNonce(value)
        }
    }
    ///Container type for all return fields from the `getNonce` function with signature `getNonce(address,uint192)` and selector `0x35567e1a`
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
    pub struct GetNonceReturn {
        pub nonce: ::ethers::core::types::U256,
    }
}
