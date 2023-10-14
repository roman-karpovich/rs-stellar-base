use stellar_xdr::{next::{ReadXdr, WriteXdr, LedgerFootprint}, curr::VecM};

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]

pub struct SorobanDataBuilder {
    data: stellar_xdr::next::SorobanTransactionData,
}

impl SorobanDataBuilder {
    pub fn new(soroban_data: Option<Either<String, stellar_xdr::next::SorobanTransactionData>>) -> Self {
        let data = match soroban_data {
            Some(Either::Left(encoded_data)) => SorobanDataBuilder::from_xdr(Either::Left(encoded_data)),
            Some(Either::Right(data_instance)) => SorobanDataBuilder::from_xdr(Either::Left(data_instance.to_xdr_base64().unwrap())),
            None => stellar_xdr::next::SorobanTransactionData {
                ext: stellar_xdr::next::ExtensionPoint::V0,
                resources: stellar_xdr::next::SorobanResources{
                    footprint: LedgerFootprint { read_only: Vec::new().try_into().unwrap(), read_write: Vec::new().try_into().unwrap()},
                    instructions: 0,
                    read_bytes: 0,
                    write_bytes: 0,
                    // extended_meta_data_size_bytes: 0,
                },
                refundable_fee: 0,
            },
        };

        Self { data }
    }

    fn from_xdr(data: Either<String, Vec<u8>>) -> stellar_xdr::next::SorobanTransactionData {
        match data {
            Either::Left(encoded) => stellar_xdr::next::SorobanTransactionData::from_xdr_base64(encoded).unwrap(),
            Either::Right(raw) => stellar_xdr::next::SorobanTransactionData::from_xdr(raw).unwrap(),
        }
    }

    // TODO: Append Footprint

    pub fn set_footprint(&mut self, read_only: Option<Vec<stellar_xdr::next::LedgerKey>>, read_write: Option<Vec<stellar_xdr::next::LedgerKey>>) -> &mut Self {
        if let Some(ros) = read_only {
            self.set_read_only(ros);
        }
        if let Some(rws) = read_write {
            self.set_read_write(rws);
        }
        self
    }

    pub fn set_refundable_fee(&mut self, fee: i64) -> &mut Self  {
        self.data.refundable_fee = fee;
        self
        
    }

    pub fn set_read_only(&mut self, read_only: Vec<stellar_xdr::next::LedgerKey>) -> &mut Self {
        self.data.resources.footprint.read_only = read_only.try_into().unwrap();
        self
    }

    pub fn set_read_write(&mut self, read_write: Vec<stellar_xdr::next::LedgerKey>) -> &mut Self {
        self.data.resources.footprint.read_write = read_write.try_into().unwrap();
        self
    }
   
    pub fn get_read_only(&self) -> &Vec<stellar_xdr::next::LedgerKey> {
        &self.data.resources.footprint.read_only
    }

    pub fn get_read_write(&self) -> Vec<stellar_xdr::next::LedgerKey> {
        self.data.resources.footprint.read_write.to_vec()
    }

    pub fn build(&self) -> stellar_xdr::next::SorobanTransactionData {
        
        stellar_xdr::next::SorobanTransactionData::from_xdr(self.data.to_xdr_base64().unwrap()).unwrap()
    }

    pub fn get_footprint(&self) -> &stellar_xdr::next::LedgerFootprint {
        &self.data.resources.footprint
    }

}

pub enum Either<L, R> {
    Left(L),
    Right(R),
}
