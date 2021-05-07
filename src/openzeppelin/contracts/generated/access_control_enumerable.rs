#[allow(dead_code)]
pub mod access_control_enumerable {
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
          # [allow (unused_mut)] let mut artifact = Artifact :: from_json ("{\n  \"_format\": \"hh-sol-artifact-1\",\n  \"contractName\": \"AccessControlEnumerable\",\n  \"sourceName\": \"contracts/access/AccessControlEnumerable.sol\",\n  \"abi\": [\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": true,\n          \"internalType\": \"bytes32\",\n          \"name\": \"role\",\n          \"type\": \"bytes32\"\n        },\n        {\n          \"indexed\": true,\n          \"internalType\": \"bytes32\",\n          \"name\": \"previousAdminRole\",\n          \"type\": \"bytes32\"\n        },\n        {\n          \"indexed\": true,\n          \"internalType\": \"bytes32\",\n          \"name\": \"newAdminRole\",\n          \"type\": \"bytes32\"\n        }\n      ],\n      \"name\": \"RoleAdminChanged\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": true,\n          \"internalType\": \"bytes32\",\n          \"name\": \"role\",\n          \"type\": \"bytes32\"\n        },\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"account\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"sender\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"RoleGranted\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": true,\n          \"internalType\": \"bytes32\",\n          \"name\": \"role\",\n          \"type\": \"bytes32\"\n        },\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"account\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"sender\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"RoleRevoked\",\n      \"type\": \"event\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"DEFAULT_ADMIN_ROLE\",\n      \"outputs\": [\n        {\n          \"internalType\": \"bytes32\",\n          \"name\": \"\",\n          \"type\": \"bytes32\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"bytes32\",\n          \"name\": \"role\",\n          \"type\": \"bytes32\"\n        }\n      ],\n      \"name\": \"getRoleAdmin\",\n      \"outputs\": [\n        {\n          \"internalType\": \"bytes32\",\n          \"name\": \"\",\n          \"type\": \"bytes32\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"bytes32\",\n          \"name\": \"role\",\n          \"type\": \"bytes32\"\n        },\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"index\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"getRoleMember\",\n      \"outputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"\",\n          \"type\": \"address\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"bytes32\",\n          \"name\": \"role\",\n          \"type\": \"bytes32\"\n        }\n      ],\n      \"name\": \"getRoleMemberCount\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"bytes32\",\n          \"name\": \"role\",\n          \"type\": \"bytes32\"\n        },\n        {\n          \"internalType\": \"address\",\n          \"name\": \"account\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"grantRole\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"bytes32\",\n          \"name\": \"role\",\n          \"type\": \"bytes32\"\n        },\n        {\n          \"internalType\": \"address\",\n          \"name\": \"account\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"hasRole\",\n      \"outputs\": [\n        {\n          \"internalType\": \"bool\",\n          \"name\": \"\",\n          \"type\": \"bool\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"bytes32\",\n          \"name\": \"role\",\n          \"type\": \"bytes32\"\n        },\n        {\n          \"internalType\": \"address\",\n          \"name\": \"account\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"renounceRole\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"bytes32\",\n          \"name\": \"role\",\n          \"type\": \"bytes32\"\n        },\n        {\n          \"internalType\": \"address\",\n          \"name\": \"account\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"revokeRole\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"bytes4\",\n          \"name\": \"interfaceId\",\n          \"type\": \"bytes4\"\n        }\n      ],\n      \"name\": \"supportsInterface\",\n      \"outputs\": [\n        {\n          \"internalType\": \"bool\",\n          \"name\": \"\",\n          \"type\": \"bool\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    }\n  ],\n  \"bytecode\": \"0x\",\n  \"deployedBytecode\": \"0x\",\n  \"linkReferences\": {},\n  \"deployedLinkReferences\": {}\n}\n") . expect ("valid artifact JSON") ;
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
      f.debug_tuple(stringify!(AccessControlEnumerable))
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
    pub fn revoke_role(
      &self,
      role: [u8; 32],
      account: self::ethcontract::Address,
    ) -> self::ethcontract::dyns::DynMethodBuilder<self::ethcontract::Void> {
      self
        .instance
        .method([213, 71, 116, 31], (role, account))
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn supports_interface(
      &self,
      interface_id: [u8; 4],
    ) -> self::ethcontract::dyns::DynMethodBuilder<bool> {
      self
        .instance
        .method([1, 255, 201, 167], (interface_id,))
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn default_admin_role(&self) -> self::ethcontract::dyns::DynMethodBuilder<[u8; 32]> {
      self
        .instance
        .method([162, 23, 253, 223], ())
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn get_role_admin(
      &self,
      role: [u8; 32],
    ) -> self::ethcontract::dyns::DynMethodBuilder<[u8; 32]> {
      self
        .instance
        .method([36, 138, 156, 163], (role,))
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn renounce_role(
      &self,
      role: [u8; 32],
      account: self::ethcontract::Address,
    ) -> self::ethcontract::dyns::DynMethodBuilder<self::ethcontract::Void> {
      self
        .instance
        .method([54, 86, 138, 190], (role, account))
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn get_role_member(
      &self,
      role: [u8; 32],
      index: self::ethcontract::U256,
    ) -> self::ethcontract::dyns::DynMethodBuilder<self::ethcontract::Address> {
      self
        .instance
        .method([144, 16, 208, 124], (role, index))
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn get_role_member_count(
      &self,
      role: [u8; 32],
    ) -> self::ethcontract::dyns::DynMethodBuilder<self::ethcontract::U256> {
      self
        .instance
        .method([202, 21, 200, 115], (role,))
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn has_role(
      &self,
      role: [u8; 32],
      account: self::ethcontract::Address,
    ) -> self::ethcontract::dyns::DynMethodBuilder<bool> {
      self
        .instance
        .method([145, 209, 72, 84], (role, account))
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn grant_role(
      &self,
      role: [u8; 32],
      account: self::ethcontract::Address,
    ) -> self::ethcontract::dyns::DynMethodBuilder<self::ethcontract::Void> {
      self
        .instance
        .method([47, 47, 241, 93], (role, account))
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
    pub struct RoleAdminChanged {
      pub role: [u8; 32],
      pub previous_admin_role: [u8; 32],
      pub new_admin_role: [u8; 32],
    }
    impl RoleAdminChanged {
      #[doc = r" Retrieves the signature for the event this data corresponds to."]
      #[doc = r" This signature is the Keccak-256 hash of the ABI signature of"]
      #[doc = r" this event."]
      pub fn signature() -> self::ethcontract::H256 {
        self::ethcontract::H256([
          189, 121, 184, 111, 254, 10, 184, 232, 119, 97, 81, 81, 66, 23, 205, 124, 172, 213, 44,
          144, 159, 102, 71, 92, 58, 244, 78, 18, 159, 11, 0, 255,
        ])
      }
      #[doc = r" Retrieves the ABI signature for the event this data corresponds"]
      #[doc = r" to. For this event the value should always be:"]
      #[doc = r""]
      #[doc = "`RoleAdminChanged(bytes32,bytes32,bytes32)`"]
      pub fn abi_signature() -> &'static str {
        "RoleAdminChanged(bytes32,bytes32,bytes32)"
      }
    }
    impl self::ethcontract::web3::contract::tokens::Detokenize for RoleAdminChanged {
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
        let role =
          <[u8; 32] as self::ethcontract::web3::contract::tokens::Tokenizable>::from_token(
            tokens.next().unwrap(),
          )?;
        let previous_admin_role =
          <[u8; 32] as self::ethcontract::web3::contract::tokens::Tokenizable>::from_token(
            tokens.next().unwrap(),
          )?;
        let new_admin_role =
          <[u8; 32] as self::ethcontract::web3::contract::tokens::Tokenizable>::from_token(
            tokens.next().unwrap(),
          )?;
        Ok(RoleAdminChanged {
          role,
          previous_admin_role,
          new_admin_role,
        })
      }
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    pub struct RoleRevoked {
      pub role: [u8; 32],
      pub account: self::ethcontract::Address,
      pub sender: self::ethcontract::Address,
    }
    impl RoleRevoked {
      #[doc = r" Retrieves the signature for the event this data corresponds to."]
      #[doc = r" This signature is the Keccak-256 hash of the ABI signature of"]
      #[doc = r" this event."]
      pub fn signature() -> self::ethcontract::H256 {
        self::ethcontract::H256([
          246, 57, 31, 92, 50, 217, 198, 157, 42, 71, 234, 103, 11, 68, 41, 116, 181, 57, 53, 209,
          237, 199, 253, 100, 235, 33, 224, 71, 168, 57, 23, 27,
        ])
      }
      #[doc = r" Retrieves the ABI signature for the event this data corresponds"]
      #[doc = r" to. For this event the value should always be:"]
      #[doc = r""]
      #[doc = "`RoleRevoked(bytes32,address,address)`"]
      pub fn abi_signature() -> &'static str {
        "RoleRevoked(bytes32,address,address)"
      }
    }
    impl self::ethcontract::web3::contract::tokens::Detokenize for RoleRevoked {
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
        let role =
          <[u8; 32] as self::ethcontract::web3::contract::tokens::Tokenizable>::from_token(
            tokens.next().unwrap(),
          )?;
        let account = < self :: ethcontract :: Address as self :: ethcontract :: web3 :: contract :: tokens :: Tokenizable > :: from_token (tokens . next () . unwrap ()) ? ;
        let sender = < self :: ethcontract :: Address as self :: ethcontract :: web3 :: contract :: tokens :: Tokenizable > :: from_token (tokens . next () . unwrap ()) ? ;
        Ok(RoleRevoked {
          role,
          account,
          sender,
        })
      }
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    pub struct RoleGranted {
      pub role: [u8; 32],
      pub account: self::ethcontract::Address,
      pub sender: self::ethcontract::Address,
    }
    impl RoleGranted {
      #[doc = r" Retrieves the signature for the event this data corresponds to."]
      #[doc = r" This signature is the Keccak-256 hash of the ABI signature of"]
      #[doc = r" this event."]
      pub fn signature() -> self::ethcontract::H256 {
        self::ethcontract::H256([
          47, 135, 136, 17, 126, 126, 255, 29, 130, 233, 38, 236, 121, 73, 1, 209, 124, 120, 2, 74,
          80, 39, 9, 64, 48, 69, 64, 167, 51, 101, 111, 13,
        ])
      }
      #[doc = r" Retrieves the ABI signature for the event this data corresponds"]
      #[doc = r" to. For this event the value should always be:"]
      #[doc = r""]
      #[doc = "`RoleGranted(bytes32,address,address)`"]
      pub fn abi_signature() -> &'static str {
        "RoleGranted(bytes32,address,address)"
      }
    }
    impl self::ethcontract::web3::contract::tokens::Detokenize for RoleGranted {
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
        let role =
          <[u8; 32] as self::ethcontract::web3::contract::tokens::Tokenizable>::from_token(
            tokens.next().unwrap(),
          )?;
        let account = < self :: ethcontract :: Address as self :: ethcontract :: web3 :: contract :: tokens :: Tokenizable > :: from_token (tokens . next () . unwrap ()) ? ;
        let sender = < self :: ethcontract :: Address as self :: ethcontract :: web3 :: contract :: tokens :: Tokenizable > :: from_token (tokens . next () . unwrap ()) ? ;
        Ok(RoleGranted {
          role,
          account,
          sender,
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
    pub fn role_admin_changed(&self) -> self::event_builders::RoleAdminChangedBuilder {
      self::event_builders::RoleAdminChangedBuilder(
        self
          .instance
          .event(self::ethcontract::H256([
            189, 121, 184, 111, 254, 10, 184, 232, 119, 97, 81, 81, 66, 23, 205, 124, 172, 213, 44,
            144, 159, 102, 71, 92, 58, 244, 78, 18, 159, 11, 0, 255,
          ]))
          .expect("generated event filter"),
      )
    }
    #[doc = r" Generated by `ethcontract`."]
    pub fn role_revoked(&self) -> self::event_builders::RoleRevokedBuilder {
      self::event_builders::RoleRevokedBuilder(
        self
          .instance
          .event(self::ethcontract::H256([
            246, 57, 31, 92, 50, 217, 198, 157, 42, 71, 234, 103, 11, 68, 41, 116, 181, 57, 53,
            209, 237, 199, 253, 100, 235, 33, 224, 71, 168, 57, 23, 27,
          ]))
          .expect("generated event filter"),
      )
    }
    #[doc = r" Generated by `ethcontract`."]
    pub fn role_granted(&self) -> self::event_builders::RoleGrantedBuilder {
      self::event_builders::RoleGrantedBuilder(
        self
          .instance
          .event(self::ethcontract::H256([
            47, 135, 136, 17, 126, 126, 255, 29, 130, 233, 38, 236, 121, 73, 1, 209, 124, 120, 2,
            74, 80, 39, 9, 64, 48, 69, 64, 167, 51, 101, 111, 13,
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
    #[doc = "A builder for creating a filtered stream of `RoleAdminChanged` events."]
    pub struct RoleAdminChangedBuilder(
      #[doc = r" The inner event builder."]
      pub  self::ethcontract::dyns::DynEventBuilder<self::event_data::RoleAdminChanged>,
    );
    impl RoleAdminChangedBuilder {
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
      #[doc = "Adds a filter for the role event parameter."]
      pub fn role(mut self, topic: self::ethcontract::Topic<[u8; 32]>) -> Self {
        self.0 = (self.0).topic0(topic);
        self
      }
      #[doc = "Adds a filter for the previousAdminRole event parameter."]
      pub fn previous_admin_role(mut self, topic: self::ethcontract::Topic<[u8; 32]>) -> Self {
        self.0 = (self.0).topic1(topic);
        self
      }
      #[doc = "Adds a filter for the newAdminRole event parameter."]
      pub fn new_admin_role(mut self, topic: self::ethcontract::Topic<[u8; 32]>) -> Self {
        self.0 = (self.0).topic2(topic);
        self
      }
      #[doc = r" Returns a future that resolves with a collection of all existing"]
      #[doc = r" logs matching the builder parameters."]
      pub async fn query(
        self,
      ) -> std::result::Result<
        std::vec::Vec<self::ethcontract::Event<self::event_data::RoleAdminChanged>>,
        self::ethcontract::errors::EventError,
      > {
        (self.0).query().await
      }
      #[doc = r" Creates an event stream from the current event builder."]
      pub fn stream(
        self,
      ) -> impl self::ethcontract::futures::stream::Stream<
        Item = std::result::Result<
          self::ethcontract::StreamEvent<self::event_data::RoleAdminChanged>,
          self::ethcontract::errors::EventError,
        >,
      > {
        (self.0).stream()
      }
    }
    #[doc = "A builder for creating a filtered stream of `RoleRevoked` events."]
    pub struct RoleRevokedBuilder(
      #[doc = r" The inner event builder."]
      pub  self::ethcontract::dyns::DynEventBuilder<self::event_data::RoleRevoked>,
    );
    impl RoleRevokedBuilder {
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
      #[doc = "Adds a filter for the role event parameter."]
      pub fn role(mut self, topic: self::ethcontract::Topic<[u8; 32]>) -> Self {
        self.0 = (self.0).topic0(topic);
        self
      }
      #[doc = "Adds a filter for the account event parameter."]
      pub fn account(
        mut self,
        topic: self::ethcontract::Topic<self::ethcontract::Address>,
      ) -> Self {
        self.0 = (self.0).topic1(topic);
        self
      }
      #[doc = "Adds a filter for the sender event parameter."]
      pub fn sender(mut self, topic: self::ethcontract::Topic<self::ethcontract::Address>) -> Self {
        self.0 = (self.0).topic2(topic);
        self
      }
      #[doc = r" Returns a future that resolves with a collection of all existing"]
      #[doc = r" logs matching the builder parameters."]
      pub async fn query(
        self,
      ) -> std::result::Result<
        std::vec::Vec<self::ethcontract::Event<self::event_data::RoleRevoked>>,
        self::ethcontract::errors::EventError,
      > {
        (self.0).query().await
      }
      #[doc = r" Creates an event stream from the current event builder."]
      pub fn stream(
        self,
      ) -> impl self::ethcontract::futures::stream::Stream<
        Item = std::result::Result<
          self::ethcontract::StreamEvent<self::event_data::RoleRevoked>,
          self::ethcontract::errors::EventError,
        >,
      > {
        (self.0).stream()
      }
    }
    #[doc = "A builder for creating a filtered stream of `RoleGranted` events."]
    pub struct RoleGrantedBuilder(
      #[doc = r" The inner event builder."]
      pub  self::ethcontract::dyns::DynEventBuilder<self::event_data::RoleGranted>,
    );
    impl RoleGrantedBuilder {
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
      #[doc = "Adds a filter for the role event parameter."]
      pub fn role(mut self, topic: self::ethcontract::Topic<[u8; 32]>) -> Self {
        self.0 = (self.0).topic0(topic);
        self
      }
      #[doc = "Adds a filter for the account event parameter."]
      pub fn account(
        mut self,
        topic: self::ethcontract::Topic<self::ethcontract::Address>,
      ) -> Self {
        self.0 = (self.0).topic1(topic);
        self
      }
      #[doc = "Adds a filter for the sender event parameter."]
      pub fn sender(mut self, topic: self::ethcontract::Topic<self::ethcontract::Address>) -> Self {
        self.0 = (self.0).topic2(topic);
        self
      }
      #[doc = r" Returns a future that resolves with a collection of all existing"]
      #[doc = r" logs matching the builder parameters."]
      pub async fn query(
        self,
      ) -> std::result::Result<
        std::vec::Vec<self::ethcontract::Event<self::event_data::RoleGranted>>,
        self::ethcontract::errors::EventError,
      > {
        (self.0).query().await
      }
      #[doc = r" Creates an event stream from the current event builder."]
      pub fn stream(
        self,
      ) -> impl self::ethcontract::futures::stream::Stream<
        Item = std::result::Result<
          self::ethcontract::StreamEvent<self::event_data::RoleGranted>,
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
    RoleAdminChanged(self::event_data::RoleAdminChanged),
    RoleGranted(self::event_data::RoleGranted),
    RoleRevoked(self::event_data::RoleRevoked),
  }
  impl self::ethcontract::contract::ParseLog for Event {
    fn parse_log(
      log: self::ethcontract::RawLog,
    ) -> Result<Self, self::ethcontract::errors::ExecutionError> {
      let standard_event = log . topics . get (0) . copied () . map (| topic | match topic { self :: ethcontract :: H256 ([189 , 121 , 184 , 111 , 254 , 10 , 184 , 232 , 119 , 97 , 81 , 81 , 66 , 23 , 205 , 124 , 172 , 213 , 44 , 144 , 159 , 102 , 71 , 92 , 58 , 244 , 78 , 18 , 159 , 11 , 0 , 255]) => Ok (Event :: RoleAdminChanged (log . clone () . decode (& Contract :: artifact () . abi . event ("RoleAdminChanged") . expect ("generated event decode")) ?)) , self :: ethcontract :: H256 ([47 , 135 , 136 , 17 , 126 , 126 , 255 , 29 , 130 , 233 , 38 , 236 , 121 , 73 , 1 , 209 , 124 , 120 , 2 , 74 , 80 , 39 , 9 , 64 , 48 , 69 , 64 , 167 , 51 , 101 , 111 , 13]) => Ok (Event :: RoleGranted (log . clone () . decode (& Contract :: artifact () . abi . event ("RoleGranted") . expect ("generated event decode")) ?)) , self :: ethcontract :: H256 ([246 , 57 , 31 , 92 , 50 , 217 , 198 , 157 , 42 , 71 , 234 , 103 , 11 , 68 , 41 , 116 , 181 , 57 , 53 , 209 , 237 , 199 , 253 , 100 , 235 , 33 , 224 , 71 , 168 , 57 , 23 , 27]) => Ok (Event :: RoleRevoked (log . clone () . decode (& Contract :: artifact () . abi . event ("RoleRevoked") . expect ("generated event decode")) ?)) , _ => Err (self :: ethcontract :: errors :: ExecutionError :: from (self :: ethcontract :: common :: abi :: Error :: InvalidData)) , }) ;
      if let Some(Ok(data)) = standard_event {
        return Ok(data);
      }
      Err(self::ethcontract::errors::ExecutionError::from(
        self::ethcontract::common::abi::Error::InvalidData,
      ))
    }
  }
}
pub use self::access_control_enumerable::Contract as AccessControlEnumerable;
