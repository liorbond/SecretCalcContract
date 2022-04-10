use cosmwasm_std::{to_binary, Api, Binary, Env, Extern, HandleResponse, InitResponse, Querier, StdResult, Storage, Uint128, HumanAddr};
use crate::msg::{HandleMsg, InitMsg, QueryMsg, HandleAnswer, QueryAnswer};
use crate::state::{may_load, save_eq, EquationVariables, UserCalculation};

pub fn init<S: Storage, A: Api, Q: Querier>(
    _deps: &mut Extern<S, A, Q>,
    _env: Env,
    _msg: InitMsg,
) -> StdResult<InitResponse> {
    Ok(InitResponse::default())
}

fn try_add<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    eq: EquationVariables,
) -> StdResult<HandleResponse> {
    let mut status = String::from("");
    let mut response = "".to_string();
    let non_checked_result = eq.x.u128().checked_mul(eq.y.u128());
    match non_checked_result {
        None => {
            status = String::from("Add overflow");
        }
        Some(result) => {
            response = result.to_string();

            let sender_address = deps.api.canonical_address(&env.message.sender)?;
            let stored_calculation = UserCalculation{
                eq,
                op: String::from("Add"),
                res: response.clone(),
                timestamp: Uint128::from(env.block.time)
            };

            save_eq(&mut deps.storage, &sender_address.as_slice().to_vec(), stored_calculation)?;
        }
    }

    Ok(HandleResponse {
        messages: vec![],
        log: vec![],
        data: Some(to_binary(&HandleAnswer::Add {
            status,
            res: response
        })?),
    })
}

fn try_sub<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    eq: EquationVariables,
) -> StdResult<HandleResponse> {
    let mut status = String::from("");
    let mut response = "".to_string();

    let non_checked_result = eq.x.u128().checked_sub(eq.y.u128());
    match non_checked_result {
        None => {
            status = String::from("Sub underflow");
        }
        Some(result) => {
            response = result.to_string();

            let sender_address = deps.api.canonical_address(&env.message.sender)?;
            let stored_calculation = UserCalculation{
                eq,
                op: String::from("Sub"),
                res: response.clone(),
                timestamp: Uint128::from(env.block.time)
            };

            save_eq(&mut deps.storage, &sender_address.as_slice().to_vec(), stored_calculation)?;
        }
    }

    Ok(HandleResponse {
        messages: vec![],
        log: vec![],
        data: Some(to_binary(&HandleAnswer::Sub {
            status,
            res: response
        })?),
    })
}

fn try_mul<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    eq: EquationVariables,
) -> StdResult<HandleResponse> {
    let mut status = String::from("");
    let mut response = String::from("");

    let non_checked_result = eq.x.u128().checked_mul(eq.y.u128());
    match non_checked_result {
        None => {
            status = String::from("Mul overflow");
        }
        Some(result) => {
            response = result.to_string();

            let sender_address = deps.api.canonical_address(&env.message.sender)?;
            let stored_calculation = UserCalculation{
                eq,
                op: String::from("Mul"),
                res: response.clone(),
                timestamp: Uint128::from(env.block.time)
            };

            save_eq(&mut deps.storage, &sender_address.as_slice().to_vec(), stored_calculation)?;
        }
    }

    Ok(HandleResponse {
        messages: vec![],
        log: vec![],
        data: Some(to_binary(&HandleAnswer::Mul {
            status,
            res: response
        })?),
    })
}

fn uint_parts_to_string(x : u128) -> String {
    return format!("{}.{}", x / 1000, x % 1000);
}

fn try_div<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    eq: EquationVariables,
) -> StdResult<HandleResponse> {
    let mut status = String::from("");
    let mut response = "".to_string();
    let non_checked_fractions = eq.x.u128().checked_mul(1000 as u128);
    match non_checked_fractions {
        None => {
            status = String::from(format!("Div supports only number in the range of 0 to {}", u128::MAX / 1000));
        }
        Some(result) => {
            let non_checked_result = result.checked_div(eq.y.u128());
            match non_checked_result {
                None => {
                    status = String::from("Can't divide by 0");
                }
                Some(div_result) => {
                    response = uint_parts_to_string(div_result);

                    let sender_address = deps.api.canonical_address(&env.message.sender)?;
                    let stored_calculation = UserCalculation{
                        eq,
                        op: String::from("Div"),
                        res: response.clone(),
                        timestamp: Uint128::from(env.block.time)
                    };

                    save_eq(&mut deps.storage, &sender_address.as_slice().to_vec(), stored_calculation)?;
                }
            }
        }
    }

    Ok(HandleResponse {
        messages: vec![],
        log: vec![],
        data: Some(to_binary(&HandleAnswer::Div {
            status,
            res: response
        })?),
    })
}

fn sqrt(x:u128) -> u128 {
    let mut z = (x+1) / (2 as u128);
    let mut y = x;

    while z < y {
        y = z;
        z = ((x / z) as u128 + z) as u128 / 2 as u128;
    }

    return y;
}

fn try_sqrt<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    x: Uint128,
) -> StdResult<HandleResponse> {
    let mut status = String::from("");
    let mut response = "".to_string();
    let non_checked_fractions = x.u128().checked_mul(1000000 as u128);
    match non_checked_fractions {
        None => {
            status = String::from(format!("Sqrt supports only number in the range of 0 to {}", u128::MAX / 1000000 as u128));
        }
        Some(result) => {
            let sqrt_result = sqrt(result);
            response = uint_parts_to_string(sqrt_result);

            let eq = EquationVariables{ x, y: Uint128::from(0 as u128) };
            let stored_calculation = UserCalculation{
                eq,
                op: String::from("Sqrt"),
                res: response.clone(),
                timestamp: Uint128::from(env.block.time)
            };

            let sender_address = deps.api.canonical_address(&env.message.sender)?;
            save_eq(&mut deps.storage, &sender_address.as_slice().to_vec(), stored_calculation)?;


        }
    }

    Ok(HandleResponse {
        messages: vec![],
        log: vec![],
        data: Some(to_binary(&HandleAnswer::Sqrt {
            status,
            res: response
        })?),
    })
}

fn sign_public_address(address : String) -> String {
    // Obviously it is not really an encrypted address, but it is out of scope here, alternatively we could sign the public address with the contract's private key.
    return address;
}

fn get_cookie<S: Storage, A: Api, Q: Querier>(
    _deps: &mut Extern<S, A, Q>,
    env: Env,
) -> StdResult<HandleResponse> {
    Ok(HandleResponse {
        messages: vec![],
        log: vec![],
        data: Some(to_binary(&HandleAnswer::GetCookie {
            status: String::from(""),
            cookie: sign_public_address(env.message.sender.to_string())
        })?),
    })
}


pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    match msg {
        HandleMsg::Add { eq} => try_add(deps, env, eq),
        HandleMsg::Sub { eq } => try_sub(deps, env, eq),
        HandleMsg::Mul { eq } => try_mul(deps, env, eq),
        HandleMsg::Div { eq } => try_div(deps, env, eq),
        HandleMsg::Sqrt { x } => try_sqrt(deps, env, x),
        HandleMsg::GetCookie {} => get_cookie(deps, env),
    }
}

fn try_get_address( signed_address: String ) -> Option<HumanAddr> {
    return Option::from(HumanAddr { 0: signed_address });
}

fn try_get_user_calculations<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    user_cookie : String
) -> StdResult<Binary> {
    let address = try_get_address(user_cookie);
    match address {
        Some(stored_address) => {
            let user_calculations = may_load(&deps.storage, &deps.api.canonical_address(&stored_address)?.as_slice().to_vec()).ok().unwrap();
            match user_calculations {
                Some(stored_user_calculations) => {
                    to_binary(&QueryAnswer::GetUserCalculations { status: String::from(""), calculations: stored_user_calculations})
                }
                None => {
                    to_binary(&QueryAnswer::GetUserCalculations { status: String::from("No user calculations"), calculations: Vec::new()})
                }
            }

        }
        None => {
            to_binary(&QueryAnswer::GetUserCalculations { status: String::from("Wrong cookie"), calculations: Vec::new() })
        }
    }
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetUserCalculations {user_cookie} => try_get_user_calculations(deps, user_cookie),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env};
    use cosmwasm_std::{Coin, from_binary};
    use crate::msg::{HandleMsg, InitMsg, QueryMsg, HandleAnswer, QueryAnswer};

    fn ensure_success(handle_result: &HandleAnswer) -> bool {
        match handle_result {
            HandleAnswer::GetCookie { status, cookie: _ }  => {
                let _success_res = "".to_string();
                matches!(status, _success_res)
            }
            HandleAnswer::Add { status , res: _}
            | HandleAnswer::Sub { status , res: _}
            | HandleAnswer::Mul { status , res: _}
            | HandleAnswer::Div { status , res: _}
            | HandleAnswer::Sqrt { status , res: _} => {
                let _success_res = "".to_string();
                return status == &_success_res
            }
        }
    }

    // Handle messages
    #[test]
    fn test_add() {
        let mut deps = mock_dependencies(20,  &[Coin {
                denom: "uscrt".to_string(),
                amount: Uint128(10000000),
        }]);

        let msg = InitMsg{};
        let env = mock_env("init", &[]);

        let _init_res = init(&mut deps, env, msg).unwrap();

        let msg = HandleMsg::Add { eq: EquationVariables { x: Uint128(10 as u128), y: Uint128(20 as u128) } };
        let env = mock_env("normal_calc", &[]);
        let calc_res: HandleAnswer = from_binary(&handle(&mut deps, env, msg).unwrap().data.unwrap()).unwrap();

        match calc_res {
            HandleAnswer::Add{ status: _, ref res } => {
                assert!(ensure_success(&calc_res));
                assert_eq!(res, &(10 as u128 + 20 as u128).to_string());
            }
            _ => panic!("HandleAnswer for Add should be Add"),
        }

        let msg = HandleMsg::Add { eq: EquationVariables { x: Uint128(10 as u128), y: Uint128(u128::MAX) } };
        let env = mock_env("overflow_calc", &[]);
        let calc_res: HandleAnswer = from_binary(&handle(&mut deps, env, msg).unwrap().data.unwrap()).unwrap();

        match calc_res {
            HandleAnswer::Add{ status: _, ref res } => {
                assert!(!ensure_success(&calc_res));
                assert_eq!(res, &"".to_string());
            }
            _ => panic!("HandleAnswer for Add should be Add"),
        }
    }

    #[test]
    fn test_sub() {
        let mut deps = mock_dependencies(20,  &[Coin {
            denom: "uscrt".to_string(),
            amount: Uint128(10000000),
        }]);

        let msg = InitMsg{};
        let env = mock_env("init", &[]);

        let _init_res = init(&mut deps, env, msg).unwrap();

        let msg = HandleMsg::Sub { eq: EquationVariables { x: Uint128(20 as u128), y: Uint128(10 as u128) } };
        let env = mock_env("normal_calc", &[]);
        let calc_res: HandleAnswer = from_binary(&handle(&mut deps, env, msg).unwrap().data.unwrap()).unwrap();

        match calc_res {
            HandleAnswer::Sub{ status: _, ref res } => {
                assert!(ensure_success(&calc_res));
                assert_eq!(res, &(20 as u128 - 10 as u128).to_string());
            }
            _ => panic!("HandleAnswer for Sub should be Sub"),
        }

        let msg = HandleMsg::Sub { eq: EquationVariables { x: Uint128(10 as u128), y: Uint128(u128::MAX) } };
        let env = mock_env("overflow_calc", &[]);
        let calc_res: HandleAnswer = from_binary(&handle(&mut deps, env, msg).unwrap().data.unwrap()).unwrap();

        match calc_res {
            HandleAnswer::Sub{ status: _, ref res } => {
                assert!(!ensure_success(&calc_res));
                assert_eq!(res, &"".to_string());
            }
            _ => panic!("HandleAnswer for Sub should be Sub"),
        }
    }

    #[test]
    fn test_mul() {
        let mut deps = mock_dependencies(20,  &[Coin {
            denom: "uscrt".to_string(),
            amount: Uint128(10000000),
        }]);

        let msg = InitMsg{};
        let env = mock_env("init", &[]);

        let _init_res = init(&mut deps, env, msg).unwrap();

        let msg = HandleMsg::Mul { eq: EquationVariables { x: Uint128(20 as u128), y: Uint128(10 as u128) } };
        let env = mock_env("normal_calc", &[]);
        let calc_res: HandleAnswer = from_binary(&handle(&mut deps, env, msg).unwrap().data.unwrap()).unwrap();

        match calc_res {
            HandleAnswer::Mul{ status: _, ref res } => {
                assert!(ensure_success(&calc_res));
                assert_eq!(res, &(20 as u128 * 10 as u128).to_string());
            }
            _ => panic!("HandleAnswer for Mul should be Mul"),
        }

        let msg = HandleMsg::Mul { eq: EquationVariables { x: Uint128(10 as u128), y: Uint128(u128::MAX) } };
        let env = mock_env("overflow_calc", &[]);
        let calc_res: HandleAnswer = from_binary(&handle(&mut deps, env, msg).unwrap().data.unwrap()).unwrap();

        match calc_res {
            HandleAnswer::Mul{ status: _, ref res } => {
                assert!(!ensure_success(&calc_res));
                assert_eq!(res, &"".to_string());
            }
            _ => panic!("HandleAnswer for Mul should be Mul"),
        }
    }

    #[test]
    fn test_div() {
        let mut deps = mock_dependencies(20,  &[Coin {
            denom: "uscrt".to_string(),
            amount: Uint128(10000000),
        }]);

        let msg = InitMsg{};
        let env = mock_env("init", &[]);

        let _init_res = init(&mut deps, env, msg).unwrap();

        let msg = HandleMsg::Div { eq: EquationVariables { x: Uint128(20 as u128), y: Uint128(10 as u128) } };
        let env = mock_env("normal_no_rem", &[]);
        let calc_res: HandleAnswer = from_binary(&handle(&mut deps, env, msg).unwrap().data.unwrap()).unwrap();

        match calc_res {
            HandleAnswer::Div{ status: _, ref res } => {
                assert!(ensure_success(&calc_res));
                assert_eq!(res, &"2.0".to_string());
            }
            _ => panic!("HandleAnswer for Div should be Div"),
        }

        let msg = HandleMsg::Div { eq: EquationVariables { x: Uint128(23 as u128), y: Uint128(3 as u128) } };
        let env = mock_env("normal_with_rem", &[]);
        let calc_res: HandleAnswer = from_binary(&handle(&mut deps, env, msg).unwrap().data.unwrap()).unwrap();

        match calc_res {
            HandleAnswer::Div{ status: _, ref res } => {
                assert!(ensure_success(&calc_res));
                assert_eq!(res, &"7.666".to_string());
            }
            _ => panic!("HandleAnswer for Div should be Div"),
        }

        let msg = HandleMsg::Div { eq: EquationVariables { x: Uint128(u128::MAX - 100), y: Uint128(4 as u128) } };
        let env = mock_env("overflow_fraq", &[]);
        let calc_res: HandleAnswer = from_binary(&handle(&mut deps, env, msg).unwrap().data.unwrap()).unwrap();

        match calc_res {
            HandleAnswer::Div{ status: _, ref res } => {
                assert!(!ensure_success(&calc_res));
                assert_eq!(res, &"".to_string());
            }
            _ => panic!("HandleAnswer for Div should be Div"),
        }

        let msg = HandleMsg::Div { eq: EquationVariables { x: Uint128(10 as u128), y: Uint128(0 as u128) } };
        let env = mock_env("overflow_calc", &[]);
        let calc_res: HandleAnswer = from_binary(&handle(&mut deps, env, msg).unwrap().data.unwrap()).unwrap();

        match calc_res {
            HandleAnswer::Div{ status: _, ref res } => {
                assert!(!ensure_success(&calc_res));
                assert_eq!(res, &"".to_string());
            }
            _ => panic!("HandleAnswer for Div should be Div"),
        }
    }

    #[test]
    fn test_sqrt() {
        let mut deps = mock_dependencies(20,  &[Coin {
            denom: "uscrt".to_string(),
            amount: Uint128(10000000),
        }]);

        let msg = InitMsg{};
        let env = mock_env("init", &[]);

        let _init_res = init(&mut deps, env, msg).unwrap();

        let msg = HandleMsg::Sqrt { x: Uint128(25 as u128) };
        let env = mock_env("normal_no_rem", &[]);
        let calc_res: HandleAnswer = from_binary(&handle(&mut deps, env, msg).unwrap().data.unwrap()).unwrap();

        match calc_res {
            HandleAnswer::Sqrt{ status: _, ref res } => {
                assert!(ensure_success(&calc_res));
                assert_eq!(res, &"5.0".to_string());
            }
            _ => panic!("HandleAnswer for Sqrt should be Sqrt"),
        }

        let msg = HandleMsg::Sqrt { x: Uint128(2 as u128) };
        let env = mock_env("normal_with_rem", &[]);
        let calc_res: HandleAnswer = from_binary(&handle(&mut deps, env, msg).unwrap().data.unwrap()).unwrap();

        match calc_res {
            HandleAnswer::Sqrt{ status: _, ref res } => {
                assert!(ensure_success(&calc_res));
                assert_eq!(res, &"1.414".to_string());
            }
            _ => panic!("HandleAnswer for Sqrt should be Sqrt"),
        }

        let msg = HandleMsg::Sqrt { x: Uint128(u128::MAX) };
        let env = mock_env("overflow_calc", &[]);
        let calc_res: HandleAnswer = from_binary(&handle(&mut deps, env, msg).unwrap().data.unwrap()).unwrap();

        match calc_res {
            HandleAnswer::Sqrt{ status: _, ref res } => {
                assert!(!ensure_success(&calc_res));
                assert_eq!(res, &"".to_string());
            }
            _ => panic!("HandleAnswer for Sqrt should be Sqrt"),
        }
    }

    // Query tests
    #[test]
    fn test_get_user_calculations() {
        let mut deps = mock_dependencies(20,  &[Coin {
            denom: "uscrt".to_string(),
            amount: Uint128(10000000),
        }]);

        let msg = InitMsg{};
        let env = mock_env("init", &[]);

        let _init_res = init(&mut deps, env, msg).unwrap();

        let msg = HandleMsg::GetCookie{};
        let env = mock_env("cookie", &[]);
        let res: HandleAnswer = from_binary(&handle(&mut deps, env.clone(), msg).unwrap().data.unwrap()).unwrap();

        match res {
            HandleAnswer::GetCookie{ status: _, ref cookie } => {
                assert!(ensure_success(&res));
                assert_eq!(&env.message.sender.to_string(), cookie);

                let msg = QueryMsg::GetUserCalculations {user_cookie: String::from(cookie)};

                let q_res: QueryAnswer = from_binary(&query(&mut deps,  msg).unwrap()).unwrap();
                match q_res {
                    QueryAnswer::GetUserCalculations {ref status, calculations} => {
                        assert_eq!(0, calculations.len());
                        assert!(!status.is_empty());
                    }
                }

                let msg = HandleMsg::Add { eq: EquationVariables { x: Uint128(10 as u128), y: Uint128(20 as u128) } };
                handle(&mut deps, env, msg).ok();

                let msg = QueryMsg::GetUserCalculations {user_cookie: String::from(cookie)};

                let q_res: QueryAnswer = from_binary(&query(&mut deps,  msg).unwrap()).unwrap();
                match q_res {
                    QueryAnswer::GetUserCalculations {ref status, calculations} => {
                        assert_eq!(1, calculations.len());
                        assert!(status.is_empty());
                    }
                }
            }
            _ => panic!("HandleAnswer for GetCookie should be GetCookie"),
        }
    }

}