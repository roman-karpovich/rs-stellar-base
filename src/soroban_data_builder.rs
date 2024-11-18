use stellar_xdr::{
    curr::VecM,
    next::{LedgerFootprint, ReadXdr, WriteXdr},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct SorobanDataBuilder {
    data: stellar_xdr::next::SorobanTransactionData,
}

pub enum Either<L, R> {
    Left(L),
    Right(R),
}
// Define a trait for SorobanDataBuilder behavior
pub trait SorobanDataBuilderBehavior {
    fn append_footprint(
        &mut self,
        read_only: Vec<stellar_xdr::next::LedgerKey>,
        read_write: Vec<stellar_xdr::next::LedgerKey>,
    ) -> &mut Self;
    fn set_resources(&mut self, instructions: u32, read_bytes: u32, write_bytes: u32) -> &mut Self;
    fn new(soroban_data: Option<Either<String, stellar_xdr::next::SorobanTransactionData>>)
        -> Self;
    fn from_xdr(data: Either<String, Vec<u8>>) -> stellar_xdr::next::SorobanTransactionData;
    fn set_footprint(
        &mut self,
        read_only: Option<Vec<stellar_xdr::next::LedgerKey>>,
        read_write: Option<Vec<stellar_xdr::next::LedgerKey>>,
    ) -> &mut Self;
    fn set_refundable_fee(&mut self, fee: i64) -> &mut Self;
    fn set_read_only(&mut self, read_only: Vec<stellar_xdr::next::LedgerKey>) -> &mut Self;
    fn set_read_write(&mut self, read_write: Vec<stellar_xdr::next::LedgerKey>) -> &mut Self;
    fn get_read_only(&self) -> &Vec<stellar_xdr::next::LedgerKey>;
    fn get_read_write(&self) -> Vec<stellar_xdr::next::LedgerKey>;
    fn build(&self) -> stellar_xdr::next::SorobanTransactionData;
    fn get_footprint(&self) -> &stellar_xdr::next::LedgerFootprint;
}
impl SorobanDataBuilderBehavior for SorobanDataBuilder {
    fn new(
        soroban_data: Option<Either<String, stellar_xdr::next::SorobanTransactionData>>,
    ) -> Self {
        let data = match soroban_data {
            Some(Either::Left(encoded_data)) => {
                if encoded_data.is_empty() {
                    // Return default empty data for empty string
                    stellar_xdr::next::SorobanTransactionData {
                        ext: stellar_xdr::next::ExtensionPoint::V0,
                        resources: stellar_xdr::next::SorobanResources {
                            footprint: LedgerFootprint {
                                read_only: Vec::new().try_into().unwrap(),
                                read_write: Vec::new().try_into().unwrap(),
                            },
                            instructions: 0,
                            read_bytes: 0,
                            write_bytes: 0,
                        },
                        resource_fee: 0,
                    }
                } else {
                    // Only try to parse non-empty strings
                    SorobanDataBuilder::from_xdr(Either::Left(encoded_data))
                }
            }
            Some(Either::Right(data_instance)) => SorobanDataBuilder::from_xdr(Either::Left(
                data_instance
                    .to_xdr_base64(stellar_xdr::next::Limits::none())
                    .unwrap(),
            )),
            None => stellar_xdr::next::SorobanTransactionData {
                ext: stellar_xdr::next::ExtensionPoint::V0,
                resources: stellar_xdr::next::SorobanResources {
                    footprint: LedgerFootprint {
                        read_only: Vec::new().try_into().unwrap(),
                        read_write: Vec::new().try_into().unwrap(),
                    },
                    instructions: 0,
                    read_bytes: 0,
                    write_bytes: 0,
                    // extended_meta_data_size_bytes: 0,
                },
                resource_fee: 0,
            },
        };

        Self { data }
    }

    fn from_xdr(data: Either<String, Vec<u8>>) -> stellar_xdr::next::SorobanTransactionData {
        match data {
            Either::Left(encoded) => stellar_xdr::next::SorobanTransactionData::from_xdr_base64(
                encoded,
                stellar_xdr::next::Limits::none(),
            )
            .unwrap(),
            Either::Right(raw) => stellar_xdr::next::SorobanTransactionData::from_xdr(
                raw,
                stellar_xdr::next::Limits::none(),
            )
            .unwrap(),
        }
    }

    fn append_footprint(
        &mut self,
        read_only: Vec<stellar_xdr::next::LedgerKey>,
        read_write: Vec<stellar_xdr::next::LedgerKey>,
    ) -> &mut Self {
        // Get current footprints
        let mut current_read_only = self.get_read_only().clone();
        let mut current_read_write = self.get_read_write();

        // Append new keys
        current_read_only.extend(read_only);
        current_read_write.extend(read_write);

        // Set the combined footprints
        self.set_footprint(Some(current_read_only), Some(current_read_write))
    }

    fn set_footprint(
        &mut self,
        read_only: Option<Vec<stellar_xdr::next::LedgerKey>>,
        read_write: Option<Vec<stellar_xdr::next::LedgerKey>>,
    ) -> &mut Self {
        if let Some(ros) = read_only {
            self.set_read_only(ros);
        }
        if let Some(rws) = read_write {
            self.set_read_write(rws);
        }
        self
    }

    fn set_refundable_fee(&mut self, fee: i64) -> &mut Self {
        self.data.resource_fee = fee;
        self
    }

    fn set_read_only(&mut self, read_only: Vec<stellar_xdr::next::LedgerKey>) -> &mut Self {
        self.data.resources.footprint.read_only = read_only.try_into().unwrap();
        self
    }

    fn set_read_write(&mut self, read_write: Vec<stellar_xdr::next::LedgerKey>) -> &mut Self {
        self.data.resources.footprint.read_write = read_write.try_into().unwrap();
        self
    }

    fn get_read_only(&self) -> &Vec<stellar_xdr::next::LedgerKey> {
        &self.data.resources.footprint.read_only
    }

    fn get_read_write(&self) -> Vec<stellar_xdr::next::LedgerKey> {
        self.data.resources.footprint.read_write.to_vec()
    }

    fn build(&self) -> stellar_xdr::next::SorobanTransactionData {
        stellar_xdr::next::SorobanTransactionData::from_xdr_base64(
            self.data
                .to_xdr_base64(stellar_xdr::next::Limits::none())
                .unwrap(),
            stellar_xdr::next::Limits::none(),
        )
        .unwrap()
    }

    fn get_footprint(&self) -> &stellar_xdr::next::LedgerFootprint {
        &self.data.resources.footprint
    }

    fn set_resources(&mut self, instructions: u32, read_bytes: u32, write_bytes: u32) -> &mut Self {
        self.data.resources.instructions = instructions;
        self.data.resources.read_bytes = read_bytes;
        self.data.resources.write_bytes = write_bytes;
        self
    }
}
#[cfg(test)]
mod tests {
    use crate::contract::{ContractBehavior, Contracts};

    use super::*;
    use stellar_xdr::next::{
        ExtensionPoint, LedgerFootprint, LedgerKey, Limits, SorobanResources,
        SorobanTransactionData,
    };

    #[test]
    fn test_constructs_from_xdr_base64_and_nothing() {
        // Create sentinel data that matches the JS test
        let sentinel = SorobanTransactionData {
            ext: ExtensionPoint::V0,
            resources: SorobanResources {
                footprint: LedgerFootprint {
                    read_only: Vec::new().try_into().unwrap(),
                    read_write: Vec::new().try_into().unwrap(),
                },
                instructions: 1,
                read_bytes: 2,
                write_bytes: 3,
            },
            resource_fee: 5,
        };

        // Test construction from nothing (equivalent to new dataBuilder())
        let _ = SorobanDataBuilder::new(None);

        // Test construction from raw XDR (equivalent to fromRaw)
        let from_raw = SorobanDataBuilder::new(Some(Either::Right(sentinel.clone()))).build();
        assert_eq!(from_raw, sentinel);

        // Test construction from base64 string (equivalent to fromStr)
        let base64_str = sentinel.to_xdr_base64(Limits::none()).unwrap();
        let from_str = SorobanDataBuilder::new(Some(Either::Left(base64_str))).build();
        assert_eq!(from_str, sentinel);

        // Create baseline for falsy comparison
        let baseline = SorobanDataBuilder::new(None).build();

        // Test with falsy values
        let empty_string = SorobanDataBuilder::new(Some(Either::Left(String::new()))).build();
        assert_eq!(empty_string, baseline);

        // Note: null and 0 don't need separate tests in Rust due to the type system
        // In Rust, we handle this through the Option type in the constructor
        let none_value = SorobanDataBuilder::new(None).build();
        assert_eq!(none_value, baseline);
    }

    #[test]
    fn test_sets_properties_as_expected() {
        // Create sentinel data
        let sentinel = SorobanTransactionData {
            ext: ExtensionPoint::V0,
            resources: SorobanResources {
                footprint: LedgerFootprint {
                    read_only: Vec::new().try_into().unwrap(),
                    read_write: Vec::new().try_into().unwrap(),
                },
                instructions: 1,
                read_bytes: 2,
                write_bytes: 3,
            },
            resource_fee: 5,
        };

        // Test setting resources and resource fee
        let mut binding = SorobanDataBuilder::new(None);
        let builder = binding.set_resources(1, 2, 3).set_refundable_fee(5);
        assert_eq!(builder.build(), sentinel);

        let contract_id = "CA3D5KRYM6CB7OWQ6TWYRR3Z4T7GNZLKERYNZGGA5SOAOPIFY6YQGAXE";
        let c = Contracts::new(contract_id).unwrap();
        let key = c.get_footprint();

        let with_footprint = SorobanDataBuilder::new(None)
            .set_footprint(Some(vec![key.clone()]), Some(vec![key.clone()]))
            .build();
        assert_eq!(with_footprint.resources.footprint.read_only[0], key);
        assert_eq!(with_footprint.resources.footprint.read_write[0], key);
    }

    #[test]
    fn test_leaves_untouched_footprints_untouched() {
        use crate::contract::{ContractBehavior, Contracts};

        // Create a contract key for testing
        let contract_id = "CA3D5KRYM6CB7OWQ6TWYRR3Z4T7GNZLKERYNZGGA5SOAOPIFY6YQGAXE";
        let c = Contracts::new(contract_id).unwrap();
        let key = c.get_footprint();

        // First builder - set both read_only and read_write footprints
        let mut builder = SorobanDataBuilder::new(None);
        let data = builder
            .set_footprint(Some(vec![key.clone()]), Some(vec![key.clone()]))
            .build();

        // Second builder - constructed from first data, only modify read_write
        let data2 = SorobanDataBuilder::new(Some(Either::Right(data.clone())))
            .set_footprint(None, Some(vec![]))
            .build();

        // Verify first data has both footprints set
        assert_eq!(data.resources.footprint.read_only.len(), 1);
        assert_eq!(data.resources.footprint.read_write.len(), 1);
        assert_eq!(data.resources.footprint.read_only[0], key);
        assert_eq!(data.resources.footprint.read_write[0], key);

        // Verify second data preserved read_only but cleared read_write
        assert_eq!(data2.resources.footprint.read_only.len(), 1);
        assert_eq!(data2.resources.footprint.read_write.len(), 0);
        assert_eq!(data2.resources.footprint.read_only[0], key);
    }
    //TODO: Remaining Tests

    #[test]
    fn test_appends_footprints() {
        // Create a contract key for testing
        let contract_id = "CA3D5KRYM6CB7OWQ6TWYRR3Z4T7GNZLKERYNZGGA5SOAOPIFY6YQGAXE";
        let c = Contracts::new(contract_id).unwrap();
        let key = c.get_footprint();

        // Create builder and chain operations
        let mut builder = SorobanDataBuilder::new(None);
        builder
            .set_footprint(Some(vec![key.clone()]), Some(vec![key.clone()]))
            .append_footprint(vec![key.clone(), key.clone()], vec![]);

        // Test the builder's current state
        assert_eq!(builder.get_read_only().len(), 3);
        assert_eq!(builder.get_read_write().len(), 1);

        // Verify read_only contains three copies of the key
        assert_eq!(builder.get_read_only()[0], key);
        assert_eq!(builder.get_read_only()[1], key);
        assert_eq!(builder.get_read_only()[2], key);

        // Verify read_write contains one copy of the key
        assert_eq!(builder.get_read_write()[0], key);

        // Build and verify the final state
        let built = builder.build();

        // Verify the built data has the same footprint structure
        assert_eq!(built.resources.footprint.read_only.len(), 3);
        assert_eq!(built.resources.footprint.read_write.len(), 1);

        assert_eq!(built.resources.footprint.read_only[0], key);
        assert_eq!(built.resources.footprint.read_only[1], key);
        assert_eq!(built.resources.footprint.read_only[2], key);
        assert_eq!(built.resources.footprint.read_write[0], key);
    }

    #[test]
    fn test_makes_copies_on_build() {
        // Create a builder
        let mut builder = SorobanDataBuilder::new(None);

        // Get first build
        let first = builder.build();

        // Modify builder and get second build
        let second = builder.set_refundable_fee(100).build();

        // Verify that the first build wasn't affected by later modifications
        assert_ne!(first.resource_fee, second.resource_fee);

        // Additional verification of exact values
        assert_eq!(first.resource_fee, 0); // Default value
        assert_eq!(second.resource_fee, 100); // Modified value
    }
}
