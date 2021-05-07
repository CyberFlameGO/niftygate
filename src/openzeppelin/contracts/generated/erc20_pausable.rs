#[allow(dead_code)]
pub mod erc20_pausable {
  # [rustfmt :: skip] use ethcontract as ethcontract ;
  #[doc = "Generated by `ethcontract`"]
  #[derive(Clone)]
  pub struct Contract {
    methods: Methods,
  }
  impl Contract {
    #[doc = r" Retrieves the truffle artifact used to generate the type safe"]
    #[doc = r" API for this contract."]
    pub fn artifact() -> &'static self::ethcontract::Artifact {
      use self::ethcontract::private::lazy_static;
      use self::ethcontract::Artifact;
      lazy_static! {
        pub static ref ARTIFACT: Artifact = {
          # [allow (unused_mut)] let mut artifact = Artifact :: from_json ("{\n  \"_format\": \"hh-sol-artifact-1\",\n  \"contractName\": \"ERC20Pausable\",\n  \"sourceName\": \"contracts/token/ERC20/extensions/ERC20Pausable.sol\",\n  \"abi\": [\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"owner\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"spender\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"value\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"Approval\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": false,\n          \"internalType\": \"address\",\n          \"name\": \"account\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"Paused\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"from\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"to\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"value\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"Transfer\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": false,\n          \"internalType\": \"address\",\n          \"name\": \"account\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"Unpaused\",\n      \"type\": \"event\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"owner\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"address\",\n          \"name\": \"spender\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"allowance\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"spender\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"amount\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"approve\",\n      \"outputs\": [\n        {\n          \"internalType\": \"bool\",\n          \"name\": \"\",\n          \"type\": \"bool\"\n        }\n      ],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"account\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"balanceOf\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"decimals\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint8\",\n          \"name\": \"\",\n          \"type\": \"uint8\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"spender\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"subtractedValue\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"decreaseAllowance\",\n      \"outputs\": [\n        {\n          \"internalType\": \"bool\",\n          \"name\": \"\",\n          \"type\": \"bool\"\n        }\n      ],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"spender\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"addedValue\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"increaseAllowance\",\n      \"outputs\": [\n        {\n          \"internalType\": \"bool\",\n          \"name\": \"\",\n          \"type\": \"bool\"\n        }\n      ],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"name\",\n      \"outputs\": [\n        {\n          \"internalType\": \"string\",\n          \"name\": \"\",\n          \"type\": \"string\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"paused\",\n      \"outputs\": [\n        {\n          \"internalType\": \"bool\",\n          \"name\": \"\",\n          \"type\": \"bool\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"symbol\",\n      \"outputs\": [\n        {\n          \"internalType\": \"string\",\n          \"name\": \"\",\n          \"type\": \"string\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"totalSupply\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"recipient\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"amount\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"transfer\",\n      \"outputs\": [\n        {\n          \"internalType\": \"bool\",\n          \"name\": \"\",\n          \"type\": \"bool\"\n        }\n      ],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"sender\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"address\",\n          \"name\": \"recipient\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"amount\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"transferFrom\",\n      \"outputs\": [\n        {\n          \"internalType\": \"bool\",\n          \"name\": \"\",\n          \"type\": \"bool\"\n        }\n      ],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    }\n  ],\n  \"bytecode\": \"0x\",\n  \"deployedBytecode\": \"0x\",\n  \"linkReferences\": {},\n  \"deployedLinkReferences\": {}\n}\n") . expect ("valid artifact JSON") ;
          artifact
        };
      }
      &ARTIFACT
    }
    #[doc = r" Creates a new contract instance with the specified `web3`"]
    #[doc = r" provider at the given `Address`."]
    #[doc = r""]
    #[doc = r" Note that this does not verify that a contract with a maching"]
    #[doc = r" `Abi` is actually deployed at the given address."]
    pub fn at<F, T>(
      web3: &self::ethcontract::web3::api::Web3<T>,
      address: self::ethcontract::Address,
    ) -> Self
    where
      F: std::future::Future<
          Output = Result<self::ethcontract::json::Value, self::ethcontract::web3::Error>,
        > + Send
        + Unpin
        + 'static,
      T: self::ethcontract::web3::Transport<Out = F> + Send + Sync + 'static,
    {
      Contract::with_deployment_info(web3, address, None)
    }
    #[doc = r" Creates a new contract instance with the specified `web3` provider with"]
    #[doc = r" the given `Abi` at the given `Address` and an optional transaction hash."]
    #[doc = r" This hash is used to retrieve contract related information such as the"]
    #[doc = r" creation block (which is useful for fetching all historic events)."]
    #[doc = r""]
    #[doc = r" Note that this does not verify that a contract with a matching `Abi` is"]
    #[doc = r" actually deployed at the given address nor that the transaction hash,"]
    #[doc = r" when provided, is actually for this contract deployment."]
    pub fn with_deployment_info<F, T>(
      web3: &self::ethcontract::web3::api::Web3<T>,
      address: self::ethcontract::Address,
      deployment_information: Option<ethcontract::common::DeploymentInformation>,
    ) -> Self
    where
      F: std::future::Future<
          Output = Result<self::ethcontract::json::Value, self::ethcontract::web3::Error>,
        > + Send
        + Unpin
        + 'static,
      T: self::ethcontract::web3::Transport<Out = F> + Send + Sync + 'static,
    {
      use self::ethcontract::transport::DynTransport;
      use self::ethcontract::web3::api::Web3;
      use self::ethcontract::Instance;
      let transport = DynTransport::new(web3.transport().clone());
      let web3 = Web3::new(transport);
      let abi = Self::artifact().abi.clone();
      let instance = Instance::with_deployment_info(web3, abi, address, deployment_information);
      Contract::from_raw(instance)
    }
    #[doc = r" Creates a contract from a raw instance."]
    fn from_raw(instance: self::ethcontract::dyns::DynInstance) -> Self {
      let methods = Methods { instance };
      Contract { methods }
    }
    #[doc = r" Returns the contract address being used by this instance."]
    pub fn address(&self) -> self::ethcontract::Address {
      self.raw_instance().address()
    }
    #[doc = r" Returns the deployment information of the contract"]
    #[doc = r" if it is known, `None` otherwise."]
    pub fn deployment_information(&self) -> Option<ethcontract::common::DeploymentInformation> {
      self.raw_instance().deployment_information()
    }
    #[doc = r" Returns a reference to the default method options used by this"]
    #[doc = r" contract."]
    pub fn defaults(&self) -> &self::ethcontract::contract::MethodDefaults {
      &self.raw_instance().defaults
    }
    #[doc = r" Returns a mutable reference to the default method options used"]
    #[doc = r" by this contract."]
    pub fn defaults_mut(&mut self) -> &mut self::ethcontract::contract::MethodDefaults {
      &mut self.raw_instance_mut().defaults
    }
    #[doc = r" Returns a reference to the raw runtime instance used by this"]
    #[doc = r" contract."]
    pub fn raw_instance(&self) -> &self::ethcontract::dyns::DynInstance {
      &self.methods.instance
    }
    #[doc = r" Returns a mutable reference to the raw runtime instance used by"]
    #[doc = r" this contract."]
    fn raw_instance_mut(&mut self) -> &mut self::ethcontract::dyns::DynInstance {
      &mut self.methods.instance
    }
  }
  impl std::fmt::Debug for Contract {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      f.debug_tuple(stringify!(ERC20Pausable))
        .field(&self.address())
        .finish()
    }
  }
  impl Contract {
    #[doc = r" Retrives a reference to type containing all the generated"]
    #[doc = r" contract methods. This can be used for methods where the name"]
    #[doc = r" would collide with a common method (like `at` or `deployed`)."]
    pub fn methods(&self) -> &Methods {
      &self.methods
    }
  }
  #[doc = r" Type containing all contract methods for generated contract type."]
  #[derive(Clone)]
  pub struct Methods {
    instance: self::ethcontract::dyns::DynInstance,
  }
  #[allow(clippy::too_many_arguments, clippy::type_complexity)]
  impl Methods {
    #[doc = "Generated by `ethcontract`"]
    pub fn symbol(&self) -> self::ethcontract::dyns::DynMethodBuilder<String> {
      self
        .instance
        .method([149, 216, 155, 65], ())
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn transfer(
      &self,
      recipient: self::ethcontract::Address,
      amount: self::ethcontract::U256,
    ) -> self::ethcontract::dyns::DynMethodBuilder<bool> {
      self
        .instance
        .method([169, 5, 156, 187], (recipient, amount))
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn transfer_from(
      &self,
      sender: self::ethcontract::Address,
      recipient: self::ethcontract::Address,
      amount: self::ethcontract::U256,
    ) -> self::ethcontract::dyns::DynMethodBuilder<bool> {
      self
        .instance
        .method([35, 184, 114, 221], (sender, recipient, amount))
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn paused(&self) -> self::ethcontract::dyns::DynMethodBuilder<bool> {
      self
        .instance
        .method([92, 151, 90, 187], ())
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn increase_allowance(
      &self,
      spender: self::ethcontract::Address,
      added_value: self::ethcontract::U256,
    ) -> self::ethcontract::dyns::DynMethodBuilder<bool> {
      self
        .instance
        .method([57, 80, 147, 81], (spender, added_value))
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn decrease_allowance(
      &self,
      spender: self::ethcontract::Address,
      subtracted_value: self::ethcontract::U256,
    ) -> self::ethcontract::dyns::DynMethodBuilder<bool> {
      self
        .instance
        .method([164, 87, 194, 215], (spender, subtracted_value))
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn total_supply(
      &self,
    ) -> self::ethcontract::dyns::DynMethodBuilder<self::ethcontract::U256> {
      self
        .instance
        .method([24, 22, 13, 221], ())
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn name(&self) -> self::ethcontract::dyns::DynMethodBuilder<String> {
      self
        .instance
        .method([6, 253, 222, 3], ())
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn allowance(
      &self,
      owner: self::ethcontract::Address,
      spender: self::ethcontract::Address,
    ) -> self::ethcontract::dyns::DynMethodBuilder<self::ethcontract::U256> {
      self
        .instance
        .method([221, 98, 237, 62], (owner, spender))
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn balance_of(
      &self,
      account: self::ethcontract::Address,
    ) -> self::ethcontract::dyns::DynMethodBuilder<self::ethcontract::U256> {
      self
        .instance
        .method([112, 160, 130, 49], (account,))
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn approve(
      &self,
      spender: self::ethcontract::Address,
      amount: self::ethcontract::U256,
    ) -> self::ethcontract::dyns::DynMethodBuilder<bool> {
      self
        .instance
        .method([9, 94, 167, 179], (spender, amount))
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn decimals(&self) -> self::ethcontract::dyns::DynMethodBuilder<u8> {
      self
        .instance
        .method([49, 60, 229, 103], ())
        .expect("generated call")
    }
  }
  impl std::ops::Deref for Contract {
    type Target = Methods;
    fn deref(&self) -> &Self::Target {
      &self.methods
    }
  }
  #[doc = r" Module containing all generated data models for this contract's"]
  #[doc = r" events."]
  pub mod event_data {
    use super::ethcontract;
    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    pub struct Paused {
      pub account: self::ethcontract::Address,
    }
    impl Paused {
      #[doc = r" Retrieves the signature for the event this data corresponds to."]
      #[doc = r" This signature is the Keccak-256 hash of the ABI signature of"]
      #[doc = r" this event."]
      pub fn signature() -> self::ethcontract::H256 {
        self::ethcontract::H256([
          98, 231, 140, 234, 1, 190, 227, 32, 205, 78, 66, 2, 112, 181, 234, 116, 0, 13, 17, 176,
          201, 247, 71, 84, 235, 219, 252, 84, 75, 5, 162, 88,
        ])
      }
      #[doc = r" Retrieves the ABI signature for the event this data corresponds"]
      #[doc = r" to. For this event the value should always be:"]
      #[doc = r""]
      #[doc = "`Paused(address)`"]
      pub fn abi_signature() -> &'static str {
        "Paused(address)"
      }
    }
    impl self::ethcontract::web3::contract::tokens::Detokenize for Paused {
      fn from_tokens(
        tokens: Vec<self::ethcontract::common::abi::Token>,
      ) -> Result<Self, self::ethcontract::web3::contract::Error> {
        if tokens.len() != 1 {
          return Err(self::ethcontract::web3::contract::Error::InvalidOutputType(
            format!("Expected {} tokens, got {}: {:?}", 1, tokens.len(), tokens),
          ));
        }
        #[allow(unused_mut)]
        let mut tokens = tokens.into_iter();
        let account = < self :: ethcontract :: Address as self :: ethcontract :: web3 :: contract :: tokens :: Tokenizable > :: from_token (tokens . next () . unwrap ()) ? ;
        Ok(Paused { account })
      }
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    pub struct Unpaused {
      pub account: self::ethcontract::Address,
    }
    impl Unpaused {
      #[doc = r" Retrieves the signature for the event this data corresponds to."]
      #[doc = r" This signature is the Keccak-256 hash of the ABI signature of"]
      #[doc = r" this event."]
      pub fn signature() -> self::ethcontract::H256 {
        self::ethcontract::H256([
          93, 185, 238, 10, 73, 91, 242, 230, 255, 156, 145, 167, 131, 76, 27, 164, 253, 210, 68,
          165, 232, 170, 78, 83, 123, 211, 138, 234, 228, 176, 115, 170,
        ])
      }
      #[doc = r" Retrieves the ABI signature for the event this data corresponds"]
      #[doc = r" to. For this event the value should always be:"]
      #[doc = r""]
      #[doc = "`Unpaused(address)`"]
      pub fn abi_signature() -> &'static str {
        "Unpaused(address)"
      }
    }
    impl self::ethcontract::web3::contract::tokens::Detokenize for Unpaused {
      fn from_tokens(
        tokens: Vec<self::ethcontract::common::abi::Token>,
      ) -> Result<Self, self::ethcontract::web3::contract::Error> {
        if tokens.len() != 1 {
          return Err(self::ethcontract::web3::contract::Error::InvalidOutputType(
            format!("Expected {} tokens, got {}: {:?}", 1, tokens.len(), tokens),
          ));
        }
        #[allow(unused_mut)]
        let mut tokens = tokens.into_iter();
        let account = < self :: ethcontract :: Address as self :: ethcontract :: web3 :: contract :: tokens :: Tokenizable > :: from_token (tokens . next () . unwrap ()) ? ;
        Ok(Unpaused { account })
      }
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    pub struct Transfer {
      pub from: self::ethcontract::Address,
      pub to: self::ethcontract::Address,
      pub value: self::ethcontract::U256,
    }
    impl Transfer {
      #[doc = r" Retrieves the signature for the event this data corresponds to."]
      #[doc = r" This signature is the Keccak-256 hash of the ABI signature of"]
      #[doc = r" this event."]
      pub fn signature() -> self::ethcontract::H256 {
        self::ethcontract::H256([
          221, 242, 82, 173, 27, 226, 200, 155, 105, 194, 176, 104, 252, 55, 141, 170, 149, 43,
          167, 241, 99, 196, 161, 22, 40, 245, 90, 77, 245, 35, 179, 239,
        ])
      }
      #[doc = r" Retrieves the ABI signature for the event this data corresponds"]
      #[doc = r" to. For this event the value should always be:"]
      #[doc = r""]
      #[doc = "`Transfer(address,address,uint256)`"]
      pub fn abi_signature() -> &'static str {
        "Transfer(address,address,uint256)"
      }
    }
    impl self::ethcontract::web3::contract::tokens::Detokenize for Transfer {
      fn from_tokens(
        tokens: Vec<self::ethcontract::common::abi::Token>,
      ) -> Result<Self, self::ethcontract::web3::contract::Error> {
        if tokens.len() != 3 {
          return Err(self::ethcontract::web3::contract::Error::InvalidOutputType(
            format!("Expected {} tokens, got {}: {:?}", 3, tokens.len(), tokens),
          ));
        }
        #[allow(unused_mut)]
        let mut tokens = tokens.into_iter();
        let from = < self :: ethcontract :: Address as self :: ethcontract :: web3 :: contract :: tokens :: Tokenizable > :: from_token (tokens . next () . unwrap ()) ? ;
        let to = < self :: ethcontract :: Address as self :: ethcontract :: web3 :: contract :: tokens :: Tokenizable > :: from_token (tokens . next () . unwrap ()) ? ;
        let value = < self :: ethcontract :: U256 as self :: ethcontract :: web3 :: contract :: tokens :: Tokenizable > :: from_token (tokens . next () . unwrap ()) ? ;
        Ok(Transfer { from, to, value })
      }
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    pub struct Approval {
      pub owner: self::ethcontract::Address,
      pub spender: self::ethcontract::Address,
      pub value: self::ethcontract::U256,
    }
    impl Approval {
      #[doc = r" Retrieves the signature for the event this data corresponds to."]
      #[doc = r" This signature is the Keccak-256 hash of the ABI signature of"]
      #[doc = r" this event."]
      pub fn signature() -> self::ethcontract::H256 {
        self::ethcontract::H256([
          140, 91, 225, 229, 235, 236, 125, 91, 209, 79, 113, 66, 125, 30, 132, 243, 221, 3, 20,
          192, 247, 178, 41, 30, 91, 32, 10, 200, 199, 195, 185, 37,
        ])
      }
      #[doc = r" Retrieves the ABI signature for the event this data corresponds"]
      #[doc = r" to. For this event the value should always be:"]
      #[doc = r""]
      #[doc = "`Approval(address,address,uint256)`"]
      pub fn abi_signature() -> &'static str {
        "Approval(address,address,uint256)"
      }
    }
    impl self::ethcontract::web3::contract::tokens::Detokenize for Approval {
      fn from_tokens(
        tokens: Vec<self::ethcontract::common::abi::Token>,
      ) -> Result<Self, self::ethcontract::web3::contract::Error> {
        if tokens.len() != 3 {
          return Err(self::ethcontract::web3::contract::Error::InvalidOutputType(
            format!("Expected {} tokens, got {}: {:?}", 3, tokens.len(), tokens),
          ));
        }
        #[allow(unused_mut)]
        let mut tokens = tokens.into_iter();
        let owner = < self :: ethcontract :: Address as self :: ethcontract :: web3 :: contract :: tokens :: Tokenizable > :: from_token (tokens . next () . unwrap ()) ? ;
        let spender = < self :: ethcontract :: Address as self :: ethcontract :: web3 :: contract :: tokens :: Tokenizable > :: from_token (tokens . next () . unwrap ()) ? ;
        let value = < self :: ethcontract :: U256 as self :: ethcontract :: web3 :: contract :: tokens :: Tokenizable > :: from_token (tokens . next () . unwrap ()) ? ;
        Ok(Approval {
          owner,
          spender,
          value,
        })
      }
    }
  }
  impl Contract {
    #[doc = r" Retrieves a handle to a type containing for creating event"]
    #[doc = r" streams for all the contract events."]
    pub fn events(&self) -> Events<'_> {
      Events {
        instance: self.raw_instance(),
      }
    }
  }
  pub struct Events<'a> {
    instance: &'a self::ethcontract::dyns::DynInstance,
  }
  impl Events<'_> {
    #[doc = r" Generated by `ethcontract`."]
    pub fn paused(&self) -> self::event_builders::PausedBuilder {
      self::event_builders::PausedBuilder(
        self
          .instance
          .event(self::ethcontract::H256([
            98, 231, 140, 234, 1, 190, 227, 32, 205, 78, 66, 2, 112, 181, 234, 116, 0, 13, 17, 176,
            201, 247, 71, 84, 235, 219, 252, 84, 75, 5, 162, 88,
          ]))
          .expect("generated event filter"),
      )
    }
    #[doc = r" Generated by `ethcontract`."]
    pub fn unpaused(&self) -> self::event_builders::UnpausedBuilder {
      self::event_builders::UnpausedBuilder(
        self
          .instance
          .event(self::ethcontract::H256([
            93, 185, 238, 10, 73, 91, 242, 230, 255, 156, 145, 167, 131, 76, 27, 164, 253, 210, 68,
            165, 232, 170, 78, 83, 123, 211, 138, 234, 228, 176, 115, 170,
          ]))
          .expect("generated event filter"),
      )
    }
    #[doc = r" Generated by `ethcontract`."]
    pub fn transfer(&self) -> self::event_builders::TransferBuilder {
      self::event_builders::TransferBuilder(
        self
          .instance
          .event(self::ethcontract::H256([
            221, 242, 82, 173, 27, 226, 200, 155, 105, 194, 176, 104, 252, 55, 141, 170, 149, 43,
            167, 241, 99, 196, 161, 22, 40, 245, 90, 77, 245, 35, 179, 239,
          ]))
          .expect("generated event filter"),
      )
    }
    #[doc = r" Generated by `ethcontract`."]
    pub fn approval(&self) -> self::event_builders::ApprovalBuilder {
      self::event_builders::ApprovalBuilder(
        self
          .instance
          .event(self::ethcontract::H256([
            140, 91, 225, 229, 235, 236, 125, 91, 209, 79, 113, 66, 125, 30, 132, 243, 221, 3, 20,
            192, 247, 178, 41, 30, 91, 32, 10, 200, 199, 195, 185, 37,
          ]))
          .expect("generated event filter"),
      )
    }
  }
  #[doc = r" Module containing the generated event stream builders with type safe"]
  #[doc = r" filter methods for this contract's events."]
  pub mod event_builders {
    use super::ethcontract;
    use super::event_data;
    #[doc = "A builder for creating a filtered stream of `Paused` events."]
    pub struct PausedBuilder(
      #[doc = r" The inner event builder."]
      pub  self::ethcontract::dyns::DynEventBuilder<self::event_data::Paused>,
    );
    impl PausedBuilder {
      #[doc = r" Sets the starting block from which to stream logs for."]
      #[doc = r""]
      #[doc = r" If left unset defaults to the latest block."]
      #[allow(clippy::wrong_self_convention)]
      pub fn from_block(mut self, block: self::ethcontract::BlockNumber) -> Self {
        self.0 = (self.0).from_block(block);
        self
      }
      #[doc = r" Sets the last block from which to stream logs for."]
      #[doc = r""]
      #[doc = r" If left unset defaults to the streaming until the end of days."]
      #[allow(clippy::wrong_self_convention)]
      pub fn to_block(mut self, block: self::ethcontract::BlockNumber) -> Self {
        self.0 = (self.0).to_block(block);
        self
      }
      #[doc = r" Limit the number of events that can be retrieved by this filter."]
      #[doc = r""]
      #[doc = r" Note that this parameter is non-standard."]
      pub fn limit(mut self, value: usize) -> Self {
        self.0 = (self.0).limit(value);
        self
      }
      #[doc = r" The polling interval. This is used as the interval between"]
      #[doc = r" consecutive `eth_getFilterChanges` calls to get filter updates."]
      pub fn poll_interval(mut self, value: std::time::Duration) -> Self {
        self.0 = (self.0).poll_interval(value);
        self
      }
      #[doc = r" Returns a future that resolves with a collection of all existing"]
      #[doc = r" logs matching the builder parameters."]
      pub async fn query(
        self,
      ) -> std::result::Result<
        std::vec::Vec<self::ethcontract::Event<self::event_data::Paused>>,
        self::ethcontract::errors::EventError,
      > {
        (self.0).query().await
      }
      #[doc = r" Creates an event stream from the current event builder."]
      pub fn stream(
        self,
      ) -> impl self::ethcontract::futures::stream::Stream<
        Item = std::result::Result<
          self::ethcontract::StreamEvent<self::event_data::Paused>,
          self::ethcontract::errors::EventError,
        >,
      > {
        (self.0).stream()
      }
    }
    #[doc = "A builder for creating a filtered stream of `Unpaused` events."]
    pub struct UnpausedBuilder(
      #[doc = r" The inner event builder."]
      pub  self::ethcontract::dyns::DynEventBuilder<self::event_data::Unpaused>,
    );
    impl UnpausedBuilder {
      #[doc = r" Sets the starting block from which to stream logs for."]
      #[doc = r""]
      #[doc = r" If left unset defaults to the latest block."]
      #[allow(clippy::wrong_self_convention)]
      pub fn from_block(mut self, block: self::ethcontract::BlockNumber) -> Self {
        self.0 = (self.0).from_block(block);
        self
      }
      #[doc = r" Sets the last block from which to stream logs for."]
      #[doc = r""]
      #[doc = r" If left unset defaults to the streaming until the end of days."]
      #[allow(clippy::wrong_self_convention)]
      pub fn to_block(mut self, block: self::ethcontract::BlockNumber) -> Self {
        self.0 = (self.0).to_block(block);
        self
      }
      #[doc = r" Limit the number of events that can be retrieved by this filter."]
      #[doc = r""]
      #[doc = r" Note that this parameter is non-standard."]
      pub fn limit(mut self, value: usize) -> Self {
        self.0 = (self.0).limit(value);
        self
      }
      #[doc = r" The polling interval. This is used as the interval between"]
      #[doc = r" consecutive `eth_getFilterChanges` calls to get filter updates."]
      pub fn poll_interval(mut self, value: std::time::Duration) -> Self {
        self.0 = (self.0).poll_interval(value);
        self
      }
      #[doc = r" Returns a future that resolves with a collection of all existing"]
      #[doc = r" logs matching the builder parameters."]
      pub async fn query(
        self,
      ) -> std::result::Result<
        std::vec::Vec<self::ethcontract::Event<self::event_data::Unpaused>>,
        self::ethcontract::errors::EventError,
      > {
        (self.0).query().await
      }
      #[doc = r" Creates an event stream from the current event builder."]
      pub fn stream(
        self,
      ) -> impl self::ethcontract::futures::stream::Stream<
        Item = std::result::Result<
          self::ethcontract::StreamEvent<self::event_data::Unpaused>,
          self::ethcontract::errors::EventError,
        >,
      > {
        (self.0).stream()
      }
    }
    #[doc = "A builder for creating a filtered stream of `Transfer` events."]
    pub struct TransferBuilder(
      #[doc = r" The inner event builder."]
      pub  self::ethcontract::dyns::DynEventBuilder<self::event_data::Transfer>,
    );
    impl TransferBuilder {
      #[doc = r" Sets the starting block from which to stream logs for."]
      #[doc = r""]
      #[doc = r" If left unset defaults to the latest block."]
      #[allow(clippy::wrong_self_convention)]
      pub fn from_block(mut self, block: self::ethcontract::BlockNumber) -> Self {
        self.0 = (self.0).from_block(block);
        self
      }
      #[doc = r" Sets the last block from which to stream logs for."]
      #[doc = r""]
      #[doc = r" If left unset defaults to the streaming until the end of days."]
      #[allow(clippy::wrong_self_convention)]
      pub fn to_block(mut self, block: self::ethcontract::BlockNumber) -> Self {
        self.0 = (self.0).to_block(block);
        self
      }
      #[doc = r" Limit the number of events that can be retrieved by this filter."]
      #[doc = r""]
      #[doc = r" Note that this parameter is non-standard."]
      pub fn limit(mut self, value: usize) -> Self {
        self.0 = (self.0).limit(value);
        self
      }
      #[doc = r" The polling interval. This is used as the interval between"]
      #[doc = r" consecutive `eth_getFilterChanges` calls to get filter updates."]
      pub fn poll_interval(mut self, value: std::time::Duration) -> Self {
        self.0 = (self.0).poll_interval(value);
        self
      }
      #[doc = "Adds a filter for the from event parameter."]
      pub fn from(mut self, topic: self::ethcontract::Topic<self::ethcontract::Address>) -> Self {
        self.0 = (self.0).topic0(topic);
        self
      }
      #[doc = "Adds a filter for the to event parameter."]
      pub fn to(mut self, topic: self::ethcontract::Topic<self::ethcontract::Address>) -> Self {
        self.0 = (self.0).topic1(topic);
        self
      }
      #[doc = r" Returns a future that resolves with a collection of all existing"]
      #[doc = r" logs matching the builder parameters."]
      pub async fn query(
        self,
      ) -> std::result::Result<
        std::vec::Vec<self::ethcontract::Event<self::event_data::Transfer>>,
        self::ethcontract::errors::EventError,
      > {
        (self.0).query().await
      }
      #[doc = r" Creates an event stream from the current event builder."]
      pub fn stream(
        self,
      ) -> impl self::ethcontract::futures::stream::Stream<
        Item = std::result::Result<
          self::ethcontract::StreamEvent<self::event_data::Transfer>,
          self::ethcontract::errors::EventError,
        >,
      > {
        (self.0).stream()
      }
    }
    #[doc = "A builder for creating a filtered stream of `Approval` events."]
    pub struct ApprovalBuilder(
      #[doc = r" The inner event builder."]
      pub  self::ethcontract::dyns::DynEventBuilder<self::event_data::Approval>,
    );
    impl ApprovalBuilder {
      #[doc = r" Sets the starting block from which to stream logs for."]
      #[doc = r""]
      #[doc = r" If left unset defaults to the latest block."]
      #[allow(clippy::wrong_self_convention)]
      pub fn from_block(mut self, block: self::ethcontract::BlockNumber) -> Self {
        self.0 = (self.0).from_block(block);
        self
      }
      #[doc = r" Sets the last block from which to stream logs for."]
      #[doc = r""]
      #[doc = r" If left unset defaults to the streaming until the end of days."]
      #[allow(clippy::wrong_self_convention)]
      pub fn to_block(mut self, block: self::ethcontract::BlockNumber) -> Self {
        self.0 = (self.0).to_block(block);
        self
      }
      #[doc = r" Limit the number of events that can be retrieved by this filter."]
      #[doc = r""]
      #[doc = r" Note that this parameter is non-standard."]
      pub fn limit(mut self, value: usize) -> Self {
        self.0 = (self.0).limit(value);
        self
      }
      #[doc = r" The polling interval. This is used as the interval between"]
      #[doc = r" consecutive `eth_getFilterChanges` calls to get filter updates."]
      pub fn poll_interval(mut self, value: std::time::Duration) -> Self {
        self.0 = (self.0).poll_interval(value);
        self
      }
      #[doc = "Adds a filter for the owner event parameter."]
      pub fn owner(mut self, topic: self::ethcontract::Topic<self::ethcontract::Address>) -> Self {
        self.0 = (self.0).topic0(topic);
        self
      }
      #[doc = "Adds a filter for the spender event parameter."]
      pub fn spender(
        mut self,
        topic: self::ethcontract::Topic<self::ethcontract::Address>,
      ) -> Self {
        self.0 = (self.0).topic1(topic);
        self
      }
      #[doc = r" Returns a future that resolves with a collection of all existing"]
      #[doc = r" logs matching the builder parameters."]
      pub async fn query(
        self,
      ) -> std::result::Result<
        std::vec::Vec<self::ethcontract::Event<self::event_data::Approval>>,
        self::ethcontract::errors::EventError,
      > {
        (self.0).query().await
      }
      #[doc = r" Creates an event stream from the current event builder."]
      pub fn stream(
        self,
      ) -> impl self::ethcontract::futures::stream::Stream<
        Item = std::result::Result<
          self::ethcontract::StreamEvent<self::event_data::Approval>,
          self::ethcontract::errors::EventError,
        >,
      > {
        (self.0).stream()
      }
    }
  }
  impl Contract {
    #[doc = r" Returns a log stream with all events."]
    pub fn all_events(&self) -> self::ethcontract::dyns::DynAllEventsBuilder<Event> {
      self::ethcontract::dyns::DynAllEventsBuilder::new(
        self.raw_instance().web3(),
        self.address(),
        self.deployment_information(),
      )
    }
  }
  #[doc = r" A contract event."]
  #[derive(Clone, Debug, Eq, PartialEq)]
  pub enum Event {
    Approval(self::event_data::Approval),
    Paused(self::event_data::Paused),
    Transfer(self::event_data::Transfer),
    Unpaused(self::event_data::Unpaused),
  }
  impl self::ethcontract::contract::ParseLog for Event {
    fn parse_log(
      log: self::ethcontract::RawLog,
    ) -> Result<Self, self::ethcontract::errors::ExecutionError> {
      let standard_event = log . topics . get (0) . copied () . map (| topic | match topic { self :: ethcontract :: H256 ([140 , 91 , 225 , 229 , 235 , 236 , 125 , 91 , 209 , 79 , 113 , 66 , 125 , 30 , 132 , 243 , 221 , 3 , 20 , 192 , 247 , 178 , 41 , 30 , 91 , 32 , 10 , 200 , 199 , 195 , 185 , 37]) => Ok (Event :: Approval (log . clone () . decode (& Contract :: artifact () . abi . event ("Approval") . expect ("generated event decode")) ?)) , self :: ethcontract :: H256 ([98 , 231 , 140 , 234 , 1 , 190 , 227 , 32 , 205 , 78 , 66 , 2 , 112 , 181 , 234 , 116 , 0 , 13 , 17 , 176 , 201 , 247 , 71 , 84 , 235 , 219 , 252 , 84 , 75 , 5 , 162 , 88]) => Ok (Event :: Paused (log . clone () . decode (& Contract :: artifact () . abi . event ("Paused") . expect ("generated event decode")) ?)) , self :: ethcontract :: H256 ([221 , 242 , 82 , 173 , 27 , 226 , 200 , 155 , 105 , 194 , 176 , 104 , 252 , 55 , 141 , 170 , 149 , 43 , 167 , 241 , 99 , 196 , 161 , 22 , 40 , 245 , 90 , 77 , 245 , 35 , 179 , 239]) => Ok (Event :: Transfer (log . clone () . decode (& Contract :: artifact () . abi . event ("Transfer") . expect ("generated event decode")) ?)) , self :: ethcontract :: H256 ([93 , 185 , 238 , 10 , 73 , 91 , 242 , 230 , 255 , 156 , 145 , 167 , 131 , 76 , 27 , 164 , 253 , 210 , 68 , 165 , 232 , 170 , 78 , 83 , 123 , 211 , 138 , 234 , 228 , 176 , 115 , 170]) => Ok (Event :: Unpaused (log . clone () . decode (& Contract :: artifact () . abi . event ("Unpaused") . expect ("generated event decode")) ?)) , _ => Err (self :: ethcontract :: errors :: ExecutionError :: from (self :: ethcontract :: common :: abi :: Error :: InvalidData)) , }) ;
      if let Some(Ok(data)) = standard_event {
        return Ok(data);
      }
      Err(self::ethcontract::errors::ExecutionError::from(
        self::ethcontract::common::abi::Error::InvalidData,
      ))
    }
  }
}
pub use self::erc20_pausable::Contract as ERC20Pausable;
