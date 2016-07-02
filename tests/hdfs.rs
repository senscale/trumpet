extern crate trumpet;

use trumpet::common::HadoopConf;
use trumpet::hdfs::Hdfs;

#[test]
fn get_fs_stats() {

  let conf = HadoopConf::new();
  let hdfs = Hdfs::new(&conf);

  let stats = hdfs.get_fs_stats();

}
