#[allow(dead_code)]
pub mod ierc1820_registry {
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
          # [allow (unused_mut)] let mut artifact = Artifact :: from_json ("{\n  \"_format\": \"hh-sol-artifact-1\",\n  \"contractName\": \"IERC1820Registry\",\n  \"sourceName\": \"contracts/utils/introspection/IERC1820Registry.sol\",\n  \"abi\": [\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"account\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": true,\n          \"internalType\": \"bytes32\",\n          \"name\": \"interfaceHash\",\n          \"type\": \"bytes32\"\n        },\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"implementer\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"InterfaceImplementerSet\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"account\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"newManager\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"ManagerChanged\",\n      \"type\": \"event\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"account\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"bytes32\",\n          \"name\": \"_interfaceHash\",\n          \"type\": \"bytes32\"\n        }\n      ],\n      \"name\": \"getInterfaceImplementer\",\n      \"outputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"\",\n          \"type\": \"address\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"account\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"getManager\",\n      \"outputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"\",\n          \"type\": \"address\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"account\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"bytes4\",\n          \"name\": \"interfaceId\",\n          \"type\": \"bytes4\"\n        }\n      ],\n      \"name\": \"implementsERC165Interface\",\n      \"outputs\": [\n        {\n          \"internalType\": \"bool\",\n          \"name\": \"\",\n          \"type\": \"bool\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"account\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"bytes4\",\n          \"name\": \"interfaceId\",\n          \"type\": \"bytes4\"\n        }\n      ],\n      \"name\": \"implementsERC165InterfaceNoCache\",\n      \"outputs\": [\n        {\n          \"internalType\": \"bool\",\n          \"name\": \"\",\n          \"type\": \"bool\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"string\",\n          \"name\": \"interfaceName\",\n          \"type\": \"string\"\n        }\n      ],\n      \"name\": \"interfaceHash\",\n      \"outputs\": [\n        {\n          \"internalType\": \"bytes32\",\n          \"name\": \"\",\n          \"type\": \"bytes32\"\n        }\n      ],\n      \"stateMutability\": \"pure\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"account\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"bytes32\",\n          \"name\": \"_interfaceHash\",\n          \"type\": \"bytes32\"\n        },\n        {\n          \"internalType\": \"address\",\n          \"name\": \"implementer\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"setInterfaceImplementer\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"account\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"address\",\n          \"name\": \"newManager\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"setManager\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"account\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"bytes4\",\n          \"name\": \"interfaceId\",\n          \"type\": \"bytes4\"\n        }\n      ],\n      \"name\": \"updateERC165Cache\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    }\n  ],\n  \"bytecode\": \"0x\",\n  \"deployedBytecode\": \"0x\",\n  \"linkReferences\": {},\n  \"deployedLinkReferences\": {}\n}\n") . expect ("valid artifact JSON") ;
          artifact
        };
      }
      &ARTIFACT
    }
    #[doc = r" Creates a new contract instance with the specified `web3`"]
    #[doc = r" provider at the given `Address`."]
    #[doc = r""]
    #[doc = r" Note that this does not verify that a contract with a matching"]
    #[doc = r" `Abi` is actually deployed at the given address."]
    pub fn at<F, T>(
      web3: &self::ethcontract::web3::api::Web3<T>,
      address: self::ethcontract::Address,
    ) -> Self
    where
      F: std::future::Future<
          Output = Result<self::ethcontract::json::Value, self::ethcontract::web3::Error>,
        > + Send
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
      f.debug_tuple(stringify!(IERC1820Registry))
        .field(&self.address())
        .finish()
    }
  }
  impl Contract {
    #[doc = r" Retrieves a reference to type containing all the generated"]
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
    pub fn implements_erc165_interface(
      &self,
      account: self::ethcontract::Address,
      interface_id: self::ethcontract::tokens::Bytes<[u8; 4]>,
    ) -> self::ethcontract::dyns::DynViewMethodBuilder<bool> {
      self
        .instance
        .view_method([247, 18, 243, 232], (account, interface_id))
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn get_manager(
      &self,
      account: self::ethcontract::Address,
    ) -> self::ethcontract::dyns::DynViewMethodBuilder<self::ethcontract::Address> {
      self
        .instance
        .view_method([61, 88, 64, 99], (account,))
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn implements_erc165_interface_no_cache(
      &self,
      account: self::ethcontract::Address,
      interface_id: self::ethcontract::tokens::Bytes<[u8; 4]>,
    ) -> self::ethcontract::dyns::DynViewMethodBuilder<bool> {
      self
        .instance
        .view_method([183, 5, 103, 101], (account, interface_id))
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn update_erc165_cache(
      &self,
      account: self::ethcontract::Address,
      interface_id: self::ethcontract::tokens::Bytes<[u8; 4]>,
    ) -> self::ethcontract::dyns::DynMethodBuilder<()> {
      self
        .instance
        .method([164, 30, 125, 81], (account, interface_id))
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn set_manager(
      &self,
      account: self::ethcontract::Address,
      new_manager: self::ethcontract::Address,
    ) -> self::ethcontract::dyns::DynMethodBuilder<()> {
      self
        .instance
        .method([93, 248, 18, 47], (account, new_manager))
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn get_interface_implementer(
      &self,
      account: self::ethcontract::Address,
      interface_hash: self::ethcontract::tokens::Bytes<[u8; 32]>,
    ) -> self::ethcontract::dyns::DynViewMethodBuilder<self::ethcontract::Address> {
      self
        .instance
        .view_method([170, 187, 184, 202], (account, interface_hash))
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn interface_hash(
      &self,
      interface_name: String,
    ) -> self::ethcontract::dyns::DynViewMethodBuilder<self::ethcontract::tokens::Bytes<[u8; 32]>>
    {
      self
        .instance
        .view_method([101, 186, 54, 193], (interface_name,))
        .expect("generated call")
    }
    #[doc = "Generated by `ethcontract`"]
    pub fn set_interface_implementer(
      &self,
      account: self::ethcontract::Address,
      interface_hash: self::ethcontract::tokens::Bytes<[u8; 32]>,
      implementer: self::ethcontract::Address,
    ) -> self::ethcontract::dyns::DynMethodBuilder<()> {
      self
        .instance
        .method([41, 150, 90, 29], (account, interface_hash, implementer))
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
    #[derive(Clone, Debug, Default, Eq, PartialEq, serde :: Deserialize, serde :: Serialize)]
    pub struct ManagerChanged {
      pub account: self::ethcontract::Address,
      pub new_manager: self::ethcontract::Address,
    }
    impl ManagerChanged {
      #[doc = r" Retrieves the signature for the event this data corresponds to."]
      #[doc = r" This signature is the Keccak-256 hash of the ABI signature of"]
      #[doc = r" this event."]
      pub fn signature() -> self::ethcontract::H256 {
        self::ethcontract::H256([
          96, 92, 45, 191, 118, 46, 95, 125, 96, 165, 70, 212, 46, 114, 5, 220, 177, 176, 17, 235,
          198, 42, 97, 115, 106, 87, 201, 8, 157, 58, 67, 80,
        ])
      }
      #[doc = r" Retrieves the ABI signature for the event this data corresponds"]
      #[doc = r" to. For this event the value should always be:"]
      #[doc = r""]
      #[doc = "`ManagerChanged(address,address)`"]
      pub fn abi_signature() -> &'static str {
        "ManagerChanged(address,address)"
      }
    }
    impl self::ethcontract::tokens::Tokenize for ManagerChanged {
      fn from_token(
        token: self::ethcontract::common::abi::Token,
      ) -> Result<Self, self::ethcontract::tokens::Error> {
        let (account, new_manager) = self::ethcontract::tokens::Tokenize::from_token(token)?;
        Ok(ManagerChanged {
          account,
          new_manager,
        })
      }
      fn into_token(self) -> self::ethcontract::common::abi::Token {
        unimplemented!("events are only decoded, not encoded")
      }
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, serde :: Deserialize, serde :: Serialize)]
    pub struct InterfaceImplementerSet {
      pub account: self::ethcontract::Address,
      pub interface_hash: self::ethcontract::tokens::Bytes<[u8; 32]>,
      pub implementer: self::ethcontract::Address,
    }
    impl InterfaceImplementerSet {
      #[doc = r" Retrieves the signature for the event this data corresponds to."]
      #[doc = r" This signature is the Keccak-256 hash of the ABI signature of"]
      #[doc = r" this event."]
      pub fn signature() -> self::ethcontract::H256 {
        self::ethcontract::H256([
          147, 186, 166, 239, 189, 34, 68, 36, 59, 254, 230, 206, 76, 253, 209, 208, 79, 196, 192,
          233, 167, 134, 171, 211, 164, 19, 19, 189, 53, 45, 177, 83,
        ])
      }
      #[doc = r" Retrieves the ABI signature for the event this data corresponds"]
      #[doc = r" to. For this event the value should always be:"]
      #[doc = r""]
      #[doc = "`InterfaceImplementerSet(address,bytes32,address)`"]
      pub fn abi_signature() -> &'static str {
        "InterfaceImplementerSet(address,bytes32,address)"
      }
    }
    impl self::ethcontract::tokens::Tokenize for InterfaceImplementerSet {
      fn from_token(
        token: self::ethcontract::common::abi::Token,
      ) -> Result<Self, self::ethcontract::tokens::Error> {
        let (account, interface_hash, implementer) =
          self::ethcontract::tokens::Tokenize::from_token(token)?;
        Ok(InterfaceImplementerSet {
          account,
          interface_hash,
          implementer,
        })
      }
      fn into_token(self) -> self::ethcontract::common::abi::Token {
        unimplemented!("events are only decoded, not encoded")
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
    pub fn manager_changed(&self) -> self::event_builders::ManagerChangedBuilder {
      self::event_builders::ManagerChangedBuilder(
        self
          .instance
          .event(self::ethcontract::H256([
            96, 92, 45, 191, 118, 46, 95, 125, 96, 165, 70, 212, 46, 114, 5, 220, 177, 176, 17,
            235, 198, 42, 97, 115, 106, 87, 201, 8, 157, 58, 67, 80,
          ]))
          .expect("generated event filter"),
      )
    }
    #[doc = r" Generated by `ethcontract`."]
    pub fn interface_implementer_set(
      &self,
    ) -> self::event_builders::InterfaceImplementerSetBuilder {
      self::event_builders::InterfaceImplementerSetBuilder(
        self
          .instance
          .event(self::ethcontract::H256([
            147, 186, 166, 239, 189, 34, 68, 36, 59, 254, 230, 206, 76, 253, 209, 208, 79, 196,
            192, 233, 167, 134, 171, 211, 164, 19, 19, 189, 53, 45, 177, 83,
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
    #[doc = "A builder for creating a filtered stream of `ManagerChanged` events."]
    pub struct ManagerChangedBuilder(
      #[doc = r" The inner event builder."]
      pub  self::ethcontract::dyns::DynEventBuilder<self::event_data::ManagerChanged>,
    );
    impl ManagerChangedBuilder {
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
      #[doc = "Adds a filter for the account event parameter."]
      pub fn account(
        mut self,
        topic: self::ethcontract::Topic<self::ethcontract::Address>,
      ) -> Self {
        self.0 = (self.0).topic0(topic);
        self
      }
      #[doc = "Adds a filter for the newManager event parameter."]
      pub fn new_manager(
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
        std::vec::Vec<self::ethcontract::Event<self::event_data::ManagerChanged>>,
        self::ethcontract::errors::EventError,
      > {
        (self.0).query().await
      }
      #[doc = r" Creates an event stream from the current event builder."]
      pub fn stream(
        self,
      ) -> impl self::ethcontract::futures::stream::Stream<
        Item = std::result::Result<
          self::ethcontract::StreamEvent<self::event_data::ManagerChanged>,
          self::ethcontract::errors::EventError,
        >,
      > {
        (self.0).stream()
      }
    }
    #[doc = "A builder for creating a filtered stream of `InterfaceImplementerSet` events."]
    pub struct InterfaceImplementerSetBuilder(
      #[doc = r" The inner event builder."]
      pub  self::ethcontract::dyns::DynEventBuilder<self::event_data::InterfaceImplementerSet>,
    );
    impl InterfaceImplementerSetBuilder {
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
      #[doc = "Adds a filter for the account event parameter."]
      pub fn account(
        mut self,
        topic: self::ethcontract::Topic<self::ethcontract::Address>,
      ) -> Self {
        self.0 = (self.0).topic0(topic);
        self
      }
      #[doc = "Adds a filter for the interfaceHash event parameter."]
      pub fn interface_hash(
        mut self,
        topic: self::ethcontract::Topic<self::ethcontract::tokens::Bytes<[u8; 32]>>,
      ) -> Self {
        self.0 = (self.0).topic1(topic);
        self
      }
      #[doc = "Adds a filter for the implementer event parameter."]
      pub fn implementer(
        mut self,
        topic: self::ethcontract::Topic<self::ethcontract::Address>,
      ) -> Self {
        self.0 = (self.0).topic2(topic);
        self
      }
      #[doc = r" Returns a future that resolves with a collection of all existing"]
      #[doc = r" logs matching the builder parameters."]
      pub async fn query(
        self,
      ) -> std::result::Result<
        std::vec::Vec<self::ethcontract::Event<self::event_data::InterfaceImplementerSet>>,
        self::ethcontract::errors::EventError,
      > {
        (self.0).query().await
      }
      #[doc = r" Creates an event stream from the current event builder."]
      pub fn stream(
        self,
      ) -> impl self::ethcontract::futures::stream::Stream<
        Item = std::result::Result<
          self::ethcontract::StreamEvent<self::event_data::InterfaceImplementerSet>,
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
  #[derive(Clone, Debug, Eq, PartialEq, serde :: Deserialize, serde :: Serialize)]
  pub enum Event {
    InterfaceImplementerSet(self::event_data::InterfaceImplementerSet),
    ManagerChanged(self::event_data::ManagerChanged),
  }
  impl self::ethcontract::contract::ParseLog for Event {
    fn parse_log(
      log: self::ethcontract::RawLog,
    ) -> Result<Self, self::ethcontract::errors::ExecutionError> {
      let standard_event = log . topics . get (0) . copied () . map (| topic | match topic { self :: ethcontract :: H256 ([147 , 186 , 166 , 239 , 189 , 34 , 68 , 36 , 59 , 254 , 230 , 206 , 76 , 253 , 209 , 208 , 79 , 196 , 192 , 233 , 167 , 134 , 171 , 211 , 164 , 19 , 19 , 189 , 53 , 45 , 177 , 83]) => Ok (Event :: InterfaceImplementerSet (log . clone () . decode (Contract :: artifact () . abi . event ("InterfaceImplementerSet") . expect ("generated event decode")) ?)) , self :: ethcontract :: H256 ([96 , 92 , 45 , 191 , 118 , 46 , 95 , 125 , 96 , 165 , 70 , 212 , 46 , 114 , 5 , 220 , 177 , 176 , 17 , 235 , 198 , 42 , 97 , 115 , 106 , 87 , 201 , 8 , 157 , 58 , 67 , 80]) => Ok (Event :: ManagerChanged (log . clone () . decode (Contract :: artifact () . abi . event ("ManagerChanged") . expect ("generated event decode")) ?)) , _ => Err (self :: ethcontract :: errors :: ExecutionError :: from (self :: ethcontract :: common :: abi :: Error :: InvalidData)) , }) ;
      if let Some(Ok(data)) = standard_event {
        return Ok(data);
      }
      Err(self::ethcontract::errors::ExecutionError::from(
        self::ethcontract::common::abi::Error::InvalidData,
      ))
    }
  }
}
pub use self::ierc1820_registry::Contract as IERC1820Registry;
