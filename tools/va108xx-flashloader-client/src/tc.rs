use spacepackets::{CcsdsPacketCreatorOwned, SpHeader};

pub fn create_tc(request: &models::Request) -> CcsdsPacketCreatorOwned {
    let req_raw = postcard::to_allocvec(&request).unwrap();
    let sp_header = SpHeader::new_from_apid(models::APID);
    CcsdsPacketCreatorOwned::new_tc_with_checksum(sp_header, &req_raw).unwrap()
}

pub fn create_tc_with_additional_payload(request: &models::Request, payload: &[u8]) -> CcsdsPacketCreatorOwned {
    let mut req_raw = postcard::to_allocvec(&request).unwrap();
    req_raw.extend_from_slice(payload);
    let sp_header = SpHeader::new_from_apid(models::APID);
    CcsdsPacketCreatorOwned::new_tc_with_checksum(sp_header, &req_raw).unwrap()
}
