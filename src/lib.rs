#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![doc = include_str!("../README.md")]

pub mod carddav;
pub mod cli;
pub mod completion;
pub mod http;
pub mod manual;
pub mod tcp;
pub mod tls;
