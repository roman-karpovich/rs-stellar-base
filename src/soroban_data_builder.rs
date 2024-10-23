use stellar_xdr::{
    curr::VecM,
    next::{LedgerFootprint, ReadXdr, WriteXdr},
};

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]

pub struct SorobanDataBuilder {
    data: stellar_xdr::next::SorobanTransactionData,
}

pub enum Either<L, R> {
    Left(L),
    Right(R),
}
// Define a trait for SorobanDataBuilder behavior
pub trait SorobanDataBuilderBehavior {
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

    // TODO: Append Footprint

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
}
#[cfg(test)]
mod tests {
    use super::*;
    use stellar_xdr::next::{
        ExtensionPoint, LedgerFootprint, SorobanResources, SorobanTransactionData, Limits,
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

    //TODO: Remaining Tests
}