use openbrush::contracts::traits::psp22::extensions::wrapper::*;
use openbrush::contracts::traits::psp34::extensions::enumerable::*;
use openbrush::contracts::traits::psp37::extensions::enumerable::*;

#[openbrush::trait_definition]
pub trait GOV22: PSP22 + PSP22Wrapper {}
#[openbrush::wrapper]
pub type GOV22Ref = dyn PSP22 + PSP22Wrapper;

#[openbrush::trait_definition]
pub trait GOV34: PSP34 + PSP34Enumerable {}
#[openbrush::wrapper]
pub type GOV34Ref = dyn PSP34 + PSP34Enumerable;

#[openbrush::trait_definition]
pub trait GOV37: PSP37 + PSP37Enumerable {}
#[openbrush::wrapper]
pub type GOV37Ref = dyn PSP37 + PSP37Enumerable;
