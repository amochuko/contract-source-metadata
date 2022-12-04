use near_sdk::near_bindgen;

/// A structure of the contract metadata
#[near_bindgen]
pub struct ContractSourceMetadata<'a> {
    /// A version can represent the the commit hash to that deployed the contract
    pub version: &'a str,
    /// A link to a public repo of the contract
    pub link: &'a str,
}

/// A trait to be implemented by the `ContractSourceMetadata`
///
/// # Example
/// ```
/// //  impl TContractSourceMetadata for Contract {
///  //   fn contract_source_metadata(&self) -> ContractSourceMetadata {
///   //       ContractSourceMetadata {
///   //              version: "0.0.1",
///   //              link: "https://github.com/iamochuko/contract-source-metadata",
///    //       }
///    //  }
/// // }
/// ```
pub trait TContractSourceMetadata {
    fn contract_source_metadata(&self) -> ContractSourceMetadata;
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Struct to assume a Contract
    #[allow(dead_code)]
    struct Contract {
        name: String,
        age: u8,
    }

    /// Implement trait for contract
    impl TContractSourceMetadata for Contract {
        fn contract_source_metadata(&self) -> ContractSourceMetadata {
            ContractSourceMetadata {
                version: "0.0.1",
                link: "https://github.com/iamochuko/contract-source-metadata",
            }
        }
    }

    /// Fetch `Cargo.toml` to read the repository path
    fn get_toml() -> String {
        let lines = std::fs::read_to_string("./Cargo.toml");
        let mut word = String::new();

        for line in lines {
            for v in line.split("\n").enumerate() {
                if v.1.split('=').collect::<String>().contains("repository") {
                    let (_, b) = v;
                    word = b.to_string().split_off(13).splitn(4, '"').collect()
                }
            }
        }
        word
    }

    #[test]
    fn source_metadata() {
        let contract = Contract {
            name: "David".to_string(),
            age: 34,
        };

        let expect = contract.contract_source_metadata().link;
        assert_eq!(expect, get_toml())
    }
}
