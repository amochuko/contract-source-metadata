use near_sdk::near_bindgen;

/// A structure of the contract metadata
#[near_bindgen]
pub struct ContractSourceMetadata {
    /// A version can represent the the commit hash to that deployed the contract
    pub version: String,
    /// A link to a public repo of the contract
    pub link: String,
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

/// Fetch `Cargo.toml` to read the repository path
pub fn get_repo_link_from_toml() -> String {
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
            let repo_link = get_repo_link_from_toml();

            ContractSourceMetadata {
                version: String::from("0.0.1"),
                link: repo_link,
            }
        }
    }

    #[test]
    fn source_metadata() {
        let contract = Contract {
            name: "David".to_string(),
            age: 34,
        };

        let expect = contract.contract_source_metadata().link;
        assert_eq!(expect, get_repo_link_from_toml())
    }
}
