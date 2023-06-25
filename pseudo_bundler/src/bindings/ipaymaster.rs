pub use ipaymaster::*;
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
pub mod ipaymaster {
    const _: () = {
        ::core::include_bytes!(
            "/home/harry/misc/eth-waterloo/pseudo_bundler/src/abi/IPaymaster.json"
        );
    };
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"enum IPaymaster.PostOpMode\",\"name\":\"mode\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"context\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"actualGasCost\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"postOp\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"struct UserOperation\",\"name\":\"userOp\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"initCode\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"callData\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"callGasLimit\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"verificationGasLimit\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"preVerificationGas\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"maxFeePerGas\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"maxPriorityFeePerGas\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"paymasterAndData\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]}]},{\"internalType\":\"bytes32\",\"name\":\"userOpHash\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"maxCost\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"validatePaymasterUserOp\",\"outputs\":[{\"internalType\":\"bytes\",\"name\":\"context\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"validationData\",\"type\":\"uint256\",\"components\":[]}]}]";
    ///The parsed JSON ABI of the contract.
    pub static IPAYMASTER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(|| {
            ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid")
        });
    pub struct ipaymaster<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ipaymaster<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ipaymaster<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ipaymaster<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ipaymaster<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(ipaymaster))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ipaymaster<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                IPAYMASTER_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `postOp` (0xa9a23409) function
        pub fn post_op(
            &self,
            mode: u8,
            context: ::ethers::core::types::Bytes,
            actual_gas_cost: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([169, 162, 52, 9], (mode, context, actual_gas_cost))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `validatePaymasterUserOp` (0xf465c77e) function
        pub fn validate_paymaster_user_op(
            &self,
            user_op: UserOperation,
            user_op_hash: [u8; 32],
            max_cost: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::Bytes, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([244, 101, 199, 126], (user_op, user_op_hash, max_cost))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for ipaymaster<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `postOp` function with signature `postOp(uint8,bytes,uint256)` and selector `0xa9a23409`
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
    #[ethcall(name = "postOp", abi = "postOp(uint8,bytes,uint256)")]
    pub struct PostOpCall {
        pub mode: u8,
        pub context: ::ethers::core::types::Bytes,
        pub actual_gas_cost: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `validatePaymasterUserOp` function with signature `validatePaymasterUserOp((address,uint256,bytes,bytes,uint256,uint256,uint256,uint256,uint256,bytes,bytes),bytes32,uint256)` and selector `0xf465c77e`
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
    #[ethcall(
        name = "validatePaymasterUserOp",
        abi = "validatePaymasterUserOp((address,uint256,bytes,bytes,uint256,uint256,uint256,uint256,uint256,bytes,bytes),bytes32,uint256)"
    )]
    pub struct ValidatePaymasterUserOpCall {
        pub user_op: UserOperation,
        pub user_op_hash: [u8; 32],
        pub max_cost: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ipaymasterCalls {
        PostOp(PostOpCall),
        ValidatePaymasterUserOp(ValidatePaymasterUserOpCall),
    }
    impl ::ethers::core::abi::AbiDecode for ipaymasterCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <PostOpCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PostOp(decoded));
            }
            if let Ok(decoded) =
                <ValidatePaymasterUserOpCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ValidatePaymasterUserOp(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ipaymasterCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::PostOp(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ValidatePaymasterUserOp(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for ipaymasterCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::PostOp(element) => ::core::fmt::Display::fmt(element, f),
                Self::ValidatePaymasterUserOp(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<PostOpCall> for ipaymasterCalls {
        fn from(value: PostOpCall) -> Self {
            Self::PostOp(value)
        }
    }
    impl ::core::convert::From<ValidatePaymasterUserOpCall> for ipaymasterCalls {
        fn from(value: ValidatePaymasterUserOpCall) -> Self {
            Self::ValidatePaymasterUserOp(value)
        }
    }
    ///Container type for all return fields from the `validatePaymasterUserOp` function with signature `validatePaymasterUserOp((address,uint256,bytes,bytes,uint256,uint256,uint256,uint256,uint256,bytes,bytes),bytes32,uint256)` and selector `0xf465c77e`
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
    pub struct ValidatePaymasterUserOpReturn {
        pub context: ::ethers::core::types::Bytes,
        pub validation_data: ::ethers::core::types::U256,
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
        Hash,
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
