mod state_test;

use std::error::Error;
use serde::{Deserialize, Serialize};
use tokio::time::{sleep, Duration};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct State {
    box_id: String,
    next_box_id: Option<String>,
    last_interaction_id: String,
    retry_count: u8,
    routing_script_id: String,
    tenant_number: String,
    timeout_retry_count: u8,
    error_retry_count: u8,
    box_number: u16,
}

impl State {
    // Constructor method to create a new State instance
    pub fn new(
        box_id: String,
        next_box_id: Option<String>,
        last_interaction_id: String,
        retry_count: u8,
        routing_script_id: String,
        tenant_number: String,
        timeout_retry_count: u8,
        error_retry_count: u8,
        box_number: u16,
    ) -> Self {
        Self {
            box_id,
            next_box_id,
            last_interaction_id,
            retry_count,
            routing_script_id,
            tenant_number,
            timeout_retry_count,
            error_retry_count,
            box_number,
        }
    }

    // Cannot assign a new value to `field of immutable binding
    // so the self needs to be mutable
    pub fn set_box_id(&mut self,new_id:String){
        self.box_id = new_id;
    }
}

async fn async_function() {
    println!("Async function started!");
    sleep(Duration::from_secs(2)).await;
    println!("Async function completed after 2 seconds!");
}

use std::result::Result as StdResult;
pub type Result<T> = StdResult<T, Box<dyn Error>>;

async fn construct_state() -> Result<State>{
    sleep(Duration::from_secs(1)).await;
    let mut state = State::new(
        "box_123".to_string(),
        Some("box_456".to_string()),
        "interaction_789".to_string(),
        3,
        "script_012".to_string(),
        "tenant_345".to_string(),
        2,
        1,
        654,
    );
    Ok(state)
}

async fn construct_state_() -> Result<Option<State>>{
    let mut state = State::new(
        "box_123".to_string(),
        Some("box_456".to_string()),
        "interaction_789".to_string(),
        3,
        "script_012".to_string(),
        "tenant_345".to_string(),
        2,
        1,
        654,
    );
    Ok(Some(state))
}

async fn get_state() -> Result<State>{
    // match construct_state().await {
    //     Ok(state) => {
    //         // println!("State created: {:?}", &state);
    //         Ok(state)
    //     }
    //     Err(err) => {
    //         println!("Error constructing state: {}", err);
    //         Err(err)
    //     }
    // }
    if let Some(state) = construct_state_().await? {
        Ok(state)
    } else {
        construct_state().await // conversation does not exist and start the
    }
}

#[tokio::main]
async fn main(){
    println!("Starting main function!");
    let mut rstate = get_state().await;
    if rstate.is_err(){}
    let mut state = rstate.unwrap();
    println!("{}", format!("latest State1 {:?}", &state));
    fun1(&mut state);
    fun2(&mut state);
    println!("{}", format!("latest State2 {:?}", &state));
    println!("Main function completed!");
}

fn fun1(state: &mut State){
    //fun2(state);
}

fn fun2(state: &mut State){

}