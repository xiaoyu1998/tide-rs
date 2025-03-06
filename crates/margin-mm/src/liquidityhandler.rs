///Module containing a contract's types and functions.
/**

```solidity
library LiquidityUtils {
    struct AddParams { address token0; address token1; address to; }
    struct RemoveParams { address token0; address token1; uint256 liquidity; address to; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style
)]
pub mod LiquidityUtils {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
struct AddParams { address token0; address token1; address to; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AddParams {
        pub token0: alloy::sol_types::private::Address,
        pub token1: alloy::sol_types::private::Address,
        pub to: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<AddParams> for UnderlyingRustTuple<'_> {
            fn from(value: AddParams) -> Self {
                (value.token0, value.token1, value.to)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for AddParams {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    token0: tuple.0,
                    token1: tuple.1,
                    to: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for AddParams {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for AddParams {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.token0,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.token1,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.to,
                    ),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for AddParams {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for AddParams {
            const NAME: &'static str = "AddParams";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "AddParams(address token0,address token1,address to)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.token0,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.token1,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.to,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for AddParams {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.token0,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.token1,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.to,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.token0,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.token1,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.to,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    /**```solidity
struct RemoveParams { address token0; address token1; uint256 liquidity; address to; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct RemoveParams {
        pub token0: alloy::sol_types::private::Address,
        pub token1: alloy::sol_types::private::Address,
        pub liquidity: alloy::sol_types::private::primitives::aliases::U256,
        pub to: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Address,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::Address,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<RemoveParams> for UnderlyingRustTuple<'_> {
            fn from(value: RemoveParams) -> Self {
                (value.token0, value.token1, value.liquidity, value.to)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for RemoveParams {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    token0: tuple.0,
                    token1: tuple.1,
                    liquidity: tuple.2,
                    to: tuple.3,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for RemoveParams {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for RemoveParams {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.token0,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.token1,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.liquidity),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.to,
                    ),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for RemoveParams {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for RemoveParams {
            const NAME: &'static str = "RemoveParams";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "RemoveParams(address token0,address token1,uint256 liquidity,address to)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.token0,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.token1,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.liquidity)
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.to,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for RemoveParams {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.token0,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.token1,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.liquidity,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.to,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.token0,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.token1,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.liquidity,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.to,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`LiquidityUtils`](self) contract instance.

See the [wrapper's documentation](`LiquidityUtilsInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> LiquidityUtilsInstance<T, P, N> {
        LiquidityUtilsInstance::<T, P, N>::new(address, provider)
    }
    /**A [`LiquidityUtils`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`LiquidityUtils`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct LiquidityUtilsInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for LiquidityUtilsInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("LiquidityUtilsInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > LiquidityUtilsInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`LiquidityUtils`](self) contract instance.

See the [wrapper's documentation](`LiquidityUtilsInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            provider: P,
        ) -> Self {
            Self {
                address,
                provider,
                _network_transport: ::core::marker::PhantomData,
            }
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<T, P: ::core::clone::Clone, N> LiquidityUtilsInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> LiquidityUtilsInstance<T, P, N> {
            LiquidityUtilsInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > LiquidityUtilsInstance<T, P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > LiquidityUtilsInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
/**

Generated by the following Solidity interface...
```solidity
library LiquidityUtils {
    struct AddParams {
        address token0;
        address token1;
        address to;
    }
    struct RemoveParams {
        address token0;
        address token1;
        uint256 liquidity;
        address to;
    }
}

interface LiquidityHandler {
    error EmptyAddAmounts();
    error EmptyPool(bytes32 key);
    error EmptyRemoveAmounts();
    error InsufficientUserBalance(uint256 balance, uint256 liquidity);
    error MathOverflowedMulDiv();
    error Reserve0Insufficient(uint256 amount0, uint256 availableReserve0);
    error Reserve1Insufficient(uint256 amount1, uint256 availableReserve1);
    error Unauthorized(address msgSender, string role);

    constructor(address _roleStore, address _dataStore, address _eventEmitter);

    function dataStore() external view returns (address);
    function eventEmitter() external view returns (address);
    function executeAdd(address account, LiquidityUtils.AddParams memory AddParams) external;
    function executeRemove(address account, LiquidityUtils.RemoveParams memory removeParams) external;
    function roleStore() external view returns (address);
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [
      {
        "name": "_roleStore",
        "type": "address",
        "internalType": "contract RoleStore"
      },
      {
        "name": "_dataStore",
        "type": "address",
        "internalType": "contract DataStore"
      },
      {
        "name": "_eventEmitter",
        "type": "address",
        "internalType": "contract EventEmitter"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "dataStore",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract DataStore"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "eventEmitter",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract EventEmitter"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "executeAdd",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "AddParams",
        "type": "tuple",
        "internalType": "struct LiquidityUtils.AddParams",
        "components": [
          {
            "name": "token0",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "token1",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "to",
            "type": "address",
            "internalType": "address"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "executeRemove",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "removeParams",
        "type": "tuple",
        "internalType": "struct LiquidityUtils.RemoveParams",
        "components": [
          {
            "name": "token0",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "token1",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "liquidity",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "to",
            "type": "address",
            "internalType": "address"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "roleStore",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract RoleStore"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "error",
    "name": "EmptyAddAmounts",
    "inputs": []
  },
  {
    "type": "error",
    "name": "EmptyPool",
    "inputs": [
      {
        "name": "key",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ]
  },
  {
    "type": "error",
    "name": "EmptyRemoveAmounts",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InsufficientUserBalance",
    "inputs": [
      {
        "name": "balance",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "liquidity",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "MathOverflowedMulDiv",
    "inputs": []
  },
  {
    "type": "error",
    "name": "Reserve0Insufficient",
    "inputs": [
      {
        "name": "amount0",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "availableReserve0",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "Reserve1Insufficient",
    "inputs": [
      {
        "name": "amount1",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "availableReserve1",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "Unauthorized",
    "inputs": [
      {
        "name": "msgSender",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "role",
        "type": "string",
        "internalType": "string"
      }
    ]
  }
]
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style
)]
pub mod LiquidityHandler {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60e060405234801561000f575f5ffd5b5060405161535738038061535783398101604081905261002e91610062565b6001600160a01b0391821660805291811660a0521660c0526100ac565b6001600160a01b038116811461005f575f5ffd5b50565b5f5f5f60608486031215610074575f5ffd5b835161007f8161004b565b60208501519093506100908161004b565b60408501519092506100a18161004b565b809150509250925092565b60805160a05160c05161524c61010b5f395f818160f0015281816101c601526102de01525f8181605e015261055601525f818160b601528181610197015281816102af0152818161037f015281816104850152610776015261524c5ff3fe608060405234801561000f575f5ffd5b5060043610610055575f3560e01c80634a4a7b04146100595780635ecd44e81461009c578063660d0d67146100b15780637d237c99146100d85780639ff78c30146100eb575b5f5ffd5b6100807f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b03909116815260200160405180910390f35b6100af6100aa366004614e0a565b610112565b005b6100807f000000000000000000000000000000000000000000000000000000000000000081565b6100af6100e6366004614e48565b610268565b6100807f000000000000000000000000000000000000000000000000000000000000000081565b61011a61037c565b61018960405160200161014b906020808252600a908201526921a7a72a2927a62622a960b11b604082015260600190565b604051602081830303815290604052805190602001206040518060400160405280600a81526020016921a7a72a2927a62622a960b11b81525061053a565b5f6040518060a001604052807f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031681526020017f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602001835f0160208101906102059190614e78565b6001600160a01b031681526020018360200160208101906102269190614e78565b6001600160a01b031681526020016102446060850160408601614e78565b6001600160a01b03169052905061025b83826105e8565b50610264610774565b5050565b61027061037c565b6102a160405160200161014b906020808252600a908201526921a7a72a2927a62622a960b11b604082015260600190565b5f6040518060c001604052807f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031681526020017f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602001835f01602081019061031d9190614e78565b6001600160a01b0316815260200183602001602081019061033e9190614e78565b6001600160a01b031681526040808501356020830152016103656080850160608601614e78565b6001600160a01b03169052905061025b838261082d565b5f7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663bd02d0f56040516020016103bb90614e93565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016103ef91815260200190565b602060405180830381865afa15801561040a573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061042e9190614eca565b905080156104835760405162461bcd60e51b815260206004820152601f60248201527f5265656e7472616e637947756172643a207265656e7472616e742063616c6c0060448201526064015b60405180910390fd5b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663e2a4853a6040516020016104c190614e93565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b1682526004820152600160248201526044016020604051808303815f875af1158015610516573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906102649190614eca565b60405163ac4ab3fb60e01b8152336004820152602481018390527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063ac4ab3fb90604401602060405180830381865afa1580156105a3573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906105c79190614ee1565b61026457338160405163a35b150b60e01b815260040161047a929190614f2e565b5f5f610600835f015184604001518560600151610a59565b915091505f61060e83610a9c565b905061061e845f01518483610b56565b6020830151604085810151905163352f9aed60e01b81526001600160a01b0391821660048201525f9183169063352f9aed906024016020604051808303815f875af115801561066f573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906106939190614eca565b606087015160405163352f9aed60e01b81526001600160a01b0391821660048201529192505f919084169063352f9aed906024016020604051808303815f875af11580156106e3573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906107079190614eca565b9050610713828261105c565b610724868484848b60800151611085565b865161073090876111c3565b61073e876020015187611207565b865161074b90868861122b565b61076a87602001518989604001518a606001518b608001518787612447565b5050505050505050565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663e2a4853a6040516020016107b290614e93565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b16825260048201525f60248201526044016020604051808303815f875af1158015610806573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061082a9190614eca565b50565b610835614cb3565b61084b825f015183604001518460600151610a59565b6020830152808252608083015161086391859161249b565b815181516080840151610877929190612558565b60c0850181905260a08501829052610120850183905261010085018490526108a1939291906127ad565b8051602001516001600160a01b03908116604080840182905260808501519051632770a7eb60e21b81529286166004840152602483015290639dc29fac906044015f604051808303815f87803b1580156108f9575f5ffd5b505af115801561090b573d5f5f3e3d5ffd5b505050506040818101518382015160a0850151610100850151935163078d3b7960e01b81526001600160a01b03928316600482015290821660248201526044810193909352169063078d3b79906064015f604051808303815f87803b158015610972575f5ffd5b505af1158015610984573d5f5f3e3d5ffd5b50505050604081810151606084015160a0850151610120850151935163078d3b7960e01b81526001600160a01b03928316600482015290821660248201526044810193909352169063078d3b79906064015f604051808303815f87803b1580156109ec575f5ffd5b505af11580156109fe573d5f5f3e3d5ffd5b50505060208301518251610a129250611207565b815160208201518251610a2692919061122b565b610a548260200151848460400151856060015186608001518760a00151876101000151886101200151612803565b505050565b610a61614d0c565b5f5f610a6d8585612890565b90505f610a7a8783612938565b9050610a868183613b3a565b610a8f81613b68565b925090505b935093915050565b5f5f610aaa835f5f5f613c14565b50505090505f610abd8460015f5f613c14565b5050509050815f1480610ace575080155b15610adc57505f9392505050565b5f610aeb85606001515f613cfd565b90505f610afd86606001516001613cfd565b90505f610b2085676765c793fa10079d601b1b610b1b86600a615040565b613d2b565b90505f610b3e85676765c793fa10079d601b1b610b1b86600a615040565b9050610b4a8282613deb565b98975050505050505050565b805f03610b6257505050565b610b836040518060600160405280602581526020016151f260259139613e26565b60408051610140810182525f80825260208201819052918101829052606081018290526080810182905260a0810182905260c0810182905260e081018290526101008101829052610120810191909152610bdd8484613e4a565b63ffffffff168152610bf46401000000004261505f565b63ffffffff90811660208301528151165f03610cb057610c1984848360200151613ec0565b610c2484845f613f44565b610c3384848360200151613f83565b610c3e84845f613f9b565b610c49848484613fb3565b610c836040518060400160405280600e81526020016d0626c6f636b54696d655374616d760941b815250826020015163ffffffff16613fcb565b610caa60405180604001604052806005815260200164707269636560d81b81525083613fcb565b50505050565b80516020820151610cc19190615072565b63ffffffff90811660408084019182528051808201909152600b81526a1d1a5b59515b185c1cd95960aa1b60208201529051610cfd9216613fcb565b604081015163ffffffff1615610caa57610d178484613ff4565b60608201526040810151610d319063ffffffff168361508e565b8160600151610d4091906150a5565b6080820152604080518082019091526005815264707269636560d81b6020820152610d6b9083613fcb565b610da26040518060400160405280600b81526020016a1d1a5b59515b185c1cd95960aa1b815250826040015163ffffffff16613fcb565b610ddb604051806040016040528060138152602001721c1c9a58d950dd5b5d5b185d1a5d9953185cdd606a1b8152508260600151613fcb565b610e106040518060400160405280600f81526020016e707269636543756d756c617469766560881b8152508260800151613fcb565b610e1f84848360800151613f44565b610e2e84848360200151613ec0565b610e38848461400d565b63ffffffff1660a082018190526020820151610e549190615072565b63ffffffff1660c0820152610e6884614026565b63ffffffff90811660e083019081526040805180820190915260068152651411549253d160d21b60208201529051610ea09216613fcb565b610ee96040518060400160405280601b81526020017f6c61737455706461746554696d657374616d704279506572696f6400000000008152508260a0015163ffffffff16613fcb565b610f28604051806040016040528060138152602001721d1a5b59515b185c1cd959109e54195c9a5bd9606a1b8152508260c0015163ffffffff16613fcb565b8060e0015163ffffffff168160c0015163ffffffff161115610caa57610f4e84846140d8565b610100820181905260c0820151608083015163ffffffff90911691610f72916150b8565b610f7c91906150cb565b61012082015260408051808201909152601b81527f707269636543756d756c61746976654c6173744279506572696f6400000000006020820152610100820151610fc69190613fcb565b610ffb6040518060400160405280600f81526020016e707269636543756d756c617469766560881b8152508260800151613fcb565b61102e6040518060400160405280600c81526020016b70726963654176657261676560a01b815250826101200151613fcb565b61103d84848360200151613f83565b61104c84848360800151613f9b565b610caa8484836101200151613fb3565b811580611067575080155b1561026457604051631a5df28360e21b815260040160405180910390fd5b5f61109386858560016140f1565b90505f856001600160a01b03166318160ddd6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156110d2573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906110f69190614eca565b9050805f0361115c576040516340c10f1960e01b81525f60048201526103e860248201526001600160a01b038716906340c10f19906044015f604051808303815f87803b158015611145575f5ffd5b505af1158015611157573d5f5f3e3d5ffd5b505050505b6040516340c10f1960e01b81526001600160a01b038481166004830152602482018490528716906340c10f19906044015b5f604051808303815f87803b1580156111a4575f5ffd5b505af11580156111b6573d5f5f3e3d5ffd5b5050505050505050505050565b6060810151660800000000000016610264575f5f6111e1848461424d565b91509150818110610caa5760608301516111fc9060016142ae565b606084015250505050565b611210816142d8565b91518051602090910151604090810193909352919091015250565b5f839050806001600160a01b031663c80f4c6260405160200161126b906020808252600990820152681413d3d317d31254d560ba1b604082015260600190565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b1682526004820152602481018690526044015f604051808303815f87803b1580156112bb575f5ffd5b505af11580156112cd573d5f5f3e3d5ffd5b50505050806001600160a01b031663ca446dd984604051602001611311906020808252600c908201526b0504f4f4c5f544f4b454e5f360a41b604082015260600190565b60405160208183030381529060405280519060200120604051602001611341929190918252602082015260400190565b60408051808303601f19018152908290528051602090910120855151516001600160e01b031960e085901b16835260048301919091526001600160a01b031660248201526044016020604051808303815f875af11580156113a4573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906113c891906150f2565b50806001600160a01b031663e2a4853a84604051602001611410906020808252601390820152720504f4f4c5f424f52524f575f494e4445585f3606c1b604082015260600190565b60405160208183030381529060405280519060200120604051602001611440929190918252602082015260400190565b60408051808303601f19018152908290528051602091820120865151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af115801561149d573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906114c19190614eca565b50806001600160a01b031663e2a4853a84604051602001611508906020808252601290820152710504f4f4c5f424f52524f575f524154455f360741b604082015260600190565b60405160208183030381529060405280519060200120604051602001611538929190918252602082015260400190565b60408051808303601f190181528282528051602090910120865151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af1158015611594573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906115b89190614eca565b50806001600160a01b031663e2a4853a84604051602001611604906020808252601790820152760504f4f4c5f544f54414c5f434f4c4c41544552414c5f3604c1b604082015260600190565b60405160208183030381529060405280519060200120604051602001611634929190918252602082015260400190565b60408051808303601f190181529082905280516020909101208551516060015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015611691573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906116b59190614eca565b50806001600160a01b031663e2a4853a846040516020016116d59061510d565b60405160208183030381529060405280519060200120604051602001611705929190918252602082015260400190565b60408051808303601f190181529082905280516020909101208551516080015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015611762573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906117869190614eca565b50806001600160a01b031663e2a4853a846040516020016117d3906020808252601890820152770504f4f4c5f544f54414c5f5343414c45445f444542545f360441b604082015260600190565b60405160208183030381529060405280519060200120604051602001611803929190918252602082015260400190565b60408051808303601f1901815290829052805160209091012085515160a0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015611860573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906118849190614eca565b50806001600160a01b031663e2a4853a846040516020016118cd906020808252601490820152730504f4f4c5f554e434c41494d45445f4645455f360641b604082015260600190565b604051602081830303815290604052805190602001206040516020016118fd929190918252602082015260400190565b60408051808303601f1901815290829052805160209091012085515160c0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af115801561195a573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061197e9190614eca565b50806001600160a01b031663ca446dd9846040516020016119bf906020808252600c908201526b504f4f4c5f544f4b454e5f3160a01b604082015260600190565b604051602081830303815290604052805190602001206040516020016119ef929190918252602082015260400190565b60408051808303601f19018152908290528051602091820120865190910151516001600160e01b031960e085901b16835260048301919091526001600160a01b031660248201526044016020604051808303815f875af1158015611a55573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611a7991906150f2565b50806001600160a01b031663e2a4853a84604051602001611ac190602080825260139082015272504f4f4c5f424f52524f575f494e4445585f3160681b604082015260600190565b60405160208183030381529060405280519060200120604051602001611af1929190918252602082015260400190565b60408051808303601f190181529082905280516020918201208651820151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af1158015611b50573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611b749190614eca565b50806001600160a01b031663e2a4853a84604051602001611bbb90602080825260129082015271504f4f4c5f424f52524f575f524154455f3160701b604082015260600190565b60405160208183030381529060405280519060200120604051602001611beb929190918252602082015260400190565b60408051808303601f190181528282528051602091820120875190910151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af1158015611c4a573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611c6e9190614eca565b50806001600160a01b031663e2a4853a84604051602001611cba90602080825260179082015276504f4f4c5f544f54414c5f434f4c4c41544552414c5f3160481b604082015260600190565b60405160208183030381529060405280519060200120604051602001611cea929190918252602082015260400190565b60408051808303601f190181529082905280516020918201208651909101516060015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015611d4a573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611d6e9190614eca565b50806001600160a01b031663e2a4853a84604051602001611d8e9061514e565b60405160208183030381529060405280519060200120604051602001611dbe929190918252602082015260400190565b60408051808303601f190181529082905280516020918201208651909101516080015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015611e1e573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611e429190614eca565b50806001600160a01b031663e2a4853a84604051602001611e8f90602080825260189082015277504f4f4c5f544f54414c5f5343414c45445f444542545f3160401b604082015260600190565b60405160208183030381529060405280519060200120604051602001611ebf929190918252602082015260400190565b60408051808303601f1901815290829052805160209182012086519091015160a0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015611f1f573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611f439190614eca565b50806001600160a01b031663e2a4853a84604051602001611f8c90602080825260149082015273504f4f4c5f554e434c41494d45445f4645455f3160601b604082015260600190565b60405160208183030381529060405280519060200120604051602001611fbc929190918252602082015260400190565b60408051808303601f1901815290829052805160209182012086519091015160c0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af115801561201c573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906120409190614eca565b50806001600160a01b031663ca446dd98460405160200161207e90602080825260099082015268504f4f4c5f42414e4b60b81b604082015260600190565b604051602081830303815290604052805190602001206040516020016120ae929190918252602082015260400190565b6040516020818303038152906040528051906020012084602001516040518363ffffffff1660e01b81526004016120f89291909182526001600160a01b0316602082015260400190565b6020604051808303815f875af1158015612114573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061213891906150f2565b50806001600160a01b031663ca446dd98460405160200161218a906020808252601b908201527f504f4f4c5f494e5445524553545f524154455f53545241544547590000000000604082015260600190565b604051602081830303815290604052805190602001206040516020016121ba929190918252602082015260400190565b60408051601f198184030181528282528051602090910120908601516001600160e01b031960e085901b168352612205926004019182526001600160a01b0316602082015260400190565b6020604051808303815f875af1158015612221573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061224591906150f2565b50806001600160a01b031663e2a4853a8460405160200161228c906020808252601290820152712827a7a62fa1a7a72324a3aaa920aa24a7a760711b604082015260600190565b604051602081830303815290604052805190602001206040516020016122bc929190918252602082015260400190565b6040516020818303038152906040528051906020012084606001516040518363ffffffff1660e01b81526004016122fd929190918252602082015260400190565b6020604051808303815f875af1158015612319573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061233d9190614eca565b50806001600160a01b031663e2a4853a8460405160200161238f906020808252601b908201527f504f4f4c5f4c4153545f5550444154455f54494d455f5354414d500000000000604082015260600190565b604051602081830303815290604052805190602001206040516020016123bf929190918252602082015260400190565b6040516020818303038152906040528051906020012084608001516040518363ffffffff1660e01b8152600401612400929190918252602082015260400190565b6020604051808303815f875af115801561241c573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906124409190614eca565b5050505050565b604051639ed486eb60e01b81526001600160a01b0387811660048301528681166024830152858116604483015284811660648301526084820184905260a48201839052881690639ed486eb9060c40161118d565b805f036124bb57604051635186591160e01b815260040160405180910390fd5b60208201516040516370a0823160e01b81526001600160a01b0385811660048301525f91908316906370a0823190602401602060405180830381865afa158015612507573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061252b9190614eca565b905080831115612440576040516302fa826960e51b8152600481018290526024810184905260440161047a565b5f5f5f5f612564614d40565b61256d886144e7565b8161014001818152505086602001516001600160a01b03166318160ddd6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156125b7573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906125db9190614eca565b8160200181815250506125f5875f5f846101400151613c14565b5060808401525060408201526101408101516126179088906001905f90613c14565b5060a084015250606082015260208101515f03612640575f5f5f5f9450945094509450506127a4565b6126538682604001518360200151613d2b565b6101008201526060810151602082015161266e918891613d2b565b816101200181815250506126ae6040518060400160405280601081526020016f766172732e746f74616c537570706c7960801b8152508260200151613fcb565b6126e6604051806040016040528060128152602001710766172732e707269636552657365727665360741b8152508260400151613fcb565b61271e60405180604001604052806012815260200171766172732e7072696365526573657276653160701b8152508260600151613fcb565b6127516040518060400160405280600c81526020016b0766172732e616d6f756e74360a41b815250826101000151613fcb565b6127846040518060400160405280600c81526020016b766172732e616d6f756e743160a01b815250826101200151613fcb565b80610100015181610120015182608001518360a001519450945094509450505b93509350935093565b818411156127d857604051631fc107c160e01b8152600481018590526024810183905260440161047a565b80831115610caa57604051630e793baf60e01b8152600481018490526024810182905260440161047a565b604051631495739160e11b81526001600160a01b0388811660048301528781166024830152868116604483015260648201869052848116608483015260a4820184905260c4820183905289169063292ae7229060e4015f604051808303815f87803b158015612870575f5ffd5b505af1158015612882573d5f5f3e3d5ffd5b505050505050505050505050565b5f816001600160a01b0316836001600160a01b0316106128b15781836128b4565b82825b60405191945092506128e1906020016020808252600490820152631413d3d360e21b604082015260600190565b60408051601f198184030181528282528051602091820120908301526001600160a01b0380861691830191909152831660608201526080016040516020818303038152906040528051906020012090505b92915050565b612940614d0c565b82612949614d0c565b816001600160a01b03166391d4403c604051602001612985906020808252600990820152681413d3d317d31254d560ba1b604082015260600190565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b168252600482015260248101879052604401602060405180830381865afa1580156129d9573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906129fd9190614ee1565b612a0a5791506129329050565b816001600160a01b03166321f8a72185604051602001612a4a906020808252600c908201526b0504f4f4c5f544f4b454e5f360a41b604082015260600190565b60405160208183030381529060405280519060200120604051602001612a7a929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401612aae91815260200190565b602060405180830381865afa158015612ac9573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612aed91906150f2565b8151516001600160a01b03918216905260408051602081810152601391810191909152720504f4f4c5f424f52524f575f494e4445585f3606c1b60608201529083169063bd02d0f590869060800160405160208183030381529060405280519060200120604051602001612b6b929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401612b9f91815260200190565b602060405180830381865afa158015612bba573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612bde9190614eca565b81515f60200201516020018181525050816001600160a01b031663bd02d0f585604051602001612c34906020808252601290820152710504f4f4c5f424f52524f575f524154455f360741b604082015260600190565b60405160208183030381529060405280519060200120604051602001612c64929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401612c9891815260200190565b602060405180830381865afa158015612cb3573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612cd79190614eca565b81515f60200201516040018181525050816001600160a01b031663bd02d0f585604051602001612d32906020808252601790820152760504f4f4c5f544f54414c5f434f4c4c41544552414c5f3604c1b604082015260600190565b60405160208183030381529060405280519060200120604051602001612d62929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401612d9691815260200190565b602060405180830381865afa158015612db1573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612dd59190614eca565b815151606001526040516001600160a01b0383169063bd02d0f5908690612dfe9060200161510d565b60405160208183030381529060405280519060200120604051602001612e2e929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401612e6291815260200190565b602060405180830381865afa158015612e7d573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612ea19190614eca565b81515f60200201516080018181525050816001600160a01b031663bd02d0f585604051602001612efd906020808252601890820152770504f4f4c5f544f54414c5f5343414c45445f444542545f360441b604082015260600190565b60405160208183030381529060405280519060200120604051602001612f2d929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401612f6191815260200190565b602060405180830381865afa158015612f7c573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612fa09190614eca565b81515160a0015260408051602081810152601491810191909152730504f4f4c5f554e434c41494d45445f4645455f360641b60608201526001600160a01b0383169063bd02d0f59086906080016040516020818303038152906040528051906020012060405160200161301d929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161305191815260200190565b602060405180830381865afa15801561306c573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906130909190614eca565b81515160c0015260408051602081810152600c918101919091526b504f4f4c5f544f4b454e5f3160a01b60608201526001600160a01b038316906321f8a72190869060800160405160208183030381529060405280519060200120604051602001613105929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161313991815260200190565b602060405180830381865afa158015613154573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061317891906150f2565b81516020908101516001600160a01b0392831690526040805180830183905260138183015272504f4f4c5f424f52524f575f494e4445585f3160681b60608083019190915282518083039091018152608082019092528151919092012060a0820187905260c08201529083169063bd02d0f59060e001604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161322291815260200190565b602060405180830381865afa15801561323d573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906132619190614eca565b8151600160200201516020018181525050816001600160a01b031663bd02d0f5856040516020016132b890602080825260129082015271504f4f4c5f424f52524f575f524154455f3160701b604082015260600190565b604051602081830303815290604052805190602001206040516020016132e8929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161331c91815260200190565b602060405180830381865afa158015613337573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061335b9190614eca565b8151600160200201516040018181525050816001600160a01b031663bd02d0f5856040516020016133b790602080825260179082015276504f4f4c5f544f54414c5f434f4c4c41544552414c5f3160481b604082015260600190565b604051602081830303815290604052805190602001206040516020016133e7929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161341b91815260200190565b602060405180830381865afa158015613436573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061345a9190614eca565b8151600160200201516060018181525050816001600160a01b031663bd02d0f58560405160200161348a9061514e565b604051602081830303815290604052805190602001206040516020016134ba929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016134ee91815260200190565b602060405180830381865afa158015613509573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061352d9190614eca565b8151600160200201516080018181525050816001600160a01b031663bd02d0f58560405160200161358a90602080825260189082015277504f4f4c5f544f54414c5f5343414c45445f444542545f3160401b604082015260600190565b604051602081830303815290604052805190602001206040516020016135ba929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016135ee91815260200190565b602060405180830381865afa158015613609573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061362d9190614eca565b81516001602002015160a0018181525050816001600160a01b031663bd02d0f58560405160200161368690602080825260149082015273504f4f4c5f554e434c41494d45445f4645455f3160601b604082015260600190565b604051602081830303815290604052805190602001206040516020016136b6929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016136ea91815260200190565b602060405180830381865afa158015613705573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906137299190614eca565b81516001602002015160c0018181525050816001600160a01b03166321f8a7218560405160200161377790602080825260099082015268504f4f4c5f42414e4b60b81b604082015260600190565b604051602081830303815290604052805190602001206040516020016137a7929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016137db91815260200190565b602060405180830381865afa1580156137f6573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061381a91906150f2565b81602001906001600160a01b031690816001600160a01b031681525050816001600160a01b03166321f8a72185604051602001613888906020808252601b908201527f504f4f4c5f494e5445524553545f524154455f53545241544547590000000000604082015260600190565b604051602081830303815290604052805190602001206040516020016138b8929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016138ec91815260200190565b602060405180830381865afa158015613907573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061392b91906150f2565b81604001906001600160a01b031690816001600160a01b031681525050816001600160a01b031663bd02d0f58560405160200161398e906020808252601290820152712827a7a62fa1a7a72324a3aaa920aa24a7a760711b604082015260600190565b604051602081830303815290604052805190602001206040516020016139be929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016139f291815260200190565b602060405180830381865afa158015613a0d573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613a319190614eca565b60608201526040516001600160a01b0383169063bd02d0f5908690613a8a906020016020808252601b908201527f504f4f4c5f4c4153545f5550444154455f54494d455f5354414d500000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001613aba929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401613aee91815260200190565b602060405180830381865afa158015613b09573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613b2d9190614eca565b6080820152949350505050565b60208201516001600160a01b031661026457604051637357d91f60e01b81526004810182905260240161047a565b60808101514290819003613b7a575050565b81515160a0015115613bd05781515f90613ba390825b6020020151604001518460800151614538565b8351909150613bc79082905f5b60200201516020015161456c90919063ffffffff16565b83515160200152505b81516020015160a0015115613c0c5781515f90613bee906001613b90565b8351909150613c009082906001613bb0565b83516020908101510152505b608090910152565b5f5f5f5f5f885f01518860ff1660028110613c3157613c316150de565b602002015190505f613c438a8a6145ad565b9050805f03613c5f575f5f5f5f95509550955095505050613cf2565b5f613c6e838c6080015161469b565b90505f613c7b828a61456c565b90505f8915613ca057818410613c9a57613c9584836146cb565b613ca2565b5f613ca2565b5f5b90505f613caf858d61456c565b90505f8c15613cd457848210613cce57613cc982866146cb565b613cd6565b5f613cd6565b5f5b9050613ce285876150a5565b9a50949850909650929450505050505b945094509450949050565b5f60ff60581b1960585f1960ff851601613d1d575060ff60601b19905060605b90198416901c905092915050565b5f838302815f1985870982811083820303915050805f03613d5f57838281613d5557613d5561504b565b0492505050613de4565b808411613d7f5760405163227bc15360e01b815260040160405180910390fd5b5f848688095f868103871696879004966002600389028118808a02820302808a02820302808a02820302808a02820302808a02820302808a02909103029181900381900460010186841190950394909402919094039290920491909117919091029150505b9392505050565b5f8115676765c793fa10079d601b1b60028404190484111715613e0c575f5ffd5b50676765c793fa10079d601b1b9190910260028204010490565b61082a60405180604001604052806002815260200161257360f01b81525082614720565b5f826001600160a01b031663bd02d0f5613e6384614765565b6040518263ffffffff1660e01b8152600401613e8191815260200190565b602060405180830381865afa158015613e9c573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613de49190614eca565b826001600160a01b031663e2a4853a613ed884614765565b6040516001600160e01b031960e084901b168152600481019190915263ffffffff841660248201526044015b6020604051808303815f875af1158015613f20573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610caa9190614eca565b826001600160a01b031663e2a4853a613f5c84614818565b836040518363ffffffff1660e01b8152600401613f04929190918252602082015260400190565b826001600160a01b031663e2a4853a613ed88461486c565b826001600160a01b031663e2a4853a613f5c846148cd565b826001600160a01b031663e2a4853a613f5c84614933565b610264604051806040016040528060068152602001652573202d257360d01b815250838361497c565b5f826001600160a01b031663bd02d0f5613e6384614818565b5f826001600160a01b031663bd02d0f5613e638461486c565b5f816001600160a01b031663bd02d0f5604051602001614065906020808252600b908201526a1515d05417d411549253d160aa1b604082015260600190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161409991815260200190565b602060405180830381865afa1580156140b4573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906129329190614eca565b5f826001600160a01b031663bd02d0f5613e63846148cd565b5f6140fa614d40565b85602001516001600160a01b03166318160ddd6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561413a573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061415e9190614eca565b602082015261416f865f8080613c14565b50505060c08201526141848660015f80613c14565b50505060e082015282156141bf57848160c0018181516141a491906150b8565b90525060e0810180518591906141bb9083906150b8565b9052505b80602001515f036141ef576141e86103e86141e26141dd88886149c3565b614a29565b906146cb565b8152614241565b60c08101511580614202575060e0810151155b15614210575f915050614245565b61423e6142268683602001518460c00151613d2b565b6142398684602001518560e00151613d2b565b614b09565b81525b5190505b949350505050565b5f5f5f61425e84606001515f613cfd565b90505f61426a86614b1e565b90505f61428d8261427c85600a615040565b676765c793fa10079d601b1b613d2b565b90505f61429c875f5f5f613c14565b50939a91995090975050505050505050565b5f6033826142bc575f6142bf565b60015b60ff16901b660800000000000019841617905092915050565b5f5f61430d6040518060c001604052805f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b614317845f614b6f565b60208301528152606084015161432d905f613cfd565b60608201819052815161435291676765c793fa10079d601b1b90610b1b90600a615040565b604082015260208101515f0361436d575f608082015261440d565b60408481015181516060810183528351815260208085015190820190815284840151828501908152935163fdd63ecf60e01b815291516004830152516024820152915160448301526001600160a01b03169063fdd63ecf90606401602060405180830381865afa1580156143e3573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906144079190614eca565b60808201525b614418846001614b6f565b602083018190529082525f03614433575f60a08201526144d3565b60408481015181516060810183528351815260208085015190820190815284840151828501908152935163fdd63ecf60e01b815291516004830152516024820152915160448301526001600160a01b03169063fdd63ecf90606401602060405180830381865afa1580156144a9573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906144cd9190614eca565b60a08201525b80608001518160a001519250925050915091565b5f816001600160a01b031663bd02d0f5604051602001614065906020808252601e908201527f5452414441424c455f444542545f4d554c5449504945525f464143544f520000604082015260600190565b5f8061454483426150b8565b61454e908561508e565b6301e133809004905061424581676765c793fa10079d601b1b6150a5565b5f81156b019d971e4fe8401e74000000198390048411151761458c575f5ffd5b50676765c793fa10079d601b1b91026b019d971e4fe8401e74000000010490565b5f5f835f01518360ff16600281106145c7576145c76150de565b60209081029190910151908501518151604051637216047960e11b81526001600160a01b0391821660048201529293505f9291169063e42c08f290602401602060405180830381865afa158015614620573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906146449190614eca565b9050805f03614657575f92505050612932565b606082015160c083015161466b90826150a5565b821061468f5760c083015161468082846150b8565b61468a91906150b8565b614691565b5f5b9695505050505050565b5f8260a001515f036146ae57505f612932565b5f6146b98484614bb5565b60a0850151909150614245908261456c565b5f826146d783826150b8565b91508111156129325760405162461bcd60e51b815260206004820152601560248201527464732d6d6174682d7375622d756e646572666c6f7760581b604482015260640161047a565b610264828260405160240161473692919061518f565b60408051601f198184030181529190526020810180516001600160e01b0316634b5c427760e01b179052614bf8565b80515f90819061478690825b60200201515184516001602002015151612890565b9050806040516020016147ca906020808252601a908201527f545741505f4c4153545f424c4f434b5f54494d455f5354414d50000000000000604082015260600190565b604051602081830303815290604052805190602001206040516020016147fa929190918252602082015260400190565b60405160208183030381529060405280519060200120915050919050565b80515f9081906148289082614771565b9050806040516020016147ca906020808252601f908201527f545741505f4c4153545f50524943455f43554d4d554c41544956455f4b455900604082015260600190565b80515f90819061487c9082614771565b9050806040516020016147ca9060208082526024908201527f545741505f4c4153545f424c4f434b5f54494d455f5354414d505f42595f5045604082015263149253d160e21b606082015260800190565b80515f9081906148dd9082614771565b9050806040516020016147ca9060208082526029908201527f545741505f4c4153545f50524943455f43554d4d554c41544956455f4b45595f604082015268109657d411549253d160ba1b606082015260800190565b80515f9081906149439082614771565b9050806040516020016147ca90602080825260129082015271545741505f415645524147455f505249434560701b604082015260600190565b610a54838383604051602401614994939291906151bc565b60408051601f198184030181529190526020810180516001600160e01b0316635821efa160e01b179052614bf8565b5f8115806149e6575082826149d8818361508e565b92506149e490836150cb565b145b6129325760405162461bcd60e51b815260206004820152601460248201527364732d6d6174682d6d756c2d6f766572666c6f7760601b604482015260640161047a565b5f815f03614a3857505f919050565b5f6001614a4484614c01565b901c6001901b90506001818481614a5d57614a5d61504b565b048201901c90506001818481614a7557614a7561504b565b048201901c90506001818481614a8d57614a8d61504b565b048201901c90506001818481614aa557614aa561504b565b048201901c90506001818481614abd57614abd61504b565b048201901c90506001818481614ad557614ad561504b565b048201901c90506001818481614aed57614aed61504b565b048201901c9050613de481828581614b0757614b0761504b565b045b5f818310614b175781613de4565b5090919050565b5f816001600160a01b031663bd02d0f56040516020016140659060208082526019908201527f53484f52545f4c49515549444954595f5448524553484f4c4400000000000000604082015260600190565b5f5f5f614b9c855f01518560ff1660028110614b8d57614b8d6150de565b6020020151866080015161469b565b90505f614ba986866145ad565b96919550909350505050565b5f428203614bc857506020820151612932565b5f614bd7846040015184614538565b9050614bf084602001518261456c90919063ffffffff16565b915050612932565b61082a81614c94565b5f80608083901c15614c1557608092831c92015b604083901c15614c2757604092831c92015b602083901c15614c3957602092831c92015b601083901c15614c4b57601092831c92015b600883901c15614c5d57600892831c92015b600483901c15614c6f57600492831c92015b600283901c15614c8157600292831c92015b600183901c156129325760010192915050565b5f6a636f6e736f6c652e6c6f6790505f5f835160208501845afa505050565b604051806101400160405280614cc7614d0c565b81526020015f81526020015f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b6040518060a00160405280614d1f614d8f565b81525f60208201819052604082018190526060820181905260809091015290565b6040518061016001604052805f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b60405180604001604052806002905b614de06040518060e001604052805f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b815260200190600190039081614d9e5790505090565b6001600160a01b038116811461082a575f5ffd5b5f5f8284036080811215614e1c575f5ffd5b8335614e2781614df6565b92506060601f1982011215614e3a575f5ffd5b506020830190509250929050565b5f5f82840360a0811215614e5a575f5ffd5b8335614e6581614df6565b92506080601f1982011215614e3a575f5ffd5b5f60208284031215614e88575f5ffd5b8135613de481614df6565b60208082526017908201527f5245454e5452414e43595f47554152445f535441545553000000000000000000604082015260600190565b5f60208284031215614eda575f5ffd5b5051919050565b5f60208284031215614ef1575f5ffd5b81518015158114613de4575f5ffd5b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b6001600160a01b03831681526040602082018190525f9061424590830184614f00565b634e487b7160e01b5f52601160045260245ffd5b6001815b6001841115610a9457808504811115614f8457614f84614f51565b6001841615614f9257908102905b60019390931c928002614f69565b5f82614fae57506001612932565b81614fba57505f612932565b8160018114614fd05760028114614fda57614ff6565b6001915050612932565b60ff841115614feb57614feb614f51565b50506001821b612932565b5060208310610133831016604e8410600b8410161715615019575081810a612932565b6150255f198484614f65565b805f190482111561503857615038614f51565b029392505050565b5f613de48383614fa0565b634e487b7160e01b5f52601260045260245ffd5b5f8261506d5761506d61504b565b500690565b63ffffffff828116828216039081111561293257612932614f51565b808202811582820484141761293257612932614f51565b8082018082111561293257612932614f51565b8181038181111561293257612932614f51565b5f826150d9576150d961504b565b500490565b634e487b7160e01b5f52603260045260245ffd5b5f60208284031215615102575f5ffd5b8151613de481614df6565b60208082526021908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f574954485f444542545f6040820152600360fc1b606082015260800190565b60208082526021908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f574954485f444542545f6040820152603160f81b606082015260800190565b604081525f6151a16040830185614f00565b82810360208401526151b38185614f00565b95945050505050565b606081525f6151ce6060830186614f00565b82810360208401526151e08186614f00565b91505082604083015294935050505056fe2d2d2d2d2d2d2d2d2d2d2d7570646174655477617050726963652d2d2d2d2d2d2d2d2d2d2da2646970667358221220a8eadec501cf81c9ee7a9901f62b324b60c02b67738b872532655dea61ad05ea64736f6c634300081c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xE0`@R4\x80\x15a\0\x0FW__\xFD[P`@QaSW8\x03\x80aSW\x839\x81\x01`@\x81\x90Ra\0.\x91a\0bV[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x80R\x91\x81\x16`\xA0R\x16`\xC0Ra\0\xACV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0_W__\xFD[PV[___``\x84\x86\x03\x12\x15a\0tW__\xFD[\x83Qa\0\x7F\x81a\0KV[` \x85\x01Q\x90\x93Pa\0\x90\x81a\0KV[`@\x85\x01Q\x90\x92Pa\0\xA1\x81a\0KV[\x80\x91PP\x92P\x92P\x92V[`\x80Q`\xA0Q`\xC0QaRLa\x01\x0B_9_\x81\x81`\xF0\x01R\x81\x81a\x01\xC6\x01Ra\x02\xDE\x01R_\x81\x81`^\x01Ra\x05V\x01R_\x81\x81`\xB6\x01R\x81\x81a\x01\x97\x01R\x81\x81a\x02\xAF\x01R\x81\x81a\x03\x7F\x01R\x81\x81a\x04\x85\x01Ra\x07v\x01RaRL_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0UW_5`\xE0\x1C\x80cJJ{\x04\x14a\0YW\x80c^\xCDD\xE8\x14a\0\x9CW\x80cf\r\rg\x14a\0\xB1W\x80c}#|\x99\x14a\0\xD8W\x80c\x9F\xF7\x8C0\x14a\0\xEBW[__\xFD[a\0\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0\xAFa\0\xAA6`\x04aN\nV[a\x01\x12V[\0[a\0\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\0\xAFa\0\xE66`\x04aNHV[a\x02hV[a\0\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01\x1Aa\x03|V[a\x01\x89`@Q` \x01a\x01K\x90` \x80\x82R`\n\x90\x82\x01Ri!\xA7\xA7*)'\xA6&\"\xA9`\xB1\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x80`@\x01`@R\x80`\n\x81R` \x01i!\xA7\xA7*)'\xA6&\"\xA9`\xB1\x1B\x81RPa\x05:V[_`@Q\x80`\xA0\x01`@R\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83_\x01` \x81\x01\x90a\x02\x05\x91\x90aNxV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83` \x01` \x81\x01\x90a\x02&\x91\x90aNxV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01a\x02D``\x85\x01`@\x86\x01aNxV[`\x01`\x01`\xA0\x1B\x03\x16\x90R\x90Pa\x02[\x83\x82a\x05\xE8V[Pa\x02da\x07tV[PPV[a\x02pa\x03|V[a\x02\xA1`@Q` \x01a\x01K\x90` \x80\x82R`\n\x90\x82\x01Ri!\xA7\xA7*)'\xA6&\"\xA9`\xB1\x1B`@\x82\x01R``\x01\x90V[_`@Q\x80`\xC0\x01`@R\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83_\x01` \x81\x01\x90a\x03\x1D\x91\x90aNxV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83` \x01` \x81\x01\x90a\x03>\x91\x90aNxV[`\x01`\x01`\xA0\x1B\x03\x16\x81R`@\x80\x85\x015` \x83\x01R\x01a\x03e`\x80\x85\x01``\x86\x01aNxV[`\x01`\x01`\xA0\x1B\x03\x16\x90R\x90Pa\x02[\x83\x82a\x08-V[_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x03\xBB\x90aN\x93V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x03\xEF\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\nW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04.\x91\x90aN\xCAV[\x90P\x80\x15a\x04\x83W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:`@Q` \x01a\x04\xC1\x90aN\x93V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`\x01`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x05\x16W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02d\x91\x90aN\xCAV[`@Qc\xACJ\xB3\xFB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x83\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xACJ\xB3\xFB\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xA3W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xC7\x91\x90aN\xE1V[a\x02dW3\x81`@Qc\xA3[\x15\x0B`\xE0\x1B\x81R`\x04\x01a\x04z\x92\x91\x90aO.V[__a\x06\0\x83_\x01Q\x84`@\x01Q\x85``\x01Qa\nYV[\x91P\x91P_a\x06\x0E\x83a\n\x9CV[\x90Pa\x06\x1E\x84_\x01Q\x84\x83a\x0BVV[` \x83\x01Q`@\x85\x81\x01Q\x90Qc5/\x9A\xED`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R_\x91\x83\x16\x90c5/\x9A\xED\x90`$\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x06oW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x93\x91\x90aN\xCAV[``\x87\x01Q`@Qc5/\x9A\xED`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x91\x92P_\x91\x90\x84\x16\x90c5/\x9A\xED\x90`$\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x06\xE3W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x07\x91\x90aN\xCAV[\x90Pa\x07\x13\x82\x82a\x10\\V[a\x07$\x86\x84\x84\x84\x8B`\x80\x01Qa\x10\x85V[\x86Qa\x070\x90\x87a\x11\xC3V[a\x07>\x87` \x01Q\x87a\x12\x07V[\x86Qa\x07K\x90\x86\x88a\x12+V[a\x07j\x87` \x01Q\x89\x89`@\x01Q\x8A``\x01Q\x8B`\x80\x01Q\x87\x87a$GV[PPPPPPPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:`@Q` \x01a\x07\xB2\x90aN\x93V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R_`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x08\x06W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08*\x91\x90aN\xCAV[PV[a\x085aL\xB3V[a\x08K\x82_\x01Q\x83`@\x01Q\x84``\x01Qa\nYV[` \x83\x01R\x80\x82R`\x80\x83\x01Qa\x08c\x91\x85\x91a$\x9BV[\x81Q\x81Q`\x80\x84\x01Qa\x08w\x92\x91\x90a%XV[`\xC0\x85\x01\x81\x90R`\xA0\x85\x01\x82\x90Ra\x01 \x85\x01\x83\x90Ra\x01\0\x85\x01\x84\x90Ra\x08\xA1\x93\x92\x91\x90a'\xADV[\x80Q` \x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`@\x80\x84\x01\x82\x90R`\x80\x85\x01Q\x90Qc'p\xA7\xEB`\xE2\x1B\x81R\x92\x86\x16`\x04\x84\x01R`$\x83\x01R\x90c\x9D\xC2\x9F\xAC\x90`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x08\xF9W__\xFD[PZ\xF1\x15\x80\x15a\t\x0BW=__>=_\xFD[PPPP`@\x81\x81\x01Q\x83\x82\x01Q`\xA0\x85\x01Qa\x01\0\x85\x01Q\x93Qc\x07\x8D;y`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x04\x82\x01R\x90\x82\x16`$\x82\x01R`D\x81\x01\x93\x90\x93R\x16\x90c\x07\x8D;y\x90`d\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\trW__\xFD[PZ\xF1\x15\x80\x15a\t\x84W=__>=_\xFD[PPPP`@\x81\x81\x01Q``\x84\x01Q`\xA0\x85\x01Qa\x01 \x85\x01Q\x93Qc\x07\x8D;y`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x04\x82\x01R\x90\x82\x16`$\x82\x01R`D\x81\x01\x93\x90\x93R\x16\x90c\x07\x8D;y\x90`d\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\t\xECW__\xFD[PZ\xF1\x15\x80\x15a\t\xFEW=__>=_\xFD[PPP` \x83\x01Q\x82Qa\n\x12\x92Pa\x12\x07V[\x81Q` \x82\x01Q\x82Qa\n&\x92\x91\x90a\x12+V[a\nT\x82` \x01Q\x84\x84`@\x01Q\x85``\x01Q\x86`\x80\x01Q\x87`\xA0\x01Q\x87a\x01\0\x01Q\x88a\x01 \x01Qa(\x03V[PPPV[a\naaM\x0CV[__a\nm\x85\x85a(\x90V[\x90P_a\nz\x87\x83a)8V[\x90Pa\n\x86\x81\x83a;:V[a\n\x8F\x81a;hV[\x92P\x90P[\x93P\x93\x91PPV[__a\n\xAA\x83___a<\x14V[PPP\x90P_a\n\xBD\x84`\x01__a<\x14V[PPP\x90P\x81_\x14\x80a\n\xCEWP\x80\x15[\x15a\n\xDCWP_\x93\x92PPPV[_a\n\xEB\x85``\x01Q_a<\xFDV[\x90P_a\n\xFD\x86``\x01Q`\x01a<\xFDV[\x90P_a\x0B \x85gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba\x0B\x1B\x86`\naP@V[a=+V[\x90P_a\x0B>\x85gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba\x0B\x1B\x86`\naP@V[\x90Pa\x0BJ\x82\x82a=\xEBV[\x98\x97PPPPPPPPV[\x80_\x03a\x0BbWPPPV[a\x0B\x83`@Q\x80``\x01`@R\x80`%\x81R` \x01aQ\xF2`%\x919a>&V[`@\x80Qa\x01@\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x82\x90Ra\x01\0\x81\x01\x82\x90Ra\x01 \x81\x01\x91\x90\x91Ra\x0B\xDD\x84\x84a>JV[c\xFF\xFF\xFF\xFF\x16\x81Ra\x0B\xF4d\x01\0\0\0\0BaP_V[c\xFF\xFF\xFF\xFF\x90\x81\x16` \x83\x01R\x81Q\x16_\x03a\x0C\xB0Wa\x0C\x19\x84\x84\x83` \x01Qa>\xC0V[a\x0C$\x84\x84_a?DV[a\x0C3\x84\x84\x83` \x01Qa?\x83V[a\x0C>\x84\x84_a?\x9BV[a\x0CI\x84\x84\x84a?\xB3V[a\x0C\x83`@Q\x80`@\x01`@R\x80`\x0E\x81R` \x01m\x06&\xC6\xF66\xB5F\x96\xD6U7F\x16\xD7`\x94\x1B\x81RP\x82` \x01Qc\xFF\xFF\xFF\xFF\x16a?\xCBV[a\x0C\xAA`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01dprice`\xD8\x1B\x81RP\x83a?\xCBV[PPPPV[\x80Q` \x82\x01Qa\x0C\xC1\x91\x90aPrV[c\xFF\xFF\xFF\xFF\x90\x81\x16`@\x80\x84\x01\x91\x82R\x80Q\x80\x82\x01\x90\x91R`\x0B\x81Rj\x1D\x1A[YQ[\x18\\\x1C\xD9Y`\xAA\x1B` \x82\x01R\x90Qa\x0C\xFD\x92\x16a?\xCBV[`@\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x15a\x0C\xAAWa\r\x17\x84\x84a?\xF4V[``\x82\x01R`@\x81\x01Qa\r1\x90c\xFF\xFF\xFF\xFF\x16\x83aP\x8EV[\x81``\x01Qa\r@\x91\x90aP\xA5V[`\x80\x82\x01R`@\x80Q\x80\x82\x01\x90\x91R`\x05\x81Rdprice`\xD8\x1B` \x82\x01Ra\rk\x90\x83a?\xCBV[a\r\xA2`@Q\x80`@\x01`@R\x80`\x0B\x81R` \x01j\x1D\x1A[YQ[\x18\\\x1C\xD9Y`\xAA\x1B\x81RP\x82`@\x01Qc\xFF\xFF\xFF\xFF\x16a?\xCBV[a\r\xDB`@Q\x80`@\x01`@R\x80`\x13\x81R` \x01r\x1C\x1C\x9AX\xD9P\xDD[][\x18]\x1A]\x99S\x18\\\xDD`j\x1B\x81RP\x82``\x01Qa?\xCBV[a\x0E\x10`@Q\x80`@\x01`@R\x80`\x0F\x81R` \x01npriceCumulative`\x88\x1B\x81RP\x82`\x80\x01Qa?\xCBV[a\x0E\x1F\x84\x84\x83`\x80\x01Qa?DV[a\x0E.\x84\x84\x83` \x01Qa>\xC0V[a\x0E8\x84\x84a@\rV[c\xFF\xFF\xFF\xFF\x16`\xA0\x82\x01\x81\x90R` \x82\x01Qa\x0ET\x91\x90aPrV[c\xFF\xFF\xFF\xFF\x16`\xC0\x82\x01Ra\x0Eh\x84a@&V[c\xFF\xFF\xFF\xFF\x90\x81\x16`\xE0\x83\x01\x90\x81R`@\x80Q\x80\x82\x01\x90\x91R`\x06\x81Re\x14\x11T\x92S\xD1`\xD2\x1B` \x82\x01R\x90Qa\x0E\xA0\x92\x16a?\xCBV[a\x0E\xE9`@Q\x80`@\x01`@R\x80`\x1B\x81R` \x01\x7FlastUpdateTimestampByPeriod\0\0\0\0\0\x81RP\x82`\xA0\x01Qc\xFF\xFF\xFF\xFF\x16a?\xCBV[a\x0F(`@Q\x80`@\x01`@R\x80`\x13\x81R` \x01r\x1D\x1A[YQ[\x18\\\x1C\xD9Y\x10\x9ET\x19\\\x9A[\xD9`j\x1B\x81RP\x82`\xC0\x01Qc\xFF\xFF\xFF\xFF\x16a?\xCBV[\x80`\xE0\x01Qc\xFF\xFF\xFF\xFF\x16\x81`\xC0\x01Qc\xFF\xFF\xFF\xFF\x16\x11\x15a\x0C\xAAWa\x0FN\x84\x84a@\xD8V[a\x01\0\x82\x01\x81\x90R`\xC0\x82\x01Q`\x80\x83\x01Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x91a\x0Fr\x91aP\xB8V[a\x0F|\x91\x90aP\xCBV[a\x01 \x82\x01R`@\x80Q\x80\x82\x01\x90\x91R`\x1B\x81R\x7FpriceCumulativeLastByPeriod\0\0\0\0\0` \x82\x01Ra\x01\0\x82\x01Qa\x0F\xC6\x91\x90a?\xCBV[a\x0F\xFB`@Q\x80`@\x01`@R\x80`\x0F\x81R` \x01npriceCumulative`\x88\x1B\x81RP\x82`\x80\x01Qa?\xCBV[a\x10.`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01kpriceAverage`\xA0\x1B\x81RP\x82a\x01 \x01Qa?\xCBV[a\x10=\x84\x84\x83` \x01Qa?\x83V[a\x10L\x84\x84\x83`\x80\x01Qa?\x9BV[a\x0C\xAA\x84\x84\x83a\x01 \x01Qa?\xB3V[\x81\x15\x80a\x10gWP\x80\x15[\x15a\x02dW`@Qc\x1A]\xF2\x83`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x10\x93\x86\x85\x85`\x01a@\xF1V[\x90P_\x85`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10\xD2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\xF6\x91\x90aN\xCAV[\x90P\x80_\x03a\x11\\W`@Qc@\xC1\x0F\x19`\xE0\x1B\x81R_`\x04\x82\x01Ra\x03\xE8`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x87\x16\x90c@\xC1\x0F\x19\x90`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x11EW__\xFD[PZ\xF1\x15\x80\x15a\x11WW=__>=_\xFD[PPPP[`@Qc@\xC1\x0F\x19`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R`$\x82\x01\x84\x90R\x87\x16\x90c@\xC1\x0F\x19\x90`D\x01[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x11\xA4W__\xFD[PZ\xF1\x15\x80\x15a\x11\xB6W=__>=_\xFD[PPPPPPPPPPPV[``\x81\x01Qf\x08\0\0\0\0\0\0\x16a\x02dW__a\x11\xE1\x84\x84aBMV[\x91P\x91P\x81\x81\x10a\x0C\xAAW``\x83\x01Qa\x11\xFC\x90`\x01aB\xAEV[``\x84\x01RPPPPV[a\x12\x10\x81aB\xD8V[\x91Q\x80Q` \x90\x91\x01Q`@\x90\x81\x01\x93\x90\x93R\x91\x90\x91\x01RPV[_\x83\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xC8\x0FLb`@Q` \x01a\x12k\x90` \x80\x82R`\t\x90\x82\x01Rh\x14\x13\xD3\xD3\x17\xD3\x12T\xD5`\xBA\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x86\x90R`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x12\xBBW__\xFD[PZ\xF1\x15\x80\x15a\x12\xCDW=__>=_\xFD[PPPP\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a\x13\x11\x90` \x80\x82R`\x0C\x90\x82\x01Rk\x05\x04\xF4\xF4\xC5\xF5D\xF4\xB4T\xE5\xF3`\xA4\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x13A\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQQ`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x13\xA4W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\xC8\x91\x90aP\xF2V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x14\x10\x90` \x80\x82R`\x13\x90\x82\x01Rr\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF4\x94\xE4DU\x85\xF3`l\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x14@\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86QQ\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x14\x9DW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\xC1\x91\x90aN\xCAV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x15\x08\x90` \x80\x82R`\x12\x90\x82\x01Rq\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF5$\x15DU\xF3`t\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x158\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82R\x80Q` \x90\x91\x01 \x86QQ\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x15\x94W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\xB8\x91\x90aN\xCAV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x16\x04\x90` \x80\x82R`\x17\x90\x82\x01Rv\x05\x04\xF4\xF4\xC5\xF5D\xF5D\x14\xC5\xF44\xF4\xC4\xC4\x15DU$\x14\xC5\xF3`L\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x164\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ``\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x16\x91W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\xB5\x91\x90aN\xCAV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x16\xD5\x90aQ\rV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x17\x05\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ`\x80\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x17bW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\x86\x91\x90aN\xCAV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x17\xD3\x90` \x80\x82R`\x18\x90\x82\x01Rw\x05\x04\xF4\xF4\xC5\xF5D\xF5D\x14\xC5\xF544\x14\xC4TE\xF4DT%E\xF3`D\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x18\x03\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ`\xA0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x18`W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18\x84\x91\x90aN\xCAV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x18\xCD\x90` \x80\x82R`\x14\x90\x82\x01Rs\x05\x04\xF4\xF4\xC5\xF5T\xE44\xC4\x14\x94\xD4TE\xF4dTU\xF3`d\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x18\xFD\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ`\xC0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x19ZW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19~\x91\x90aN\xCAV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a\x19\xBF\x90` \x80\x82R`\x0C\x90\x82\x01RkPOOL_TOKEN_1`\xA0\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x19\xEF\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01QQ`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1AUW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Ay\x91\x90aP\xF2V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x1A\xC1\x90` \x80\x82R`\x13\x90\x82\x01RrPOOL_BORROW_INDEX_1`h\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1A\xF1\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x82\x01Q\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1BPW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Bt\x91\x90aN\xCAV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x1B\xBB\x90` \x80\x82R`\x12\x90\x82\x01RqPOOL_BORROW_RATE_1`p\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1B\xEB\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x87Q\x90\x91\x01Q\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1CJW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Cn\x91\x90aN\xCAV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x1C\xBA\x90` \x80\x82R`\x17\x90\x82\x01RvPOOL_TOTAL_COLLATERAL_1`H\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1C\xEA\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q``\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1DJW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Dn\x91\x90aN\xCAV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x1D\x8E\x90aQNV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1D\xBE\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q`\x80\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1E\x1EW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1EB\x91\x90aN\xCAV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x1E\x8F\x90` \x80\x82R`\x18\x90\x82\x01RwPOOL_TOTAL_SCALED_DEBT_1`@\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1E\xBF\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q`\xA0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1F\x1FW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1FC\x91\x90aN\xCAV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x1F\x8C\x90` \x80\x82R`\x14\x90\x82\x01RsPOOL_UNCLAIMED_FEE_1``\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1F\xBC\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q`\xC0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a \x1CW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a @\x91\x90aN\xCAV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a ~\x90` \x80\x82R`\t\x90\x82\x01RhPOOL_BANK`\xB8\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a \xAE\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84` \x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a \xF8\x92\x91\x90\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a!\x14W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!8\x91\x90aP\xF2V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a!\x8A\x90` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_INTEREST_RATE_STRATEGY\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a!\xBA\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x90\x91\x01 \x90\x86\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83Ra\"\x05\x92`\x04\x01\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\"!W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\"E\x91\x90aP\xF2V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\"\x8C\x90` \x80\x82R`\x12\x90\x82\x01Rq('\xA7\xA6/\xA1\xA7\xA7#$\xA3\xAA\xA9 \xAA$\xA7\xA7`q\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\"\xBC\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84``\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\"\xFD\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a#\x19W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a#=\x91\x90aN\xCAV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a#\x8F\x90` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_LAST_UPDATE_TIME_STAMP\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a#\xBF\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84`\x80\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a$\0\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a$\x1CW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a$@\x91\x90aN\xCAV[PPPPPV[`@Qc\x9E\xD4\x86\xEB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x81\x16`\x04\x83\x01R\x86\x81\x16`$\x83\x01R\x85\x81\x16`D\x83\x01R\x84\x81\x16`d\x83\x01R`\x84\x82\x01\x84\x90R`\xA4\x82\x01\x83\x90R\x88\x16\x90c\x9E\xD4\x86\xEB\x90`\xC4\x01a\x11\x8DV[\x80_\x03a$\xBBW`@QcQ\x86Y\x11`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x82\x01Q`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R_\x91\x90\x83\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a%\x07W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a%+\x91\x90aN\xCAV[\x90P\x80\x83\x11\x15a$@W`@Qc\x02\xFA\x82i`\xE5\x1B\x81R`\x04\x81\x01\x82\x90R`$\x81\x01\x84\x90R`D\x01a\x04zV[____a%daM@V[a%m\x88aD\xE7V[\x81a\x01@\x01\x81\x81RPP\x86` \x01Q`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a%\xB7W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a%\xDB\x91\x90aN\xCAV[\x81` \x01\x81\x81RPPa%\xF5\x87__\x84a\x01@\x01Qa<\x14V[P`\x80\x84\x01RP`@\x82\x01Ra\x01@\x81\x01Qa&\x17\x90\x88\x90`\x01\x90_\x90a<\x14V[P`\xA0\x84\x01RP``\x82\x01R` \x81\x01Q_\x03a&@W____\x94P\x94P\x94P\x94PPa'\xA4V[a&S\x86\x82`@\x01Q\x83` \x01Qa=+V[a\x01\0\x82\x01R``\x81\x01Q` \x82\x01Qa&n\x91\x88\x91a=+V[\x81a\x01 \x01\x81\x81RPPa&\xAE`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01ovars.totalSupply`\x80\x1B\x81RP\x82` \x01Qa?\xCBV[a&\xE6`@Q\x80`@\x01`@R\x80`\x12\x81R` \x01q\x07f\x17'2\xE7\x07&\x966U&W6W'fS`t\x1B\x81RP\x82`@\x01Qa?\xCBV[a'\x1E`@Q\x80`@\x01`@R\x80`\x12\x81R` \x01qvars.priceReserve1`p\x1B\x81RP\x82``\x01Qa?\xCBV[a'Q`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01k\x07f\x17'2\xE6\x16\xD6\xF7V\xE7C`\xA4\x1B\x81RP\x82a\x01\0\x01Qa?\xCBV[a'\x84`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01kvars.amount1`\xA0\x1B\x81RP\x82a\x01 \x01Qa?\xCBV[\x80a\x01\0\x01Q\x81a\x01 \x01Q\x82`\x80\x01Q\x83`\xA0\x01Q\x94P\x94P\x94P\x94PP[\x93P\x93P\x93P\x93V[\x81\x84\x11\x15a'\xD8W`@Qc\x1F\xC1\x07\xC1`\xE0\x1B\x81R`\x04\x81\x01\x85\x90R`$\x81\x01\x83\x90R`D\x01a\x04zV[\x80\x83\x11\x15a\x0C\xAAW`@Qc\x0Ey;\xAF`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x81\x01\x82\x90R`D\x01a\x04zV[`@Qc\x14\x95s\x91`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x88\x81\x16`\x04\x83\x01R\x87\x81\x16`$\x83\x01R\x86\x81\x16`D\x83\x01R`d\x82\x01\x86\x90R\x84\x81\x16`\x84\x83\x01R`\xA4\x82\x01\x84\x90R`\xC4\x82\x01\x83\x90R\x89\x16\x90c)*\xE7\"\x90`\xE4\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a(pW__\xFD[PZ\xF1\x15\x80\x15a(\x82W=__>=_\xFD[PPPPPPPPPPPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x10a(\xB1W\x81\x83a(\xB4V[\x82\x82[`@Q\x91\x94P\x92Pa(\xE1\x90` \x01` \x80\x82R`\x04\x90\x82\x01Rc\x14\x13\xD3\xD3`\xE2\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01R`\x01`\x01`\xA0\x1B\x03\x80\x86\x16\x91\x83\x01\x91\x90\x91R\x83\x16``\x82\x01R`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P[\x92\x91PPV[a)@aM\x0CV[\x82a)IaM\x0CV[\x81`\x01`\x01`\xA0\x1B\x03\x16c\x91\xD4@<`@Q` \x01a)\x85\x90` \x80\x82R`\t\x90\x82\x01Rh\x14\x13\xD3\xD3\x17\xD3\x12T\xD5`\xBA\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x87\x90R`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a)\xD9W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a)\xFD\x91\x90aN\xE1V[a*\nW\x91Pa)2\x90PV[\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a*J\x90` \x80\x82R`\x0C\x90\x82\x01Rk\x05\x04\xF4\xF4\xC5\xF5D\xF4\xB4T\xE5\xF3`\xA4\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a*z\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a*\xAE\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a*\xC9W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a*\xED\x91\x90aP\xF2V[\x81QQ`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x90R`@\x80Q` \x81\x81\x01R`\x13\x91\x81\x01\x91\x90\x91Rr\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF4\x94\xE4DU\x85\xF3`l\x1B``\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a+k\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a+\x9F\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a+\xBAW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a+\xDE\x91\x90aN\xCAV[\x81Q_` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a,4\x90` \x80\x82R`\x12\x90\x82\x01Rq\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF5$\x15DU\xF3`t\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a,d\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a,\x98\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a,\xB3W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a,\xD7\x91\x90aN\xCAV[\x81Q_` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a-2\x90` \x80\x82R`\x17\x90\x82\x01Rv\x05\x04\xF4\xF4\xC5\xF5D\xF5D\x14\xC5\xF44\xF4\xC4\xC4\x15DU$\x14\xC5\xF3`L\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a-b\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a-\x96\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a-\xB1W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a-\xD5\x91\x90aN\xCAV[\x81QQ``\x01R`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90a-\xFE\x90` \x01aQ\rV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a..\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a.b\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a.}W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a.\xA1\x91\x90aN\xCAV[\x81Q_` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a.\xFD\x90` \x80\x82R`\x18\x90\x82\x01Rw\x05\x04\xF4\xF4\xC5\xF5D\xF5D\x14\xC5\xF544\x14\xC4TE\xF4DT%E\xF3`D\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a/-\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a/a\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a/|W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a/\xA0\x91\x90aN\xCAV[\x81QQ`\xA0\x01R`@\x80Q` \x81\x81\x01R`\x14\x91\x81\x01\x91\x90\x91Rs\x05\x04\xF4\xF4\xC5\xF5T\xE44\xC4\x14\x94\xD4TE\xF4dTU\xF3`d\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a0\x1D\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a0Q\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a0lW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a0\x90\x91\x90aN\xCAV[\x81QQ`\xC0\x01R`@\x80Q` \x81\x81\x01R`\x0C\x91\x81\x01\x91\x90\x91RkPOOL_TOKEN_1`\xA0\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c!\xF8\xA7!\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a1\x05\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a19\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a1TW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a1x\x91\x90aP\xF2V[\x81Q` \x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x90R`@\x80Q\x80\x83\x01\x83\x90R`\x13\x81\x83\x01RrPOOL_BORROW_INDEX_1`h\x1B``\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x90\x92R\x81Q\x91\x90\x92\x01 `\xA0\x82\x01\x87\x90R`\xC0\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a2\"\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a2=W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a2a\x91\x90aN\xCAV[\x81Q`\x01` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a2\xB8\x90` \x80\x82R`\x12\x90\x82\x01RqPOOL_BORROW_RATE_1`p\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a2\xE8\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a3\x1C\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a37W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a3[\x91\x90aN\xCAV[\x81Q`\x01` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a3\xB7\x90` \x80\x82R`\x17\x90\x82\x01RvPOOL_TOTAL_COLLATERAL_1`H\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a3\xE7\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a4\x1B\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a46W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a4Z\x91\x90aN\xCAV[\x81Q`\x01` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a4\x8A\x90aQNV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a4\xBA\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a4\xEE\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a5\tW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a5-\x91\x90aN\xCAV[\x81Q`\x01` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a5\x8A\x90` \x80\x82R`\x18\x90\x82\x01RwPOOL_TOTAL_SCALED_DEBT_1`@\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a5\xBA\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a5\xEE\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a6\tW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a6-\x91\x90aN\xCAV[\x81Q`\x01` \x02\x01Q`\xA0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a6\x86\x90` \x80\x82R`\x14\x90\x82\x01RsPOOL_UNCLAIMED_FEE_1``\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a6\xB6\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a6\xEA\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a7\x05W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a7)\x91\x90aN\xCAV[\x81Q`\x01` \x02\x01Q`\xC0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a7w\x90` \x80\x82R`\t\x90\x82\x01RhPOOL_BANK`\xB8\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a7\xA7\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a7\xDB\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a7\xF6W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a8\x1A\x91\x90aP\xF2V[\x81` \x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a8\x88\x90` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_INTEREST_RATE_STRATEGY\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a8\xB8\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a8\xEC\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a9\x07W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a9+\x91\x90aP\xF2V[\x81`@\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a9\x8E\x90` \x80\x82R`\x12\x90\x82\x01Rq('\xA7\xA6/\xA1\xA7\xA7#$\xA3\xAA\xA9 \xAA$\xA7\xA7`q\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a9\xBE\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a9\xF2\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a:\rW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a:1\x91\x90aN\xCAV[``\x82\x01R`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90a:\x8A\x90` \x01` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_LAST_UPDATE_TIME_STAMP\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a:\xBA\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a:\xEE\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a;\tW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a;-\x91\x90aN\xCAV[`\x80\x82\x01R\x94\x93PPPPV[` \x82\x01Q`\x01`\x01`\xA0\x1B\x03\x16a\x02dW`@QcsW\xD9\x1F`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x04zV[`\x80\x81\x01QB\x90\x81\x90\x03a;zWPPV[\x81QQ`\xA0\x01Q\x15a;\xD0W\x81Q_\x90a;\xA3\x90\x82[` \x02\x01Q`@\x01Q\x84`\x80\x01QaE8V[\x83Q\x90\x91Pa;\xC7\x90\x82\x90_[` \x02\x01Q` \x01QaEl\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x83QQ` \x01RP[\x81Q` \x01Q`\xA0\x01Q\x15a<\x0CW\x81Q_\x90a;\xEE\x90`\x01a;\x90V[\x83Q\x90\x91Pa<\0\x90\x82\x90`\x01a;\xB0V[\x83Q` \x90\x81\x01Q\x01RP[`\x80\x90\x91\x01RV[_____\x88_\x01Q\x88`\xFF\x16`\x02\x81\x10a<1Wa<1aP\xDEV[` \x02\x01Q\x90P_a<C\x8A\x8AaE\xADV[\x90P\x80_\x03a<_W____\x95P\x95P\x95P\x95PPPa<\xF2V[_a<n\x83\x8C`\x80\x01QaF\x9BV[\x90P_a<{\x82\x8AaElV[\x90P_\x89\x15a<\xA0W\x81\x84\x10a<\x9AWa<\x95\x84\x83aF\xCBV[a<\xA2V[_a<\xA2V[_[\x90P_a<\xAF\x85\x8DaElV[\x90P_\x8C\x15a<\xD4W\x84\x82\x10a<\xCEWa<\xC9\x82\x86aF\xCBV[a<\xD6V[_a<\xD6V[_[\x90Pa<\xE2\x85\x87aP\xA5V[\x9AP\x94\x98P\x90\x96P\x92\x94PPPPP[\x94P\x94P\x94P\x94\x90PV[_`\xFF`X\x1B\x19`X_\x19`\xFF\x85\x16\x01a=\x1DWP`\xFF``\x1B\x19\x90P``[\x90\x19\x84\x16\x90\x1C\x90P\x92\x91PPV[_\x83\x83\x02\x81_\x19\x85\x87\t\x82\x81\x10\x83\x82\x03\x03\x91PP\x80_\x03a=_W\x83\x82\x81a=UWa=UaPKV[\x04\x92PPPa=\xE4V[\x80\x84\x11a=\x7FW`@Qc\"{\xC1S`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x84\x86\x88\t_\x86\x81\x03\x87\x16\x96\x87\x90\x04\x96`\x02`\x03\x89\x02\x81\x18\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x90\x91\x03\x02\x91\x81\x90\x03\x81\x90\x04`\x01\x01\x86\x84\x11\x90\x95\x03\x94\x90\x94\x02\x91\x90\x94\x03\x92\x90\x92\x04\x91\x90\x91\x17\x91\x90\x91\x02\x91PP[\x93\x92PPPV[_\x81\x15gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B`\x02\x84\x04\x19\x04\x84\x11\x17\x15a>\x0CW__\xFD[Pgge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x91\x90\x91\x02`\x02\x82\x04\x01\x04\x90V[a\x08*`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a%s`\xF0\x1B\x81RP\x82aG V[_\x82`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5a>c\x84aGeV[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a>\x81\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a>\x9CW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a=\xE4\x91\x90aN\xCAV[\x82`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:a>\xD8\x84aGeV[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81R`\x04\x81\x01\x91\x90\x91Rc\xFF\xFF\xFF\xFF\x84\x16`$\x82\x01R`D\x01[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a? W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xAA\x91\x90aN\xCAV[\x82`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:a?\\\x84aH\x18V[\x83`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a?\x04\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[\x82`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:a>\xD8\x84aHlV[\x82`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:a?\\\x84aH\xCDV[\x82`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:a?\\\x84aI3V[a\x02d`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01e%s -%s`\xD0\x1B\x81RP\x83\x83aI|V[_\x82`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5a>c\x84aH\x18V[_\x82`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5a>c\x84aHlV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a@e\x90` \x80\x82R`\x0B\x90\x82\x01Rj\x15\x15\xD0T\x17\xD4\x11T\x92S\xD1`\xAA\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a@\x99\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a@\xB4W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a)2\x91\x90aN\xCAV[_\x82`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5a>c\x84aH\xCDV[_a@\xFAaM@V[\x85` \x01Q`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aA:W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aA^\x91\x90aN\xCAV[` \x82\x01RaAo\x86_\x80\x80a<\x14V[PPP`\xC0\x82\x01RaA\x84\x86`\x01_\x80a<\x14V[PPP`\xE0\x82\x01R\x82\x15aA\xBFW\x84\x81`\xC0\x01\x81\x81QaA\xA4\x91\x90aP\xB8V[\x90RP`\xE0\x81\x01\x80Q\x85\x91\x90aA\xBB\x90\x83\x90aP\xB8V[\x90RP[\x80` \x01Q_\x03aA\xEFWaA\xE8a\x03\xE8aA\xE2aA\xDD\x88\x88aI\xC3V[aJ)V[\x90aF\xCBV[\x81RaBAV[`\xC0\x81\x01Q\x15\x80aB\x02WP`\xE0\x81\x01Q\x15[\x15aB\x10W_\x91PPaBEV[aB>aB&\x86\x83` \x01Q\x84`\xC0\x01Qa=+V[aB9\x86\x84` \x01Q\x85`\xE0\x01Qa=+V[aK\tV[\x81R[Q\x90P[\x94\x93PPPPV[___aB^\x84``\x01Q_a<\xFDV[\x90P_aBj\x86aK\x1EV[\x90P_aB\x8D\x82aB|\x85`\naP@V[gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba=+V[\x90P_aB\x9C\x87___a<\x14V[P\x93\x9A\x91\x99P\x90\x97PPPPPPPPV[_`3\x82aB\xBCW_aB\xBFV[`\x01[`\xFF\x16\x90\x1Bf\x08\0\0\0\0\0\0\x19\x84\x16\x17\x90P\x92\x91PPV[__aC\r`@Q\x80`\xC0\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[aC\x17\x84_aKoV[` \x83\x01R\x81R``\x84\x01QaC-\x90_a<\xFDV[``\x82\x01\x81\x90R\x81QaCR\x91gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x90a\x0B\x1B\x90`\naP@V[`@\x82\x01R` \x81\x01Q_\x03aCmW_`\x80\x82\x01RaD\rV[`@\x84\x81\x01Q\x81Q``\x81\x01\x83R\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01\x90\x81R\x84\x84\x01Q\x82\x85\x01\x90\x81R\x93Qc\xFD\xD6>\xCF`\xE0\x1B\x81R\x91Q`\x04\x83\x01RQ`$\x82\x01R\x91Q`D\x83\x01R`\x01`\x01`\xA0\x1B\x03\x16\x90c\xFD\xD6>\xCF\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aC\xE3W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aD\x07\x91\x90aN\xCAV[`\x80\x82\x01R[aD\x18\x84`\x01aKoV[` \x83\x01\x81\x90R\x90\x82R_\x03aD3W_`\xA0\x82\x01RaD\xD3V[`@\x84\x81\x01Q\x81Q``\x81\x01\x83R\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01\x90\x81R\x84\x84\x01Q\x82\x85\x01\x90\x81R\x93Qc\xFD\xD6>\xCF`\xE0\x1B\x81R\x91Q`\x04\x83\x01RQ`$\x82\x01R\x91Q`D\x83\x01R`\x01`\x01`\xA0\x1B\x03\x16\x90c\xFD\xD6>\xCF\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aD\xA9W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aD\xCD\x91\x90aN\xCAV[`\xA0\x82\x01R[\x80`\x80\x01Q\x81`\xA0\x01Q\x92P\x92PP\x91P\x91V[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a@e\x90` \x80\x82R`\x1E\x90\x82\x01R\x7FTRADABLE_DEBT_MULTIPIER_FACTOR\0\0`@\x82\x01R``\x01\x90V[_\x80aED\x83BaP\xB8V[aEN\x90\x85aP\x8EV[c\x01\xE13\x80\x90\x04\x90PaBE\x81gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1BaP\xA5V[_\x81\x15k\x01\x9D\x97\x1EO\xE8@\x1Et\0\0\0\x19\x83\x90\x04\x84\x11\x15\x17aE\x8CW__\xFD[Pgge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x91\x02k\x01\x9D\x97\x1EO\xE8@\x1Et\0\0\0\x01\x04\x90V[__\x83_\x01Q\x83`\xFF\x16`\x02\x81\x10aE\xC7WaE\xC7aP\xDEV[` \x90\x81\x02\x91\x90\x91\x01Q\x90\x85\x01Q\x81Q`@Qcr\x16\x04y`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x92\x93P_\x92\x91\x16\x90c\xE4,\x08\xF2\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aF W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aFD\x91\x90aN\xCAV[\x90P\x80_\x03aFWW_\x92PPPa)2V[``\x82\x01Q`\xC0\x83\x01QaFk\x90\x82aP\xA5V[\x82\x10aF\x8FW`\xC0\x83\x01QaF\x80\x82\x84aP\xB8V[aF\x8A\x91\x90aP\xB8V[aF\x91V[_[\x96\x95PPPPPPV[_\x82`\xA0\x01Q_\x03aF\xAEWP_a)2V[_aF\xB9\x84\x84aK\xB5V[`\xA0\x85\x01Q\x90\x91PaBE\x90\x82aElV[_\x82aF\xD7\x83\x82aP\xB8V[\x91P\x81\x11\x15a)2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rtds-math-sub-underflow`X\x1B`D\x82\x01R`d\x01a\x04zV[a\x02d\x82\x82`@Q`$\x01aG6\x92\x91\x90aQ\x8FV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16cK\\Bw`\xE0\x1B\x17\x90RaK\xF8V[\x80Q_\x90\x81\x90aG\x86\x90\x82[` \x02\x01QQ\x84Q`\x01` \x02\x01QQa(\x90V[\x90P\x80`@Q` \x01aG\xCA\x90` \x80\x82R`\x1A\x90\x82\x01R\x7FTWAP_LAST_BLOCK_TIME_STAMP\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aG\xFA\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x91PP\x91\x90PV[\x80Q_\x90\x81\x90aH(\x90\x82aGqV[\x90P\x80`@Q` \x01aG\xCA\x90` \x80\x82R`\x1F\x90\x82\x01R\x7FTWAP_LAST_PRICE_CUMMULATIVE_KEY\0`@\x82\x01R``\x01\x90V[\x80Q_\x90\x81\x90aH|\x90\x82aGqV[\x90P\x80`@Q` \x01aG\xCA\x90` \x80\x82R`$\x90\x82\x01R\x7FTWAP_LAST_BLOCK_TIME_STAMP_BY_PE`@\x82\x01Rc\x14\x92S\xD1`\xE2\x1B``\x82\x01R`\x80\x01\x90V[\x80Q_\x90\x81\x90aH\xDD\x90\x82aGqV[\x90P\x80`@Q` \x01aG\xCA\x90` \x80\x82R`)\x90\x82\x01R\x7FTWAP_LAST_PRICE_CUMMULATIVE_KEY_`@\x82\x01Rh\x10\x96W\xD4\x11T\x92S\xD1`\xBA\x1B``\x82\x01R`\x80\x01\x90V[\x80Q_\x90\x81\x90aIC\x90\x82aGqV[\x90P\x80`@Q` \x01aG\xCA\x90` \x80\x82R`\x12\x90\x82\x01RqTWAP_AVERAGE_PRICE`p\x1B`@\x82\x01R``\x01\x90V[a\nT\x83\x83\x83`@Q`$\x01aI\x94\x93\x92\x91\x90aQ\xBCV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16cX!\xEF\xA1`\xE0\x1B\x17\x90RaK\xF8V[_\x81\x15\x80aI\xE6WP\x82\x82aI\xD8\x81\x83aP\x8EV[\x92PaI\xE4\x90\x83aP\xCBV[\x14[a)2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rsds-math-mul-overflow``\x1B`D\x82\x01R`d\x01a\x04zV[_\x81_\x03aJ8WP_\x91\x90PV[_`\x01aJD\x84aL\x01V[\x90\x1C`\x01\x90\x1B\x90P`\x01\x81\x84\x81aJ]WaJ]aPKV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81aJuWaJuaPKV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81aJ\x8DWaJ\x8DaPKV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81aJ\xA5WaJ\xA5aPKV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81aJ\xBDWaJ\xBDaPKV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81aJ\xD5WaJ\xD5aPKV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81aJ\xEDWaJ\xEDaPKV[\x04\x82\x01\x90\x1C\x90Pa=\xE4\x81\x82\x85\x81aK\x07WaK\x07aPKV[\x04[_\x81\x83\x10aK\x17W\x81a=\xE4V[P\x90\x91\x90PV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a@e\x90` \x80\x82R`\x19\x90\x82\x01R\x7FSHORT_LIQUIDITY_THRESHOLD\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[___aK\x9C\x85_\x01Q\x85`\xFF\x16`\x02\x81\x10aK\x8DWaK\x8DaP\xDEV[` \x02\x01Q\x86`\x80\x01QaF\x9BV[\x90P_aK\xA9\x86\x86aE\xADV[\x96\x91\x95P\x90\x93PPPPV[_B\x82\x03aK\xC8WP` \x82\x01Qa)2V[_aK\xD7\x84`@\x01Q\x84aE8V[\x90PaK\xF0\x84` \x01Q\x82aEl\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x91PPa)2V[a\x08*\x81aL\x94V[_\x80`\x80\x83\x90\x1C\x15aL\x15W`\x80\x92\x83\x1C\x92\x01[`@\x83\x90\x1C\x15aL'W`@\x92\x83\x1C\x92\x01[` \x83\x90\x1C\x15aL9W` \x92\x83\x1C\x92\x01[`\x10\x83\x90\x1C\x15aLKW`\x10\x92\x83\x1C\x92\x01[`\x08\x83\x90\x1C\x15aL]W`\x08\x92\x83\x1C\x92\x01[`\x04\x83\x90\x1C\x15aLoW`\x04\x92\x83\x1C\x92\x01[`\x02\x83\x90\x1C\x15aL\x81W`\x02\x92\x83\x1C\x92\x01[`\x01\x83\x90\x1C\x15a)2W`\x01\x01\x92\x91PPV[_jconsole.log\x90P__\x83Q` \x85\x01\x84Z\xFAPPPV[`@Q\x80a\x01@\x01`@R\x80aL\xC7aM\x0CV[\x81R` \x01_\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80`\xA0\x01`@R\x80aM\x1FaM\x8FV[\x81R_` \x82\x01\x81\x90R`@\x82\x01\x81\x90R``\x82\x01\x81\x90R`\x80\x90\x91\x01R\x90V[`@Q\x80a\x01`\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[aM\xE0`@Q\x80`\xE0\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81aM\x9EW\x90PP\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x08*W__\xFD[__\x82\x84\x03`\x80\x81\x12\x15aN\x1CW__\xFD[\x835aN'\x81aM\xF6V[\x92P```\x1F\x19\x82\x01\x12\x15aN:W__\xFD[P` \x83\x01\x90P\x92P\x92\x90PV[__\x82\x84\x03`\xA0\x81\x12\x15aNZW__\xFD[\x835aNe\x81aM\xF6V[\x92P`\x80`\x1F\x19\x82\x01\x12\x15aN:W__\xFD[_` \x82\x84\x03\x12\x15aN\x88W__\xFD[\x815a=\xE4\x81aM\xF6V[` \x80\x82R`\x17\x90\x82\x01R\x7FREENTRANCY_GUARD_STATUS\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[_` \x82\x84\x03\x12\x15aN\xDAW__\xFD[PQ\x91\x90PV[_` \x82\x84\x03\x12\x15aN\xF1W__\xFD[\x81Q\x80\x15\x15\x81\x14a=\xE4W__\xFD[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01\x81\x90R_\x90aBE\x90\x83\x01\x84aO\0V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[`\x01\x81[`\x01\x84\x11\x15a\n\x94W\x80\x85\x04\x81\x11\x15aO\x84WaO\x84aOQV[`\x01\x84\x16\x15aO\x92W\x90\x81\x02\x90[`\x01\x93\x90\x93\x1C\x92\x80\x02aOiV[_\x82aO\xAEWP`\x01a)2V[\x81aO\xBAWP_a)2V[\x81`\x01\x81\x14aO\xD0W`\x02\x81\x14aO\xDAWaO\xF6V[`\x01\x91PPa)2V[`\xFF\x84\x11\x15aO\xEBWaO\xEBaOQV[PP`\x01\x82\x1Ba)2V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15aP\x19WP\x81\x81\na)2V[aP%_\x19\x84\x84aOeV[\x80_\x19\x04\x82\x11\x15aP8WaP8aOQV[\x02\x93\x92PPPV[_a=\xE4\x83\x83aO\xA0V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[_\x82aPmWaPmaPKV[P\x06\x90V[c\xFF\xFF\xFF\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a)2Wa)2aOQV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a)2Wa)2aOQV[\x80\x82\x01\x80\x82\x11\x15a)2Wa)2aOQV[\x81\x81\x03\x81\x81\x11\x15a)2Wa)2aOQV[_\x82aP\xD9WaP\xD9aPKV[P\x04\x90V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15aQ\x02W__\xFD[\x81Qa=\xE4\x81aM\xF6V[` \x80\x82R`!\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_WITH_DEBT_`@\x82\x01R`\x03`\xFC\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`!\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_WITH_DEBT_`@\x82\x01R`1`\xF8\x1B``\x82\x01R`\x80\x01\x90V[`@\x81R_aQ\xA1`@\x83\x01\x85aO\0V[\x82\x81\x03` \x84\x01RaQ\xB3\x81\x85aO\0V[\x95\x94PPPPPV[``\x81R_aQ\xCE``\x83\x01\x86aO\0V[\x82\x81\x03` \x84\x01RaQ\xE0\x81\x86aO\0V[\x91PP\x82`@\x83\x01R\x94\x93PPPPV\xFE-----------updateTwapPrice-----------\xA2dipfsX\"\x12 \xA8\xEA\xDE\xC5\x01\xCF\x81\xC9\xEEz\x99\x01\xF6+2K`\xC0+gs\x8B\x87%2e]\xEAa\xAD\x05\xEAdsolcC\0\x08\x1C\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b5060043610610055575f3560e01c80634a4a7b04146100595780635ecd44e81461009c578063660d0d67146100b15780637d237c99146100d85780639ff78c30146100eb575b5f5ffd5b6100807f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b03909116815260200160405180910390f35b6100af6100aa366004614e0a565b610112565b005b6100807f000000000000000000000000000000000000000000000000000000000000000081565b6100af6100e6366004614e48565b610268565b6100807f000000000000000000000000000000000000000000000000000000000000000081565b61011a61037c565b61018960405160200161014b906020808252600a908201526921a7a72a2927a62622a960b11b604082015260600190565b604051602081830303815290604052805190602001206040518060400160405280600a81526020016921a7a72a2927a62622a960b11b81525061053a565b5f6040518060a001604052807f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031681526020017f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602001835f0160208101906102059190614e78565b6001600160a01b031681526020018360200160208101906102269190614e78565b6001600160a01b031681526020016102446060850160408601614e78565b6001600160a01b03169052905061025b83826105e8565b50610264610774565b5050565b61027061037c565b6102a160405160200161014b906020808252600a908201526921a7a72a2927a62622a960b11b604082015260600190565b5f6040518060c001604052807f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031681526020017f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602001835f01602081019061031d9190614e78565b6001600160a01b0316815260200183602001602081019061033e9190614e78565b6001600160a01b031681526040808501356020830152016103656080850160608601614e78565b6001600160a01b03169052905061025b838261082d565b5f7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663bd02d0f56040516020016103bb90614e93565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016103ef91815260200190565b602060405180830381865afa15801561040a573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061042e9190614eca565b905080156104835760405162461bcd60e51b815260206004820152601f60248201527f5265656e7472616e637947756172643a207265656e7472616e742063616c6c0060448201526064015b60405180910390fd5b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663e2a4853a6040516020016104c190614e93565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b1682526004820152600160248201526044016020604051808303815f875af1158015610516573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906102649190614eca565b60405163ac4ab3fb60e01b8152336004820152602481018390527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063ac4ab3fb90604401602060405180830381865afa1580156105a3573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906105c79190614ee1565b61026457338160405163a35b150b60e01b815260040161047a929190614f2e565b5f5f610600835f015184604001518560600151610a59565b915091505f61060e83610a9c565b905061061e845f01518483610b56565b6020830151604085810151905163352f9aed60e01b81526001600160a01b0391821660048201525f9183169063352f9aed906024016020604051808303815f875af115801561066f573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906106939190614eca565b606087015160405163352f9aed60e01b81526001600160a01b0391821660048201529192505f919084169063352f9aed906024016020604051808303815f875af11580156106e3573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906107079190614eca565b9050610713828261105c565b610724868484848b60800151611085565b865161073090876111c3565b61073e876020015187611207565b865161074b90868861122b565b61076a87602001518989604001518a606001518b608001518787612447565b5050505050505050565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663e2a4853a6040516020016107b290614e93565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b16825260048201525f60248201526044016020604051808303815f875af1158015610806573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061082a9190614eca565b50565b610835614cb3565b61084b825f015183604001518460600151610a59565b6020830152808252608083015161086391859161249b565b815181516080840151610877929190612558565b60c0850181905260a08501829052610120850183905261010085018490526108a1939291906127ad565b8051602001516001600160a01b03908116604080840182905260808501519051632770a7eb60e21b81529286166004840152602483015290639dc29fac906044015f604051808303815f87803b1580156108f9575f5ffd5b505af115801561090b573d5f5f3e3d5ffd5b505050506040818101518382015160a0850151610100850151935163078d3b7960e01b81526001600160a01b03928316600482015290821660248201526044810193909352169063078d3b79906064015f604051808303815f87803b158015610972575f5ffd5b505af1158015610984573d5f5f3e3d5ffd5b50505050604081810151606084015160a0850151610120850151935163078d3b7960e01b81526001600160a01b03928316600482015290821660248201526044810193909352169063078d3b79906064015f604051808303815f87803b1580156109ec575f5ffd5b505af11580156109fe573d5f5f3e3d5ffd5b50505060208301518251610a129250611207565b815160208201518251610a2692919061122b565b610a548260200151848460400151856060015186608001518760a00151876101000151886101200151612803565b505050565b610a61614d0c565b5f5f610a6d8585612890565b90505f610a7a8783612938565b9050610a868183613b3a565b610a8f81613b68565b925090505b935093915050565b5f5f610aaa835f5f5f613c14565b50505090505f610abd8460015f5f613c14565b5050509050815f1480610ace575080155b15610adc57505f9392505050565b5f610aeb85606001515f613cfd565b90505f610afd86606001516001613cfd565b90505f610b2085676765c793fa10079d601b1b610b1b86600a615040565b613d2b565b90505f610b3e85676765c793fa10079d601b1b610b1b86600a615040565b9050610b4a8282613deb565b98975050505050505050565b805f03610b6257505050565b610b836040518060600160405280602581526020016151f260259139613e26565b60408051610140810182525f80825260208201819052918101829052606081018290526080810182905260a0810182905260c0810182905260e081018290526101008101829052610120810191909152610bdd8484613e4a565b63ffffffff168152610bf46401000000004261505f565b63ffffffff90811660208301528151165f03610cb057610c1984848360200151613ec0565b610c2484845f613f44565b610c3384848360200151613f83565b610c3e84845f613f9b565b610c49848484613fb3565b610c836040518060400160405280600e81526020016d0626c6f636b54696d655374616d760941b815250826020015163ffffffff16613fcb565b610caa60405180604001604052806005815260200164707269636560d81b81525083613fcb565b50505050565b80516020820151610cc19190615072565b63ffffffff90811660408084019182528051808201909152600b81526a1d1a5b59515b185c1cd95960aa1b60208201529051610cfd9216613fcb565b604081015163ffffffff1615610caa57610d178484613ff4565b60608201526040810151610d319063ffffffff168361508e565b8160600151610d4091906150a5565b6080820152604080518082019091526005815264707269636560d81b6020820152610d6b9083613fcb565b610da26040518060400160405280600b81526020016a1d1a5b59515b185c1cd95960aa1b815250826040015163ffffffff16613fcb565b610ddb604051806040016040528060138152602001721c1c9a58d950dd5b5d5b185d1a5d9953185cdd606a1b8152508260600151613fcb565b610e106040518060400160405280600f81526020016e707269636543756d756c617469766560881b8152508260800151613fcb565b610e1f84848360800151613f44565b610e2e84848360200151613ec0565b610e38848461400d565b63ffffffff1660a082018190526020820151610e549190615072565b63ffffffff1660c0820152610e6884614026565b63ffffffff90811660e083019081526040805180820190915260068152651411549253d160d21b60208201529051610ea09216613fcb565b610ee96040518060400160405280601b81526020017f6c61737455706461746554696d657374616d704279506572696f6400000000008152508260a0015163ffffffff16613fcb565b610f28604051806040016040528060138152602001721d1a5b59515b185c1cd959109e54195c9a5bd9606a1b8152508260c0015163ffffffff16613fcb565b8060e0015163ffffffff168160c0015163ffffffff161115610caa57610f4e84846140d8565b610100820181905260c0820151608083015163ffffffff90911691610f72916150b8565b610f7c91906150cb565b61012082015260408051808201909152601b81527f707269636543756d756c61746976654c6173744279506572696f6400000000006020820152610100820151610fc69190613fcb565b610ffb6040518060400160405280600f81526020016e707269636543756d756c617469766560881b8152508260800151613fcb565b61102e6040518060400160405280600c81526020016b70726963654176657261676560a01b815250826101200151613fcb565b61103d84848360200151613f83565b61104c84848360800151613f9b565b610caa8484836101200151613fb3565b811580611067575080155b1561026457604051631a5df28360e21b815260040160405180910390fd5b5f61109386858560016140f1565b90505f856001600160a01b03166318160ddd6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156110d2573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906110f69190614eca565b9050805f0361115c576040516340c10f1960e01b81525f60048201526103e860248201526001600160a01b038716906340c10f19906044015f604051808303815f87803b158015611145575f5ffd5b505af1158015611157573d5f5f3e3d5ffd5b505050505b6040516340c10f1960e01b81526001600160a01b038481166004830152602482018490528716906340c10f19906044015b5f604051808303815f87803b1580156111a4575f5ffd5b505af11580156111b6573d5f5f3e3d5ffd5b5050505050505050505050565b6060810151660800000000000016610264575f5f6111e1848461424d565b91509150818110610caa5760608301516111fc9060016142ae565b606084015250505050565b611210816142d8565b91518051602090910151604090810193909352919091015250565b5f839050806001600160a01b031663c80f4c6260405160200161126b906020808252600990820152681413d3d317d31254d560ba1b604082015260600190565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b1682526004820152602481018690526044015f604051808303815f87803b1580156112bb575f5ffd5b505af11580156112cd573d5f5f3e3d5ffd5b50505050806001600160a01b031663ca446dd984604051602001611311906020808252600c908201526b0504f4f4c5f544f4b454e5f360a41b604082015260600190565b60405160208183030381529060405280519060200120604051602001611341929190918252602082015260400190565b60408051808303601f19018152908290528051602090910120855151516001600160e01b031960e085901b16835260048301919091526001600160a01b031660248201526044016020604051808303815f875af11580156113a4573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906113c891906150f2565b50806001600160a01b031663e2a4853a84604051602001611410906020808252601390820152720504f4f4c5f424f52524f575f494e4445585f3606c1b604082015260600190565b60405160208183030381529060405280519060200120604051602001611440929190918252602082015260400190565b60408051808303601f19018152908290528051602091820120865151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af115801561149d573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906114c19190614eca565b50806001600160a01b031663e2a4853a84604051602001611508906020808252601290820152710504f4f4c5f424f52524f575f524154455f360741b604082015260600190565b60405160208183030381529060405280519060200120604051602001611538929190918252602082015260400190565b60408051808303601f190181528282528051602090910120865151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af1158015611594573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906115b89190614eca565b50806001600160a01b031663e2a4853a84604051602001611604906020808252601790820152760504f4f4c5f544f54414c5f434f4c4c41544552414c5f3604c1b604082015260600190565b60405160208183030381529060405280519060200120604051602001611634929190918252602082015260400190565b60408051808303601f190181529082905280516020909101208551516060015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015611691573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906116b59190614eca565b50806001600160a01b031663e2a4853a846040516020016116d59061510d565b60405160208183030381529060405280519060200120604051602001611705929190918252602082015260400190565b60408051808303601f190181529082905280516020909101208551516080015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015611762573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906117869190614eca565b50806001600160a01b031663e2a4853a846040516020016117d3906020808252601890820152770504f4f4c5f544f54414c5f5343414c45445f444542545f360441b604082015260600190565b60405160208183030381529060405280519060200120604051602001611803929190918252602082015260400190565b60408051808303601f1901815290829052805160209091012085515160a0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015611860573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906118849190614eca565b50806001600160a01b031663e2a4853a846040516020016118cd906020808252601490820152730504f4f4c5f554e434c41494d45445f4645455f360641b604082015260600190565b604051602081830303815290604052805190602001206040516020016118fd929190918252602082015260400190565b60408051808303601f1901815290829052805160209091012085515160c0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af115801561195a573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061197e9190614eca565b50806001600160a01b031663ca446dd9846040516020016119bf906020808252600c908201526b504f4f4c5f544f4b454e5f3160a01b604082015260600190565b604051602081830303815290604052805190602001206040516020016119ef929190918252602082015260400190565b60408051808303601f19018152908290528051602091820120865190910151516001600160e01b031960e085901b16835260048301919091526001600160a01b031660248201526044016020604051808303815f875af1158015611a55573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611a7991906150f2565b50806001600160a01b031663e2a4853a84604051602001611ac190602080825260139082015272504f4f4c5f424f52524f575f494e4445585f3160681b604082015260600190565b60405160208183030381529060405280519060200120604051602001611af1929190918252602082015260400190565b60408051808303601f190181529082905280516020918201208651820151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af1158015611b50573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611b749190614eca565b50806001600160a01b031663e2a4853a84604051602001611bbb90602080825260129082015271504f4f4c5f424f52524f575f524154455f3160701b604082015260600190565b60405160208183030381529060405280519060200120604051602001611beb929190918252602082015260400190565b60408051808303601f190181528282528051602091820120875190910151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af1158015611c4a573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611c6e9190614eca565b50806001600160a01b031663e2a4853a84604051602001611cba90602080825260179082015276504f4f4c5f544f54414c5f434f4c4c41544552414c5f3160481b604082015260600190565b60405160208183030381529060405280519060200120604051602001611cea929190918252602082015260400190565b60408051808303601f190181529082905280516020918201208651909101516060015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015611d4a573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611d6e9190614eca565b50806001600160a01b031663e2a4853a84604051602001611d8e9061514e565b60405160208183030381529060405280519060200120604051602001611dbe929190918252602082015260400190565b60408051808303601f190181529082905280516020918201208651909101516080015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015611e1e573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611e429190614eca565b50806001600160a01b031663e2a4853a84604051602001611e8f90602080825260189082015277504f4f4c5f544f54414c5f5343414c45445f444542545f3160401b604082015260600190565b60405160208183030381529060405280519060200120604051602001611ebf929190918252602082015260400190565b60408051808303601f1901815290829052805160209182012086519091015160a0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015611f1f573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611f439190614eca565b50806001600160a01b031663e2a4853a84604051602001611f8c90602080825260149082015273504f4f4c5f554e434c41494d45445f4645455f3160601b604082015260600190565b60405160208183030381529060405280519060200120604051602001611fbc929190918252602082015260400190565b60408051808303601f1901815290829052805160209182012086519091015160c0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af115801561201c573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906120409190614eca565b50806001600160a01b031663ca446dd98460405160200161207e90602080825260099082015268504f4f4c5f42414e4b60b81b604082015260600190565b604051602081830303815290604052805190602001206040516020016120ae929190918252602082015260400190565b6040516020818303038152906040528051906020012084602001516040518363ffffffff1660e01b81526004016120f89291909182526001600160a01b0316602082015260400190565b6020604051808303815f875af1158015612114573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061213891906150f2565b50806001600160a01b031663ca446dd98460405160200161218a906020808252601b908201527f504f4f4c5f494e5445524553545f524154455f53545241544547590000000000604082015260600190565b604051602081830303815290604052805190602001206040516020016121ba929190918252602082015260400190565b60408051601f198184030181528282528051602090910120908601516001600160e01b031960e085901b168352612205926004019182526001600160a01b0316602082015260400190565b6020604051808303815f875af1158015612221573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061224591906150f2565b50806001600160a01b031663e2a4853a8460405160200161228c906020808252601290820152712827a7a62fa1a7a72324a3aaa920aa24a7a760711b604082015260600190565b604051602081830303815290604052805190602001206040516020016122bc929190918252602082015260400190565b6040516020818303038152906040528051906020012084606001516040518363ffffffff1660e01b81526004016122fd929190918252602082015260400190565b6020604051808303815f875af1158015612319573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061233d9190614eca565b50806001600160a01b031663e2a4853a8460405160200161238f906020808252601b908201527f504f4f4c5f4c4153545f5550444154455f54494d455f5354414d500000000000604082015260600190565b604051602081830303815290604052805190602001206040516020016123bf929190918252602082015260400190565b6040516020818303038152906040528051906020012084608001516040518363ffffffff1660e01b8152600401612400929190918252602082015260400190565b6020604051808303815f875af115801561241c573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906124409190614eca565b5050505050565b604051639ed486eb60e01b81526001600160a01b0387811660048301528681166024830152858116604483015284811660648301526084820184905260a48201839052881690639ed486eb9060c40161118d565b805f036124bb57604051635186591160e01b815260040160405180910390fd5b60208201516040516370a0823160e01b81526001600160a01b0385811660048301525f91908316906370a0823190602401602060405180830381865afa158015612507573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061252b9190614eca565b905080831115612440576040516302fa826960e51b8152600481018290526024810184905260440161047a565b5f5f5f5f612564614d40565b61256d886144e7565b8161014001818152505086602001516001600160a01b03166318160ddd6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156125b7573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906125db9190614eca565b8160200181815250506125f5875f5f846101400151613c14565b5060808401525060408201526101408101516126179088906001905f90613c14565b5060a084015250606082015260208101515f03612640575f5f5f5f9450945094509450506127a4565b6126538682604001518360200151613d2b565b6101008201526060810151602082015161266e918891613d2b565b816101200181815250506126ae6040518060400160405280601081526020016f766172732e746f74616c537570706c7960801b8152508260200151613fcb565b6126e6604051806040016040528060128152602001710766172732e707269636552657365727665360741b8152508260400151613fcb565b61271e60405180604001604052806012815260200171766172732e7072696365526573657276653160701b8152508260600151613fcb565b6127516040518060400160405280600c81526020016b0766172732e616d6f756e74360a41b815250826101000151613fcb565b6127846040518060400160405280600c81526020016b766172732e616d6f756e743160a01b815250826101200151613fcb565b80610100015181610120015182608001518360a001519450945094509450505b93509350935093565b818411156127d857604051631fc107c160e01b8152600481018590526024810183905260440161047a565b80831115610caa57604051630e793baf60e01b8152600481018490526024810182905260440161047a565b604051631495739160e11b81526001600160a01b0388811660048301528781166024830152868116604483015260648201869052848116608483015260a4820184905260c4820183905289169063292ae7229060e4015f604051808303815f87803b158015612870575f5ffd5b505af1158015612882573d5f5f3e3d5ffd5b505050505050505050505050565b5f816001600160a01b0316836001600160a01b0316106128b15781836128b4565b82825b60405191945092506128e1906020016020808252600490820152631413d3d360e21b604082015260600190565b60408051601f198184030181528282528051602091820120908301526001600160a01b0380861691830191909152831660608201526080016040516020818303038152906040528051906020012090505b92915050565b612940614d0c565b82612949614d0c565b816001600160a01b03166391d4403c604051602001612985906020808252600990820152681413d3d317d31254d560ba1b604082015260600190565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b168252600482015260248101879052604401602060405180830381865afa1580156129d9573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906129fd9190614ee1565b612a0a5791506129329050565b816001600160a01b03166321f8a72185604051602001612a4a906020808252600c908201526b0504f4f4c5f544f4b454e5f360a41b604082015260600190565b60405160208183030381529060405280519060200120604051602001612a7a929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401612aae91815260200190565b602060405180830381865afa158015612ac9573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612aed91906150f2565b8151516001600160a01b03918216905260408051602081810152601391810191909152720504f4f4c5f424f52524f575f494e4445585f3606c1b60608201529083169063bd02d0f590869060800160405160208183030381529060405280519060200120604051602001612b6b929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401612b9f91815260200190565b602060405180830381865afa158015612bba573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612bde9190614eca565b81515f60200201516020018181525050816001600160a01b031663bd02d0f585604051602001612c34906020808252601290820152710504f4f4c5f424f52524f575f524154455f360741b604082015260600190565b60405160208183030381529060405280519060200120604051602001612c64929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401612c9891815260200190565b602060405180830381865afa158015612cb3573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612cd79190614eca565b81515f60200201516040018181525050816001600160a01b031663bd02d0f585604051602001612d32906020808252601790820152760504f4f4c5f544f54414c5f434f4c4c41544552414c5f3604c1b604082015260600190565b60405160208183030381529060405280519060200120604051602001612d62929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401612d9691815260200190565b602060405180830381865afa158015612db1573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612dd59190614eca565b815151606001526040516001600160a01b0383169063bd02d0f5908690612dfe9060200161510d565b60405160208183030381529060405280519060200120604051602001612e2e929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401612e6291815260200190565b602060405180830381865afa158015612e7d573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612ea19190614eca565b81515f60200201516080018181525050816001600160a01b031663bd02d0f585604051602001612efd906020808252601890820152770504f4f4c5f544f54414c5f5343414c45445f444542545f360441b604082015260600190565b60405160208183030381529060405280519060200120604051602001612f2d929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401612f6191815260200190565b602060405180830381865afa158015612f7c573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612fa09190614eca565b81515160a0015260408051602081810152601491810191909152730504f4f4c5f554e434c41494d45445f4645455f360641b60608201526001600160a01b0383169063bd02d0f59086906080016040516020818303038152906040528051906020012060405160200161301d929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161305191815260200190565b602060405180830381865afa15801561306c573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906130909190614eca565b81515160c0015260408051602081810152600c918101919091526b504f4f4c5f544f4b454e5f3160a01b60608201526001600160a01b038316906321f8a72190869060800160405160208183030381529060405280519060200120604051602001613105929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161313991815260200190565b602060405180830381865afa158015613154573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061317891906150f2565b81516020908101516001600160a01b0392831690526040805180830183905260138183015272504f4f4c5f424f52524f575f494e4445585f3160681b60608083019190915282518083039091018152608082019092528151919092012060a0820187905260c08201529083169063bd02d0f59060e001604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161322291815260200190565b602060405180830381865afa15801561323d573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906132619190614eca565b8151600160200201516020018181525050816001600160a01b031663bd02d0f5856040516020016132b890602080825260129082015271504f4f4c5f424f52524f575f524154455f3160701b604082015260600190565b604051602081830303815290604052805190602001206040516020016132e8929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161331c91815260200190565b602060405180830381865afa158015613337573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061335b9190614eca565b8151600160200201516040018181525050816001600160a01b031663bd02d0f5856040516020016133b790602080825260179082015276504f4f4c5f544f54414c5f434f4c4c41544552414c5f3160481b604082015260600190565b604051602081830303815290604052805190602001206040516020016133e7929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161341b91815260200190565b602060405180830381865afa158015613436573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061345a9190614eca565b8151600160200201516060018181525050816001600160a01b031663bd02d0f58560405160200161348a9061514e565b604051602081830303815290604052805190602001206040516020016134ba929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016134ee91815260200190565b602060405180830381865afa158015613509573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061352d9190614eca565b8151600160200201516080018181525050816001600160a01b031663bd02d0f58560405160200161358a90602080825260189082015277504f4f4c5f544f54414c5f5343414c45445f444542545f3160401b604082015260600190565b604051602081830303815290604052805190602001206040516020016135ba929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016135ee91815260200190565b602060405180830381865afa158015613609573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061362d9190614eca565b81516001602002015160a0018181525050816001600160a01b031663bd02d0f58560405160200161368690602080825260149082015273504f4f4c5f554e434c41494d45445f4645455f3160601b604082015260600190565b604051602081830303815290604052805190602001206040516020016136b6929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016136ea91815260200190565b602060405180830381865afa158015613705573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906137299190614eca565b81516001602002015160c0018181525050816001600160a01b03166321f8a7218560405160200161377790602080825260099082015268504f4f4c5f42414e4b60b81b604082015260600190565b604051602081830303815290604052805190602001206040516020016137a7929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016137db91815260200190565b602060405180830381865afa1580156137f6573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061381a91906150f2565b81602001906001600160a01b031690816001600160a01b031681525050816001600160a01b03166321f8a72185604051602001613888906020808252601b908201527f504f4f4c5f494e5445524553545f524154455f53545241544547590000000000604082015260600190565b604051602081830303815290604052805190602001206040516020016138b8929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016138ec91815260200190565b602060405180830381865afa158015613907573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061392b91906150f2565b81604001906001600160a01b031690816001600160a01b031681525050816001600160a01b031663bd02d0f58560405160200161398e906020808252601290820152712827a7a62fa1a7a72324a3aaa920aa24a7a760711b604082015260600190565b604051602081830303815290604052805190602001206040516020016139be929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016139f291815260200190565b602060405180830381865afa158015613a0d573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613a319190614eca565b60608201526040516001600160a01b0383169063bd02d0f5908690613a8a906020016020808252601b908201527f504f4f4c5f4c4153545f5550444154455f54494d455f5354414d500000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001613aba929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401613aee91815260200190565b602060405180830381865afa158015613b09573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613b2d9190614eca565b6080820152949350505050565b60208201516001600160a01b031661026457604051637357d91f60e01b81526004810182905260240161047a565b60808101514290819003613b7a575050565b81515160a0015115613bd05781515f90613ba390825b6020020151604001518460800151614538565b8351909150613bc79082905f5b60200201516020015161456c90919063ffffffff16565b83515160200152505b81516020015160a0015115613c0c5781515f90613bee906001613b90565b8351909150613c009082906001613bb0565b83516020908101510152505b608090910152565b5f5f5f5f5f885f01518860ff1660028110613c3157613c316150de565b602002015190505f613c438a8a6145ad565b9050805f03613c5f575f5f5f5f95509550955095505050613cf2565b5f613c6e838c6080015161469b565b90505f613c7b828a61456c565b90505f8915613ca057818410613c9a57613c9584836146cb565b613ca2565b5f613ca2565b5f5b90505f613caf858d61456c565b90505f8c15613cd457848210613cce57613cc982866146cb565b613cd6565b5f613cd6565b5f5b9050613ce285876150a5565b9a50949850909650929450505050505b945094509450949050565b5f60ff60581b1960585f1960ff851601613d1d575060ff60601b19905060605b90198416901c905092915050565b5f838302815f1985870982811083820303915050805f03613d5f57838281613d5557613d5561504b565b0492505050613de4565b808411613d7f5760405163227bc15360e01b815260040160405180910390fd5b5f848688095f868103871696879004966002600389028118808a02820302808a02820302808a02820302808a02820302808a02820302808a02909103029181900381900460010186841190950394909402919094039290920491909117919091029150505b9392505050565b5f8115676765c793fa10079d601b1b60028404190484111715613e0c575f5ffd5b50676765c793fa10079d601b1b9190910260028204010490565b61082a60405180604001604052806002815260200161257360f01b81525082614720565b5f826001600160a01b031663bd02d0f5613e6384614765565b6040518263ffffffff1660e01b8152600401613e8191815260200190565b602060405180830381865afa158015613e9c573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613de49190614eca565b826001600160a01b031663e2a4853a613ed884614765565b6040516001600160e01b031960e084901b168152600481019190915263ffffffff841660248201526044015b6020604051808303815f875af1158015613f20573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610caa9190614eca565b826001600160a01b031663e2a4853a613f5c84614818565b836040518363ffffffff1660e01b8152600401613f04929190918252602082015260400190565b826001600160a01b031663e2a4853a613ed88461486c565b826001600160a01b031663e2a4853a613f5c846148cd565b826001600160a01b031663e2a4853a613f5c84614933565b610264604051806040016040528060068152602001652573202d257360d01b815250838361497c565b5f826001600160a01b031663bd02d0f5613e6384614818565b5f826001600160a01b031663bd02d0f5613e638461486c565b5f816001600160a01b031663bd02d0f5604051602001614065906020808252600b908201526a1515d05417d411549253d160aa1b604082015260600190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161409991815260200190565b602060405180830381865afa1580156140b4573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906129329190614eca565b5f826001600160a01b031663bd02d0f5613e63846148cd565b5f6140fa614d40565b85602001516001600160a01b03166318160ddd6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561413a573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061415e9190614eca565b602082015261416f865f8080613c14565b50505060c08201526141848660015f80613c14565b50505060e082015282156141bf57848160c0018181516141a491906150b8565b90525060e0810180518591906141bb9083906150b8565b9052505b80602001515f036141ef576141e86103e86141e26141dd88886149c3565b614a29565b906146cb565b8152614241565b60c08101511580614202575060e0810151155b15614210575f915050614245565b61423e6142268683602001518460c00151613d2b565b6142398684602001518560e00151613d2b565b614b09565b81525b5190505b949350505050565b5f5f5f61425e84606001515f613cfd565b90505f61426a86614b1e565b90505f61428d8261427c85600a615040565b676765c793fa10079d601b1b613d2b565b90505f61429c875f5f5f613c14565b50939a91995090975050505050505050565b5f6033826142bc575f6142bf565b60015b60ff16901b660800000000000019841617905092915050565b5f5f61430d6040518060c001604052805f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b614317845f614b6f565b60208301528152606084015161432d905f613cfd565b60608201819052815161435291676765c793fa10079d601b1b90610b1b90600a615040565b604082015260208101515f0361436d575f608082015261440d565b60408481015181516060810183528351815260208085015190820190815284840151828501908152935163fdd63ecf60e01b815291516004830152516024820152915160448301526001600160a01b03169063fdd63ecf90606401602060405180830381865afa1580156143e3573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906144079190614eca565b60808201525b614418846001614b6f565b602083018190529082525f03614433575f60a08201526144d3565b60408481015181516060810183528351815260208085015190820190815284840151828501908152935163fdd63ecf60e01b815291516004830152516024820152915160448301526001600160a01b03169063fdd63ecf90606401602060405180830381865afa1580156144a9573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906144cd9190614eca565b60a08201525b80608001518160a001519250925050915091565b5f816001600160a01b031663bd02d0f5604051602001614065906020808252601e908201527f5452414441424c455f444542545f4d554c5449504945525f464143544f520000604082015260600190565b5f8061454483426150b8565b61454e908561508e565b6301e133809004905061424581676765c793fa10079d601b1b6150a5565b5f81156b019d971e4fe8401e74000000198390048411151761458c575f5ffd5b50676765c793fa10079d601b1b91026b019d971e4fe8401e74000000010490565b5f5f835f01518360ff16600281106145c7576145c76150de565b60209081029190910151908501518151604051637216047960e11b81526001600160a01b0391821660048201529293505f9291169063e42c08f290602401602060405180830381865afa158015614620573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906146449190614eca565b9050805f03614657575f92505050612932565b606082015160c083015161466b90826150a5565b821061468f5760c083015161468082846150b8565b61468a91906150b8565b614691565b5f5b9695505050505050565b5f8260a001515f036146ae57505f612932565b5f6146b98484614bb5565b60a0850151909150614245908261456c565b5f826146d783826150b8565b91508111156129325760405162461bcd60e51b815260206004820152601560248201527464732d6d6174682d7375622d756e646572666c6f7760581b604482015260640161047a565b610264828260405160240161473692919061518f565b60408051601f198184030181529190526020810180516001600160e01b0316634b5c427760e01b179052614bf8565b80515f90819061478690825b60200201515184516001602002015151612890565b9050806040516020016147ca906020808252601a908201527f545741505f4c4153545f424c4f434b5f54494d455f5354414d50000000000000604082015260600190565b604051602081830303815290604052805190602001206040516020016147fa929190918252602082015260400190565b60405160208183030381529060405280519060200120915050919050565b80515f9081906148289082614771565b9050806040516020016147ca906020808252601f908201527f545741505f4c4153545f50524943455f43554d4d554c41544956455f4b455900604082015260600190565b80515f90819061487c9082614771565b9050806040516020016147ca9060208082526024908201527f545741505f4c4153545f424c4f434b5f54494d455f5354414d505f42595f5045604082015263149253d160e21b606082015260800190565b80515f9081906148dd9082614771565b9050806040516020016147ca9060208082526029908201527f545741505f4c4153545f50524943455f43554d4d554c41544956455f4b45595f604082015268109657d411549253d160ba1b606082015260800190565b80515f9081906149439082614771565b9050806040516020016147ca90602080825260129082015271545741505f415645524147455f505249434560701b604082015260600190565b610a54838383604051602401614994939291906151bc565b60408051601f198184030181529190526020810180516001600160e01b0316635821efa160e01b179052614bf8565b5f8115806149e6575082826149d8818361508e565b92506149e490836150cb565b145b6129325760405162461bcd60e51b815260206004820152601460248201527364732d6d6174682d6d756c2d6f766572666c6f7760601b604482015260640161047a565b5f815f03614a3857505f919050565b5f6001614a4484614c01565b901c6001901b90506001818481614a5d57614a5d61504b565b048201901c90506001818481614a7557614a7561504b565b048201901c90506001818481614a8d57614a8d61504b565b048201901c90506001818481614aa557614aa561504b565b048201901c90506001818481614abd57614abd61504b565b048201901c90506001818481614ad557614ad561504b565b048201901c90506001818481614aed57614aed61504b565b048201901c9050613de481828581614b0757614b0761504b565b045b5f818310614b175781613de4565b5090919050565b5f816001600160a01b031663bd02d0f56040516020016140659060208082526019908201527f53484f52545f4c49515549444954595f5448524553484f4c4400000000000000604082015260600190565b5f5f5f614b9c855f01518560ff1660028110614b8d57614b8d6150de565b6020020151866080015161469b565b90505f614ba986866145ad565b96919550909350505050565b5f428203614bc857506020820151612932565b5f614bd7846040015184614538565b9050614bf084602001518261456c90919063ffffffff16565b915050612932565b61082a81614c94565b5f80608083901c15614c1557608092831c92015b604083901c15614c2757604092831c92015b602083901c15614c3957602092831c92015b601083901c15614c4b57601092831c92015b600883901c15614c5d57600892831c92015b600483901c15614c6f57600492831c92015b600283901c15614c8157600292831c92015b600183901c156129325760010192915050565b5f6a636f6e736f6c652e6c6f6790505f5f835160208501845afa505050565b604051806101400160405280614cc7614d0c565b81526020015f81526020015f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b6040518060a00160405280614d1f614d8f565b81525f60208201819052604082018190526060820181905260809091015290565b6040518061016001604052805f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b60405180604001604052806002905b614de06040518060e001604052805f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b815260200190600190039081614d9e5790505090565b6001600160a01b038116811461082a575f5ffd5b5f5f8284036080811215614e1c575f5ffd5b8335614e2781614df6565b92506060601f1982011215614e3a575f5ffd5b506020830190509250929050565b5f5f82840360a0811215614e5a575f5ffd5b8335614e6581614df6565b92506080601f1982011215614e3a575f5ffd5b5f60208284031215614e88575f5ffd5b8135613de481614df6565b60208082526017908201527f5245454e5452414e43595f47554152445f535441545553000000000000000000604082015260600190565b5f60208284031215614eda575f5ffd5b5051919050565b5f60208284031215614ef1575f5ffd5b81518015158114613de4575f5ffd5b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b6001600160a01b03831681526040602082018190525f9061424590830184614f00565b634e487b7160e01b5f52601160045260245ffd5b6001815b6001841115610a9457808504811115614f8457614f84614f51565b6001841615614f9257908102905b60019390931c928002614f69565b5f82614fae57506001612932565b81614fba57505f612932565b8160018114614fd05760028114614fda57614ff6565b6001915050612932565b60ff841115614feb57614feb614f51565b50506001821b612932565b5060208310610133831016604e8410600b8410161715615019575081810a612932565b6150255f198484614f65565b805f190482111561503857615038614f51565b029392505050565b5f613de48383614fa0565b634e487b7160e01b5f52601260045260245ffd5b5f8261506d5761506d61504b565b500690565b63ffffffff828116828216039081111561293257612932614f51565b808202811582820484141761293257612932614f51565b8082018082111561293257612932614f51565b8181038181111561293257612932614f51565b5f826150d9576150d961504b565b500490565b634e487b7160e01b5f52603260045260245ffd5b5f60208284031215615102575f5ffd5b8151613de481614df6565b60208082526021908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f574954485f444542545f6040820152600360fc1b606082015260800190565b60208082526021908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f574954485f444542545f6040820152603160f81b606082015260800190565b604081525f6151a16040830185614f00565b82810360208401526151b38185614f00565b95945050505050565b606081525f6151ce6060830186614f00565b82810360208401526151e08186614f00565b91505082604083015294935050505056fe2d2d2d2d2d2d2d2d2d2d2d7570646174655477617050726963652d2d2d2d2d2d2d2d2d2d2da2646970667358221220a8eadec501cf81c9ee7a9901f62b324b60c02b67738b872532655dea61ad05ea64736f6c634300081c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0UW_5`\xE0\x1C\x80cJJ{\x04\x14a\0YW\x80c^\xCDD\xE8\x14a\0\x9CW\x80cf\r\rg\x14a\0\xB1W\x80c}#|\x99\x14a\0\xD8W\x80c\x9F\xF7\x8C0\x14a\0\xEBW[__\xFD[a\0\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0\xAFa\0\xAA6`\x04aN\nV[a\x01\x12V[\0[a\0\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\0\xAFa\0\xE66`\x04aNHV[a\x02hV[a\0\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01\x1Aa\x03|V[a\x01\x89`@Q` \x01a\x01K\x90` \x80\x82R`\n\x90\x82\x01Ri!\xA7\xA7*)'\xA6&\"\xA9`\xB1\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x80`@\x01`@R\x80`\n\x81R` \x01i!\xA7\xA7*)'\xA6&\"\xA9`\xB1\x1B\x81RPa\x05:V[_`@Q\x80`\xA0\x01`@R\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83_\x01` \x81\x01\x90a\x02\x05\x91\x90aNxV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83` \x01` \x81\x01\x90a\x02&\x91\x90aNxV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01a\x02D``\x85\x01`@\x86\x01aNxV[`\x01`\x01`\xA0\x1B\x03\x16\x90R\x90Pa\x02[\x83\x82a\x05\xE8V[Pa\x02da\x07tV[PPV[a\x02pa\x03|V[a\x02\xA1`@Q` \x01a\x01K\x90` \x80\x82R`\n\x90\x82\x01Ri!\xA7\xA7*)'\xA6&\"\xA9`\xB1\x1B`@\x82\x01R``\x01\x90V[_`@Q\x80`\xC0\x01`@R\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83_\x01` \x81\x01\x90a\x03\x1D\x91\x90aNxV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83` \x01` \x81\x01\x90a\x03>\x91\x90aNxV[`\x01`\x01`\xA0\x1B\x03\x16\x81R`@\x80\x85\x015` \x83\x01R\x01a\x03e`\x80\x85\x01``\x86\x01aNxV[`\x01`\x01`\xA0\x1B\x03\x16\x90R\x90Pa\x02[\x83\x82a\x08-V[_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x03\xBB\x90aN\x93V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x03\xEF\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\nW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04.\x91\x90aN\xCAV[\x90P\x80\x15a\x04\x83W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:`@Q` \x01a\x04\xC1\x90aN\x93V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`\x01`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x05\x16W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02d\x91\x90aN\xCAV[`@Qc\xACJ\xB3\xFB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x83\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xACJ\xB3\xFB\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xA3W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xC7\x91\x90aN\xE1V[a\x02dW3\x81`@Qc\xA3[\x15\x0B`\xE0\x1B\x81R`\x04\x01a\x04z\x92\x91\x90aO.V[__a\x06\0\x83_\x01Q\x84`@\x01Q\x85``\x01Qa\nYV[\x91P\x91P_a\x06\x0E\x83a\n\x9CV[\x90Pa\x06\x1E\x84_\x01Q\x84\x83a\x0BVV[` \x83\x01Q`@\x85\x81\x01Q\x90Qc5/\x9A\xED`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R_\x91\x83\x16\x90c5/\x9A\xED\x90`$\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x06oW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x93\x91\x90aN\xCAV[``\x87\x01Q`@Qc5/\x9A\xED`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x91\x92P_\x91\x90\x84\x16\x90c5/\x9A\xED\x90`$\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x06\xE3W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x07\x91\x90aN\xCAV[\x90Pa\x07\x13\x82\x82a\x10\\V[a\x07$\x86\x84\x84\x84\x8B`\x80\x01Qa\x10\x85V[\x86Qa\x070\x90\x87a\x11\xC3V[a\x07>\x87` \x01Q\x87a\x12\x07V[\x86Qa\x07K\x90\x86\x88a\x12+V[a\x07j\x87` \x01Q\x89\x89`@\x01Q\x8A``\x01Q\x8B`\x80\x01Q\x87\x87a$GV[PPPPPPPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:`@Q` \x01a\x07\xB2\x90aN\x93V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R_`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x08\x06W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08*\x91\x90aN\xCAV[PV[a\x085aL\xB3V[a\x08K\x82_\x01Q\x83`@\x01Q\x84``\x01Qa\nYV[` \x83\x01R\x80\x82R`\x80\x83\x01Qa\x08c\x91\x85\x91a$\x9BV[\x81Q\x81Q`\x80\x84\x01Qa\x08w\x92\x91\x90a%XV[`\xC0\x85\x01\x81\x90R`\xA0\x85\x01\x82\x90Ra\x01 \x85\x01\x83\x90Ra\x01\0\x85\x01\x84\x90Ra\x08\xA1\x93\x92\x91\x90a'\xADV[\x80Q` \x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`@\x80\x84\x01\x82\x90R`\x80\x85\x01Q\x90Qc'p\xA7\xEB`\xE2\x1B\x81R\x92\x86\x16`\x04\x84\x01R`$\x83\x01R\x90c\x9D\xC2\x9F\xAC\x90`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x08\xF9W__\xFD[PZ\xF1\x15\x80\x15a\t\x0BW=__>=_\xFD[PPPP`@\x81\x81\x01Q\x83\x82\x01Q`\xA0\x85\x01Qa\x01\0\x85\x01Q\x93Qc\x07\x8D;y`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x04\x82\x01R\x90\x82\x16`$\x82\x01R`D\x81\x01\x93\x90\x93R\x16\x90c\x07\x8D;y\x90`d\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\trW__\xFD[PZ\xF1\x15\x80\x15a\t\x84W=__>=_\xFD[PPPP`@\x81\x81\x01Q``\x84\x01Q`\xA0\x85\x01Qa\x01 \x85\x01Q\x93Qc\x07\x8D;y`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x04\x82\x01R\x90\x82\x16`$\x82\x01R`D\x81\x01\x93\x90\x93R\x16\x90c\x07\x8D;y\x90`d\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\t\xECW__\xFD[PZ\xF1\x15\x80\x15a\t\xFEW=__>=_\xFD[PPP` \x83\x01Q\x82Qa\n\x12\x92Pa\x12\x07V[\x81Q` \x82\x01Q\x82Qa\n&\x92\x91\x90a\x12+V[a\nT\x82` \x01Q\x84\x84`@\x01Q\x85``\x01Q\x86`\x80\x01Q\x87`\xA0\x01Q\x87a\x01\0\x01Q\x88a\x01 \x01Qa(\x03V[PPPV[a\naaM\x0CV[__a\nm\x85\x85a(\x90V[\x90P_a\nz\x87\x83a)8V[\x90Pa\n\x86\x81\x83a;:V[a\n\x8F\x81a;hV[\x92P\x90P[\x93P\x93\x91PPV[__a\n\xAA\x83___a<\x14V[PPP\x90P_a\n\xBD\x84`\x01__a<\x14V[PPP\x90P\x81_\x14\x80a\n\xCEWP\x80\x15[\x15a\n\xDCWP_\x93\x92PPPV[_a\n\xEB\x85``\x01Q_a<\xFDV[\x90P_a\n\xFD\x86``\x01Q`\x01a<\xFDV[\x90P_a\x0B \x85gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba\x0B\x1B\x86`\naP@V[a=+V[\x90P_a\x0B>\x85gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba\x0B\x1B\x86`\naP@V[\x90Pa\x0BJ\x82\x82a=\xEBV[\x98\x97PPPPPPPPV[\x80_\x03a\x0BbWPPPV[a\x0B\x83`@Q\x80``\x01`@R\x80`%\x81R` \x01aQ\xF2`%\x919a>&V[`@\x80Qa\x01@\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x82\x90Ra\x01\0\x81\x01\x82\x90Ra\x01 \x81\x01\x91\x90\x91Ra\x0B\xDD\x84\x84a>JV[c\xFF\xFF\xFF\xFF\x16\x81Ra\x0B\xF4d\x01\0\0\0\0BaP_V[c\xFF\xFF\xFF\xFF\x90\x81\x16` \x83\x01R\x81Q\x16_\x03a\x0C\xB0Wa\x0C\x19\x84\x84\x83` \x01Qa>\xC0V[a\x0C$\x84\x84_a?DV[a\x0C3\x84\x84\x83` \x01Qa?\x83V[a\x0C>\x84\x84_a?\x9BV[a\x0CI\x84\x84\x84a?\xB3V[a\x0C\x83`@Q\x80`@\x01`@R\x80`\x0E\x81R` \x01m\x06&\xC6\xF66\xB5F\x96\xD6U7F\x16\xD7`\x94\x1B\x81RP\x82` \x01Qc\xFF\xFF\xFF\xFF\x16a?\xCBV[a\x0C\xAA`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01dprice`\xD8\x1B\x81RP\x83a?\xCBV[PPPPV[\x80Q` \x82\x01Qa\x0C\xC1\x91\x90aPrV[c\xFF\xFF\xFF\xFF\x90\x81\x16`@\x80\x84\x01\x91\x82R\x80Q\x80\x82\x01\x90\x91R`\x0B\x81Rj\x1D\x1A[YQ[\x18\\\x1C\xD9Y`\xAA\x1B` \x82\x01R\x90Qa\x0C\xFD\x92\x16a?\xCBV[`@\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x15a\x0C\xAAWa\r\x17\x84\x84a?\xF4V[``\x82\x01R`@\x81\x01Qa\r1\x90c\xFF\xFF\xFF\xFF\x16\x83aP\x8EV[\x81``\x01Qa\r@\x91\x90aP\xA5V[`\x80\x82\x01R`@\x80Q\x80\x82\x01\x90\x91R`\x05\x81Rdprice`\xD8\x1B` \x82\x01Ra\rk\x90\x83a?\xCBV[a\r\xA2`@Q\x80`@\x01`@R\x80`\x0B\x81R` \x01j\x1D\x1A[YQ[\x18\\\x1C\xD9Y`\xAA\x1B\x81RP\x82`@\x01Qc\xFF\xFF\xFF\xFF\x16a?\xCBV[a\r\xDB`@Q\x80`@\x01`@R\x80`\x13\x81R` \x01r\x1C\x1C\x9AX\xD9P\xDD[][\x18]\x1A]\x99S\x18\\\xDD`j\x1B\x81RP\x82``\x01Qa?\xCBV[a\x0E\x10`@Q\x80`@\x01`@R\x80`\x0F\x81R` \x01npriceCumulative`\x88\x1B\x81RP\x82`\x80\x01Qa?\xCBV[a\x0E\x1F\x84\x84\x83`\x80\x01Qa?DV[a\x0E.\x84\x84\x83` \x01Qa>\xC0V[a\x0E8\x84\x84a@\rV[c\xFF\xFF\xFF\xFF\x16`\xA0\x82\x01\x81\x90R` \x82\x01Qa\x0ET\x91\x90aPrV[c\xFF\xFF\xFF\xFF\x16`\xC0\x82\x01Ra\x0Eh\x84a@&V[c\xFF\xFF\xFF\xFF\x90\x81\x16`\xE0\x83\x01\x90\x81R`@\x80Q\x80\x82\x01\x90\x91R`\x06\x81Re\x14\x11T\x92S\xD1`\xD2\x1B` \x82\x01R\x90Qa\x0E\xA0\x92\x16a?\xCBV[a\x0E\xE9`@Q\x80`@\x01`@R\x80`\x1B\x81R` \x01\x7FlastUpdateTimestampByPeriod\0\0\0\0\0\x81RP\x82`\xA0\x01Qc\xFF\xFF\xFF\xFF\x16a?\xCBV[a\x0F(`@Q\x80`@\x01`@R\x80`\x13\x81R` \x01r\x1D\x1A[YQ[\x18\\\x1C\xD9Y\x10\x9ET\x19\\\x9A[\xD9`j\x1B\x81RP\x82`\xC0\x01Qc\xFF\xFF\xFF\xFF\x16a?\xCBV[\x80`\xE0\x01Qc\xFF\xFF\xFF\xFF\x16\x81`\xC0\x01Qc\xFF\xFF\xFF\xFF\x16\x11\x15a\x0C\xAAWa\x0FN\x84\x84a@\xD8V[a\x01\0\x82\x01\x81\x90R`\xC0\x82\x01Q`\x80\x83\x01Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x91a\x0Fr\x91aP\xB8V[a\x0F|\x91\x90aP\xCBV[a\x01 \x82\x01R`@\x80Q\x80\x82\x01\x90\x91R`\x1B\x81R\x7FpriceCumulativeLastByPeriod\0\0\0\0\0` \x82\x01Ra\x01\0\x82\x01Qa\x0F\xC6\x91\x90a?\xCBV[a\x0F\xFB`@Q\x80`@\x01`@R\x80`\x0F\x81R` \x01npriceCumulative`\x88\x1B\x81RP\x82`\x80\x01Qa?\xCBV[a\x10.`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01kpriceAverage`\xA0\x1B\x81RP\x82a\x01 \x01Qa?\xCBV[a\x10=\x84\x84\x83` \x01Qa?\x83V[a\x10L\x84\x84\x83`\x80\x01Qa?\x9BV[a\x0C\xAA\x84\x84\x83a\x01 \x01Qa?\xB3V[\x81\x15\x80a\x10gWP\x80\x15[\x15a\x02dW`@Qc\x1A]\xF2\x83`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x10\x93\x86\x85\x85`\x01a@\xF1V[\x90P_\x85`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10\xD2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\xF6\x91\x90aN\xCAV[\x90P\x80_\x03a\x11\\W`@Qc@\xC1\x0F\x19`\xE0\x1B\x81R_`\x04\x82\x01Ra\x03\xE8`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x87\x16\x90c@\xC1\x0F\x19\x90`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x11EW__\xFD[PZ\xF1\x15\x80\x15a\x11WW=__>=_\xFD[PPPP[`@Qc@\xC1\x0F\x19`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R`$\x82\x01\x84\x90R\x87\x16\x90c@\xC1\x0F\x19\x90`D\x01[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x11\xA4W__\xFD[PZ\xF1\x15\x80\x15a\x11\xB6W=__>=_\xFD[PPPPPPPPPPPV[``\x81\x01Qf\x08\0\0\0\0\0\0\x16a\x02dW__a\x11\xE1\x84\x84aBMV[\x91P\x91P\x81\x81\x10a\x0C\xAAW``\x83\x01Qa\x11\xFC\x90`\x01aB\xAEV[``\x84\x01RPPPPV[a\x12\x10\x81aB\xD8V[\x91Q\x80Q` \x90\x91\x01Q`@\x90\x81\x01\x93\x90\x93R\x91\x90\x91\x01RPV[_\x83\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xC8\x0FLb`@Q` \x01a\x12k\x90` \x80\x82R`\t\x90\x82\x01Rh\x14\x13\xD3\xD3\x17\xD3\x12T\xD5`\xBA\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x86\x90R`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x12\xBBW__\xFD[PZ\xF1\x15\x80\x15a\x12\xCDW=__>=_\xFD[PPPP\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a\x13\x11\x90` \x80\x82R`\x0C\x90\x82\x01Rk\x05\x04\xF4\xF4\xC5\xF5D\xF4\xB4T\xE5\xF3`\xA4\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x13A\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQQ`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x13\xA4W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\xC8\x91\x90aP\xF2V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x14\x10\x90` \x80\x82R`\x13\x90\x82\x01Rr\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF4\x94\xE4DU\x85\xF3`l\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x14@\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86QQ\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x14\x9DW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\xC1\x91\x90aN\xCAV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x15\x08\x90` \x80\x82R`\x12\x90\x82\x01Rq\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF5$\x15DU\xF3`t\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x158\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82R\x80Q` \x90\x91\x01 \x86QQ\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x15\x94W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\xB8\x91\x90aN\xCAV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x16\x04\x90` \x80\x82R`\x17\x90\x82\x01Rv\x05\x04\xF4\xF4\xC5\xF5D\xF5D\x14\xC5\xF44\xF4\xC4\xC4\x15DU$\x14\xC5\xF3`L\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x164\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ``\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x16\x91W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\xB5\x91\x90aN\xCAV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x16\xD5\x90aQ\rV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x17\x05\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ`\x80\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x17bW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\x86\x91\x90aN\xCAV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x17\xD3\x90` \x80\x82R`\x18\x90\x82\x01Rw\x05\x04\xF4\xF4\xC5\xF5D\xF5D\x14\xC5\xF544\x14\xC4TE\xF4DT%E\xF3`D\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x18\x03\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ`\xA0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x18`W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18\x84\x91\x90aN\xCAV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x18\xCD\x90` \x80\x82R`\x14\x90\x82\x01Rs\x05\x04\xF4\xF4\xC5\xF5T\xE44\xC4\x14\x94\xD4TE\xF4dTU\xF3`d\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x18\xFD\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ`\xC0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x19ZW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19~\x91\x90aN\xCAV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a\x19\xBF\x90` \x80\x82R`\x0C\x90\x82\x01RkPOOL_TOKEN_1`\xA0\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x19\xEF\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01QQ`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1AUW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Ay\x91\x90aP\xF2V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x1A\xC1\x90` \x80\x82R`\x13\x90\x82\x01RrPOOL_BORROW_INDEX_1`h\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1A\xF1\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x82\x01Q\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1BPW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Bt\x91\x90aN\xCAV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x1B\xBB\x90` \x80\x82R`\x12\x90\x82\x01RqPOOL_BORROW_RATE_1`p\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1B\xEB\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x87Q\x90\x91\x01Q\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1CJW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Cn\x91\x90aN\xCAV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x1C\xBA\x90` \x80\x82R`\x17\x90\x82\x01RvPOOL_TOTAL_COLLATERAL_1`H\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1C\xEA\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q``\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1DJW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Dn\x91\x90aN\xCAV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x1D\x8E\x90aQNV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1D\xBE\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q`\x80\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1E\x1EW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1EB\x91\x90aN\xCAV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x1E\x8F\x90` \x80\x82R`\x18\x90\x82\x01RwPOOL_TOTAL_SCALED_DEBT_1`@\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1E\xBF\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q`\xA0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1F\x1FW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1FC\x91\x90aN\xCAV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x1F\x8C\x90` \x80\x82R`\x14\x90\x82\x01RsPOOL_UNCLAIMED_FEE_1``\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1F\xBC\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q`\xC0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a \x1CW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a @\x91\x90aN\xCAV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a ~\x90` \x80\x82R`\t\x90\x82\x01RhPOOL_BANK`\xB8\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a \xAE\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84` \x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a \xF8\x92\x91\x90\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a!\x14W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!8\x91\x90aP\xF2V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a!\x8A\x90` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_INTEREST_RATE_STRATEGY\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a!\xBA\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x90\x91\x01 \x90\x86\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83Ra\"\x05\x92`\x04\x01\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\"!W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\"E\x91\x90aP\xF2V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\"\x8C\x90` \x80\x82R`\x12\x90\x82\x01Rq('\xA7\xA6/\xA1\xA7\xA7#$\xA3\xAA\xA9 \xAA$\xA7\xA7`q\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\"\xBC\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84``\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\"\xFD\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a#\x19W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a#=\x91\x90aN\xCAV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a#\x8F\x90` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_LAST_UPDATE_TIME_STAMP\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a#\xBF\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84`\x80\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a$\0\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a$\x1CW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a$@\x91\x90aN\xCAV[PPPPPV[`@Qc\x9E\xD4\x86\xEB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x81\x16`\x04\x83\x01R\x86\x81\x16`$\x83\x01R\x85\x81\x16`D\x83\x01R\x84\x81\x16`d\x83\x01R`\x84\x82\x01\x84\x90R`\xA4\x82\x01\x83\x90R\x88\x16\x90c\x9E\xD4\x86\xEB\x90`\xC4\x01a\x11\x8DV[\x80_\x03a$\xBBW`@QcQ\x86Y\x11`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x82\x01Q`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R_\x91\x90\x83\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a%\x07W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a%+\x91\x90aN\xCAV[\x90P\x80\x83\x11\x15a$@W`@Qc\x02\xFA\x82i`\xE5\x1B\x81R`\x04\x81\x01\x82\x90R`$\x81\x01\x84\x90R`D\x01a\x04zV[____a%daM@V[a%m\x88aD\xE7V[\x81a\x01@\x01\x81\x81RPP\x86` \x01Q`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a%\xB7W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a%\xDB\x91\x90aN\xCAV[\x81` \x01\x81\x81RPPa%\xF5\x87__\x84a\x01@\x01Qa<\x14V[P`\x80\x84\x01RP`@\x82\x01Ra\x01@\x81\x01Qa&\x17\x90\x88\x90`\x01\x90_\x90a<\x14V[P`\xA0\x84\x01RP``\x82\x01R` \x81\x01Q_\x03a&@W____\x94P\x94P\x94P\x94PPa'\xA4V[a&S\x86\x82`@\x01Q\x83` \x01Qa=+V[a\x01\0\x82\x01R``\x81\x01Q` \x82\x01Qa&n\x91\x88\x91a=+V[\x81a\x01 \x01\x81\x81RPPa&\xAE`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01ovars.totalSupply`\x80\x1B\x81RP\x82` \x01Qa?\xCBV[a&\xE6`@Q\x80`@\x01`@R\x80`\x12\x81R` \x01q\x07f\x17'2\xE7\x07&\x966U&W6W'fS`t\x1B\x81RP\x82`@\x01Qa?\xCBV[a'\x1E`@Q\x80`@\x01`@R\x80`\x12\x81R` \x01qvars.priceReserve1`p\x1B\x81RP\x82``\x01Qa?\xCBV[a'Q`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01k\x07f\x17'2\xE6\x16\xD6\xF7V\xE7C`\xA4\x1B\x81RP\x82a\x01\0\x01Qa?\xCBV[a'\x84`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01kvars.amount1`\xA0\x1B\x81RP\x82a\x01 \x01Qa?\xCBV[\x80a\x01\0\x01Q\x81a\x01 \x01Q\x82`\x80\x01Q\x83`\xA0\x01Q\x94P\x94P\x94P\x94PP[\x93P\x93P\x93P\x93V[\x81\x84\x11\x15a'\xD8W`@Qc\x1F\xC1\x07\xC1`\xE0\x1B\x81R`\x04\x81\x01\x85\x90R`$\x81\x01\x83\x90R`D\x01a\x04zV[\x80\x83\x11\x15a\x0C\xAAW`@Qc\x0Ey;\xAF`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x81\x01\x82\x90R`D\x01a\x04zV[`@Qc\x14\x95s\x91`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x88\x81\x16`\x04\x83\x01R\x87\x81\x16`$\x83\x01R\x86\x81\x16`D\x83\x01R`d\x82\x01\x86\x90R\x84\x81\x16`\x84\x83\x01R`\xA4\x82\x01\x84\x90R`\xC4\x82\x01\x83\x90R\x89\x16\x90c)*\xE7\"\x90`\xE4\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a(pW__\xFD[PZ\xF1\x15\x80\x15a(\x82W=__>=_\xFD[PPPPPPPPPPPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x10a(\xB1W\x81\x83a(\xB4V[\x82\x82[`@Q\x91\x94P\x92Pa(\xE1\x90` \x01` \x80\x82R`\x04\x90\x82\x01Rc\x14\x13\xD3\xD3`\xE2\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01R`\x01`\x01`\xA0\x1B\x03\x80\x86\x16\x91\x83\x01\x91\x90\x91R\x83\x16``\x82\x01R`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P[\x92\x91PPV[a)@aM\x0CV[\x82a)IaM\x0CV[\x81`\x01`\x01`\xA0\x1B\x03\x16c\x91\xD4@<`@Q` \x01a)\x85\x90` \x80\x82R`\t\x90\x82\x01Rh\x14\x13\xD3\xD3\x17\xD3\x12T\xD5`\xBA\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x87\x90R`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a)\xD9W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a)\xFD\x91\x90aN\xE1V[a*\nW\x91Pa)2\x90PV[\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a*J\x90` \x80\x82R`\x0C\x90\x82\x01Rk\x05\x04\xF4\xF4\xC5\xF5D\xF4\xB4T\xE5\xF3`\xA4\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a*z\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a*\xAE\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a*\xC9W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a*\xED\x91\x90aP\xF2V[\x81QQ`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x90R`@\x80Q` \x81\x81\x01R`\x13\x91\x81\x01\x91\x90\x91Rr\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF4\x94\xE4DU\x85\xF3`l\x1B``\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a+k\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a+\x9F\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a+\xBAW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a+\xDE\x91\x90aN\xCAV[\x81Q_` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a,4\x90` \x80\x82R`\x12\x90\x82\x01Rq\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF5$\x15DU\xF3`t\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a,d\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a,\x98\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a,\xB3W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a,\xD7\x91\x90aN\xCAV[\x81Q_` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a-2\x90` \x80\x82R`\x17\x90\x82\x01Rv\x05\x04\xF4\xF4\xC5\xF5D\xF5D\x14\xC5\xF44\xF4\xC4\xC4\x15DU$\x14\xC5\xF3`L\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a-b\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a-\x96\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a-\xB1W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a-\xD5\x91\x90aN\xCAV[\x81QQ``\x01R`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90a-\xFE\x90` \x01aQ\rV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a..\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a.b\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a.}W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a.\xA1\x91\x90aN\xCAV[\x81Q_` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a.\xFD\x90` \x80\x82R`\x18\x90\x82\x01Rw\x05\x04\xF4\xF4\xC5\xF5D\xF5D\x14\xC5\xF544\x14\xC4TE\xF4DT%E\xF3`D\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a/-\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a/a\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a/|W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a/\xA0\x91\x90aN\xCAV[\x81QQ`\xA0\x01R`@\x80Q` \x81\x81\x01R`\x14\x91\x81\x01\x91\x90\x91Rs\x05\x04\xF4\xF4\xC5\xF5T\xE44\xC4\x14\x94\xD4TE\xF4dTU\xF3`d\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a0\x1D\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a0Q\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a0lW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a0\x90\x91\x90aN\xCAV[\x81QQ`\xC0\x01R`@\x80Q` \x81\x81\x01R`\x0C\x91\x81\x01\x91\x90\x91RkPOOL_TOKEN_1`\xA0\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c!\xF8\xA7!\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a1\x05\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a19\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a1TW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a1x\x91\x90aP\xF2V[\x81Q` \x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x90R`@\x80Q\x80\x83\x01\x83\x90R`\x13\x81\x83\x01RrPOOL_BORROW_INDEX_1`h\x1B``\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x90\x92R\x81Q\x91\x90\x92\x01 `\xA0\x82\x01\x87\x90R`\xC0\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a2\"\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a2=W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a2a\x91\x90aN\xCAV[\x81Q`\x01` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a2\xB8\x90` \x80\x82R`\x12\x90\x82\x01RqPOOL_BORROW_RATE_1`p\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a2\xE8\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a3\x1C\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a37W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a3[\x91\x90aN\xCAV[\x81Q`\x01` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a3\xB7\x90` \x80\x82R`\x17\x90\x82\x01RvPOOL_TOTAL_COLLATERAL_1`H\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a3\xE7\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a4\x1B\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a46W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a4Z\x91\x90aN\xCAV[\x81Q`\x01` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a4\x8A\x90aQNV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a4\xBA\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a4\xEE\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a5\tW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a5-\x91\x90aN\xCAV[\x81Q`\x01` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a5\x8A\x90` \x80\x82R`\x18\x90\x82\x01RwPOOL_TOTAL_SCALED_DEBT_1`@\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a5\xBA\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a5\xEE\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a6\tW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a6-\x91\x90aN\xCAV[\x81Q`\x01` \x02\x01Q`\xA0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a6\x86\x90` \x80\x82R`\x14\x90\x82\x01RsPOOL_UNCLAIMED_FEE_1``\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a6\xB6\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a6\xEA\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a7\x05W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a7)\x91\x90aN\xCAV[\x81Q`\x01` \x02\x01Q`\xC0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a7w\x90` \x80\x82R`\t\x90\x82\x01RhPOOL_BANK`\xB8\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a7\xA7\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a7\xDB\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a7\xF6W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a8\x1A\x91\x90aP\xF2V[\x81` \x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a8\x88\x90` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_INTEREST_RATE_STRATEGY\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a8\xB8\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a8\xEC\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a9\x07W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a9+\x91\x90aP\xF2V[\x81`@\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a9\x8E\x90` \x80\x82R`\x12\x90\x82\x01Rq('\xA7\xA6/\xA1\xA7\xA7#$\xA3\xAA\xA9 \xAA$\xA7\xA7`q\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a9\xBE\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a9\xF2\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a:\rW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a:1\x91\x90aN\xCAV[``\x82\x01R`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90a:\x8A\x90` \x01` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_LAST_UPDATE_TIME_STAMP\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a:\xBA\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a:\xEE\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a;\tW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a;-\x91\x90aN\xCAV[`\x80\x82\x01R\x94\x93PPPPV[` \x82\x01Q`\x01`\x01`\xA0\x1B\x03\x16a\x02dW`@QcsW\xD9\x1F`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x04zV[`\x80\x81\x01QB\x90\x81\x90\x03a;zWPPV[\x81QQ`\xA0\x01Q\x15a;\xD0W\x81Q_\x90a;\xA3\x90\x82[` \x02\x01Q`@\x01Q\x84`\x80\x01QaE8V[\x83Q\x90\x91Pa;\xC7\x90\x82\x90_[` \x02\x01Q` \x01QaEl\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x83QQ` \x01RP[\x81Q` \x01Q`\xA0\x01Q\x15a<\x0CW\x81Q_\x90a;\xEE\x90`\x01a;\x90V[\x83Q\x90\x91Pa<\0\x90\x82\x90`\x01a;\xB0V[\x83Q` \x90\x81\x01Q\x01RP[`\x80\x90\x91\x01RV[_____\x88_\x01Q\x88`\xFF\x16`\x02\x81\x10a<1Wa<1aP\xDEV[` \x02\x01Q\x90P_a<C\x8A\x8AaE\xADV[\x90P\x80_\x03a<_W____\x95P\x95P\x95P\x95PPPa<\xF2V[_a<n\x83\x8C`\x80\x01QaF\x9BV[\x90P_a<{\x82\x8AaElV[\x90P_\x89\x15a<\xA0W\x81\x84\x10a<\x9AWa<\x95\x84\x83aF\xCBV[a<\xA2V[_a<\xA2V[_[\x90P_a<\xAF\x85\x8DaElV[\x90P_\x8C\x15a<\xD4W\x84\x82\x10a<\xCEWa<\xC9\x82\x86aF\xCBV[a<\xD6V[_a<\xD6V[_[\x90Pa<\xE2\x85\x87aP\xA5V[\x9AP\x94\x98P\x90\x96P\x92\x94PPPPP[\x94P\x94P\x94P\x94\x90PV[_`\xFF`X\x1B\x19`X_\x19`\xFF\x85\x16\x01a=\x1DWP`\xFF``\x1B\x19\x90P``[\x90\x19\x84\x16\x90\x1C\x90P\x92\x91PPV[_\x83\x83\x02\x81_\x19\x85\x87\t\x82\x81\x10\x83\x82\x03\x03\x91PP\x80_\x03a=_W\x83\x82\x81a=UWa=UaPKV[\x04\x92PPPa=\xE4V[\x80\x84\x11a=\x7FW`@Qc\"{\xC1S`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x84\x86\x88\t_\x86\x81\x03\x87\x16\x96\x87\x90\x04\x96`\x02`\x03\x89\x02\x81\x18\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x90\x91\x03\x02\x91\x81\x90\x03\x81\x90\x04`\x01\x01\x86\x84\x11\x90\x95\x03\x94\x90\x94\x02\x91\x90\x94\x03\x92\x90\x92\x04\x91\x90\x91\x17\x91\x90\x91\x02\x91PP[\x93\x92PPPV[_\x81\x15gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B`\x02\x84\x04\x19\x04\x84\x11\x17\x15a>\x0CW__\xFD[Pgge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x91\x90\x91\x02`\x02\x82\x04\x01\x04\x90V[a\x08*`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a%s`\xF0\x1B\x81RP\x82aG V[_\x82`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5a>c\x84aGeV[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a>\x81\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a>\x9CW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a=\xE4\x91\x90aN\xCAV[\x82`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:a>\xD8\x84aGeV[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81R`\x04\x81\x01\x91\x90\x91Rc\xFF\xFF\xFF\xFF\x84\x16`$\x82\x01R`D\x01[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a? W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xAA\x91\x90aN\xCAV[\x82`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:a?\\\x84aH\x18V[\x83`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a?\x04\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[\x82`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:a>\xD8\x84aHlV[\x82`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:a?\\\x84aH\xCDV[\x82`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:a?\\\x84aI3V[a\x02d`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01e%s -%s`\xD0\x1B\x81RP\x83\x83aI|V[_\x82`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5a>c\x84aH\x18V[_\x82`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5a>c\x84aHlV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a@e\x90` \x80\x82R`\x0B\x90\x82\x01Rj\x15\x15\xD0T\x17\xD4\x11T\x92S\xD1`\xAA\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a@\x99\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a@\xB4W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a)2\x91\x90aN\xCAV[_\x82`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5a>c\x84aH\xCDV[_a@\xFAaM@V[\x85` \x01Q`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aA:W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aA^\x91\x90aN\xCAV[` \x82\x01RaAo\x86_\x80\x80a<\x14V[PPP`\xC0\x82\x01RaA\x84\x86`\x01_\x80a<\x14V[PPP`\xE0\x82\x01R\x82\x15aA\xBFW\x84\x81`\xC0\x01\x81\x81QaA\xA4\x91\x90aP\xB8V[\x90RP`\xE0\x81\x01\x80Q\x85\x91\x90aA\xBB\x90\x83\x90aP\xB8V[\x90RP[\x80` \x01Q_\x03aA\xEFWaA\xE8a\x03\xE8aA\xE2aA\xDD\x88\x88aI\xC3V[aJ)V[\x90aF\xCBV[\x81RaBAV[`\xC0\x81\x01Q\x15\x80aB\x02WP`\xE0\x81\x01Q\x15[\x15aB\x10W_\x91PPaBEV[aB>aB&\x86\x83` \x01Q\x84`\xC0\x01Qa=+V[aB9\x86\x84` \x01Q\x85`\xE0\x01Qa=+V[aK\tV[\x81R[Q\x90P[\x94\x93PPPPV[___aB^\x84``\x01Q_a<\xFDV[\x90P_aBj\x86aK\x1EV[\x90P_aB\x8D\x82aB|\x85`\naP@V[gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba=+V[\x90P_aB\x9C\x87___a<\x14V[P\x93\x9A\x91\x99P\x90\x97PPPPPPPPV[_`3\x82aB\xBCW_aB\xBFV[`\x01[`\xFF\x16\x90\x1Bf\x08\0\0\0\0\0\0\x19\x84\x16\x17\x90P\x92\x91PPV[__aC\r`@Q\x80`\xC0\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[aC\x17\x84_aKoV[` \x83\x01R\x81R``\x84\x01QaC-\x90_a<\xFDV[``\x82\x01\x81\x90R\x81QaCR\x91gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x90a\x0B\x1B\x90`\naP@V[`@\x82\x01R` \x81\x01Q_\x03aCmW_`\x80\x82\x01RaD\rV[`@\x84\x81\x01Q\x81Q``\x81\x01\x83R\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01\x90\x81R\x84\x84\x01Q\x82\x85\x01\x90\x81R\x93Qc\xFD\xD6>\xCF`\xE0\x1B\x81R\x91Q`\x04\x83\x01RQ`$\x82\x01R\x91Q`D\x83\x01R`\x01`\x01`\xA0\x1B\x03\x16\x90c\xFD\xD6>\xCF\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aC\xE3W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aD\x07\x91\x90aN\xCAV[`\x80\x82\x01R[aD\x18\x84`\x01aKoV[` \x83\x01\x81\x90R\x90\x82R_\x03aD3W_`\xA0\x82\x01RaD\xD3V[`@\x84\x81\x01Q\x81Q``\x81\x01\x83R\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01\x90\x81R\x84\x84\x01Q\x82\x85\x01\x90\x81R\x93Qc\xFD\xD6>\xCF`\xE0\x1B\x81R\x91Q`\x04\x83\x01RQ`$\x82\x01R\x91Q`D\x83\x01R`\x01`\x01`\xA0\x1B\x03\x16\x90c\xFD\xD6>\xCF\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aD\xA9W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aD\xCD\x91\x90aN\xCAV[`\xA0\x82\x01R[\x80`\x80\x01Q\x81`\xA0\x01Q\x92P\x92PP\x91P\x91V[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a@e\x90` \x80\x82R`\x1E\x90\x82\x01R\x7FTRADABLE_DEBT_MULTIPIER_FACTOR\0\0`@\x82\x01R``\x01\x90V[_\x80aED\x83BaP\xB8V[aEN\x90\x85aP\x8EV[c\x01\xE13\x80\x90\x04\x90PaBE\x81gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1BaP\xA5V[_\x81\x15k\x01\x9D\x97\x1EO\xE8@\x1Et\0\0\0\x19\x83\x90\x04\x84\x11\x15\x17aE\x8CW__\xFD[Pgge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x91\x02k\x01\x9D\x97\x1EO\xE8@\x1Et\0\0\0\x01\x04\x90V[__\x83_\x01Q\x83`\xFF\x16`\x02\x81\x10aE\xC7WaE\xC7aP\xDEV[` \x90\x81\x02\x91\x90\x91\x01Q\x90\x85\x01Q\x81Q`@Qcr\x16\x04y`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x92\x93P_\x92\x91\x16\x90c\xE4,\x08\xF2\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aF W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aFD\x91\x90aN\xCAV[\x90P\x80_\x03aFWW_\x92PPPa)2V[``\x82\x01Q`\xC0\x83\x01QaFk\x90\x82aP\xA5V[\x82\x10aF\x8FW`\xC0\x83\x01QaF\x80\x82\x84aP\xB8V[aF\x8A\x91\x90aP\xB8V[aF\x91V[_[\x96\x95PPPPPPV[_\x82`\xA0\x01Q_\x03aF\xAEWP_a)2V[_aF\xB9\x84\x84aK\xB5V[`\xA0\x85\x01Q\x90\x91PaBE\x90\x82aElV[_\x82aF\xD7\x83\x82aP\xB8V[\x91P\x81\x11\x15a)2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rtds-math-sub-underflow`X\x1B`D\x82\x01R`d\x01a\x04zV[a\x02d\x82\x82`@Q`$\x01aG6\x92\x91\x90aQ\x8FV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16cK\\Bw`\xE0\x1B\x17\x90RaK\xF8V[\x80Q_\x90\x81\x90aG\x86\x90\x82[` \x02\x01QQ\x84Q`\x01` \x02\x01QQa(\x90V[\x90P\x80`@Q` \x01aG\xCA\x90` \x80\x82R`\x1A\x90\x82\x01R\x7FTWAP_LAST_BLOCK_TIME_STAMP\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aG\xFA\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x91PP\x91\x90PV[\x80Q_\x90\x81\x90aH(\x90\x82aGqV[\x90P\x80`@Q` \x01aG\xCA\x90` \x80\x82R`\x1F\x90\x82\x01R\x7FTWAP_LAST_PRICE_CUMMULATIVE_KEY\0`@\x82\x01R``\x01\x90V[\x80Q_\x90\x81\x90aH|\x90\x82aGqV[\x90P\x80`@Q` \x01aG\xCA\x90` \x80\x82R`$\x90\x82\x01R\x7FTWAP_LAST_BLOCK_TIME_STAMP_BY_PE`@\x82\x01Rc\x14\x92S\xD1`\xE2\x1B``\x82\x01R`\x80\x01\x90V[\x80Q_\x90\x81\x90aH\xDD\x90\x82aGqV[\x90P\x80`@Q` \x01aG\xCA\x90` \x80\x82R`)\x90\x82\x01R\x7FTWAP_LAST_PRICE_CUMMULATIVE_KEY_`@\x82\x01Rh\x10\x96W\xD4\x11T\x92S\xD1`\xBA\x1B``\x82\x01R`\x80\x01\x90V[\x80Q_\x90\x81\x90aIC\x90\x82aGqV[\x90P\x80`@Q` \x01aG\xCA\x90` \x80\x82R`\x12\x90\x82\x01RqTWAP_AVERAGE_PRICE`p\x1B`@\x82\x01R``\x01\x90V[a\nT\x83\x83\x83`@Q`$\x01aI\x94\x93\x92\x91\x90aQ\xBCV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16cX!\xEF\xA1`\xE0\x1B\x17\x90RaK\xF8V[_\x81\x15\x80aI\xE6WP\x82\x82aI\xD8\x81\x83aP\x8EV[\x92PaI\xE4\x90\x83aP\xCBV[\x14[a)2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rsds-math-mul-overflow``\x1B`D\x82\x01R`d\x01a\x04zV[_\x81_\x03aJ8WP_\x91\x90PV[_`\x01aJD\x84aL\x01V[\x90\x1C`\x01\x90\x1B\x90P`\x01\x81\x84\x81aJ]WaJ]aPKV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81aJuWaJuaPKV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81aJ\x8DWaJ\x8DaPKV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81aJ\xA5WaJ\xA5aPKV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81aJ\xBDWaJ\xBDaPKV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81aJ\xD5WaJ\xD5aPKV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81aJ\xEDWaJ\xEDaPKV[\x04\x82\x01\x90\x1C\x90Pa=\xE4\x81\x82\x85\x81aK\x07WaK\x07aPKV[\x04[_\x81\x83\x10aK\x17W\x81a=\xE4V[P\x90\x91\x90PV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a@e\x90` \x80\x82R`\x19\x90\x82\x01R\x7FSHORT_LIQUIDITY_THRESHOLD\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[___aK\x9C\x85_\x01Q\x85`\xFF\x16`\x02\x81\x10aK\x8DWaK\x8DaP\xDEV[` \x02\x01Q\x86`\x80\x01QaF\x9BV[\x90P_aK\xA9\x86\x86aE\xADV[\x96\x91\x95P\x90\x93PPPPV[_B\x82\x03aK\xC8WP` \x82\x01Qa)2V[_aK\xD7\x84`@\x01Q\x84aE8V[\x90PaK\xF0\x84` \x01Q\x82aEl\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x91PPa)2V[a\x08*\x81aL\x94V[_\x80`\x80\x83\x90\x1C\x15aL\x15W`\x80\x92\x83\x1C\x92\x01[`@\x83\x90\x1C\x15aL'W`@\x92\x83\x1C\x92\x01[` \x83\x90\x1C\x15aL9W` \x92\x83\x1C\x92\x01[`\x10\x83\x90\x1C\x15aLKW`\x10\x92\x83\x1C\x92\x01[`\x08\x83\x90\x1C\x15aL]W`\x08\x92\x83\x1C\x92\x01[`\x04\x83\x90\x1C\x15aLoW`\x04\x92\x83\x1C\x92\x01[`\x02\x83\x90\x1C\x15aL\x81W`\x02\x92\x83\x1C\x92\x01[`\x01\x83\x90\x1C\x15a)2W`\x01\x01\x92\x91PPV[_jconsole.log\x90P__\x83Q` \x85\x01\x84Z\xFAPPPV[`@Q\x80a\x01@\x01`@R\x80aL\xC7aM\x0CV[\x81R` \x01_\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80`\xA0\x01`@R\x80aM\x1FaM\x8FV[\x81R_` \x82\x01\x81\x90R`@\x82\x01\x81\x90R``\x82\x01\x81\x90R`\x80\x90\x91\x01R\x90V[`@Q\x80a\x01`\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[aM\xE0`@Q\x80`\xE0\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81aM\x9EW\x90PP\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x08*W__\xFD[__\x82\x84\x03`\x80\x81\x12\x15aN\x1CW__\xFD[\x835aN'\x81aM\xF6V[\x92P```\x1F\x19\x82\x01\x12\x15aN:W__\xFD[P` \x83\x01\x90P\x92P\x92\x90PV[__\x82\x84\x03`\xA0\x81\x12\x15aNZW__\xFD[\x835aNe\x81aM\xF6V[\x92P`\x80`\x1F\x19\x82\x01\x12\x15aN:W__\xFD[_` \x82\x84\x03\x12\x15aN\x88W__\xFD[\x815a=\xE4\x81aM\xF6V[` \x80\x82R`\x17\x90\x82\x01R\x7FREENTRANCY_GUARD_STATUS\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[_` \x82\x84\x03\x12\x15aN\xDAW__\xFD[PQ\x91\x90PV[_` \x82\x84\x03\x12\x15aN\xF1W__\xFD[\x81Q\x80\x15\x15\x81\x14a=\xE4W__\xFD[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01\x81\x90R_\x90aBE\x90\x83\x01\x84aO\0V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[`\x01\x81[`\x01\x84\x11\x15a\n\x94W\x80\x85\x04\x81\x11\x15aO\x84WaO\x84aOQV[`\x01\x84\x16\x15aO\x92W\x90\x81\x02\x90[`\x01\x93\x90\x93\x1C\x92\x80\x02aOiV[_\x82aO\xAEWP`\x01a)2V[\x81aO\xBAWP_a)2V[\x81`\x01\x81\x14aO\xD0W`\x02\x81\x14aO\xDAWaO\xF6V[`\x01\x91PPa)2V[`\xFF\x84\x11\x15aO\xEBWaO\xEBaOQV[PP`\x01\x82\x1Ba)2V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15aP\x19WP\x81\x81\na)2V[aP%_\x19\x84\x84aOeV[\x80_\x19\x04\x82\x11\x15aP8WaP8aOQV[\x02\x93\x92PPPV[_a=\xE4\x83\x83aO\xA0V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[_\x82aPmWaPmaPKV[P\x06\x90V[c\xFF\xFF\xFF\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a)2Wa)2aOQV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a)2Wa)2aOQV[\x80\x82\x01\x80\x82\x11\x15a)2Wa)2aOQV[\x81\x81\x03\x81\x81\x11\x15a)2Wa)2aOQV[_\x82aP\xD9WaP\xD9aPKV[P\x04\x90V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15aQ\x02W__\xFD[\x81Qa=\xE4\x81aM\xF6V[` \x80\x82R`!\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_WITH_DEBT_`@\x82\x01R`\x03`\xFC\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`!\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_WITH_DEBT_`@\x82\x01R`1`\xF8\x1B``\x82\x01R`\x80\x01\x90V[`@\x81R_aQ\xA1`@\x83\x01\x85aO\0V[\x82\x81\x03` \x84\x01RaQ\xB3\x81\x85aO\0V[\x95\x94PPPPPV[``\x81R_aQ\xCE``\x83\x01\x86aO\0V[\x82\x81\x03` \x84\x01RaQ\xE0\x81\x86aO\0V[\x91PP\x82`@\x83\x01R\x94\x93PPPPV\xFE-----------updateTwapPrice-----------\xA2dipfsX\"\x12 \xA8\xEA\xDE\xC5\x01\xCF\x81\xC9\xEEz\x99\x01\xF6+2K`\xC0+gs\x8B\x87%2e]\xEAa\xAD\x05\xEAdsolcC\0\x08\x1C\x003",
    );
    /**Custom error with signature `EmptyAddAmounts()` and selector `0x6977ca0c`.
```solidity
error EmptyAddAmounts();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EmptyAddAmounts {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<EmptyAddAmounts> for UnderlyingRustTuple<'_> {
            fn from(value: EmptyAddAmounts) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for EmptyAddAmounts {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for EmptyAddAmounts {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "EmptyAddAmounts()";
            const SELECTOR: [u8; 4] = [105u8, 119u8, 202u8, 12u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `EmptyPool(bytes32)` and selector `0x7357d91f`.
```solidity
error EmptyPool(bytes32 key);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EmptyPool {
        pub key: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<EmptyPool> for UnderlyingRustTuple<'_> {
            fn from(value: EmptyPool) -> Self {
                (value.key,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for EmptyPool {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { key: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for EmptyPool {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "EmptyPool(bytes32)";
            const SELECTOR: [u8; 4] = [115u8, 87u8, 217u8, 31u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.key),
                )
            }
        }
    };
    /**Custom error with signature `EmptyRemoveAmounts()` and selector `0x51865911`.
```solidity
error EmptyRemoveAmounts();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EmptyRemoveAmounts {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<EmptyRemoveAmounts> for UnderlyingRustTuple<'_> {
            fn from(value: EmptyRemoveAmounts) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for EmptyRemoveAmounts {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for EmptyRemoveAmounts {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "EmptyRemoveAmounts()";
            const SELECTOR: [u8; 4] = [81u8, 134u8, 89u8, 17u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `InsufficientUserBalance(uint256,uint256)` and selector `0x5f504d20`.
```solidity
error InsufficientUserBalance(uint256 balance, uint256 liquidity);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InsufficientUserBalance {
        pub balance: alloy::sol_types::private::primitives::aliases::U256,
        pub liquidity: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InsufficientUserBalance> for UnderlyingRustTuple<'_> {
            fn from(value: InsufficientUserBalance) -> Self {
                (value.balance, value.liquidity)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InsufficientUserBalance {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    balance: tuple.0,
                    liquidity: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InsufficientUserBalance {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InsufficientUserBalance(uint256,uint256)";
            const SELECTOR: [u8; 4] = [95u8, 80u8, 77u8, 32u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.balance),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.liquidity),
                )
            }
        }
    };
    /**Custom error with signature `MathOverflowedMulDiv()` and selector `0x227bc153`.
```solidity
error MathOverflowedMulDiv();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MathOverflowedMulDiv {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<MathOverflowedMulDiv> for UnderlyingRustTuple<'_> {
            fn from(value: MathOverflowedMulDiv) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for MathOverflowedMulDiv {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for MathOverflowedMulDiv {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "MathOverflowedMulDiv()";
            const SELECTOR: [u8; 4] = [34u8, 123u8, 193u8, 83u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `Reserve0Insufficient(uint256,uint256)` and selector `0x1fc107c1`.
```solidity
error Reserve0Insufficient(uint256 amount0, uint256 availableReserve0);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Reserve0Insufficient {
        pub amount0: alloy::sol_types::private::primitives::aliases::U256,
        pub availableReserve0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<Reserve0Insufficient> for UnderlyingRustTuple<'_> {
            fn from(value: Reserve0Insufficient) -> Self {
                (value.amount0, value.availableReserve0)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Reserve0Insufficient {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    amount0: tuple.0,
                    availableReserve0: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for Reserve0Insufficient {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "Reserve0Insufficient(uint256,uint256)";
            const SELECTOR: [u8; 4] = [31u8, 193u8, 7u8, 193u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount0),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.availableReserve0),
                )
            }
        }
    };
    /**Custom error with signature `Reserve1Insufficient(uint256,uint256)` and selector `0x0e793baf`.
```solidity
error Reserve1Insufficient(uint256 amount1, uint256 availableReserve1);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Reserve1Insufficient {
        pub amount1: alloy::sol_types::private::primitives::aliases::U256,
        pub availableReserve1: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<Reserve1Insufficient> for UnderlyingRustTuple<'_> {
            fn from(value: Reserve1Insufficient) -> Self {
                (value.amount1, value.availableReserve1)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Reserve1Insufficient {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    amount1: tuple.0,
                    availableReserve1: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for Reserve1Insufficient {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "Reserve1Insufficient(uint256,uint256)";
            const SELECTOR: [u8; 4] = [14u8, 121u8, 59u8, 175u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount1),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.availableReserve1),
                )
            }
        }
    };
    /**Custom error with signature `Unauthorized(address,string)` and selector `0xa35b150b`.
```solidity
error Unauthorized(address msgSender, string role);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Unauthorized {
        pub msgSender: alloy::sol_types::private::Address,
        pub role: alloy::sol_types::private::String,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::String,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::String,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<Unauthorized> for UnderlyingRustTuple<'_> {
            fn from(value: Unauthorized) -> Self {
                (value.msgSender, value.role)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Unauthorized {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    msgSender: tuple.0,
                    role: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for Unauthorized {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "Unauthorized(address,string)";
            const SELECTOR: [u8; 4] = [163u8, 91u8, 21u8, 11u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.msgSender,
                    ),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.role,
                    ),
                )
            }
        }
    };
    /**Constructor`.
```solidity
constructor(address _roleStore, address _dataStore, address _eventEmitter);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        pub _roleStore: alloy::sol_types::private::Address,
        pub _dataStore: alloy::sol_types::private::Address,
        pub _eventEmitter: alloy::sol_types::private::Address,
    }
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    (value._roleStore, value._dataStore, value._eventEmitter)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _roleStore: tuple.0,
                        _dataStore: tuple.1,
                        _eventEmitter: tuple.2,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._roleStore,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._dataStore,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._eventEmitter,
                    ),
                )
            }
        }
    };
    /**Function with signature `dataStore()` and selector `0x660d0d67`.
```solidity
function dataStore() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct dataStoreCall {}
    ///Container type for the return parameters of the [`dataStore()`](dataStoreCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct dataStoreReturn {
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<dataStoreCall> for UnderlyingRustTuple<'_> {
                fn from(value: dataStoreCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for dataStoreCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<dataStoreReturn> for UnderlyingRustTuple<'_> {
                fn from(value: dataStoreReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for dataStoreReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for dataStoreCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = dataStoreReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "dataStore()";
            const SELECTOR: [u8; 4] = [102u8, 13u8, 13u8, 103u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `eventEmitter()` and selector `0x9ff78c30`.
```solidity
function eventEmitter() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct eventEmitterCall {}
    ///Container type for the return parameters of the [`eventEmitter()`](eventEmitterCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct eventEmitterReturn {
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<eventEmitterCall> for UnderlyingRustTuple<'_> {
                fn from(value: eventEmitterCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for eventEmitterCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<eventEmitterReturn> for UnderlyingRustTuple<'_> {
                fn from(value: eventEmitterReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for eventEmitterReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for eventEmitterCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = eventEmitterReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "eventEmitter()";
            const SELECTOR: [u8; 4] = [159u8, 247u8, 140u8, 48u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `executeAdd(address,(address,address,address))` and selector `0x5ecd44e8`.
```solidity
function executeAdd(address account, LiquidityUtils.AddParams memory AddParams) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct executeAddCall {
        pub account: alloy::sol_types::private::Address,
        pub AddParams: <LiquidityUtils::AddParams as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`executeAdd(address,(address,address,address))`](executeAddCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct executeAddReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                LiquidityUtils::AddParams,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                <LiquidityUtils::AddParams as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<executeAddCall> for UnderlyingRustTuple<'_> {
                fn from(value: executeAddCall) -> Self {
                    (value.account, value.AddParams)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for executeAddCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        account: tuple.0,
                        AddParams: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<executeAddReturn> for UnderlyingRustTuple<'_> {
                fn from(value: executeAddReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for executeAddReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for executeAddCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                LiquidityUtils::AddParams,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = executeAddReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "executeAdd(address,(address,address,address))";
            const SELECTOR: [u8; 4] = [94u8, 205u8, 68u8, 232u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
                    ),
                    <LiquidityUtils::AddParams as alloy_sol_types::SolType>::tokenize(
                        &self.AddParams,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `executeRemove(address,(address,address,uint256,address))` and selector `0x7d237c99`.
```solidity
function executeRemove(address account, LiquidityUtils.RemoveParams memory removeParams) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct executeRemoveCall {
        pub account: alloy::sol_types::private::Address,
        pub removeParams: <LiquidityUtils::RemoveParams as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`executeRemove(address,(address,address,uint256,address))`](executeRemoveCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct executeRemoveReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                LiquidityUtils::RemoveParams,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                <LiquidityUtils::RemoveParams as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<executeRemoveCall> for UnderlyingRustTuple<'_> {
                fn from(value: executeRemoveCall) -> Self {
                    (value.account, value.removeParams)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for executeRemoveCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        account: tuple.0,
                        removeParams: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<executeRemoveReturn> for UnderlyingRustTuple<'_> {
                fn from(value: executeRemoveReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for executeRemoveReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for executeRemoveCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                LiquidityUtils::RemoveParams,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = executeRemoveReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "executeRemove(address,(address,address,uint256,address))";
            const SELECTOR: [u8; 4] = [125u8, 35u8, 124u8, 153u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
                    ),
                    <LiquidityUtils::RemoveParams as alloy_sol_types::SolType>::tokenize(
                        &self.removeParams,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `roleStore()` and selector `0x4a4a7b04`.
```solidity
function roleStore() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct roleStoreCall {}
    ///Container type for the return parameters of the [`roleStore()`](roleStoreCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct roleStoreReturn {
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<roleStoreCall> for UnderlyingRustTuple<'_> {
                fn from(value: roleStoreCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for roleStoreCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<roleStoreReturn> for UnderlyingRustTuple<'_> {
                fn from(value: roleStoreReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for roleStoreReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for roleStoreCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = roleStoreReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "roleStore()";
            const SELECTOR: [u8; 4] = [74u8, 74u8, 123u8, 4u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    ///Container for all the [`LiquidityHandler`](self) function calls.
    pub enum LiquidityHandlerCalls {
        dataStore(dataStoreCall),
        eventEmitter(eventEmitterCall),
        executeAdd(executeAddCall),
        executeRemove(executeRemoveCall),
        roleStore(roleStoreCall),
    }
    #[automatically_derived]
    impl LiquidityHandlerCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [74u8, 74u8, 123u8, 4u8],
            [94u8, 205u8, 68u8, 232u8],
            [102u8, 13u8, 13u8, 103u8],
            [125u8, 35u8, 124u8, 153u8],
            [159u8, 247u8, 140u8, 48u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for LiquidityHandlerCalls {
        const NAME: &'static str = "LiquidityHandlerCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 5usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::dataStore(_) => {
                    <dataStoreCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::eventEmitter(_) => {
                    <eventEmitterCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::executeAdd(_) => {
                    <executeAddCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::executeRemove(_) => {
                    <executeRemoveCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::roleStore(_) => {
                    <roleStoreCall as alloy_sol_types::SolCall>::SELECTOR
                }
            }
        }
        #[inline]
        fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
            Self::SELECTORS.get(i).copied()
        }
        #[inline]
        fn valid_selector(selector: [u8; 4]) -> bool {
            Self::SELECTORS.binary_search(&selector).is_ok()
        }
        #[inline]
        #[allow(unsafe_code, non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
                bool,
            ) -> alloy_sol_types::Result<LiquidityHandlerCalls>] = &[
                {
                    fn roleStore(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<LiquidityHandlerCalls> {
                        <roleStoreCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(LiquidityHandlerCalls::roleStore)
                    }
                    roleStore
                },
                {
                    fn executeAdd(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<LiquidityHandlerCalls> {
                        <executeAddCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(LiquidityHandlerCalls::executeAdd)
                    }
                    executeAdd
                },
                {
                    fn dataStore(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<LiquidityHandlerCalls> {
                        <dataStoreCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(LiquidityHandlerCalls::dataStore)
                    }
                    dataStore
                },
                {
                    fn executeRemove(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<LiquidityHandlerCalls> {
                        <executeRemoveCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(LiquidityHandlerCalls::executeRemove)
                    }
                    executeRemove
                },
                {
                    fn eventEmitter(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<LiquidityHandlerCalls> {
                        <eventEmitterCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(LiquidityHandlerCalls::eventEmitter)
                    }
                    eventEmitter
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            (unsafe { DECODE_SHIMS.get_unchecked(idx) })(data, validate)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::dataStore(inner) => {
                    <dataStoreCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::eventEmitter(inner) => {
                    <eventEmitterCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::executeAdd(inner) => {
                    <executeAddCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::executeRemove(inner) => {
                    <executeRemoveCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::roleStore(inner) => {
                    <roleStoreCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::dataStore(inner) => {
                    <dataStoreCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::eventEmitter(inner) => {
                    <eventEmitterCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::executeAdd(inner) => {
                    <executeAddCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::executeRemove(inner) => {
                    <executeRemoveCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::roleStore(inner) => {
                    <roleStoreCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`LiquidityHandler`](self) custom errors.
    pub enum LiquidityHandlerErrors {
        EmptyAddAmounts(EmptyAddAmounts),
        EmptyPool(EmptyPool),
        EmptyRemoveAmounts(EmptyRemoveAmounts),
        InsufficientUserBalance(InsufficientUserBalance),
        MathOverflowedMulDiv(MathOverflowedMulDiv),
        Reserve0Insufficient(Reserve0Insufficient),
        Reserve1Insufficient(Reserve1Insufficient),
        Unauthorized(Unauthorized),
    }
    #[automatically_derived]
    impl LiquidityHandlerErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [14u8, 121u8, 59u8, 175u8],
            [31u8, 193u8, 7u8, 193u8],
            [34u8, 123u8, 193u8, 83u8],
            [81u8, 134u8, 89u8, 17u8],
            [95u8, 80u8, 77u8, 32u8],
            [105u8, 119u8, 202u8, 12u8],
            [115u8, 87u8, 217u8, 31u8],
            [163u8, 91u8, 21u8, 11u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for LiquidityHandlerErrors {
        const NAME: &'static str = "LiquidityHandlerErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 8usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::EmptyAddAmounts(_) => {
                    <EmptyAddAmounts as alloy_sol_types::SolError>::SELECTOR
                }
                Self::EmptyPool(_) => <EmptyPool as alloy_sol_types::SolError>::SELECTOR,
                Self::EmptyRemoveAmounts(_) => {
                    <EmptyRemoveAmounts as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InsufficientUserBalance(_) => {
                    <InsufficientUserBalance as alloy_sol_types::SolError>::SELECTOR
                }
                Self::MathOverflowedMulDiv(_) => {
                    <MathOverflowedMulDiv as alloy_sol_types::SolError>::SELECTOR
                }
                Self::Reserve0Insufficient(_) => {
                    <Reserve0Insufficient as alloy_sol_types::SolError>::SELECTOR
                }
                Self::Reserve1Insufficient(_) => {
                    <Reserve1Insufficient as alloy_sol_types::SolError>::SELECTOR
                }
                Self::Unauthorized(_) => {
                    <Unauthorized as alloy_sol_types::SolError>::SELECTOR
                }
            }
        }
        #[inline]
        fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
            Self::SELECTORS.get(i).copied()
        }
        #[inline]
        fn valid_selector(selector: [u8; 4]) -> bool {
            Self::SELECTORS.binary_search(&selector).is_ok()
        }
        #[inline]
        #[allow(unsafe_code, non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
                bool,
            ) -> alloy_sol_types::Result<LiquidityHandlerErrors>] = &[
                {
                    fn Reserve1Insufficient(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<LiquidityHandlerErrors> {
                        <Reserve1Insufficient as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(LiquidityHandlerErrors::Reserve1Insufficient)
                    }
                    Reserve1Insufficient
                },
                {
                    fn Reserve0Insufficient(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<LiquidityHandlerErrors> {
                        <Reserve0Insufficient as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(LiquidityHandlerErrors::Reserve0Insufficient)
                    }
                    Reserve0Insufficient
                },
                {
                    fn MathOverflowedMulDiv(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<LiquidityHandlerErrors> {
                        <MathOverflowedMulDiv as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(LiquidityHandlerErrors::MathOverflowedMulDiv)
                    }
                    MathOverflowedMulDiv
                },
                {
                    fn EmptyRemoveAmounts(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<LiquidityHandlerErrors> {
                        <EmptyRemoveAmounts as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(LiquidityHandlerErrors::EmptyRemoveAmounts)
                    }
                    EmptyRemoveAmounts
                },
                {
                    fn InsufficientUserBalance(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<LiquidityHandlerErrors> {
                        <InsufficientUserBalance as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(LiquidityHandlerErrors::InsufficientUserBalance)
                    }
                    InsufficientUserBalance
                },
                {
                    fn EmptyAddAmounts(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<LiquidityHandlerErrors> {
                        <EmptyAddAmounts as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(LiquidityHandlerErrors::EmptyAddAmounts)
                    }
                    EmptyAddAmounts
                },
                {
                    fn EmptyPool(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<LiquidityHandlerErrors> {
                        <EmptyPool as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(LiquidityHandlerErrors::EmptyPool)
                    }
                    EmptyPool
                },
                {
                    fn Unauthorized(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<LiquidityHandlerErrors> {
                        <Unauthorized as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(LiquidityHandlerErrors::Unauthorized)
                    }
                    Unauthorized
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            (unsafe { DECODE_SHIMS.get_unchecked(idx) })(data, validate)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::EmptyAddAmounts(inner) => {
                    <EmptyAddAmounts as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::EmptyPool(inner) => {
                    <EmptyPool as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::EmptyRemoveAmounts(inner) => {
                    <EmptyRemoveAmounts as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InsufficientUserBalance(inner) => {
                    <InsufficientUserBalance as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::MathOverflowedMulDiv(inner) => {
                    <MathOverflowedMulDiv as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::Reserve0Insufficient(inner) => {
                    <Reserve0Insufficient as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::Reserve1Insufficient(inner) => {
                    <Reserve1Insufficient as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::Unauthorized(inner) => {
                    <Unauthorized as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::EmptyAddAmounts(inner) => {
                    <EmptyAddAmounts as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::EmptyPool(inner) => {
                    <EmptyPool as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::EmptyRemoveAmounts(inner) => {
                    <EmptyRemoveAmounts as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InsufficientUserBalance(inner) => {
                    <InsufficientUserBalance as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::MathOverflowedMulDiv(inner) => {
                    <MathOverflowedMulDiv as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::Reserve0Insufficient(inner) => {
                    <Reserve0Insufficient as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::Reserve1Insufficient(inner) => {
                    <Reserve1Insufficient as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::Unauthorized(inner) => {
                    <Unauthorized as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`LiquidityHandler`](self) contract instance.

See the [wrapper's documentation](`LiquidityHandlerInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> LiquidityHandlerInstance<T, P, N> {
        LiquidityHandlerInstance::<T, P, N>::new(address, provider)
    }
    /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
    #[inline]
    pub fn deploy<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        provider: P,
        _roleStore: alloy::sol_types::private::Address,
        _dataStore: alloy::sol_types::private::Address,
        _eventEmitter: alloy::sol_types::private::Address,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<LiquidityHandlerInstance<T, P, N>>,
    > {
        LiquidityHandlerInstance::<
            T,
            P,
            N,
        >::deploy(provider, _roleStore, _dataStore, _eventEmitter)
    }
    /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
    #[inline]
    pub fn deploy_builder<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        provider: P,
        _roleStore: alloy::sol_types::private::Address,
        _dataStore: alloy::sol_types::private::Address,
        _eventEmitter: alloy::sol_types::private::Address,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        LiquidityHandlerInstance::<
            T,
            P,
            N,
        >::deploy_builder(provider, _roleStore, _dataStore, _eventEmitter)
    }
    /**A [`LiquidityHandler`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`LiquidityHandler`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct LiquidityHandlerInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for LiquidityHandlerInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("LiquidityHandlerInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > LiquidityHandlerInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`LiquidityHandler`](self) contract instance.

See the [wrapper's documentation](`LiquidityHandlerInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            provider: P,
        ) -> Self {
            Self {
                address,
                provider,
                _network_transport: ::core::marker::PhantomData,
            }
        }
        /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
        #[inline]
        pub async fn deploy(
            provider: P,
            _roleStore: alloy::sol_types::private::Address,
            _dataStore: alloy::sol_types::private::Address,
            _eventEmitter: alloy::sol_types::private::Address,
        ) -> alloy_contract::Result<LiquidityHandlerInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(
                provider,
                _roleStore,
                _dataStore,
                _eventEmitter,
            );
            let contract_address = call_builder.deploy().await?;
            Ok(Self::new(contract_address, call_builder.provider))
        }
        /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
        #[inline]
        pub fn deploy_builder(
            provider: P,
            _roleStore: alloy::sol_types::private::Address,
            _dataStore: alloy::sol_types::private::Address,
            _eventEmitter: alloy::sol_types::private::Address,
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall {
                            _roleStore,
                            _dataStore,
                            _eventEmitter,
                        },
                    )[..],
                ]
                    .concat()
                    .into(),
            )
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<T, P: ::core::clone::Clone, N> LiquidityHandlerInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> LiquidityHandlerInstance<T, P, N> {
            LiquidityHandlerInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > LiquidityHandlerInstance<T, P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
        ///Creates a new call builder for the [`dataStore`] function.
        pub fn dataStore(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, dataStoreCall, N> {
            self.call_builder(&dataStoreCall {})
        }
        ///Creates a new call builder for the [`eventEmitter`] function.
        pub fn eventEmitter(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, eventEmitterCall, N> {
            self.call_builder(&eventEmitterCall {})
        }
        ///Creates a new call builder for the [`executeAdd`] function.
        pub fn executeAdd(
            &self,
            account: alloy::sol_types::private::Address,
            AddParams: <LiquidityUtils::AddParams as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, executeAddCall, N> {
            self.call_builder(
                &executeAddCall {
                    account,
                    AddParams,
                },
            )
        }
        ///Creates a new call builder for the [`executeRemove`] function.
        pub fn executeRemove(
            &self,
            account: alloy::sol_types::private::Address,
            removeParams: <LiquidityUtils::RemoveParams as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, executeRemoveCall, N> {
            self.call_builder(
                &executeRemoveCall {
                    account,
                    removeParams,
                },
            )
        }
        ///Creates a new call builder for the [`roleStore`] function.
        pub fn roleStore(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, roleStoreCall, N> {
            self.call_builder(&roleStoreCall {})
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > LiquidityHandlerInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
