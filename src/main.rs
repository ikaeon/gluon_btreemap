#[macro_use]
extern crate gluon_vm;

use std::fs::File; 
use std::io::prelude::*;
use std::env;
use gluon::import::add_extern_module;
use std::collections::BTreeMap;


use gluon::{
    new_vm,
    vm::{
        api::{
            IO,
        },
    },
    Thread, ThreadExt,
};

fn tupletest(a:i32) -> Vec<(String,i32)> {
  vec![("a".to_string(),a),("b".to_string(),a+1)]
}

fn btreemap(a:i32) -> BTreeMap<String,i32> {
  let mut r = BTreeMap::new();
//  r.insert(String::from("NAB"),a);
//  r.insert(String::from("NNAB"),a);
  r
}


fn load_btree_map(vm: &Thread) -> gluon::vm::Result<gluon::vm::ExternModule> {

    gluon::vm::ExternModule::new(vm, record!{btree => primitive!(1, btreemap), tupletest=>primitive!(1,tupletest)})
}

fn main() -> std::io::Result<()> {
  match env::args().nth(1){
    Some(glu_file) => { 
      let vm = new_vm();
      add_extern_module(&vm, "testb", load_btree_map);
      vm.get_database_mut().run_io(true);

      let mut source = String::new();
      let mut file = File::open(&glu_file)?;
      file.read_to_string(&mut source)?;
 
      match vm.run_expr::<IO<()>>("example", &source){
          Ok(_y) => Ok(()),
          Err(x) => {println!("{:?}",x); Ok(())}

      }

    },None => Ok(())
  }
}


