#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![doc = include_str!("../README.md")]

// lib

pub mod carddav;
pub mod http;
pub mod tcp;
pub mod tls;

// cli

pub mod account;
pub mod cli;
pub mod completion;
pub mod config;
pub mod manual;
