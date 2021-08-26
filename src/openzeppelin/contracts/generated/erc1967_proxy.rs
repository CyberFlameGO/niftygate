#[allow(dead_code)]
pub mod erc1967_proxy {
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
          # [allow (unused_mut)] let mut artifact = Artifact :: from_json ("{\n  \"_format\": \"hh-sol-artifact-1\",\n  \"contractName\": \"ERC1967Proxy\",\n  \"sourceName\": \"contracts/proxy/ERC1967/ERC1967Proxy.sol\",\n  \"abi\": [\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"_logic\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"bytes\",\n          \"name\": \"_data\",\n          \"type\": \"bytes\"\n        }\n      ],\n      \"stateMutability\": \"payable\",\n      \"type\": \"constructor\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": false,\n          \"internalType\": \"address\",\n          \"name\": \"previousAdmin\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"address\",\n          \"name\": \"newAdmin\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"AdminChanged\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"beacon\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"BeaconUpgraded\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"implementation\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"Upgraded\",\n      \"type\": \"event\"\n    },\n    {\n      \"stateMutability\": \"payable\",\n      \"type\": \"fallback\"\n    },\n    {\n      \"stateMutability\": \"payable\",\n      \"type\": \"receive\"\n    }\n  ],\n  \"bytecode\": \"0x608060405260405161077e38038061077e83398101604081905261002291610303565b61004d60017f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbd61041b565b6000805160206107378339815191521461007757634e487b7160e01b600052600160045260246000fd5b6100838282600061008a565b5050610480565b610093836100c0565b6000825111806100a05750805b156100bb576100b9838361010060201b6100291760201c565b505b505050565b6100c98161012c565b6040516001600160a01b038216907fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b90600090a250565b60606101258383604051806060016040528060278152602001610757602791396101ec565b9392505050565b61013f816102c160201b6100551760201c565b6101a65760405162461bcd60e51b815260206004820152602d60248201527f455243313936373a206e657720696d706c656d656e746174696f6e206973206e60448201526c1bdd08184818dbdb9d1c9858dd609a1b60648201526084015b60405180910390fd5b806101cb60008051602061073783398151915260001b6102c760201b61005b1760201c565b80546001600160a01b0319166001600160a01b039290921691909117905550565b6060833b61024b5760405162461bcd60e51b815260206004820152602660248201527f416464726573733a2064656c65676174652063616c6c20746f206e6f6e2d636f6044820152651b9d1c9858dd60d21b606482015260840161019d565b600080856001600160a01b03168560405161026691906103cc565b600060405180830381855af49150503d80600081146102a1576040519150601f19603f3d011682016040523d82523d6000602084013e6102a6565b606091505b5090925090506102b78282866102ca565b9695505050505050565b3b151590565b90565b606083156102d9575081610125565b8251156102e95782518084602001fd5b8160405162461bcd60e51b815260040161019d91906103e8565b60008060408385031215610315578182fd5b82516001600160a01b038116811461032b578283fd5b60208401519092506001600160401b0380821115610347578283fd5b818501915085601f83011261035a578283fd5b81518181111561036c5761036c61046a565b604051601f8201601f19908116603f011681019083821181831017156103945761039461046a565b816040528281528860208487010111156103ac578586fd5b6103bd83602083016020880161043e565b80955050505050509250929050565b600082516103de81846020870161043e565b9190910192915050565b600060208252825180602084015261040781604085016020870161043e565b601f01601f19169190910160400192915050565b60008282101561043957634e487b7160e01b81526011600452602481fd5b500390565b60005b83811015610459578181015183820152602001610441565b838111156100b95750506000910152565b634e487b7160e01b600052604160045260246000fd5b6102a88061048f6000396000f3fe60806040523661001357610011610017565b005b6100115b61002761002261005e565b610096565b565b606061004e838360405180606001604052806027815260200161024c602791396100ba565b9392505050565b3b151590565b90565b60006100917f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc546001600160a01b031690565b905090565b3660008037600080366000845af43d6000803e8080156100b5573d6000f35b3d6000fd5b6060833b61011e5760405162461bcd60e51b815260206004820152602660248201527f416464726573733a2064656c65676174652063616c6c20746f206e6f6e2d636f6044820152651b9d1c9858dd60d21b60648201526084015b60405180910390fd5b600080856001600160a01b03168560405161013991906101cc565b600060405180830381855af49150503d8060008114610174576040519150601f19603f3d011682016040523d82523d6000602084013e610179565b606091505b5091509150610189828286610193565b9695505050505050565b606083156101a257508161004e565b8251156101b25782518084602001fd5b8160405162461bcd60e51b815260040161011591906101e8565b600082516101de81846020870161021b565b9190910192915050565b600060208252825180602084015261020781604085016020870161021b565b601f01601f19169190910160400192915050565b60005b8381101561023657818101518382015260200161021e565b83811115610245576000848401525b5050505056fe416464726573733a206c6f772d6c6576656c2064656c65676174652063616c6c206661696c6564a264697066735822122045a655e4377b466c772d0002c84619390141d500175e10ed25cf2bf57537dfc564736f6c63430008030033360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc416464726573733a206c6f772d6c6576656c2064656c65676174652063616c6c206661696c6564\",\n  \"deployedBytecode\": \"0x60806040523661001357610011610017565b005b6100115b61002761002261005e565b610096565b565b606061004e838360405180606001604052806027815260200161024c602791396100ba565b9392505050565b3b151590565b90565b60006100917f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc546001600160a01b031690565b905090565b3660008037600080366000845af43d6000803e8080156100b5573d6000f35b3d6000fd5b6060833b61011e5760405162461bcd60e51b815260206004820152602660248201527f416464726573733a2064656c65676174652063616c6c20746f206e6f6e2d636f6044820152651b9d1c9858dd60d21b60648201526084015b60405180910390fd5b600080856001600160a01b03168560405161013991906101cc565b600060405180830381855af49150503d8060008114610174576040519150601f19603f3d011682016040523d82523d6000602084013e610179565b606091505b5091509150610189828286610193565b9695505050505050565b606083156101a257508161004e565b8251156101b25782518084602001fd5b8160405162461bcd60e51b815260040161011591906101e8565b600082516101de81846020870161021b565b9190910192915050565b600060208252825180602084015261020781604085016020870161021b565b601f01601f19169190910160400192915050565b60005b8381101561023657818101518382015260200161021e565b83811115610245576000848401525b5050505056fe416464726573733a206c6f772d6c6576656c2064656c65676174652063616c6c206661696c6564a264697066735822122045a655e4377b466c772d0002c84619390141d500175e10ed25cf2bf57537dfc564736f6c63430008030033\",\n  \"linkReferences\": {},\n  \"deployedLinkReferences\": {}\n}\n") . expect ("valid artifact JSON") ;
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
      f.debug_tuple(stringify!(ERC1967Proxy))
        .field(&self.address())
        .finish()
    }
  }
  impl Contract {
    #[doc = "Generated by `ethcontract`"]
    #[allow(clippy::too_many_arguments)]
    pub fn builder<F, T>(
      web3: &self::ethcontract::web3::api::Web3<T>,
      logic: self::ethcontract::Address,
      data: self::ethcontract::tokens::Bytes<Vec<u8>>,
    ) -> self::ethcontract::dyns::DynDeployBuilder<Self>
    where
      F: std::future::Future<
          Output = Result<self::ethcontract::json::Value, self::ethcontract::web3::Error>,
        > + Send
        + 'static,
      T: self::ethcontract::web3::Transport<Out = F> + Send + Sync + 'static,
    {
      use self::ethcontract::contract::DeployBuilder;
      use self::ethcontract::dyns::DynTransport;
      use self::ethcontract::web3::api::Web3;
      let transport = DynTransport::new(web3.transport().clone());
      let web3 = Web3::new(transport);
      let bytecode = Self::artifact().bytecode.clone();
      DeployBuilder::new(web3, bytecode, (logic, data)).expect("valid deployment args")
    }
  }
  impl self::ethcontract::contract::Deploy<self::ethcontract::dyns::DynTransport> for Contract {
    type Context = self::ethcontract::common::Bytecode;
    fn bytecode(cx: &Self::Context) -> &self::ethcontract::common::Bytecode {
      cx
    }
    fn abi(_: &Self::Context) -> &self::ethcontract::common::Abi {
      &Self::artifact().abi
    }
    fn from_deployment(
      web3: self::ethcontract::dyns::DynWeb3,
      address: self::ethcontract::Address,
      transaction_hash: self::ethcontract::H256,
      _: Self::Context,
    ) -> Self {
      Self::with_deployment_info(&web3, address, Some(transaction_hash.into()))
    }
  }
  #[derive(Clone)]
  struct Methods {
    instance: self::ethcontract::dyns::DynInstance,
  }
  impl Contract {
    #[doc = r" Returns a method builder to setup a call to a smart"]
    #[doc = r" contract's fallback function."]
    pub fn fallback<D>(&self, data: D) -> self::ethcontract::dyns::DynMethodBuilder<()>
    where
      D: Into<Vec<u8>>,
    {
      self
        .raw_instance()
        .fallback(data)
        .expect("generated fallback method")
    }
  }
  #[doc = r" Module containing all generated data models for this contract's"]
  #[doc = r" events."]
  pub mod event_data {
    use super::ethcontract;
    #[derive(Clone, Debug, Default, Eq, PartialEq, serde :: Deserialize, serde :: Serialize)]
    pub struct AdminChanged {
      pub previous_admin: self::ethcontract::Address,
      pub new_admin: self::ethcontract::Address,
    }
    impl AdminChanged {
      #[doc = r" Retrieves the signature for the event this data corresponds to."]
      #[doc = r" This signature is the Keccak-256 hash of the ABI signature of"]
      #[doc = r" this event."]
      pub fn signature() -> self::ethcontract::H256 {
        self::ethcontract::H256([
          126, 100, 77, 121, 66, 47, 23, 192, 30, 72, 148, 181, 244, 245, 136, 211, 49, 235, 250,
          40, 101, 61, 66, 174, 131, 45, 197, 158, 56, 201, 121, 143,
        ])
      }
      #[doc = r" Retrieves the ABI signature for the event this data corresponds"]
      #[doc = r" to. For this event the value should always be:"]
      #[doc = r""]
      #[doc = "`AdminChanged(address,address)`"]
      pub fn abi_signature() -> &'static str {
        "AdminChanged(address,address)"
      }
    }
    impl self::ethcontract::tokens::Tokenize for AdminChanged {
      fn from_token(
        token: self::ethcontract::common::abi::Token,
      ) -> Result<Self, self::ethcontract::tokens::Error> {
        let (previous_admin, new_admin) = self::ethcontract::tokens::Tokenize::from_token(token)?;
        Ok(AdminChanged {
          previous_admin,
          new_admin,
        })
      }
      fn into_token(self) -> self::ethcontract::common::abi::Token {
        unimplemented!("events are only decoded, not encoded")
      }
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, serde :: Deserialize, serde :: Serialize)]
    pub struct BeaconUpgraded {
      pub beacon: self::ethcontract::Address,
    }
    impl BeaconUpgraded {
      #[doc = r" Retrieves the signature for the event this data corresponds to."]
      #[doc = r" This signature is the Keccak-256 hash of the ABI signature of"]
      #[doc = r" this event."]
      pub fn signature() -> self::ethcontract::H256 {
        self::ethcontract::H256([
          28, 243, 176, 58, 108, 241, 159, 162, 186, 186, 77, 241, 72, 233, 220, 171, 237, 234,
          127, 138, 92, 7, 132, 14, 32, 126, 92, 8, 155, 233, 93, 62,
        ])
      }
      #[doc = r" Retrieves the ABI signature for the event this data corresponds"]
      #[doc = r" to. For this event the value should always be:"]
      #[doc = r""]
      #[doc = "`BeaconUpgraded(address)`"]
      pub fn abi_signature() -> &'static str {
        "BeaconUpgraded(address)"
      }
    }
    impl self::ethcontract::tokens::Tokenize for BeaconUpgraded {
      fn from_token(
        token: self::ethcontract::common::abi::Token,
      ) -> Result<Self, self::ethcontract::tokens::Error> {
        let (beacon,) = self::ethcontract::tokens::Tokenize::from_token(token)?;
        Ok(BeaconUpgraded { beacon })
      }
      fn into_token(self) -> self::ethcontract::common::abi::Token {
        unimplemented!("events are only decoded, not encoded")
      }
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, serde :: Deserialize, serde :: Serialize)]
    pub struct Upgraded {
      pub implementation: self::ethcontract::Address,
    }
    impl Upgraded {
      #[doc = r" Retrieves the signature for the event this data corresponds to."]
      #[doc = r" This signature is the Keccak-256 hash of the ABI signature of"]
      #[doc = r" this event."]
      pub fn signature() -> self::ethcontract::H256 {
        self::ethcontract::H256([
          188, 124, 215, 90, 32, 238, 39, 253, 154, 222, 186, 179, 32, 65, 247, 85, 33, 77, 188,
          107, 255, 169, 12, 192, 34, 91, 57, 218, 46, 92, 45, 59,
        ])
      }
      #[doc = r" Retrieves the ABI signature for the event this data corresponds"]
      #[doc = r" to. For this event the value should always be:"]
      #[doc = r""]
      #[doc = "`Upgraded(address)`"]
      pub fn abi_signature() -> &'static str {
        "Upgraded(address)"
      }
    }
    impl self::ethcontract::tokens::Tokenize for Upgraded {
      fn from_token(
        token: self::ethcontract::common::abi::Token,
      ) -> Result<Self, self::ethcontract::tokens::Error> {
        let (implementation,) = self::ethcontract::tokens::Tokenize::from_token(token)?;
        Ok(Upgraded { implementation })
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
    pub fn admin_changed(&self) -> self::event_builders::AdminChangedBuilder {
      self::event_builders::AdminChangedBuilder(
        self
          .instance
          .event(self::ethcontract::H256([
            126, 100, 77, 121, 66, 47, 23, 192, 30, 72, 148, 181, 244, 245, 136, 211, 49, 235, 250,
            40, 101, 61, 66, 174, 131, 45, 197, 158, 56, 201, 121, 143,
          ]))
          .expect("generated event filter"),
      )
    }
    #[doc = r" Generated by `ethcontract`."]
    pub fn beacon_upgraded(&self) -> self::event_builders::BeaconUpgradedBuilder {
      self::event_builders::BeaconUpgradedBuilder(
        self
          .instance
          .event(self::ethcontract::H256([
            28, 243, 176, 58, 108, 241, 159, 162, 186, 186, 77, 241, 72, 233, 220, 171, 237, 234,
            127, 138, 92, 7, 132, 14, 32, 126, 92, 8, 155, 233, 93, 62,
          ]))
          .expect("generated event filter"),
      )
    }
    #[doc = r" Generated by `ethcontract`."]
    pub fn upgraded(&self) -> self::event_builders::UpgradedBuilder {
      self::event_builders::UpgradedBuilder(
        self
          .instance
          .event(self::ethcontract::H256([
            188, 124, 215, 90, 32, 238, 39, 253, 154, 222, 186, 179, 32, 65, 247, 85, 33, 77, 188,
            107, 255, 169, 12, 192, 34, 91, 57, 218, 46, 92, 45, 59,
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
    #[doc = "A builder for creating a filtered stream of `AdminChanged` events."]
    pub struct AdminChangedBuilder(
      #[doc = r" The inner event builder."]
      pub  self::ethcontract::dyns::DynEventBuilder<self::event_data::AdminChanged>,
    );
    impl AdminChangedBuilder {
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
        std::vec::Vec<self::ethcontract::Event<self::event_data::AdminChanged>>,
        self::ethcontract::errors::EventError,
      > {
        (self.0).query().await
      }
      #[doc = r" Creates an event stream from the current event builder."]
      pub fn stream(
        self,
      ) -> impl self::ethcontract::futures::stream::Stream<
        Item = std::result::Result<
          self::ethcontract::StreamEvent<self::event_data::AdminChanged>,
          self::ethcontract::errors::EventError,
        >,
      > {
        (self.0).stream()
      }
    }
    #[doc = "A builder for creating a filtered stream of `BeaconUpgraded` events."]
    pub struct BeaconUpgradedBuilder(
      #[doc = r" The inner event builder."]
      pub  self::ethcontract::dyns::DynEventBuilder<self::event_data::BeaconUpgraded>,
    );
    impl BeaconUpgradedBuilder {
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
      #[doc = "Adds a filter for the beacon event parameter."]
      pub fn beacon(mut self, topic: self::ethcontract::Topic<self::ethcontract::Address>) -> Self {
        self.0 = (self.0).topic0(topic);
        self
      }
      #[doc = r" Returns a future that resolves with a collection of all existing"]
      #[doc = r" logs matching the builder parameters."]
      pub async fn query(
        self,
      ) -> std::result::Result<
        std::vec::Vec<self::ethcontract::Event<self::event_data::BeaconUpgraded>>,
        self::ethcontract::errors::EventError,
      > {
        (self.0).query().await
      }
      #[doc = r" Creates an event stream from the current event builder."]
      pub fn stream(
        self,
      ) -> impl self::ethcontract::futures::stream::Stream<
        Item = std::result::Result<
          self::ethcontract::StreamEvent<self::event_data::BeaconUpgraded>,
          self::ethcontract::errors::EventError,
        >,
      > {
        (self.0).stream()
      }
    }
    #[doc = "A builder for creating a filtered stream of `Upgraded` events."]
    pub struct UpgradedBuilder(
      #[doc = r" The inner event builder."]
      pub  self::ethcontract::dyns::DynEventBuilder<self::event_data::Upgraded>,
    );
    impl UpgradedBuilder {
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
      #[doc = "Adds a filter for the implementation event parameter."]
      pub fn implementation(
        mut self,
        topic: self::ethcontract::Topic<self::ethcontract::Address>,
      ) -> Self {
        self.0 = (self.0).topic0(topic);
        self
      }
      #[doc = r" Returns a future that resolves with a collection of all existing"]
      #[doc = r" logs matching the builder parameters."]
      pub async fn query(
        self,
      ) -> std::result::Result<
        std::vec::Vec<self::ethcontract::Event<self::event_data::Upgraded>>,
        self::ethcontract::errors::EventError,
      > {
        (self.0).query().await
      }
      #[doc = r" Creates an event stream from the current event builder."]
      pub fn stream(
        self,
      ) -> impl self::ethcontract::futures::stream::Stream<
        Item = std::result::Result<
          self::ethcontract::StreamEvent<self::event_data::Upgraded>,
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
    AdminChanged(self::event_data::AdminChanged),
    BeaconUpgraded(self::event_data::BeaconUpgraded),
    Upgraded(self::event_data::Upgraded),
  }
  impl self::ethcontract::contract::ParseLog for Event {
    fn parse_log(
      log: self::ethcontract::RawLog,
    ) -> Result<Self, self::ethcontract::errors::ExecutionError> {
      let standard_event = log . topics . get (0) . copied () . map (| topic | match topic { self :: ethcontract :: H256 ([126 , 100 , 77 , 121 , 66 , 47 , 23 , 192 , 30 , 72 , 148 , 181 , 244 , 245 , 136 , 211 , 49 , 235 , 250 , 40 , 101 , 61 , 66 , 174 , 131 , 45 , 197 , 158 , 56 , 201 , 121 , 143]) => Ok (Event :: AdminChanged (log . clone () . decode (Contract :: artifact () . abi . event ("AdminChanged") . expect ("generated event decode")) ?)) , self :: ethcontract :: H256 ([28 , 243 , 176 , 58 , 108 , 241 , 159 , 162 , 186 , 186 , 77 , 241 , 72 , 233 , 220 , 171 , 237 , 234 , 127 , 138 , 92 , 7 , 132 , 14 , 32 , 126 , 92 , 8 , 155 , 233 , 93 , 62]) => Ok (Event :: BeaconUpgraded (log . clone () . decode (Contract :: artifact () . abi . event ("BeaconUpgraded") . expect ("generated event decode")) ?)) , self :: ethcontract :: H256 ([188 , 124 , 215 , 90 , 32 , 238 , 39 , 253 , 154 , 222 , 186 , 179 , 32 , 65 , 247 , 85 , 33 , 77 , 188 , 107 , 255 , 169 , 12 , 192 , 34 , 91 , 57 , 218 , 46 , 92 , 45 , 59]) => Ok (Event :: Upgraded (log . clone () . decode (Contract :: artifact () . abi . event ("Upgraded") . expect ("generated event decode")) ?)) , _ => Err (self :: ethcontract :: errors :: ExecutionError :: from (self :: ethcontract :: common :: abi :: Error :: InvalidData)) , }) ;
      if let Some(Ok(data)) = standard_event {
        return Ok(data);
      }
      Err(self::ethcontract::errors::ExecutionError::from(
        self::ethcontract::common::abi::Error::InvalidData,
      ))
    }
  }
}
pub use self::erc1967_proxy::Contract as ERC1967Proxy;
