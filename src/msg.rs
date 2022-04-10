use cosmwasm_std::Uint128;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::state::EquationVariables;
use crate::state::UserCalculation;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    Add {
        eq: EquationVariables,
    },
    Sub {
        eq: EquationVariables,
    },
    Mul {
        eq: EquationVariables,
    },
    Div {
        eq: EquationVariables,
    },
    Sqrt {
        x: Uint128,
    },
    // "GetUserCalculations" don't change the state but it is user related so have to be authenticated.
    // I decided instead of making "GetUserCalculations" being a HandleMsg make a cookie to represent the identity of the user (To avoid authentication on every query)
    GetCookie {}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetUserCalculations {
        user_cookie: String,
    }
}

/// Responses from handle function
#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleAnswer {
    Add {
        status: String,
        res: String,
    },
    Sub {
        status: String,
        res: String,
    },
    Mul {
        status: String,
        res: String,
    },
    Div {
        status: String,
        res: String,
    },
    Sqrt {
        status: String,
        res: String,
    },
    GetCookie {
        status: String,
        cookie: String,
    }
}

/// Responses from query function
#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryAnswer {
    GetUserCalculations {
        status: String,
        calculations: Vec<UserCalculation>,
    }
}