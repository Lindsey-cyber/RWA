#!/bin/bash

# 颜色输出
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# 测试账户配置
ADMIN_PUBLIC_KEY=GAZ57ZNVBFTYPAR7EVW7LISVT5ZYU2FFHB7Q5YC74KDUXNILIVM7555Q
ADMIN_SECRET_KEY=SC566PQX2UK4X7JXEF7UU7PE5IYCDGPE6C6VQH26RHLYNFKEEAWGSHHV

SENIOR_PUBLIC_KEY=GCKL6GUTPTAKBEHJV27Y6UZLNB3HDLNPB4N3NPU6VWSWLMRETUT3BDQD
SENIOR_SECRET_KEY=SCYHVYFNAFCAI76WSF5THMSNBCX4PSDHF7IOEUB5JPR7JSHZFC4X52B4

JUNIOR_PUBLIC_KEY=GCD3FU576HQLD3NIY4AMH6XYHOHQUZIK2FXDTOZXP62ALNTW7RUMDOAM
JUNIOR_SECRET_KEY=SCC3TY6TAW4ACCRIRNSEXAQLR7DW4RLSJZS6PD2UI6XONGWFOLD62LTS

TOKEN_ID=CCA2BWGKIB7TU5VWHZSRDSGQPSIROSHGE4RUXOW4S6RMGU4DK5EXO7BN

# 网络配置 (使用 Testnet)
NETWORK="testnet"
RPC_URL="https://soroban-testnet.stellar.org"

echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}  Tranche 合约部署与测试脚本${NC}"
echo -e "${GREEN}========================================${NC}\n"

# 检查 soroban CLI 是否安装
if ! command -v soroban &> /dev/null; then
    echo -e "${RED}错误: soroban CLI 未安装${NC}"
    echo "请运行: cargo install --locked soroban-cli"
    exit 1
fi

echo -e "${YELLOW}步骤 1: 配置网络${NC}"
soroban network add \
  --global testnet \
  --rpc-url https://soroban-testnet.stellar.org \
  --network-passphrase "Test SDF Network ; September 2015"

echo -e "${GREEN}✓ 网络配置完成${NC}\n"

# 配置身份
echo -e "${YELLOW}步骤 2: 配置身份（使用固定测试账户）${NC}"
echo "SC566PQX2UK4X7JXEF7UU7PE5IYCDGPE6C6VQH26RHLYNFKEEAWGSHHV" | soroban keys add admin --secret-key 2>/dev/null || echo "admin 已存在"
echo "SCYHVYFNAFCAI76WSF5THMSNBCX4PSDHF7IOEUB5JPR7JSHZFC4X52B4" | soroban keys add senior --secret-key 2>/dev/null || echo "senior 已存在"
echo "SCC3TY6TAW4ACCRIRNSEXAQLR7DW4RLSJZS6PD2UI6XONGWFOLD62LTS" | soroban keys add junior --secret-key 2>/dev/null || echo "junior 已存在"
echo -e "  Admin:  GAZ57ZNVBFTYPAR7EVW7LISVT5ZYU2FFHB7Q5YC74KDUXNILIVM7555Q"
echo -e "  Senior: GCKL6GUTPTAKBEHJV27Y6UZLNB3HDLNPB4N3NPU6VWSWLMRETUT3BDQD"
echo -e "  Junior: GCD3FU576HQLD3NIY4AMH6XYHOHQUZIK2FXDTOZXP62ALNTW7RUMDOAM"
echo -e "${GREEN}✓ 身份配置完成${NC}\n"



# 查询余额
echo -e "${YELLOW}查询账户余额:${NC}"
ADMIN_BAL=$(soroban contract invoke --id $TOKEN_ID --source admin --network testnet -- balance --id $ADMIN_PUBLIC_KEY)
SENIOR_BAL=$(soroban contract invoke --id $TOKEN_ID --source admin --network testnet -- balance --id $SENIOR_PUBLIC_KEY)
JUNIOR_BAL=$(soroban contract invoke --id $TOKEN_ID --source admin --network testnet -- balance --id $JUNIOR_PUBLIC_KEY)

echo -e "Admin:  ${GREEN}$(echo "scale=2; $ADMIN_BAL / 10000000" | bc) USDC${NC}"
echo -e "Senior: ${GREEN}$(echo "scale=2; $SENIOR_BAL / 10000000" | bc) USDC${NC}"
echo -e "Junior: ${GREEN}$(echo "scale=2; $JUNIOR_BAL / 10000000" | bc) USDC${NC}\n"

# 编译 Tranche 合约
echo -e "${YELLOW}步骤 6: 编译 Tranche 合约${NC}"
cd tranche-contract # 假设你的合约在这个目录
soroban contract build

if [ ! -f "target/wasm32-unknown-unknown/release/tranche_contract.wasm" ]; then
    echo -e "${RED}错误: 合约编译失败${NC}"
    exit 1
fi

echo -e "${GREEN}✓ 合约编译完成${NC}\n"

# 部署 Tranche 合约
echo -e "${YELLOW}步骤 7: 部署 Tranche 合约${NC}"
TRANCHE_WASM=$(soroban contract install \
  --network testnet \
  --source admin \
  --wasm target/wasm32-unknown-unknown/release/tranche_contract.wasm 2>&1 | tail -1)

TRANCHE_ID=$(soroban contract deploy \
  --wasm-hash $TRANCHE_WASM \
  --source admin \
  --network testnet 2>&1 | tail -1)

echo -e "Tranche 合约地址: ${GREEN}$TRANCHE_ID${NC}\n"

# 初始化 Tranche 合约
echo -e "${YELLOW}步骤 8: 初始化 Tranche 合约${NC}"
echo "最低投资: Senior 1,000 USDC, Junior 500 USDC"

# 创建一个假的池子地址（这里用 Admin 地址代替）
POOL_ADDRESS=$ADMIN_PUBLIC_KEY

soroban contract invoke \
  --id $TRANCHE_ID \
  --source admin \
  --network testnet \
  -- initialize \
  --admin $ADMIN_PUBLIC_KEY \
  --token_contract_id $TOKEN_ID \
  --pool_contract_id $POOL_ADDRESS \
  --min_senior 10000000000 \
  --min_junior 5000000000

echo -e "${GREEN}✓ Tranche 合约初始化完成${NC}\n"

# 测试功能
echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}  开始测试合约功能${NC}"
echo -e "${GREEN}========================================${NC}\n"

# 1. 查询最低投资额
echo -e "${YELLOW}测试 1: 查询最低投资额${NC}"
MINIMUMS=$(soroban contract invoke --id $TRANCHE_ID --source admin --network testnet -- get_minimums)
echo -e "结果: ${GREEN}$MINIMUMS${NC}\n"

# 2. Senior 用户批准并认购
echo -e "${YELLOW}测试 2: Senior 用户认购 10,000 USDC${NC}"

# 先批准 Tranche 合约使用代币
soroban contract invoke \
  --id $TOKEN_ID \
  --source senior \
  --network testnet \
  -- approve \
  --from $SENIOR_PUBLIC_KEY \
  --spender $TRANCHE_ID \
  --amount 100000000000 \
  --expiration-ledger 999999

# 认购
soroban contract invoke \
  --id $TRANCHE_ID \
  --source senior \
  --network testnet \
  -- subscribe \
  --from $SENIOR_PUBLIC_KEY \
  --tranche '{"Senior":{}}' \
  --amount 100000000000

echo -e "${GREEN}✓ Senior 认购成功${NC}\n"

# 3. Junior 用户认购
echo -e "${YELLOW}测试 3: Junior 用户认购 5,000 USDC${NC}"

soroban contract invoke \
  --id $TOKEN_ID \
  --source junior \
  --network testnet \
  -- approve \
  --from $JUNIOR_PUBLIC_KEY \
  --spender $TRANCHE_ID \
  --amount 50000000000 \
  --expiration-ledger 999999

soroban contract invoke \
  --id $TRANCHE_ID \
  --source junior \
  --network testnet \
  -- subscribe \
  --from $JUNIOR_PUBLIC_KEY \
  --tranche '{"Junior":{}}' \
  --amount 50000000000

echo -e "${GREEN}✓ Junior 认购成功${NC}\n"

# 4. 查询总额
echo -e "${YELLOW}测试 4: 查询总投资额${NC}"
TOTALS=$(soroban contract invoke --id $TRANCHE_ID --source admin --network testnet -- get_totals)
echo -e "结果: ${GREEN}$TOTALS${NC}"
echo -e "Senior Total: 10,000 USDC, Junior Total: 5,000 USDC\n"

# 5. 查询用户份额
echo -e "${YELLOW}测试 5: 查询用户份额${NC}"
SENIOR_SHARE=$(soroban contract invoke --id $TRANCHE_ID --source admin --network testnet -- get_user_share --user $SENIOR_PUBLIC_KEY --tranche '{"Senior":{}}')
JUNIOR_SHARE=$(soroban contract invoke --id $TRANCHE_ID --source admin --network testnet -- get_user_share --user $JUNIOR_PUBLIC_KEY --tranche '{"Junior":{}}')
echo -e "Senior 用户份额: ${GREEN}$SENIOR_SHARE${NC}"
echo -e "Junior 用户份额: ${GREEN}$JUNIOR_SHARE${NC}\n"

# 6. 模拟收益分配
echo -e "${YELLOW}测试 6: Admin 分配 3,000 USDC 收益${NC}"

# Admin 先给合约转账（模拟池子收益）
soroban contract invoke \
  --id $TOKEN_ID \
  --source admin \
  --network testnet \
  -- transfer \
  --from $ADMIN_PUBLIC_KEY \
  --to $TRANCHE_ID \
  --amount 30000000000

# 执行分配
soroban contract invoke \
  --id $TRANCHE_ID \
  --source admin \
  --network testnet \
  -- notify_pool_payout \
  --caller $ADMIN_PUBLIC_KEY \
  --amount 30000000000

echo -e "${GREEN}✓ 收益分配完成${NC}\n"

# 7. 查询分配后的余额
echo -e "${YELLOW}测试 7: 查询分配后的余额${NC}"
SENIOR_BAL_AFTER=$(soroban contract invoke --id $TOKEN_ID --source admin --network testnet -- balance --id $SENIOR_PUBLIC_KEY)
JUNIOR_BAL_AFTER=$(soroban contract invoke --id $TOKEN_ID --source admin --network testnet -- balance --id $JUNIOR_PUBLIC_KEY)
echo -e "Senior 余额: ${GREEN}$(echo "scale=2; $SENIOR_BAL_AFTER / 10000000" | bc) USDC${NC}"
echo -e "Junior 余额: ${GREEN}$(echo "scale=2; $JUNIOR_BAL_AFTER / 10000000" | bc) USDC${NC}\n"

# 8. 测试赎回功能
echo -e "${YELLOW}测试 8: Senior 用户赎回 2,000 USDC${NC}"
soroban contract invoke \
  --id $TRANCHE_ID \
  --source senior \
  --network testnet \
  -- redeem \
  --from $SENIOR_PUBLIC_KEY \
  --tranche '{"Senior":{}}' \
  --amount 20000000000

SENIOR_SHARE_AFTER=$(soroban contract invoke --id $TRANCHE_ID --source admin --network testnet -- get_user_share --user $SENIOR_PUBLIC_KEY --tranche '{"Senior":{}}')
echo -e "赎回后 Senior 份额: ${GREEN}$SENIOR_SHARE_AFTER${NC} (应为 8,000 USDC)\n"

# 9. 测试损失应用
echo -e "${YELLOW}测试 9: Admin 应用 1,000 USDC 损失 (Junior 先承担)${NC}"
soroban contract invoke \
  --id $TRANCHE_ID \
  --source admin \
  --network testnet \
  -- apply_loss \
  --admin $ADMIN_PUBLIC_KEY \
  --loss_amount 10000000000

TOTALS_AFTER_LOSS=$(soroban contract invoke --id $TRANCHE_ID --source admin --network testnet -- get_totals)
echo -e "损失后总额: ${GREEN}$TOTALS_AFTER_LOSS${NC}\n"

# 10. 测试暂停功能
echo -e "${YELLOW}测试 10: 测试暂停/恢复功能${NC}"
soroban contract invoke \
  --id $TRANCHE_ID \
  --source admin \
  --network testnet \
  -- set_paused \
  --admin $ADMIN_PUBLIC_KEY \
  --paused true

IS_PAUSED=$(soroban contract invoke --id $TRANCHE_ID --source admin --network testnet -- is_paused)
echo -e "合约状态: ${GREEN}已暂停 = $IS_PAUSED${NC}"

# 尝试在暂停状态下认购（应该失败）
echo -e "${YELLOW}尝试在暂停状态下认购（预期失败）...${NC}"
soroban contract invoke \
  --id $TRANCHE_ID \
  --source junior \
  --network testnet \
  -- subscribe \
  --from $JUNIOR_PUBLIC_KEY \
  --tranche '{"Junior":{}}' \
  --amount 10000000000 2>&1 | grep -q "paused" && echo -e "${GREEN}✓ 正确拒绝了暂停期间的操作${NC}" || echo -e "${RED}✗ 应该拒绝操作${NC}"

# 恢复合约
soroban contract invoke \
  --id $TRANCHE_ID \
  --source admin \
  --network testnet \
  -- set_paused \
  --admin $ADMIN_PUBLIC_KEY \
  --paused false

echo -e "${GREEN}✓ 合约已恢复${NC}\n"

# 最终总结
echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}  测试完成总结${NC}"
echo -e "${GREEN}========================================${NC}"
echo -e "代币合约地址: ${GREEN}$TOKEN_ID${NC}"
echo -e "Tranche 合约地址: ${GREEN}$TRANCHE_ID${NC}"
echo -e "\n所有测试已完成！"
echo -e "\n你可以使用以下命令继续交互："
echo -e "${YELLOW}查看合约状态:${NC}"
echo -e "soroban contract invoke --id $TRANCHE_ID --source admin --network testnet -- get_totals"
echo -e "\n${YELLOW}查看用户份额:${NC}"
echo -e "soroban contract invoke --id $TRANCHE_ID --source admin --network testnet -- get_user_share --user $SENIOR_PUBLIC_KEY --tranche '{\"Senior\":{}}'"