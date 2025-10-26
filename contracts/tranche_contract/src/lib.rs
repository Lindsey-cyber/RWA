#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, token, Address, BytesN, Env, Map, Symbol,
};

/// Tranche enum
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TrancheType {
    Senior,
    Junior,
}

mod keys {
    pub const ADMIN: &str = "admin";
    pub const TOKEN: &str = "token";
    pub const POOL: &str = "pool";
    pub const S_TOT: &str = "s_total";
    pub const J_TOT: &str = "j_total";
    pub const S_MAP: &str = "s_map";
    pub const J_MAP: &str = "j_map";
    pub const MIN_S: &str = "min_sen";
    pub const MIN_J: &str = "min_jun";
    pub const INITED: &str = "inited";
    pub const PAUSED: &str = "paused";
}

#[contract]
pub struct TrancheContract;

#[contractimpl]
impl TrancheContract {
    /// 初始化合约
    pub fn initialize(
        env: Env,
        admin: Address,
        token_contract_id: Address,
        pool_contract_id: Address,
        min_senior: i128,
        min_junior: i128,
    ) {
        let inited_sym = symbol_short!("inited");
        if env
            .storage()
            .persistent()
            .get::<_, bool>(&inited_sym)
            .is_some()
        {
            panic!("already initialized");
        }

        env.storage().persistent().set(&symbol_short!("admin"), &admin);
        env.storage().persistent().set(&symbol_short!("token"), &token_contract_id);
        env.storage().persistent().set(&symbol_short!("pool"), &pool_contract_id);

        env.storage().persistent().set(&symbol_short!("s_total"), &0i128);
        env.storage().persistent().set(&symbol_short!("j_total"), &0i128);

        let senior_map: Map<Address, i128> = Map::new(&env);
        let junior_map: Map<Address, i128> = Map::new(&env);
        env.storage().persistent().set(&symbol_short!("s_map"), &senior_map);
        env.storage().persistent().set(&symbol_short!("j_map"), &junior_map);

        let min_s = if min_senior >= 0 { min_senior } else { 0i128 };
        let min_j = if min_junior >= 0 { min_junior } else { 0i128 };
        env.storage().persistent().set(&symbol_short!("min_sen"), &min_s);
        env.storage().persistent().set(&symbol_short!("min_jun"), &min_j);

        env.storage().persistent().set(&symbol_short!("paused"), &false);
        env.storage().persistent().set(&inited_sym, &true);
    }

    /// 暂停/恢复合约
    pub fn set_paused(env: Env, admin: Address, paused: bool) {
        admin.require_auth();
        Self::require_admin(&env, &admin);
        env.storage().persistent().set(&symbol_short!("paused"), &paused);
        env.events().publish((symbol_short!("pause"),), (paused,));
    }

    /// 更新最低投资额
    pub fn set_minimums(env: Env, admin: Address, min_senior: i128, min_junior: i128) {
        admin.require_auth();
        Self::require_admin(&env, &admin);
        
        let min_s = if min_senior >= 0 { min_senior } else { 0i128 };
        let min_j = if min_junior >= 0 { min_junior } else { 0i128 };
        env.storage().persistent().set(&symbol_short!("min_sen"), &min_s);
        env.storage().persistent().set(&symbol_short!("min_jun"), &min_j);

        env.events().publish((symbol_short!("minupd"),), (&admin, &min_s, &min_j));
    }

    /// 认购份额（实际转账）
    pub fn subscribe(env: Env, from: Address, tranche: TrancheType, amount: i128) {
        from.require_auth();
        Self::require_not_paused(&env);

        if amount <= 0 {
            panic!("amount must > 0");
        }

        // 检查最低投资额
        let (min_s, min_j) = Self::get_minimums(env.clone());
        match tranche {
            TrancheType::Senior => {
                if amount < min_s {
                    panic!("amount below senior minimum");
                }
            }
            TrancheType::Junior => {
                if amount < min_j {
                    panic!("amount below junior minimum");
                }
            }
        }

        // 实际转账：从用户转到合约
        let token_address: Address = env
            .storage()
            .persistent()
            .get(&symbol_short!("token"))
            .expect("not initialized");
        let token_client = token::Client::new(&env, &token_address);
        
        token_client.transfer(&from, &env.current_contract_address(), &amount);

        // 更新账本
        match tranche {
            TrancheType::Senior => {
                let key_tot = symbol_short!("s_total");
                let mut st: i128 = env
                    .storage()
                    .persistent()
                    .get(&key_tot)
                    .unwrap_or(0i128);
                st = st.checked_add(amount).expect("overflow on senior total");
                env.storage().persistent().set(&key_tot, &st);

                let key_map = symbol_short!("s_map");
                let mut m: Map<Address, i128> = env
                    .storage()
                    .persistent()
                    .get(&key_map)
                    .unwrap_or(Map::new(&env));
                let prev = m.get(from.clone()).unwrap_or(0i128);
                m.set(from.clone(), prev.checked_add(amount).expect("overflow on user balance"));
                env.storage().persistent().set(&key_map, &m);
            }
            TrancheType::Junior => {
                let key_tot = symbol_short!("j_total");
                let mut jt: i128 = env
                    .storage()
                    .persistent()
                    .get(&key_tot)
                    .unwrap_or(0i128);
                jt = jt.checked_add(amount).expect("overflow on junior total");
                env.storage().persistent().set(&key_tot, &jt);

                let key_map = symbol_short!("j_map");
                let mut m: Map<Address, i128> = env
                    .storage()
                    .persistent()
                    .get(&key_map)
                    .unwrap_or(Map::new(&env));
                let prev = m.get(from.clone()).unwrap_or(0i128);
                m.set(from.clone(), prev.checked_add(amount).expect("overflow on user balance"));
                env.storage().persistent().set(&key_map, &m);
            }
        }

        env.events().publish((symbol_short!("sub"),), (&from, &tranche, &amount));
    }

    /// 赎回份额（用户主动提现本金）
    pub fn redeem(env: Env, from: Address, tranche: TrancheType, amount: i128) {
        from.require_auth();
        Self::require_not_paused(&env);

        if amount <= 0 {
            panic!("amount must > 0");
        }

        // 检查用户余额并更新
        match tranche {
            TrancheType::Senior => {
                let key_map = symbol_short!("s_map");
                let mut m: Map<Address, i128> = env
                    .storage()
                    .persistent()
                    .get(&key_map)
                    .unwrap_or(Map::new(&env));
                let balance = m.get(from.clone()).unwrap_or(0i128);
                if balance < amount {
                    panic!("insufficient senior balance");
                }
                m.set(from.clone(), balance.checked_sub(amount).expect("underflow"));
                env.storage().persistent().set(&key_map, &m);

                let key_tot = symbol_short!("s_total");
                let mut st: i128 = env.storage().persistent().get(&key_tot).unwrap_or(0i128);
                st = st.checked_sub(amount).expect("underflow on senior total");
                env.storage().persistent().set(&key_tot, &st);
            }
            TrancheType::Junior => {
                let key_map = symbol_short!("j_map");
                let mut m: Map<Address, i128> = env
                    .storage()
                    .persistent()
                    .get(&key_map)
                    .unwrap_or(Map::new(&env));
                let balance = m.get(from.clone()).unwrap_or(0i128);
                if balance < amount {
                    panic!("insufficient junior balance");
                }
                m.set(from.clone(), balance.checked_sub(amount).expect("underflow"));
                env.storage().persistent().set(&key_map, &m);

                let key_tot = symbol_short!("j_total");
                let mut jt: i128 = env.storage().persistent().get(&key_tot).unwrap_or(0i128);
                jt = jt.checked_sub(amount).expect("underflow on junior total");
                env.storage().persistent().set(&key_tot, &jt);
            }
        }

        // 实际转账：从合约转给用户
        let token_address: Address = env
            .storage()
            .persistent()
            .get(&symbol_short!("token"))
            .expect("not initialized");
        let token_client = token::Client::new(&env, &token_address);
        token_client.transfer(&env.current_contract_address(), &from, &amount);

        env.events().publish((symbol_short!("redeem"),), (&from, &tranche, &amount));
    }

    /// 分配收益（实际转账给用户）
    pub fn notify_pool_payout(env: Env, caller: Address, amount: i128) {
        caller.require_auth();
        Self::require_admin(&env, &caller);

        if amount <= 0 {
            panic!("amount must > 0");
        }

        let st: i128 = env
            .storage()
            .persistent()
            .get(&symbol_short!("s_total"))
            .unwrap_or(0i128);
        let jt: i128 = env
            .storage()
            .persistent()
            .get(&symbol_short!("j_total"))
            .unwrap_or(0i128);
        let tot = st.checked_add(jt).unwrap_or(0i128);
        
        if tot == 0 {
            env.events().publish((symbol_short!("rfd"),), (&caller, &amount));
            return;
        }

        // 计算瀑布分配
        let senior_amount = amount.checked_mul(st).expect("overflow") / tot;
        let junior_amount = amount.checked_sub(senior_amount).expect("underflow");

        let token_address: Address = env
            .storage()
            .persistent()
            .get(&symbol_short!("token"))
            .expect("not initialized");
        let token_client = token::Client::new(&env, &token_address);

        // 分配给 Senior 持有人
        if senior_amount > 0 && st > 0 {
            let senior_map: Map<Address, i128> = env
                .storage()
                .persistent()
                .get(&symbol_short!("s_map"))
                .unwrap_or(Map::new(&env));
            
            for addr in senior_map.keys() {
                let hold = senior_map.get(addr.clone()).unwrap_or(0i128);
                if hold == 0 {
                    continue;
                }
                let pay = senior_amount.checked_mul(hold).expect("overflow") / st;
                if pay > 0 {
                    token_client.transfer(&env.current_contract_address(), &addr, &pay);
                    env.events().publish(
                        (symbol_short!("pay"),),
                        (addr.clone(), TrancheType::Senior, pay),
                    );
                }
            }
        }

        // 分配给 Junior 持有人
        if junior_amount > 0 && jt > 0 {
            let junior_map: Map<Address, i128> = env
                .storage()
                .persistent()
                .get(&symbol_short!("j_map"))
                .unwrap_or(Map::new(&env));
            
            for addr in junior_map.keys() {
                let hold = junior_map.get(addr.clone()).unwrap_or(0i128);
                if hold == 0 {
                    continue;
                }
                let pay = junior_amount.checked_mul(hold).expect("overflow") / jt;
                if pay > 0 {
                    token_client.transfer(&env.current_contract_address(), &addr, &pay);
                    env.events().publish(
                        (symbol_short!("pay"),),
                        (addr.clone(), TrancheType::Junior, pay),
                    );
                }
            }
        }

        env.events().publish(
            (symbol_short!("psum"),),
            (&caller, &amount, &senior_amount, &junior_amount),
        );
    }

    /// 应用损失（减记份额）
    pub fn apply_loss(env: Env, admin: Address, loss_amount: i128) {
        admin.require_auth();
        Self::require_admin(&env, &admin);

        if loss_amount <= 0 {
            return;
        }

        let mut jt: i128 = env
            .storage()
            .persistent()
            .get(&symbol_short!("j_total"))
            .unwrap_or(0i128);
        let mut st: i128 = env
            .storage()
            .persistent()
            .get(&symbol_short!("s_total"))
            .unwrap_or(0i128);
        let mut remaining = loss_amount;

        // Junior 先承担损失
        if jt >= remaining {
            let new_jt = jt.checked_sub(remaining).expect("underflow");
            let old_jt = jt;
            let junior_map: Map<Address, i128> = env
                .storage()
                .persistent()
                .get(&symbol_short!("j_map"))
                .unwrap_or(Map::new(&env));
            let mut new_map: Map<Address, i128> = Map::new(&env);
            
            if old_jt > 0 {
                for addr in junior_map.keys() {
                    let hold = junior_map.get(addr.clone()).unwrap_or(0i128);
                    let new_hold = hold.checked_mul(new_jt).expect("overflow") / old_jt;
                    if new_hold > 0 {
                        new_map.set(addr, new_hold);
                    }
                }
            }
            env.storage().persistent().set(&symbol_short!("j_map"), &new_map);
            env.storage().persistent().set(&symbol_short!("j_total"), &new_jt);
            remaining = 0;
        } else {
            remaining = remaining.checked_sub(jt).expect("underflow");
            let new_map: Map<Address, i128> = Map::new(&env);
            env.storage().persistent().set(&symbol_short!("j_map"), &new_map);
            env.storage().persistent().set(&symbol_short!("j_total"), &0i128);
        }

        // 剩余损失影响 Senior
        if remaining > 0 && st > 0 {
            let new_st = if remaining >= st {
                0i128
            } else {
                st.checked_sub(remaining).expect("underflow")
            };
            let old_st = st;
            let senior_map: Map<Address, i128> = env
                .storage()
                .persistent()
                .get(&symbol_short!("s_map"))
                .unwrap_or(Map::new(&env));
            let mut new_map: Map<Address, i128> = Map::new(&env);
            
            if new_st > 0 && old_st > 0 {
                for addr in senior_map.keys() {
                    let hold = senior_map.get(addr.clone()).unwrap_or(0i128);
                    let new_hold = hold.checked_mul(new_st).expect("overflow") / old_st;
                    if new_hold > 0 {
                        new_map.set(addr, new_hold);
                    }
                }
            }
            env.storage().persistent().set(&symbol_short!("s_map"), &new_map);
            env.storage().persistent().set(&symbol_short!("s_total"), &new_st);
        }

        env.events().publish((symbol_short!("loss"),), (&admin, &loss_amount));
    }

    /* 查询函数 */

    pub fn get_minimums(env: Env) -> (i128, i128) {
        let min_s: i128 = env
            .storage()
            .persistent()
            .get(&symbol_short!("min_sen"))
            .unwrap_or(0i128);
        let min_j: i128 = env
            .storage()
            .persistent()
            .get(&symbol_short!("min_jun"))
            .unwrap_or(0i128);
        (min_s, min_j)
    }

    pub fn get_totals(env: Env) -> (i128, i128) {
        let st: i128 = env
            .storage()
            .persistent()
            .get(&symbol_short!("s_total"))
            .unwrap_or(0i128);
        let jt: i128 = env
            .storage()
            .persistent()
            .get(&symbol_short!("j_total"))
            .unwrap_or(0i128);
        (st, jt)
    }

    pub fn get_user_share(env: Env, user: Address, tranche: TrancheType) -> i128 {
        match tranche {
            TrancheType::Senior => {
                let senior_map: Map<Address, i128> = env
                    .storage()
                    .persistent()
                    .get(&symbol_short!("s_map"))
                    .unwrap_or(Map::new(&env));
                senior_map.get(user).unwrap_or(0i128)
            }
            TrancheType::Junior => {
                let junior_map: Map<Address, i128> = env
                    .storage()
                    .persistent()
                    .get(&symbol_short!("j_map"))
                    .unwrap_or(Map::new(&env));
                junior_map.get(user).unwrap_or(0i128)
            }
        }
    }

    pub fn get_admin(env: Env) -> Address {
        env.storage()
            .persistent()
            .get(&symbol_short!("admin"))
            .expect("not initialized")
    }

    pub fn is_paused(env: Env) -> bool {
        env.storage()
            .persistent()
            .get(&symbol_short!("paused"))
            .unwrap_or(false)
    }

    /* 内部辅助函数 */

    fn require_admin(env: &Env, caller: &Address) {
        let admin: Address = env
            .storage()
            .persistent()
            .get(&symbol_short!("admin"))
            .expect("not initialized");
        if caller != &admin {
            panic!("only admin");
        }
    }

    fn require_not_paused(env: &Env) {
        let paused: bool = env
            .storage()
            .persistent()
            .get(&symbol_short!("paused"))
            .unwrap_or(false);
        if paused {
            panic!("contract is paused");
        }
    }
}