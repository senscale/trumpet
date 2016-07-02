extern crate uuid;

use super::common::HadoopConf;
use super::common::HadoopError;

use ::proto::common::IpcConnectionContext;
use ::proto::common::RpcHeader;

use protobuf::*;

use std::net::TcpStream;
use std::io::prelude::*;

const PROTO_VERSION : u8 = 9;
const PROTO_SERVICE_CLASS : u8 = 0;
const PROTO_AUTH : u8 = 0;


pub struct Hdfs {
  conf: HadoopConf,
}

// Helpers

fn write_fixed_length_delimited_to_bytes<T : Message>(msg : &T) -> Vec<u8> {
  let mut result : Vec<u8> = Vec::new();
  let size = &u32_to_bytes(msg.compute_size());
  let bytes = &mut msg.write_to_bytes().unwrap();
  result.extend_from_slice(size);
  result.append(bytes);
  result
}

// FIXME: Endianness!!
fn u32_to_bytes(n : u32) -> [u8; 4] {
  [(n >> 24) as u8, (n >> 16) as u8, (n >> 8) as u8, n as u8 ]
}

// -Helpers

impl Hdfs {

  pub fn new(conf : &HadoopConf) -> Hdfs  {
    Hdfs { conf: HadoopConf }
  }


  pub fn get_fs_stats(&self) -> Result<i32, HadoopError> {


    let mut stream = try!(TcpStream::connect("127.0.0.1:9000")
                          .map_err(HadoopError::ConnectionError));

    // Start RPC Handshake

    let mut handshake_msg : Vec<u8> = Vec::new();

    handshake_msg.extend_from_slice("hrpc".as_bytes());
    handshake_msg.extend_from_slice(&[PROTO_VERSION, PROTO_SERVICE_CLASS, PROTO_AUTH]);

    try!(stream.write_all(&handshake_msg)
         .map_err(HadoopError::ConnectionError));

    // RpcRequestHeader

    let mut rpc_request = RpcHeader::RpcRequestHeaderProto::new();
    rpc_request.set_rpcKind(RpcHeader::RpcKindProto::RPC_PROTOCOL_BUFFER);
    rpc_request.set_rpcOp(RpcHeader::RpcRequestHeaderProto_OperationProto::RPC_FINAL_PACKET);
    rpc_request.set_callId(-3);
    rpc_request.set_clientId(uuid::Uuid::new_v4().as_bytes().to_vec());

    // IpcConnectionContext

    let mut ipc_conn = IpcConnectionContext::IpcConnectionContextProto::new();

    let mut ipc_conn_user = IpcConnectionContext::UserInformationProto::new();
    ipc_conn_user.set_realUser("david".to_string());
    ipc_conn_user.set_effectiveUser("david".to_string());

    ipc_conn.set_userInfo(ipc_conn_user);
    ipc_conn.set_protocol("org.apache.hadoop.hdfs.protocol.ClientProtocol".to_string());

    let total_length = rpc_request.compute_size() + ipc_conn.compute_size();

    let mut rpc_conn_msg : Vec<u8> = Vec::new();

    // Add the sum of the RpcRequestHeader and IpcConnectionContext lengths
    rpc_conn_msg.extend_from_slice(&u32_to_bytes(total_length));

    // Add length delimited RpcRequestHeaderProto
    rpc_conn_msg.extend_from_slice(&rpc_request.write_length_delimited_to_bytes().unwrap());

    // Add length delimited IpcConnectionContextProto
    rpc_conn_msg.extend_from_slice(&ipc_conn.write_length_delimited_to_bytes().unwrap());

    try!(stream.write_all(&rpc_conn_msg)
         .map_err(HadoopError::ConnectionError));

    // ---- -- --- -- Actual message -- -- - -- --- ----



    // ---- - -- ------ Get Response ------ ----- ------
    let mut result = [0u8; 32];
    let x = try!(stream.read(&mut result));
    println!("{:?}: {:?}", x, &result);


    Ok(42)
  }

}
