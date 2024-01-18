
use std::borrow::Borrow;
use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Deref;
use std::ops::SubAssign;

use scrypto::prelude::*;
use transaction::manifest::ast::Instruction;
use transaction::prelude::*;

use radix_engine::system::system_modules::execution_trace::*;
use radix_engine_interface::blueprints::account::*;
use radix_engine_interface::blueprints::consensus_manager::*;
use radix_engine_common::prelude::rust::sync::Arc;

use crate::transaction_types::*;
use crate::utils::*;



#[derive(Default)]
enum WorktopTrustStatus {
    #[default]
    Uninitialized,
    Trusted,
    NotTrusted
}

#[derive(Clone, Debug)]
pub enum WorktopContentItem {
    Amount(Decimal),
    Ids(IndexSet<NonFungibleLocalId>)
}

impl From<ResourceIndicator> for WorktopContentItem {
    fn from(value: ResourceIndicator) -> Self {
        match ResourceSpecifier::from(value) {
            ResourceSpecifier::Amount(address, value) => WorktopContentItem::Amount(value),
            ResourceSpecifier::Ids(address, value) => WorktopContentItem::Ids(value),
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct WorktopContent {
    trusted: bool,
    content: HashMap<ResourceAddress, WorktopContentItem>,
}
impl WorktopContent {
    pub fn add(&mut self, resource_indicator: &ResourceIndicator) {
        self.content.entry(resource_indicator.resource_address()).and_modify(|value|{
            match value {
                WorktopContentItem::Amount(amount) => {
                    if let ResourceIndicator::Fungible(_,value) = resource_indicator {
                        amount.add_assign(*value.deref());
                    }
                }
                WorktopContentItem::Ids(list) => {
                    if let ResourceSpecifier::Ids(_,ids) = ResourceSpecifier::from(resource_indicator.clone()) {
                        list.extend(ids);
                    }
                }
            };
        }).or_insert(
            WorktopContentItem::from(resource_indicator.clone())
        );
    }
    pub fn remove(&mut self, resource_indicator: &ResourceIndicator) {
        if let Some(item) = self.content.get_mut(&resource_indicator.resource_address()) {
            match item {
                WorktopContentItem::Amount(amount) => {
                    if let ResourceIndicator::Fungible(_,value) = resource_indicator {
                        amount.sub_assign(*value.deref());
                        if amount.is_zero() {
                            self.content.remove(&resource_indicator.resource_address());
                        }
                    }
                }
                WorktopContentItem::Ids(list) => {
                    if let ResourceSpecifier::Ids(_,ids) = ResourceSpecifier::from(resource_indicator.clone()) {
                        list.retain(|item| !ids.contains(item));
                        if list.is_empty() {
                            self.content.remove(&resource_indicator.resource_address());
                        }
                    }
                }
            }
        }
    }
}
// impl Debug for WorktopContent {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.debug_struct("\nWorktopContent").field("trusted", &self.trusted).field("content", &self.content).finish()
//     }
// }

#[derive(Default)]
pub struct TrustedWorktop {
    state_per_instruction: Vec<WorktopContent>,
}

impl TrustedWorktop {
    fn add_new_instruction(&mut self) {
        let new_item = if let Some(item) = self.state_per_instruction.last() {
            item.clone()
        } else {
            WorktopContent {
                trusted: true,
                content: HashMap::new()
            }
        };

        self.state_per_instruction.push(new_item);
    }

    pub fn get_results(&mut self) -> Vec<WorktopContent> {
        self.state_per_instruction.clone()
    }
}

impl WorktopActionSubscriber for TrustedWorktop {
    fn action_called(&mut self, action: WorktopAction) {
        println!("WorktopActionSubscriber {:?}", action);
    }
}



impl ManifestSummaryCallback for TrustedWorktop {
}


impl ExecutionSummaryCallback for TrustedWorktop {

    fn on_instruction(
        &mut self,
        instruction: &InstructionV1,
        instruction_index: usize,
        _input_resources: &[ResourceSpecifier],
        _output_resources: &[ResourceSpecifier],
    ) {
        assert_eq!(instruction_index, self.state_per_instruction.len());
        self.add_new_instruction();

        println!( " ==> INSTRUCTION {}: {:?}", instruction_index, instruction);

        // match instruction {
        //     InstructionV1::
        // }
    }

    fn on_account_withdraw(
        &mut self,
        _account: &ComponentAddress,
        resource_indicator: &ResourceIndicator,
    ) {
        println!(" ==> WITHDRAW (put to worktop):     {:?}", resource_indicator);

        // put resource to worktop
        self.state_per_instruction.last_mut().expect("Must succeed").add(resource_indicator);
    }

    fn on_account_deposit(
        &mut self,
        _account: &ComponentAddress,
        resource_indicator: &ResourceIndicator,
    ) {
        println!(" ==> DEPOSIT (take from worktop):     {:?}", resource_indicator);

        // take resource from worktop
        self.state_per_instruction.last_mut().expect("Must succeed").remove(resource_indicator);
    }
}
