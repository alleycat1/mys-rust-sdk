// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// ===========================================================================
// MySons Queries
// ===========================================================================

use crate::query_types::schema;
use crate::query_types::Address as SdkAddress;

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema = "rpc",
    graphql_type = "Query",
    variables = "ResolveMySonsQueryArgs"
)]
pub struct ResolveMySonsQuery {
    #[arguments(domain: $name)]
    pub resolve_ns_address: Option<DomainAddress>,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct ResolveMySonsQueryArgs<'a> {
    pub name: &'a str,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "Address")]
pub struct DomainAddress {
    pub address: SdkAddress,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema = "rpc",
    graphql_type = "Query",
    variables = "DefaultMySonsNameQueryArgs"
)]
pub struct DefaultMySonsNameQuery {
    #[arguments(address: $address)]
    pub address: Option<AddressDefaultMySons>,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct DefaultMySonsNameQueryArgs {
    pub address: SdkAddress,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "rpc", graphql_type = "Address")]
pub struct AddressDefaultMySons {
    pub default_ns_name: Option<String>,
}
