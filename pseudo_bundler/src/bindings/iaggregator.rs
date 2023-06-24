pub use iaggregator::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod iaggregator {
    const _: () = {
        ::core::include_bytes!(
            "/home/harry/qi/eth-waterloo/pseudo_bundler/src/abi/IAggregator.json"
        );
    };
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"struct UserOperation[]\",\"name\":\"userOps\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"initCode\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"callData\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"callGasLimit\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"verificationGasLimit\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"preVerificationGas\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"maxFeePerGas\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"maxPriorityFeePerGas\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"paymasterAndData\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"aggregateSignatures\",\"outputs\":[{\"internalType\":\"bytes\",\"name\":\"aggregatedSignature\",\"type\":\"bytes\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct UserOperation[]\",\"name\":\"userOps\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"initCode\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"callData\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"callGasLimit\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"verificationGasLimit\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"preVerificationGas\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"maxFeePerGas\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"maxPriorityFeePerGas\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"paymasterAndData\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]}]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"validateSignatures\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"struct UserOperation\",\"name\":\"userOp\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"initCode\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"callData\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"callGasLimit\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"verificationGasLimit\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"preVerificationGas\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"maxFeePerGas\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"maxPriorityFeePerGas\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"paymasterAndData\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"validateUserOpSignature\",\"outputs\":[{\"internalType\":\"bytes\",\"name\":\"sigForUserOp\",\"type\":\"bytes\",\"components\":[]}]}]";
    ///The parsed JSON ABI of the contract.
    pub static IAGGREGATOR_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(||
    ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid"));
    pub struct iaggregator<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for iaggregator<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for iaggregator<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for iaggregator<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for iaggregator<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(iaggregator)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> iaggregator<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IAGGREGATOR_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `aggregateSignatures` (0x275e2d79) function
        pub fn aggregate_signatures(
            &self,
            user_ops: ::std::vec::Vec<UserOperation>,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([39, 94, 45, 121], user_ops)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `validateSignatures` (0xe3563a4f) function
        pub fn validate_signatures(
            &self,
            user_ops: ::std::vec::Vec<UserOperation>,
            signature: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([227, 86, 58, 79], (user_ops, signature))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `validateUserOpSignature` (0x64c530cd) function
        pub fn validate_user_op_signature(
            &self,
            user_op: UserOperation,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([100, 197, 48, 205], (user_op,))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for iaggregator<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `aggregateSignatures` function with signature `aggregateSignatures((address,uint256,bytes,bytes,uint256,uint256,uint256,uint256,uint256,bytes,bytes)[])` and selector `0x275e2d79`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "aggregateSignatures",
        abi = "aggregateSignatures((address,uint256,bytes,bytes,uint256,uint256,uint256,uint256,uint256,bytes,bytes)[])"
    )]
    pub struct AggregateSignaturesCall {
        pub user_ops: ::std::vec::Vec<UserOperation>,
    }
    ///Container type for all input parameters for the `validateSignatures` function with signature `validateSignatures((address,uint256,bytes,bytes,uint256,uint256,uint256,uint256,uint256,bytes,bytes)[],bytes)` and selector `0xe3563a4f`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "validateSignatures",
        abi = "validateSignatures((address,uint256,bytes,bytes,uint256,uint256,uint256,uint256,uint256,bytes,bytes)[],bytes)"
    )]
    pub struct ValidateSignaturesCall {
        pub user_ops: ::std::vec::Vec<UserOperation>,
        pub signature: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `validateUserOpSignature` function with signature `validateUserOpSignature((address,uint256,bytes,bytes,uint256,uint256,uint256,uint256,uint256,bytes,bytes))` and selector `0x64c530cd`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "validateUserOpSignature",
        abi = "validateUserOpSignature((address,uint256,bytes,bytes,uint256,uint256,uint256,uint256,uint256,bytes,bytes))"
    )]
    pub struct ValidateUserOpSignatureCall {
        pub user_op: UserOperation,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum iaggregatorCalls {
        AggregateSignatures(AggregateSignaturesCall),
        ValidateSignatures(ValidateSignaturesCall),
        ValidateUserOpSignature(ValidateUserOpSignatureCall),
    }
    impl ::ethers::core::abi::AbiDecode for iaggregatorCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <AggregateSignaturesCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::AggregateSignatures(decoded));
            }
            if let Ok(decoded)
                = <ValidateSignaturesCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ValidateSignatures(decoded));
            }
            if let Ok(decoded)
                = <ValidateUserOpSignatureCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ValidateUserOpSignature(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for iaggregatorCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AggregateSignatures(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ValidateSignatures(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ValidateUserOpSignature(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for iaggregatorCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AggregateSignatures(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ValidateSignatures(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ValidateUserOpSignature(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<AggregateSignaturesCall> for iaggregatorCalls {
        fn from(value: AggregateSignaturesCall) -> Self {
            Self::AggregateSignatures(value)
        }
    }
    impl ::core::convert::From<ValidateSignaturesCall> for iaggregatorCalls {
        fn from(value: ValidateSignaturesCall) -> Self {
            Self::ValidateSignatures(value)
        }
    }
    impl ::core::convert::From<ValidateUserOpSignatureCall> for iaggregatorCalls {
        fn from(value: ValidateUserOpSignatureCall) -> Self {
            Self::ValidateUserOpSignature(value)
        }
    }
    ///Container type for all return fields from the `aggregateSignatures` function with signature `aggregateSignatures((address,uint256,bytes,bytes,uint256,uint256,uint256,uint256,uint256,bytes,bytes)[])` and selector `0x275e2d79`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct AggregateSignaturesReturn {
        pub aggregated_signature: ::ethers::core::types::Bytes,
    }
    ///Container type for all return fields from the `validateUserOpSignature` function with signature `validateUserOpSignature((address,uint256,bytes,bytes,uint256,uint256,uint256,uint256,uint256,bytes,bytes))` and selector `0x64c530cd`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ValidateUserOpSignatureReturn {
        pub sig_for_user_op: ::ethers::core::types::Bytes,
    }
    ///`UserOperation(address,uint256,bytes,bytes,uint256,uint256,uint256,uint256,uint256,bytes,bytes)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct UserOperation {
        pub sender: ::ethers::core::types::Address,
        pub nonce: ::ethers::core::types::U256,
        pub init_code: ::ethers::core::types::Bytes,
        pub call_data: ::ethers::core::types::Bytes,
        pub call_gas_limit: ::ethers::core::types::U256,
        pub verification_gas_limit: ::ethers::core::types::U256,
        pub pre_verification_gas: ::ethers::core::types::U256,
        pub max_fee_per_gas: ::ethers::core::types::U256,
        pub max_priority_fee_per_gas: ::ethers::core::types::U256,
        pub paymaster_and_data: ::ethers::core::types::Bytes,
        pub signature: ::ethers::core::types::Bytes,
    }
}
