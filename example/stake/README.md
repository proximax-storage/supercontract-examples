### This contract allows participants to stake their funds and earn interest.
**How to use:**

Contract Creator

1. The contract creator calls deposit_interest() to initialize the staking contract.

Users

1. Users call join() for participate in the staking contract.
2. Users call withdraw() to withdraw their staked token. If the staking duration has been met, the user will receive the staked amount plus a 10% interest.

#### ```deposit_interest()```
The contract creator is required to deposit XPX into the Supercontract to fund the interest payouts for stakers. When calling this function, the caller must input the amount to deposit into the service payment field.
#### ```join()```
To participate in staking, the minimum stake is 100 XPX. The caller should input the amount of XPX they wish to stake in the service payment field and provide their address in the call parameter.
#### ```withdraw()```
To withdraw staked tokens, the user must call the withdraw function. If the staking duration has been met, the user will receive the staked amount plus a 10% interest. When calling this function, the user must input their address in the call parameter.


