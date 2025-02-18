### This contract allows participants to stake their funds and earn interest.
#### ```deposit_interest()```
The contract creator is required to deposit XPX into the Supercontract to fund the interest payouts for stakers. When calling this function, the caller must input the amount to deposit into the service payment field.
#### ```join()```
To participate in staking, the minimum stake is 100 XPX. The caller should input the amount of XPX they wish to stake in the service payment field and provide their address in the call parameter.
#### ```withdraw()```
To withdraw staked tokens, the user must call the withdraw function. If the staking duration has been met, the user will receive the staked amount plus a 10% interest. When calling this function, the user must input their address in the call parameter.


